use std::io;
use std::string;
// use std::sync::PoisonError;

use cfb8::cipher::errors::InvalidLength;
use nbt::Error as nbt_error;
use openssl::error::ErrorStack;
use serde_json::Error as serde_error;

use quick_error::quick_error;

// TODO: impl PoisonError here.

#[derive(Debug)]
pub struct InvalidValue {
    pub expected: String,
}

impl std::error::Error for InvalidValue {}

impl std::fmt::Display for InvalidValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid value received. Expected: {}", self.expected)
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io {
            from(io::Error)
        }
        FromUtf8 {
            from(string::FromUtf8Error)
        }
        Serde {
            from(serde_error)
        }
        Nbt {
            from(nbt_error)
        }
        OpenSSLErrorStack {
            from(ErrorStack)
        }
        InvalidKeyLen {
            from(InvalidLength)
        }
        InvalidValue {
            from(InvalidValue)
        }
    }
}

pub type TetsuResult<T> = Result<T, Error>;
