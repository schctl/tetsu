//! Tools to encrypt packet data sent over the network.

use crate::errors::TetsuResult;

use openssl::encrypt::Encrypter;
use openssl::pkey::{PKey, Public};
pub use openssl::rand::rand_bytes;
use openssl::rsa::{Padding, Rsa};
pub use openssl::sha::Sha1;

use aes::Aes128;
pub use cfb8::cipher::{AsyncStreamCipher, NewCipher};
use cfb8::Cfb8;

pub type PublicKey = PKey<Public>;

/// Fills `key` with random bytes.
#[inline]
pub fn generate_key(key: &mut [u8]) {
    rand_bytes(key).unwrap();
}

/// Return a PublicKey object from a DER encoded RSA key.
#[inline]
pub fn pkey_from_der(key: &[u8]) -> TetsuResult<PublicKey> {
    Ok(PKey::from_rsa(Rsa::public_key_from_der(key)?)?)
}

/// Default Minecraft stream cipher. Uses AES/CFB8.
pub struct DefaultStreamCipher<const KEY_LEN: usize> {
    /// Internal CFB8 cipher.
    cipher: Cfb8<Aes128>,
}

impl<const KEY_LEN: usize> DefaultStreamCipher<KEY_LEN> {
    /// Constructs a new stream cipher
    #[inline]
    pub fn new(key: &[u8; KEY_LEN]) -> TetsuResult<Self> {
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

/// Wrapper around an RSA public key.
pub struct RsaEncrypter<'a> {
    /// Internal RSA encryptor.
    encrypter: Encrypter<'a>,
}

impl<'a> RsaEncrypter<'a> {
    /// Returns a new RSA encryptor from a Public key.
    #[inline]
    pub fn new(key: &'a PublicKey) -> TetsuResult<Self> {
        let mut encrypter = Encrypter::new(&key)?;
        encrypter.set_rsa_padding(Padding::PKCS1)?;
        Ok(Self { encrypter })
    }

    /// Encrypt a buffer with an RSA key.
    pub fn encrypt(&self, buf: &[u8]) -> TetsuResult<Vec<u8>> {
        // Create an output buffer
        let _buffer_len = self.encrypter.encrypt_len(&buf)?;
        let mut encrypted_buf = vec![0; _buffer_len];
        // Encrypt and truncate the buffer
        let _encrypted_len = self.encrypter.encrypt(&buf, &mut encrypted_buf)?;
        encrypted_buf.truncate(_encrypted_len);
        Ok(encrypted_buf)
    }
}

/// Return a string of hex characters of a Sha1 hash.
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
