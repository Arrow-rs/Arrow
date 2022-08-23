use nbt::Blob;

use crate::{macros::data, types::varint::VarInt};

data! {
    Slot {
        data: Option<SlotData>
    };
    SlotData {
        id: VarInt,
        count: i8,
        nbt: Blob
    }
}
