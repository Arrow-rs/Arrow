use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::{
    error::{DeRes, DeserializeError, SerRes},
    types::Serialize,
};

pub struct LegacyRequest;
pub struct LegacyResponse {
    protocol_version: i32,
    minecraft_version: String,
    motd: String,
    player_count: i32,
    max_players: i32,
}

impl LegacyRequest {
    pub fn serialize(&self) -> SerRes<Vec<u8>> {
        Ok(vec![0xfe, 0x01, 0xfa])
    }

    pub fn deserialize(b: &mut Bytes) -> DeRes<Self> {
        if b.remaining() < 3 {
            Err(DeserializeError::UnexpectedEof)
        } else if b[0..3] == [0xfe, 0x01, 0xfa] {
            Ok(Self)
        } else {
            Err(DeserializeError::BrokenPacket)
        }
    }
}

impl LegacyResponse {
    pub fn serialize(&self) -> SerRes<Vec<u8>> {
        let mut buf = BytesMut::new();
        let data = format!(
            "§1\x00{}\x00{}\x00{}\x00{}\x00{}",
            self.protocol_version,
            self.minecraft_version,
            self.motd,
            self.player_count,
            self.max_players
        );

        buf.put_u16(data.len() as u16);
        buf.put_slice(
            &data
                .encode_utf16()
                .flat_map(|i| i.to_be_bytes())
                .collect::<Vec<_>>(),
        );

        Ok(buf.to_vec())
    }

    pub fn deserialize(b: &mut Bytes) -> DeRes<Self> {
        if !b.has_remaining() {
            return Err(DeserializeError::UnexpectedEof);
        }

        if b[0] != 0xff {
            return Err(DeserializeError::BrokenPacket);
        }

        let len = u16::deserialize(b)? as usize;

        if b.remaining() < len * 2 {
            return Err(DeserializeError::UnexpectedEof);
        }

        let data: Vec<u16> = b[..len * 2]
            .chunks(2)
            .map(|a| u16::from_be_bytes(a.try_into().unwrap()))
            .collect();
        let string = String::from_utf16(&data)?;

        let mut values = string.split('\x00');
        values.next().ok_or(DeserializeError::BrokenPacket)?;
        let protocol_version: i32 = values
            .next()
            .ok_or(DeserializeError::BrokenPacket)?
            .parse()
            .map_err(|_| DeserializeError::BrokenPacket)?;
        let minecraft_version = values
            .next()
            .ok_or(DeserializeError::BrokenPacket)?
            .to_string();
        let motd = values
            .next()
            .ok_or(DeserializeError::BrokenPacket)?
            .to_string();
        let player_count: i32 = values
            .next()
            .ok_or(DeserializeError::BrokenPacket)?
            .parse()
            .map_err(|_| DeserializeError::BrokenPacket)?;
        let max_players: i32 = values
            .next()
            .ok_or(DeserializeError::BrokenPacket)?
            .parse()
            .map_err(|_| DeserializeError::BrokenPacket)?;

        Ok(Self {
            protocol_version,
            minecraft_version,
            motd,
            player_count,
            max_players,
        })
    }
}
