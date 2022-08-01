pub mod position;
pub mod varint;

use bytes::{Buf, BufMut, Bytes, BytesMut};
use uuid::Uuid;

use crate::error::{ProtocolError, Res};

use self::varint::VarInt;

pub trait Serialize {
    fn serialize(&self, buf: &mut BytesMut);
    fn deserialize(buf: &mut Bytes) -> Res<Self>
    where
        Self: Sized;
}

impl Serialize for bool {
    fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u8(*self as u8);
    }

    fn deserialize(buf: &mut Bytes) -> Res<Self> {
        if !buf.has_remaining() {
            return Err(ProtocolError::UnexpectedEof);
        }

        Ok(buf.get_u8() != 0)
    }
}

impl Serialize for u8 {
    fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u8(*self);
    }

    fn deserialize(buf: &mut Bytes) -> Res<Self> {
        if !buf.has_remaining() {
            return Err(ProtocolError::UnexpectedEof);
        }

        Ok(buf.get_u8())
    }
}

impl Serialize for u16 {
    fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u16(*self);
    }

    fn deserialize(buf: &mut Bytes) -> Res<Self> {
        if buf.remaining() < 2 {
            return Err(ProtocolError::UnexpectedEof);
        }

        Ok(buf.get_u16())
    }
}

impl Serialize for u32 {
    fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u32(*self);
    }

    fn deserialize(buf: &mut Bytes) -> Res<Self> {
        if buf.remaining() < 4 {
            return Err(ProtocolError::UnexpectedEof);
        }

        Ok(buf.get_u32())
    }
}

impl Serialize for u64 {
    fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u64(*self);
    }

    fn deserialize(buf: &mut Bytes) -> Res<Self> {
        if buf.remaining() < 8 {
            return Err(ProtocolError::UnexpectedEof);
        }

        Ok(buf.get_u64())
    }
}

impl Serialize for i8 {
    fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i8(*self);
    }

    fn deserialize(buf: &mut Bytes) -> Res<Self> {
        if !buf.has_remaining() {
            return Err(ProtocolError::UnexpectedEof);
        }

        Ok(buf.get_i8())
    }
}

impl Serialize for i16 {
    fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i16(*self);
    }

    fn deserialize(buf: &mut Bytes) -> Res<Self> {
        if buf.remaining() < 2 {
            return Err(ProtocolError::UnexpectedEof);
        }

        Ok(buf.get_i16())
    }
}

impl Serialize for i32 {
    fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i32(*self);
    }

    fn deserialize(buf: &mut Bytes) -> Res<Self> {
        if buf.remaining() < 4 {
            return Err(ProtocolError::UnexpectedEof);
        }

        Ok(buf.get_i32())
    }
}

impl Serialize for i64 {
    fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i64(*self);
    }

    fn deserialize(buf: &mut Bytes) -> Res<Self> {
        if buf.remaining() < 8 {
            return Err(ProtocolError::UnexpectedEof);
        }

        Ok(buf.get_i64())
    }
}

impl Serialize for String {
    fn serialize(&self, buf: &mut BytesMut) {
        VarInt(self.len() as i32).serialize(buf);
        buf.copy_from_slice(self.as_bytes());
    }

    fn deserialize(buf: &mut Bytes) -> Res<Self> {
        let len = VarInt::deserialize(buf)?.0 as usize;

        if buf.remaining() < len {
            return Err(ProtocolError::UnexpectedEof);
        }

        let bytes = buf.slice(0..len).to_vec();

        String::from_utf8(bytes).map_err(Into::into)
    }
}

impl<T: Serialize> Serialize for Option<T> {
    fn serialize(&self, buf: &mut BytesMut) {
        self.is_some().serialize(buf);

        if self.is_some() {
            self.as_ref().unwrap().serialize(buf)
        }
    }

    fn deserialize(buf: &mut Bytes) -> Res<Self> {
        let present = bool::deserialize(buf)?;

        if present {
            Ok(Some(T::deserialize(buf)?))
        } else {
            Ok(None)
        }
    }
}

impl<T: Serialize> Serialize for Vec<T> {
    fn serialize(&self, buf: &mut BytesMut) {
        VarInt(self.len() as i32).serialize(buf);

        for x in self {
            x.serialize(buf);
        }
    }

    fn deserialize(buf: &mut Bytes) -> Res<Self> {
        let len = VarInt::deserialize(buf)?.0 as usize;

        let mut vec = Vec::with_capacity(len);

        for _ in 0..len {
            vec.push(T::deserialize(buf)?);
        }

        Ok(vec)
    }
}

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L: Serialize, R: Serialize> Serialize for Either<L, R> {
    fn serialize(&self, buf: &mut BytesMut) {
        match self {
            Either::Left(l) => {
                true.serialize(buf);
                l.serialize(buf)
            }
            Either::Right(r) => {
                false.serialize(buf);
                r.serialize(buf)
            }
        }
    }

    fn deserialize(buf: &mut Bytes) -> Res<Self>
    where
        Self: Sized,
    {
        let b = bool::deserialize(buf)?;

        if b {
            L::deserialize(buf).map(Self::Left)
        } else {
            R::deserialize(buf).map(Self::Right)
        }
    }
}

impl Serialize for Uuid {
    fn serialize(&self, buf: &mut BytesMut) {
        buf.put(&self.as_bytes()[..])
    }

    fn deserialize(buf: &mut Bytes) -> Res<Self> {
        if buf.remaining() < 16 {
            return Err(ProtocolError::UnexpectedEof);
        }

        Ok(Uuid::from_u128(buf.get_u128()))
    }
}
