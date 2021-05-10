//! Encryption for a client-server connection.

use crate::errors::TetsuResult;

pub use openssl::pkey::{Private, Public};
pub use openssl::rand::rand_bytes;
use openssl::rsa::Padding;
pub use openssl::rsa::Rsa;
pub use openssl::sha::Sha1;

use aes::Aes128;
pub use cfb8::cipher::{AsyncStreamCipher, NewCipher};
use cfb8::Cfb8;

/// Fills `key` with random bytes.
#[inline]
pub fn generate_key(key: &mut [u8]) {
    rand_bytes(key).unwrap();
}

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
