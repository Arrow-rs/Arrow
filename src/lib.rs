pub mod chat;
pub mod error;
pub mod handshake;
pub mod legacy;
pub mod login;
pub(crate) mod macros;
pub mod play;
pub mod status;
pub mod types;

use std::{
    fmt,
    io::{self, Read, Write},
};

use aes::cipher::{BlockDecryptMut, BlockEncryptMut};
use bytes::{BufMut, Bytes, BytesMut};
use error::{DeRes, DeserializeError, SerRes};
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use handshake::Handshake;
use login::Login;
use play::Play;
use status::Status;
use types::{
    varint::{read_encrypted_varint, VarInt},
    Serialize,
};

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

pub type Encryptor = cfb8::Encryptor<aes::Aes128>;
pub type Decryptor = cfb8::Decryptor<aes::Aes128>;

impl Protocol {
    pub fn serialize(
        &self,
        compression: PacketCompression,
        encryptor: Option<&mut Encryptor>,
    ) -> SerRes<Vec<u8>> {
        let (id, data) = match self {
            Protocol::Handshake(handshake) => handshake.serialize()?,
            Protocol::Status(status) => status.serialize()?,
            Protocol::Login(login) => login.serialize()?,
            Protocol::Play(play) => play.serialize()?,
        };

        let mut packet = BytesMut::new();

        let mut buf = BytesMut::new();

        VarInt(id).serialize(&mut buf)?;
        buf.extend_from_slice(&data);

        if compression.enabled && compression.threshold != 0 {
            if buf.len() >= compression.threshold {
                let data_len = VarInt(buf.len() as i32);

                let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
                encoder.write_all(&buf).unwrap();
                let compressed = encoder.finish().unwrap();

                VarInt((data_len.len() + compressed.len()) as i32).serialize(&mut packet)?;
                data_len.serialize(&mut packet)?;
                packet.put_slice(&compressed);
            } else {
                VarInt(buf.len() as i32 + 1).serialize(&mut packet)?;
                packet.put_u8(0);
                packet.put(buf);
            }
        } else {
            VarInt(buf.len() as i32).serialize(&mut packet)?;
            packet.put(buf);
        }

        let mut packet: Vec<_> = packet.into_iter().map(|b| [b].into()).collect();
        println!("{packet:?}");

        if let Some(encryptor) = encryptor {
            encryptor.encrypt_blocks_mut(&mut packet);
        }

        Ok(packet.into_iter().flatten().collect())
    }

    pub fn deserialize(
        bound: Bound,
        state: State,
        compression: PacketCompression,
        decryptor: Option<&mut Decryptor>,
        packet: &mut Bytes,
    ) -> DeRes<Self> {
        let mut bytes = if let Some(decryptor) = decryptor {
            let len = read_encrypted_varint(packet, decryptor)? as usize;
            dbg!(len);

            let packet = packet.split_to(len);

            let mut buf: Vec<_> = packet.iter().map(|b| [*b].into()).collect();

            decryptor.decrypt_blocks_mut(&mut buf);

            Bytes::from_iter(buf.into_iter().flatten())
        } else {
            let len = VarInt::deserialize(packet)?.0 as usize;

            packet.split_to(len)
        };

        if compression.enabled {
            let data_len = VarInt::deserialize(&mut bytes)?.0 as usize;

            dbg!(data_len);
            if data_len != 0 {
                let compressed = &bytes[..];
                let mut decoder = ZlibDecoder::new(compressed);
                let mut vec = Vec::with_capacity(data_len);

                if let Err(e) = decoder.read_to_end(&mut vec) {
                    match e.kind() {
                        io::ErrorKind::UnexpectedEof => {
                            return Err(DeserializeError::UnexpectedEof)
                        }
                        io::ErrorKind::InvalidInput => {
                            return Err(DeserializeError::ZlibError(e.to_string()))
                        }
                        _ => unreachable!(),
                    }
                }

                bytes = Bytes::from(vec.into_boxed_slice());
            }
        }

        let id = VarInt::deserialize(&mut bytes)?.0;

        match state {
            State::Handshake => match (bound, id) {
                (Bound::Serverbound, 0) => Ok(Self::Handshake(Handshake::deserialize(&mut bytes)?)),
                (_, _) => Err(DeserializeError::UnknownPacketId(bound, state, id)),
            },
            State::Status => Status::deserialize(bound, id, &mut bytes).map(Self::Status),
            State::Login => Login::deserialize(bound, id, &mut bytes).map(Self::Login),
            State::Play => Play::deserialize(bound, id, &mut bytes).map(Self::Play),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bound {
    Serverbound,
    Clientbound,
}

impl fmt::Display for Bound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Serverbound => f.write_str("serverbound"),
            Self::Clientbound => f.write_str("clientbound"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Handshake,
    Status,
    Login,
    Play,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            State::Handshake => "handshake",
            State::Status => "status",
            State::Login => "login",
            State::Play => "play",
        };

        f.write_str(s)
    }
}
