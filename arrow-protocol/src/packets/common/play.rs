/// All common clientbound `play` packets.
pub mod clientbound {
    use serde::{Deserialize, Serialize};

    use crate::{
        packets::{error::PacketError, version::*, Packet},
        serde::ser::Serializer,
    };

    /// The [HeldItemChange](https://wiki.vg/Protocol#Held_Item_Change_.28clientbound.29) packet.
    #[derive(Serialize, Deserialize)]
    pub struct HeldItemChange {
        /// The slot which the player has selected (0–8).
        pub slot: i8,
    }

    impl HeldItemChange {
        /// create a new [HeldItemChange] packet
        pub fn new(slot: i8) -> Self {
            Self { slot }
        }
    }

    impl Packet for HeldItemChange {
        fn id(version: i32) -> i32 {
            match version {
                V1_8..=V1_11_2 => 0x37,
                V1_12_1..=V1_12_2 => 0x3a,
                V1_13..=V1_13_2 => 0x3d,
                V1_14..=V1_14_4 => 0x3f,
                V1_15..=V1_15_2 => 0x40,
                V1_16..=V1_16_5 => 0x3f,
                V1_17..=V1_17_1 => 0x48,
                _ => panic!()
            }
        }

        fn data_bytes(&self) -> Result<Vec<u8>, PacketError> {
            let mut ser = Serializer::new();

            self.serialize(&mut ser)?;

            Ok(ser.get_bytes())
        }

        fn self_id(&self, protocol_version: i32) -> i32 {
            Self::id(protocol_version)
        }
    }
}
