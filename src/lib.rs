pub mod chat;
pub mod error;
pub mod handshake;
pub mod login;
pub(crate) mod macros;
pub mod play;
pub mod status;
pub mod types;

use bytes::Bytes;
use error::{ProtocolError, Res};
use handshake::Handshake;
use status::Status;

pub enum Protocol {
    Handshake(Handshake),
    Status(Status),
    Login(),
    Play(),
}

impl Protocol {
    pub fn serialize(&self) -> Vec<u8> {
        match self {
            Protocol::Handshake(handshake) => handshake.serialize(),
            Protocol::Status(status) => status.serialize(),
            Protocol::Login() => todo!(),
            Protocol::Play() => todo!(),
        }
    }

    pub fn deserialize(bound: Bound, state: State, id: i32, bytes: &mut Bytes) -> Res<Self> {
        match state {
            State::Handshake => match (bound, id) {
                (Bound::Serverbound, 0) => Ok(Self::Handshake(Handshake::deserialize(bytes)?)),
                (_, _) => Err(ProtocolError::UnknownPacketId(bound, state, id)),
            },
            State::Status => Ok(Self::Handshake(Handshake::deserialize(bytes)?)),
            State::Login => todo!(),
            State::Play => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bound {
    Serverbound,
    Clientbound,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Handshake,
    Status,
    Login,
    Play,
}
