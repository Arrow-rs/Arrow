use aes::cipher::BlockDecryptMut;
use bytes::{Buf, BufMut, BytesMut};

use crate::{
    error::{DeRes, DeserializeError, SerRes},
    Decryptor,
};

use super::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VarInt(pub i32);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VarLong(pub i64);

impl VarInt {
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        const MIN_1B: i32 = 0;
        const MAX_1B: i32 = 2i32.pow(7) - 1;
        const MIN_2B: i32 = 2i32.pow(7);
        const MAX_2B: i32 = 2i32.pow(2 * 7) - 1;
        const MIN_3B: i32 = 2i32.pow(2 * 7);
        const MAX_3B: i32 = 2i32.pow(3 * 7) - 1;
        const MIN_4B: i32 = 2i32.pow(3 * 7);
        const MAX_4B: i32 = 2i32.pow(4 * 7) - 1;

        match self.0 {
            MIN_1B..=MAX_1B => 1,
            MIN_2B..=MAX_2B => 2,
            MIN_3B..=MAX_3B => 3,
            MIN_4B..=MAX_4B => 4,
            _ => 5,
        }
    }
}

pub(crate) fn read_encrypted_varint(buf: &mut BytesMut, decryptor: &mut Decryptor) -> DeRes<i32> {
    let mut value = 0;
    let mut len = 0;

    while {
        if buf.remaining() == 0 {
            return Err(DeserializeError::UnexpectedEof);
        }
        let b = &mut [0];
        decryptor.decrypt_block_b2b_mut(&[buf.get_u8()].into(), b.into());
        let b = b[0] as u32;
        value |= (b & 0x7f) << (len * 7);

        len += 1;

        if len > 5 {
            return Err(DeserializeError::VarIntTooLong);
        };

        (b & 0x80) != 0
    } {}

    Ok(value as i32)
}

impl Serialize for VarInt {
    fn serialize(&self, buf: &mut BytesMut) -> SerRes<()> {
        let mut value = self.0 as u32;

        loop {
            if (value & !0x7F) == 0 {
                buf.put_u8(value as u8);
                break;
            }

            buf.put_u8((value as u8 & 0x7F) | 0x80);
            value >>= 7;
        }

        Ok(())
    }

    fn deserialize(buf: &mut BytesMut) -> DeRes<Self> {
        let mut value = 0;
        let mut len = 0;

        while {
            if buf.remaining() == 0 {
                return Err(DeserializeError::UnexpectedEof);
            }
            let b = buf.get_u8() as u32;
            value |= (b & 0x7f) << (len * 7);

            len += 1;

            if len > 5 {
                return Err(DeserializeError::VarIntTooLong);
            };

            (b & 0x80) != 0
        } {}

        Ok(Self(value as i32))
    }
}

impl Serialize for VarLong {
    fn serialize(&self, buf: &mut BytesMut) -> SerRes<()> {
        let mut value = self.0 as u64;

        loop {
            if (value & !0x7F) == 0 {
                buf.put_u8(value as u8);
                break;
            }

            buf.put_u8((value as u8 & 0x7F) | 0x80);
            value >>= 7;
        }

        Ok(())
    }

    fn deserialize(buf: &mut BytesMut) -> DeRes<Self> {
        let mut value = 0;
        let mut len = 0;

        while {
            if buf.remaining() == 0 {
                return Err(DeserializeError::UnexpectedEof);
            }
            let b = buf.get_u8() as u64;
            value |= (b & 0x7f) << (len * 7);

            len += 1;

            if len > 10 {
                return Err(DeserializeError::VarIntTooLong);
            };

            (b & 0x80) != 0
        } {}

        Ok(Self(value as i64))
    }
}

impl From<i32> for VarInt {
    fn from(i: i32) -> Self {
        Self(i)
    }
}

impl From<VarInt> for i32 {
    fn from(varint: VarInt) -> i32 {
        varint.0
    }
}

impl From<i64> for VarLong {
    fn from(i: i64) -> Self {
        Self(i)
    }
}

impl From<VarLong> for i64 {
    fn from(varlong: VarLong) -> i64 {
        varlong.0
    }
}
