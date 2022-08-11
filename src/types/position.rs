use crate::error::{DeRes, SerRes};

use super::Serialize;

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}

impl Serialize for Position {
    fn serialize(&self, buf: &mut bytes::BytesMut) -> SerRes<()> {
        let x = self.x as u64 & 0x1ffffff | ((self.x.is_negative() as u64) << 25);
        let z = self.z as u64 & 0x1ffffff | ((self.z.is_negative() as u64) << 25);
        let y = self.y as u64 & 0x7ff | ((self.y.is_negative() as u64) << 11);

        let pos = x << 38 | z << 12 | y;

        pos.serialize(buf)?;

        Ok(())
    }

    fn deserialize(buf: &mut bytes::Bytes) -> DeRes<Self>
    where
        Self: Sized,
    {
        let pos = u64::deserialize(buf)?;

        let mut x = (pos >> 38) as i32;
        let mut z = ((pos >> 12) & 0x3ffffff) as i32;
        let mut y = (pos & 0xfff) as i16;

        if x >= 1 << 25 {
            x -= 1 << 26;
        }
        if y >= 1 << 11 {
            y -= 1 << 12;
        }
        if z >= 1 << 25 {
            z -= 1 << 26;
        }

        Ok(Self { x, y, z })
    }
}
