pub mod position;
pub mod slot;
pub mod varint;

use bytes::{Buf, BufMut, BytesMut};
use rsa::{
    pkcs8::{DecodePublicKey, EncodePublicKey},
    RsaPublicKey,
};
use uuid::Uuid;

use crate::error::{DeRes, DeserializeError, SerRes};

use self::varint::VarInt;

pub trait Serialize {
    fn serialize(&self, buf: &mut BytesMut) -> SerRes<()>;
    fn deserialize(buf: &mut BytesMut) -> DeRes<Self>
    where
        Self: Sized;
}

impl Serialize for bool {
    fn serialize(&self, buf: &mut BytesMut) -> SerRes<()> {
        buf.put_u8(*self as u8);
        Ok(())
    }

    fn deserialize(buf: &mut BytesMut) -> DeRes<Self> {
        if !buf.has_remaining() {
            return Err(DeserializeError::UnexpectedEof);
        }

        Ok(buf.get_u8() != 0)
    }
}

impl Serialize for u8 {
    fn serialize(&self, buf: &mut BytesMut) -> SerRes<()> {
        buf.put_u8(*self);
        Ok(())
    }

    fn deserialize(buf: &mut BytesMut) -> DeRes<Self> {
        if !buf.has_remaining() {
            return Err(DeserializeError::UnexpectedEof);
        }

        Ok(buf.get_u8())
    }
}

impl Serialize for u16 {
    fn serialize(&self, buf: &mut BytesMut) -> SerRes<()> {
        buf.put_u16(*self);
        Ok(())
    }

    fn deserialize(buf: &mut BytesMut) -> DeRes<Self> {
        if buf.remaining() < 2 {
            return Err(DeserializeError::UnexpectedEof);
        }

        Ok(buf.get_u16())
    }
}

impl Serialize for u32 {
    fn serialize(&self, buf: &mut BytesMut) -> SerRes<()> {
        buf.put_u32(*self);
        Ok(())
    }

    fn deserialize(buf: &mut BytesMut) -> DeRes<Self> {
        if buf.remaining() < 4 {
            return Err(DeserializeError::UnexpectedEof);
        }

        Ok(buf.get_u32())
    }
}

impl Serialize for u64 {
    fn serialize(&self, buf: &mut BytesMut) -> SerRes<()> {
        buf.put_u64(*self);
        Ok(())
    }

    fn deserialize(buf: &mut BytesMut) -> DeRes<Self> {
        if buf.remaining() < 8 {
            return Err(DeserializeError::UnexpectedEof);
        }

        Ok(buf.get_u64())
    }
}

impl Serialize for i8 {
    fn serialize(&self, buf: &mut BytesMut) -> SerRes<()> {
        buf.put_i8(*self);
        Ok(())
    }

    fn deserialize(buf: &mut BytesMut) -> DeRes<Self> {
        if !buf.has_remaining() {
            return Err(DeserializeError::UnexpectedEof);
        }

        Ok(buf.get_i8())
    }
}

impl Serialize for i16 {
    fn serialize(&self, buf: &mut BytesMut) -> SerRes<()> {
        buf.put_i16(*self);
        Ok(())
    }

    fn deserialize(buf: &mut BytesMut) -> DeRes<Self> {
        if buf.remaining() < 2 {
            return Err(DeserializeError::UnexpectedEof);
        }

        Ok(buf.get_i16())
    }
}

impl Serialize for i32 {
    fn serialize(&self, buf: &mut BytesMut) -> SerRes<()> {
        buf.put_i32(*self);
        Ok(())
    }

    fn deserialize(buf: &mut BytesMut) -> DeRes<Self> {
        if buf.remaining() < 4 {
            return Err(DeserializeError::UnexpectedEof);
        }

        Ok(buf.get_i32())
    }
}

impl Serialize for i64 {
    fn serialize(&self, buf: &mut BytesMut) -> SerRes<()> {
        buf.put_i64(*self);
        Ok(())
    }

    fn deserialize(buf: &mut BytesMut) -> DeRes<Self> {
        if buf.remaining() < 8 {
            return Err(DeserializeError::UnexpectedEof);
        }

        Ok(buf.get_i64())
    }
}

impl Serialize for f32 {
    fn serialize(&self, buf: &mut BytesMut) -> SerRes<()> {
        buf.extend_from_slice(&self.to_be_bytes());

        Ok(())
    }

    fn deserialize(buf: &mut BytesMut) -> DeRes<Self>
    where
        Self: Sized,
    {
        if buf.remaining() < 4 {
            return Err(DeserializeError::UnexpectedEof);
        }

        Ok(buf.get_f32())
    }
}

impl Serialize for f64 {
    fn serialize(&self, buf: &mut BytesMut) -> SerRes<()> {
        buf.extend_from_slice(&self.to_be_bytes());

        Ok(())
    }

    fn deserialize(buf: &mut BytesMut) -> DeRes<Self>
    where
        Self: Sized,
    {
        if buf.remaining() < 8 {
            return Err(DeserializeError::UnexpectedEof);
        }

        Ok(buf.get_f64())
    }
}

impl Serialize for String {
    fn serialize(&self, buf: &mut BytesMut) -> SerRes<()> {
        VarInt(self.len() as i32).serialize(buf)?;
        buf.extend_from_slice(self.as_bytes());
        Ok(())
    }

    fn deserialize(buf: &mut BytesMut) -> DeRes<Self> {
        let len = VarInt::deserialize(buf)?.0 as usize;

        if buf.remaining() < len {
            return Err(DeserializeError::UnexpectedEof);
        }

        let bytes = buf.split_to(len).to_vec();

        String::from_utf8(bytes).map_err(Into::into)
    }
}

impl<T: Serialize> Serialize for Option<T> {
    fn serialize(&self, buf: &mut BytesMut) -> SerRes<()> {
        self.is_some().serialize(buf)?;

        if self.is_some() {
            self.as_ref().unwrap().serialize(buf)?
        }
        Ok(())
    }

    fn deserialize(buf: &mut BytesMut) -> DeRes<Self> {
        let present = bool::deserialize(buf)?;

        if present {
            Ok(Some(T::deserialize(buf)?))
        } else {
            Ok(None)
        }
    }
}

impl<T: Serialize> Serialize for Vec<T> {
    fn serialize(&self, buf: &mut BytesMut) -> SerRes<()> {
        VarInt(self.len() as i32).serialize(buf)?;

        for x in self {
            x.serialize(buf)?;
        }
        Ok(())
    }

    fn deserialize(buf: &mut BytesMut) -> DeRes<Self> {
        let len = VarInt::deserialize(buf)?.0 as usize;

        let mut vec = Vec::with_capacity(len);

        for _ in 0..len {
            vec.push(T::deserialize(buf)?);
        }

        Ok(vec)
    }
}

#[derive(Debug, Clone)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L: Serialize, R: Serialize> Serialize for Either<L, R> {
    fn serialize(&self, buf: &mut BytesMut) -> SerRes<()> {
        match self {
            Either::Left(l) => {
                true.serialize(buf)?;
                l.serialize(buf)?;
            }
            Either::Right(r) => {
                false.serialize(buf)?;
                r.serialize(buf)?;
            }
        }

        Ok(())
    }

    fn deserialize(buf: &mut BytesMut) -> DeRes<Self>
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

#[derive(Debug, Clone, Copy)]
pub struct Angle(u8);

impl Angle {
    pub fn from_degrees(angle: f32) -> Self {
        Self((angle * (256.0 / 360.0)) as u8)
    }

    pub fn to_degrees(&self) -> f32 {
        self.0 as f32 / (256.0 / 360.0)
    }
}

impl Serialize for Angle {
    fn serialize(&self, buf: &mut BytesMut) -> SerRes<()> {
        self.0.serialize(buf)
    }

    fn deserialize(buf: &mut BytesMut) -> DeRes<Self>
    where
        Self: Sized,
    {
        Ok(Self(u8::deserialize(buf)?))
    }
}

#[derive(Debug, Clone)]
pub struct InferredLenByteArray(pub Vec<u8>);

impl From<InferredLenByteArray> for Vec<u8> {
    fn from(array: InferredLenByteArray) -> Self {
        array.0
    }
}

impl From<Vec<u8>> for InferredLenByteArray {
    fn from(vec: Vec<u8>) -> Self {
        Self(vec)
    }
}

impl Serialize for InferredLenByteArray {
    fn serialize(&self, buf: &mut BytesMut) -> SerRes<()> {
        buf.extend_from_slice(self.0.as_slice());

        Ok(())
    }

    fn deserialize(buf: &mut BytesMut) -> DeRes<Self>
    where
        Self: Sized,
    {
        Ok(Self(buf[..buf.remaining()].to_vec()))
    }
}

impl Serialize for Uuid {
    fn serialize(&self, buf: &mut BytesMut) -> SerRes<()> {
        buf.put(&self.as_bytes()[..]);
        Ok(())
    }

    fn deserialize(buf: &mut BytesMut) -> DeRes<Self> {
        if buf.remaining() < 16 {
            return Err(DeserializeError::UnexpectedEof);
        }

        Ok(Uuid::from_u128(buf.get_u128()))
    }
}

impl Serialize for RsaPublicKey {
    fn serialize(&self, buf: &mut BytesMut) -> SerRes<()> {
        let document = self.to_public_key_der()?;
        let bytes = document.as_bytes();

        VarInt(bytes.len() as i32).serialize(buf)?;

        buf.extend_from_slice(bytes);

        Ok(())
    }

    fn deserialize(buf: &mut BytesMut) -> DeRes<Self>
    where
        Self: Sized,
    {
        let len = VarInt::deserialize(buf)?.0 as usize;

        if buf.remaining() < len {
            return Err(DeserializeError::UnexpectedEof);
        }

        let bytes = buf.split_to(len);

        RsaPublicKey::from_public_key_der(&bytes).map_err(Into::into)
    }
}

impl Serialize for nbt::Blob {
    fn serialize(&self, buf: &mut BytesMut) -> SerRes<()> {
        nbt::to_writer(&mut buf.writer(), &self, None).map_err(Into::into)
    }

    fn deserialize(buf: &mut BytesMut) -> DeRes<Self>
    where
        Self: Sized,
    {
        nbt::from_reader(&mut buf.reader()).map_err(Into::into)
    }
}

impl<A: Serialize> Serialize for (A,) {
    fn serialize(&self, buf: &mut BytesMut) -> SerRes<()> {
        self.0.serialize(buf)
    }

    fn deserialize(buf: &mut BytesMut) -> DeRes<Self>
    where
        Self: Sized,
    {
        Ok((A::deserialize(buf)?,))
    }
}

impl<A: Serialize, B: Serialize> Serialize for (A, B) {
    fn serialize(&self, buf: &mut BytesMut) -> SerRes<()> {
        self.0.serialize(buf)?;
        self.1.serialize(buf)
    }

    fn deserialize(buf: &mut BytesMut) -> DeRes<Self>
    where
        Self: Sized,
    {
        Ok((A::deserialize(buf)?, B::deserialize(buf)?))
    }
}
