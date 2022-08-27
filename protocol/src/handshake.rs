use crate::{packet, types::varint::VarInt, varint_enum};

packet! {
    Handshake(0x00) {
        version: VarInt,
        address: String,
        port: u16,
        next_state: NextState
    }
}

varint_enum! {
    NextState {
        Status = 1,
        Login = 2
    }
}
