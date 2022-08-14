use crate::{packets, state};

state! {
    Status;
    serverbound {
        0x00 => StatusRequest,
        0x01 => PingRequest
    };
    clientbound {
        0x00 => StatusResponse,
        0x01 => PingResponse
    }
}

packets! {
    StatusRequest(0x00);
    PingRequest(0x01) {
        payload: i64
    };
    StatusResponse(0x00) {
        response: String
    };
    PingResponse(0x01) {
        payload: i64
    }
}

impl From<PingRequest> for PingResponse {
    fn from(r: PingRequest) -> Self {
        Self { payload: r.payload }
    }
}

impl From<PingResponse> for PingRequest {
    fn from(r: PingResponse) -> Self {
        Self { payload: r.payload }
    }
}
