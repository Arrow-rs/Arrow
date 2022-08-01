use crate::{packets, state};

state! {
    Status;
    serverbound {
        0x00 => StatusRequest(StatusRequest),
        0x01 => PingRequest(Ping)
    };
    clientbound {
        0x00 => StatusResponse(StatusResponse),
        0x01 => PingResponse(Ping)
    }
}

packets! {
    StatusRequest(0x00);
    Ping(0x01) {
        payload: i64
    };
    StatusResponse(0x00) {
        response: String
    }
}
