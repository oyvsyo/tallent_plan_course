// #![deny(missing_docs)]
//! Module with key-value storage
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum KVSError {
    GeneralKVSError,
    KeyNotFoundError,
    IOError,
    SerdeJsonError,
}

impl Display for KVSError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            KVSError::KeyNotFoundError => write!(f, "Key not found"),
            KVSError::IOError => write!(f, "Imput output error"),
            KVSError::SerdeJsonError => write!(f, "Json serialization error"),
            KVSError::GeneralKVSError => write!(f, "Unknown error"),
        }
    }
}

impl Error for KVSError {
    fn description(&self) -> &str {
        "KVS error .. please get some help"
    }
}

impl From<std::io::Error> for KVSError {
    fn from(_err: std::io::Error) -> KVSError {
        KVSError::IOError
    }
}

impl From<serde_json::Error> for KVSError {
    fn from(_err: serde_json::Error) -> KVSError {
        KVSError::SerdeJsonError
    }
}

impl From<String> for KVSError {
    fn from(_err: String) -> KVSError {
        KVSError::GeneralKVSError
    }
}

pub type Result<T> = std::result::Result<T, KVSError>;
