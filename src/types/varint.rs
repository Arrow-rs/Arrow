use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::error::{ProtocolError, Res};

use super::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VarInt(pub i32);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VarLong(pub i64);

impl Serialize for VarInt {
    fn serialize(&self, buf: &mut BytesMut) {
        let mut value = self.0 as u32;

        loop {
            if (value & !0x7F) == 0 {
                buf.put_u8(value as u8);
                break;
            }

            buf.put_u8((value as u8 & 0x7F) | 0x80);
            value >>= 7;
        }
    }

    fn deserialize(buf: &mut Bytes) -> Res<Self> {
        let mut value = 0;
        let mut len = 0;

        while {
            if buf.remaining() == 0 {
                return Err(ProtocolError::UnexpectedEof);
            }
            let b = buf.get_u8() as u32;
            value |= (b & 0x7f) << (len * 7);

            len += 1;

            if len > 5 {
                return Err(ProtocolError::VarIntTooLong);
            };

            (b & 0x80) != 0
        } {}

        Ok(Self(value as i32))
    }
}

impl Serialize for VarLong {
    fn serialize(&self, buf: &mut BytesMut) {
        let mut value = self.0 as u64;

        loop {
            if (value & !0x7F) == 0 {
                buf.put_u8(value as u8);
                break;
            }

            buf.put_u8((value as u8 & 0x7F) | 0x80);
            value >>= 7;
        }
    }

    fn deserialize(buf: &mut Bytes) -> Res<Self> {
        let mut value = 0;
        let mut len = 0;

        while {
            if buf.remaining() == 0 {
                return Err(ProtocolError::UnexpectedEof);
            }
            let b = buf.get_u8() as u64;
            value |= (b & 0x7f) << (len * 7);

            len += 1;

            if len > 10 {
                return Err(ProtocolError::VarIntTooLong);
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
