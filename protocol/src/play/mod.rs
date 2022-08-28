use bytes::BufMut;

use crate::{
    bitflags, data,
    error::DeserializeError,
    int_enum, packets, state,
    types::{position::Position, slot::Slot, varint::VarInt, InferredLenByteArray, Serialize},
    varint_enum,
};

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
        0x0f => Interact
    };
    clientbound {

    }
}

packets! {
    ConfirmTeleportation(0x00) {
        id: VarInt
    };
    QueryBlockEntityTag(0x01) {
        id: VarInt,
        location: Position
    };
    ChangeDifficulty(0x02) {
        new_difficulty: Difficulty
    };
    ChatCommand(0x03) {
        command: String,
        timestamp: i64,
        salt: i64,
        arg_signatures: Vec<ArgumentSignature>,
        signed_preview: bool
    };
    ChatMessage(0x04) {
        message: String,
        timestamp: i64,
        salt: i64,
        signature: Vec<u8>,
        signed_preview: bool
    };
    ChatPreview(0x05) {
        query: i32,
        message: String
    };
    ClientCommand(0x06) {
        id: ActionId
    };
    ClientInformation(0x07) {
        locale: String,
        view_distance: u8,
        chat_mode: ChatMode,
        chat_colors: bool,
        displayed_skin_parts: SkinParts,
        main_hand: MainHand,
        enable_text_filtering: bool,
        allow_server_listings: bool
    };
    CommandSuggestionsRequest(0x08) {
        id: VarInt,
        text: String
    };
    ClickContainerButton(0x09) {
        window_id: u8,
        button_id: u8
    };
    ClickContainer(0x0a) {
        window_id: u8,
        state_id: VarInt,
        slot: i16,
        button: i8,
        mode: InventoryOperationMode,
        slots: (i16, Slot),
        carried_item: Slot
    };
    CloseContainer(0x0b) {
        window_id: u8
    };
    ServerboundPluginMessage(0x0c) {
        channel: String,
        data: InferredLenByteArray
    };
    EditBook(0x0d) {
        slot: VarInt,
        entries: Vec<String>,
        title: Option<String>
    };
    QueryEntityTag(0x0e) {
        transaction_id: VarInt,
        entity_id: VarInt
    };
    Interact(0x0f) {
        entity_id: VarInt,
        ty: InteractionType,
        sneaking: bool
    }
}

int_enum! {
    Difficulty(u8) {
        Peaceful = 0,
        Easy = 1,
        Normal = 2,
        Hard = 3
    }
}

varint_enum! {
    ActionId {
        PerformRespawn = 0,
        RequestStats = 1
    };
    ChatMode {
        Enabled = 0,
        CommandsOnly = 1,
        Hidden = 2
    };
    MainHand {
        Left = 0,
        Right = 1
    };
    InventoryOperationMode {
        MouseClick = 0,
        ShiftMouseClick = 1,
        NumKey = 2,
        MiddleClick = 3,
        DropKey = 4,
        Drag = 5,
        DoubleClick = 6
    };
    Hand {
        MainHand = 0,
        OffHand = 1
    }
}

data! {
    ArgumentSignature {
        name: String,
        signature: Vec<u8>
    }
}

bitflags! {
    SkinParts(u8) {
        const CAPE = 0x01;
        const JACKET = 0x02;
        const LEFT_SLEEVE = 0x04;
        const RIGHT_SLEEVE = 0x08;
        const LEFT_PANTS_LEG = 0x10;
        const RIGHT_PANTS_LEG = 0x20;
        const HAT = 0x40;
    }
}

#[derive(Debug, Clone)]
pub enum InteractionType {
    Interact(Hand),
    Attack,
    InteractAt {
        target_x: f32,
        target_y: f32,
        target_z: f32,
        hand: Hand,
    },
}

impl Serialize for InteractionType {
    fn serialize(&self, buf: &mut bytes::BytesMut) -> crate::error::SerRes<()> {
        match self {
            InteractionType::Interact(hand) => {
                buf.put_u8(0);
                hand.serialize(buf)?;
            }
            InteractionType::Attack => buf.put_u8(1),
            InteractionType::InteractAt {
                target_x,
                target_y,
                target_z,
                hand,
            } => {
                buf.put_u8(2);
                target_x.serialize(buf)?;
                target_y.serialize(buf)?;
                target_z.serialize(buf)?;
                hand.serialize(buf)?;
            }
        }

        Ok(())
    }

    fn deserialize(buf: &mut bytes::BytesMut) -> crate::error::DeRes<Self>
    where
        Self: Sized,
    {
        let variant = u8::deserialize(buf)?;

        match variant {
            0 => {
                let hand = Hand::deserialize(buf)?;
                Ok(Self::Interact(hand))
            }
            1 => Ok(Self::Attack),
            2 => {
                let target_x = f32::deserialize(buf)?;
                let target_y = f32::deserialize(buf)?;
                let target_z = f32::deserialize(buf)?;
                let hand = Hand::deserialize(buf)?;

                Ok(Self::InteractAt {
                    target_x,
                    target_y,
                    target_z,
                    hand,
                })
            }
            _ => Err(DeserializeError::InvalidEnumVariant(
                "InteractionType",
                variant as isize,
            )),
        }
    }
}
