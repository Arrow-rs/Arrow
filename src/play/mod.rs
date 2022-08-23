use crate::{
    data, int_enum, packets, state,
    types::{position::Position, slot::Slot, varint::VarInt, InferredLenByteArray},
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
        0x0c => ServerboundPluginMessage
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
        // TODO: Bitmask
        displayed_skin_parts: u8,
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
    }
}

data! {
    ArgumentSignature {
        name: String,
        signature: Vec<u8>
    }
}
