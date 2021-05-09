//! All possible errors.

use std::io;
use std::string;
use std::sync::{MutexGuard, PoisonError};

use cfb8::cipher::errors::InvalidLength;
use nbt::Error as nbt_error;
use openssl::error::ErrorStack;
use serde_json::Error as serde_error;

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

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    FromUtf8Error(string::FromUtf8Error),
    Serde(serde_error),
    Nbt(nbt_error),
    SSLErrorStack(ErrorStack),
    InvalidKeyLen(InvalidLength),
    InvalidValue(InvalidValue),
}

impl From<io::Error> for Error {
    fn from(item: io::Error) -> Self {
        Self::Io(item)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(item: string::FromUtf8Error) -> Self {
        Self::FromUtf8Error(item)
    }
}

impl From<serde_error> for Error {
    fn from(item: serde_error) -> Self {
        Self::Serde(item)
    }
}

impl From<nbt_error> for Error {
    fn from(item: nbt_error) -> Self {
        Self::Nbt(item)
    }
}

impl From<ErrorStack> for Error {
    fn from(item: ErrorStack) -> Self {
        Self::SSLErrorStack(item)
    }
}

impl From<InvalidLength> for Error {
    fn from(item: InvalidLength) -> Self {
        Self::InvalidKeyLen(item)
    }
}

impl From<InvalidValue> for Error {
    fn from(item: InvalidValue) -> Self {
        Self::InvalidValue(item)
    }
}

/// Error while reading/writing from a connection.
#[derive(Debug)]
pub enum ConnectionError<'a, T> {
    LockError(PoisonError<MutexGuard<'a, T>>),
    Error(Error),
}

impl<'a, T> From<PoisonError<MutexGuard<'a, T>>> for ConnectionError<'a, T> {
    fn from(item: PoisonError<MutexGuard<'a, T>>) -> Self {
        Self::LockError(item)
    }
}

impl<'a, T> From<Error> for ConnectionError<'a, T> {
    fn from(item: Error) -> Self {
        Self::Error(item)
    }
}

pub type TetsuResult<T> = Result<T, Error>;
