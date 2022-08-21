use std::string::{FromUtf16Error, FromUtf8Error};

use rsa::{errors::Error as RsaError, pkcs8::spki::Error as SpkiError};
use thiserror::Error;

use crate::{Bound, State};

pub(crate) type SerRes<T> = Result<T, SerializeError>;
pub(crate) type DeRes<T> = Result<T, DeserializeError>;

#[derive(Error, Debug)]
pub enum DeserializeError {
    #[error("Unexpected eof")]
    UnexpectedEof,
    #[error("VarInt too long")]
    VarIntTooLong,
    #[error("Invalid enum variant {1} for enum {0}")]
    InvalidEnumVariant(&'static str, isize),
    #[error("{0}")]
    FromUtf8Error(#[from] FromUtf8Error),
    #[error("{0}")]
    FromUtf16Error(#[from] FromUtf16Error),
    #[error("Unknown {0} {1} packet with id `0x{2:02x}`")]
    UnknownPacketId(Bound, State, i32),
    #[error("Broken packet")]
    BrokenPacket,
    #[error("{0}")]
    JsonError(serde_json::Error),
    #[error("{0}")]
    SpkiError(#[from] SpkiError),
    #[error("Invalid shared secret length")]
    InvalidSharedSecretLength,
    #[error("{0}")]
    RsaError(#[from] RsaError),
    #[error("{0}")]
    ZlibError(String),
    #[error("{0}")]
    NbtError(#[from] nbt::Error),
}

#[derive(Error, Debug)]
pub enum SerializeError {
    #[error("Expected RSA public key to have a size of 1024 bits, got {0} bits")]
    UnexpectedPublicKeySize(usize),
    #[error("Failed to encode RSA public key {0}")]
    SpkiError(#[from] SpkiError),
    #[error("{0}")]
    RsaError(#[from] RsaError),
    #[error("{0}")]
    NbtError(#[from] nbt::Error),
}
