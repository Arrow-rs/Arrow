/// All clientbound `play` packets for protocol versions 453 and above.
pub mod clientbound {
    use serde::{Deserialize, Serialize};

    use crate::{
        packets::{types::LengthPrefixedVec, version_specific::types::v1_14::Recipe, version::*, Packet},
        serde::ser::Serializer,
    };

    /// The DeclareRecipes packet.
    #[derive(Serialize, Deserialize)]
    pub struct DeclareRecipes<'a> {
        /// All crafting recipes.
        #[serde(borrow)]
        pub recipes: LengthPrefixedVec<'a, Recipe<'a>>,
    }

    impl<'a> Packet for DeclareRecipes<'a> {
        fn id(protocol_version: i32) -> i32
        where
            Self: Sized,
        {
            match protocol_version {
                V1_14..=V1_14_4 => 0x5A,
                V1_15..=V1_15_2 => 0x5B,
                V1_16..=V1_16_4 => 0x5A,
                V1_17..=V1_17_1 => 0x65,
                _ => unreachable!(),
            }
        }

        fn self_id(&self, protocol_version: i32) -> i32 {
            Self::id(protocol_version)
        }

        fn data_bytes(&self) -> Result<Vec<u8>, crate::packets::error::PacketError> {
            let mut ser = Serializer::new();

            self.serialize(&mut ser)?;

            Ok(ser.get_bytes())
        }
    }
}
