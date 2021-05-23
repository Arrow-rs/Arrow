/// All packets for protocol versions 552 and above.

/// All `Play` packets for protocol versions 552 and above.
pub mod play {
    /// All clientbound `Play` packets for protocol versions 552 and above.
    pub mod clientbound {
        type DimensionCodec = nbt::Blob;

        use crate::serde::varint::VarInt;

        use crate::packets::types::Gamemode;

        /// The [JoinGame](https://wiki.vg/Protocol#Join_Game) packet for version 552 or higher
        pub struct JoinGame {
            /// This is the player's Entity ID (EID).
            pub entity_id: i32,
            /// Defines if the server is on hardcore mode
            pub is_hardcore: bool,
            /// 0: survival, 1: creative, 2: adventure, 3: spectator.
            pub gamemode: Gamemode,
            /// 0: survival, 1: creative, 2: adventure, 3: spectator. The hardcore flag is not included. The previous gamemode. Defaults to -1 if there is no previous gamemode. (More information needed)
            pub previous_gamemode: Gamemode,
            /// Size of the world_names array
            pub world_cound: VarInt,
            /// Identifiers for all worlds on the server.
            pub world_names: Vec<String>,
            pub dimension_codec: DimensionCodec,
            pub dimension: Dimension,
            pub world_name: String,

        }
    }
}