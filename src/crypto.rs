//! Encryption for a client-server connection.

use crate::errors::TetsuResult;

use std::io;
pub use std::net::SocketAddr;
use std::net::TcpStream;

pub use openssl::pkey::{Private, Public};
pub use openssl::rand::rand_bytes;
use openssl::rsa::Padding;
pub use openssl::rsa::Rsa;
pub use openssl::sha::Sha1;

use aes::Aes128;
use cfb8::cipher::{AsyncStreamCipher, NewCipher};
use cfb8::Cfb8;

/// Encrypt some data with an RSA public key.
pub fn public_encrypt(key: &Rsa<Public>, data: &[u8]) -> TetsuResult<Vec<u8>> {
    let mut decrypted = vec![0; data.len()];
    let len = key.public_encrypt(&data, &mut decrypted, Padding::PKCS1)?;
    Ok(decrypted[..len].to_vec())
}

/// Encrypt some data with an RSA private key.
pub fn private_encrypt(key: &Rsa<Private>, data: &[u8]) -> TetsuResult<Vec<u8>> {
    let mut decrypted = vec![0; data.len()];
    let len = key.private_decrypt(&data, &mut decrypted, Padding::PKCS1)?;
    Ok(decrypted[..len].to_vec())
}

/// Decrypt some data with an RSA private key.
pub fn private_decrypt(key: &Rsa<Private>, data: &[u8]) -> TetsuResult<Vec<u8>> {
    let mut decrypted = vec![0; data.len()];
    let len = key.private_encrypt(&data, &mut decrypted, Padding::PKCS1)?;
    Ok(decrypted[..len].to_vec())
}

/// Default Minecraft stream cipher. Uses AES/CFB8.
pub struct DefaultStreamCipher {
    /// Internal CFB8 cipher.
    cipher: Cfb8<Aes128>,
}

impl DefaultStreamCipher {
    /// Constructs a new stream cipher
    #[inline]
    pub fn new(key: &[u8]) -> TetsuResult<Self> {
        Ok(Self {
            cipher: Cfb8::new_from_slices(key, key)?,
        })
    }

    /// Decrypt data using the internal cipher.
    #[inline]
    pub fn decrypt(&mut self, data: &mut [u8]) {
        self.cipher.decrypt(data)
    }

    /// Encrypt data using the internal cipher.
    #[inline]
    pub fn encrypt(&mut self, data: &mut [u8]) {
        self.cipher.encrypt(data)
    }
}

/// Return a string of hex characters of a [`Sha1`] hash.
/// Based on Minecraft's implementation:
/// From https://wiki.vg/Protocol_Encryption#Authentication:
/// > Note that the Sha1.hexdigest() method used by minecraft is non standard.
/// > It doesn't match the digest method found in most programming languages
/// > and libraries. It works by treating the sha1 output bytes as one large
/// > integer in two's complement and then printing the integer in base 16,
/// > placing a minus sign if the interpreted number is negative.
pub fn hexdigest(hasher: Sha1) -> String {
    let mut hash = hasher.finish();

    let negative = (hash[0] & 0x80) == 0x80;

    // Treat hash as a number and calculate 2's complement
    if negative {
        let mut carry = true;
        for i in (0..hash.len()).rev() {
            hash[i] = !hash[i];
            if carry {
                carry = hash[i] == 0xFF;
                hash[i] = hash[i].wrapping_add(1);
            }
        }
    }

    let hash_str = hash
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join("");

    if negative {
        "-".to_owned() + hash_str.trim_matches('0')
    } else {
        hash_str.trim_matches('0').to_owned()
    }
}

/// Encrypted wrapper around a [`TcpStream`].
pub struct EncryptedTcpStream {
    /// TcpStream to read from.
    stream: TcpStream,
    /// Cipher algorithm.
    cipher: Option<DefaultStreamCipher>,
}

impl EncryptedTcpStream {
    /// Create a new TCP connection to the `address`.
    #[inline]
    pub fn connect(address: &str, cipher: Option<&[u8]>) -> TetsuResult<Self> {
        Ok(Self {
            stream: TcpStream::connect(address).unwrap(),
            cipher: match cipher {
                Some(key) => Some(DefaultStreamCipher::new(key)?),
                _ => None,
            },
        })
    }

    /// Set the key to encrypt with.
    #[inline]
    pub fn set_cipher(&mut self, key: &[u8]) -> TetsuResult<()> {
        self.cipher = Some(DefaultStreamCipher::new(key)?);
        Ok(())
    }

    /// Get the current connected address.
    #[inline]
    pub fn get_address(&self) -> SocketAddr {
        self.stream.peer_addr().unwrap()
    }
}

impl io::Read for EncryptedTcpStream {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match &mut self.cipher {
            None => self.stream.read(buf),
            Some(cipher) => {
                let read = self.stream.read(buf)?;
                cipher.decrypt(&mut buf[..read]);
                Ok(read)
            }
        }
    }
}

impl io::Write for EncryptedTcpStream {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match &mut self.cipher {
            None => self.stream.write(buf),
            Some(cipher) => {
                let mut data = buf.to_owned();
                cipher.encrypt(&mut data);
                self.stream.write_all(&data).unwrap();
                Ok(data.len())
            }
        }
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.stream.flush()
    }
}
