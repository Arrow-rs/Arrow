pub mod chat;
pub mod error;
pub mod handshake;
pub mod login;
pub(crate) mod macros;
pub mod play;
pub mod status;
pub mod types;

use std::io::{self, Read, Write};

use bytes::{BufMut, Bytes, BytesMut};
use error::{ProtocolError, Res};
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use handshake::Handshake;
use login::Login;
use play::Play;
use status::Status;
use types::{varint::VarInt, Serialize};

pub enum Protocol {
    Handshake(Handshake),
    Status(Status),
    Login(Login),
    Play(Play),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PacketCompression {
    pub enabled: bool,
    pub threshold: usize,
}

impl Protocol {
    pub fn serialize(&self, compression: PacketCompression) -> Vec<u8> {
        let (id, data) = match self {
            Protocol::Handshake(handshake) => handshake.serialize(),
            Protocol::Status(status) => status.serialize(),
            Protocol::Login(login) => login.serialize(),
            Protocol::Play(play) => play.serialize(),
        };

        let mut packet = BytesMut::new();

        let mut buf = BytesMut::new();

        VarInt(id).serialize(&mut buf);
        buf.extend_from_slice(&data);

        if compression.enabled && compression.threshold != 0 {
            if buf.len() >= compression.threshold {
                let data_len = VarInt(buf.len() as i32);

                let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
                encoder.write_all(&buf).unwrap();
                let compressed = encoder.finish().unwrap();

                VarInt((data_len.len() + compressed.len()) as i32).serialize(&mut packet);
                data_len.serialize(&mut packet);
                compressed.serialize(&mut packet);
            } else {
                VarInt(buf.len() as i32 + 1).serialize(&mut packet);
                packet.put_u8(0);
                packet.put(buf);
            }
        } else {
            VarInt(buf.len() as i32).serialize(&mut packet);
            packet.put(buf);
        }

        packet.to_vec()
    }

    pub fn deserialize(
        bound: Bound,
        state: State,
        compression: PacketCompression,
        bytes: Bytes,
    ) -> Res<Self> {
        let mut bytes = bytes;

        if compression.enabled {
            let data_len = VarInt::deserialize(&mut bytes)?.0 as usize;

            let mut decoder = ZlibDecoder::new(&bytes[..]);
            let mut vec = Vec::with_capacity(data_len);

            if let Err(e) = decoder.read_to_end(&mut vec) {
                match e.kind() {
                    io::ErrorKind::UnexpectedEof => return Err(ProtocolError::UnexpectedEof),
                    _ => unreachable!(),
                }
            }

            bytes = Bytes::from(vec.into_boxed_slice());
        }

        let id = VarInt::deserialize(&mut bytes)?.0;

        match state {
            State::Handshake => match (bound, id) {
                (Bound::Serverbound, 0) => Ok(Self::Handshake(Handshake::deserialize(&mut bytes)?)),
                (_, _) => Err(ProtocolError::UnknownPacketId(bound, state, id)),
            },
            State::Status => Status::deserialize(bound, id, &mut bytes).map(Self::Status),
            State::Login => Login::deserialize(bound, id, &mut bytes).map(Self::Login),
            State::Play => Play::deserialize(bound, id, &mut bytes).map(Self::Play),
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
