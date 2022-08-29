pub mod clientbound;
pub mod common;
pub mod serverbound;

pub use clientbound::*;
pub use common::*;
pub use serverbound::*;

use crate::macros::state;

state! {
    Play;
    serverbound {
        0x00 => ConfirmTeleportation,
        0x01 => QueryBlockEntityTag,
        0x02 => ChangeDifficulty,
        0x03 => ChatCommand,
        0x04 => ChatMessage,
        0x05 => ChatPreview,
        0x06 => ClientCommand,
        0x07 => ClientInformation,
        0x08 => CommandSuggestionsRequest,
        0x09 => ClickContainerButton,
        0x0a => ClickContainer,
        0x0b => CloseContainer,
        0x0c => ServerboundPluginMessage,
        0x0d => EditBook,
        0x0e => QueryEntityTag,
        0x0f => Interact,
        0x10 => JigsawGenerate,
        0x11 => ServerboundKeepAlive,
        0x12 => LockDifficulty,
        0x13 => SetPlayerPosition,
        0x14 => SetPlayerPositionAndRotation,
        0x15 => SetPlayerRotation,
        0x16 => SetPlayerOnGround,
        0x17 => ServerboundMoveVehicle,
        0x18 => PaddleBoat,
        0x19 => PickItem,
        0x1a => PlaceRecipe,
        0x1b => PlayerAbilities,
        0x1c => PlayerAction,
        0x1d => PlayerCommand,
        0x1e => PlayerInput,
        0x1f => Pong
    };
    clientbound {
        0x00 => SpawnEntity,
        0x01 => SpawnExperienceOrb,
        0x02 => SpawnPlayer,
        0x03 => EntityAnimation
    }
}
