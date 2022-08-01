use std::string::FromUtf8Error;

use crate::{Bound, State};

pub type Res<T> = Result<T, ProtocolError>;

#[derive(Debug)]
pub enum ProtocolError {
    UnexpectedEof,
    VarIntTooLong,
    InvalidEnumVariant(&'static str, isize),
    FromUtf8Error(FromUtf8Error),
    UnknownPacketId(Bound, State, i32),
    JsonError(serde_json::Error),
}

impl From<FromUtf8Error> for ProtocolError {
    fn from(e: FromUtf8Error) -> Self {
        Self::FromUtf8Error(e)
    }
}
