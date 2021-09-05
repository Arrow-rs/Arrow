use std::marker::PhantomData;

use nbt::Blob;
use serde::{de::Visitor, Deserialize, Serialize};

use crate::packets::types::Nbt;

/// Dimension type as int enum
#[repr(i32)]
#[derive(Serialize, Deserialize)]
pub enum Dimension {
    /// nehter dimension
    Nether = -1,
    /// overworld dimension
    Overworld = 0,
    /// end dimension
    End = 1,
}

/// The [Slot](https://wiki.vg/Slot) data type.
#[derive(Serialize)]
pub struct Slot<'a> {
    id: i16,
    #[serde(borrow)]
    #[serde(flatten)]
    data: Option<SlotData<'a>>,
}

struct SlotVisitor<'a>(PhantomData<&'a ()>);

/// The data for the [`Slot`] type.
#[derive(Serialize, Deserialize)]
pub struct SlotData<'a> {
    count: u8,
    damage: i16,
    #[serde(borrow)]
    nbt: Nbt<'a, Blob>,
}

impl<'a> SlotData<'a> {
    /// Creates a new [`SlotData`].
    pub fn new(count: u8, damage: i16, nbt: Nbt<'a, Blob>) -> Self {
        Self { count, damage, nbt }
    }

    /// Get a reference to the slot data's count.
    pub fn count(&self) -> &u8 {
        &self.count
    }

    /// Get a reference to the slot data's damage.
    pub fn damage(&self) -> &i16 {
        &self.damage
    }

    /// Get a reference to the slot data's nbt.
    pub fn nbt(&self) -> &Nbt<'a, Blob> {
        &self.nbt
    }
}

impl<'a> Slot<'a> {
    /// Creates a new [`Slot`].
    pub fn new(id: i16, data: Option<SlotData<'a>>) -> Self {
        Self { id, data }
    }

    /// Get a reference to the slot's id.
    pub fn id(&self) -> &i16 {
        &self.id
    }

    /// Get a reference to the slot's data.
    pub fn data(&self) -> Option<&SlotData<'a>> {
        self.data.as_ref()
    }
}

impl<'a, 'de: 'a> Deserialize<'de> for Slot<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(SlotVisitor(PhantomData))
    }
}

impl<'a, 'de: 'a> Visitor<'de> for SlotVisitor<'a> {
    type Value = Slot<'a>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("seq")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let id: i16 = seq.next_element()?.unwrap();

        if id == -1 {
            Ok(Slot::new(id, None))
        } else {
            let count: u8 = seq.next_element()?.unwrap();
            let damage: i16 = seq.next_element()?.unwrap();
            let nbt: Nbt<'_, Blob> = seq.next_element()?.unwrap();

            Ok(Slot::new(id, Some(SlotData::new(count, damage, nbt))))
        }
    }
}
