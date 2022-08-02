use uuid::Uuid;

use crate::{
    chat::Chat,
    data, packets, state,
    types::{varint::VarInt, Either},
};

state! {
    Login;
    serverbound {
        0x00 => LoginStart(LoginStart),
        0x01 => EncryptionResponse(EncryptionResponse),
        0x02 => LoginPLuginRespnse(LoginPluginResponse)
    };
    clientbound {
        0x00 => Disconnect(LoginDisconnect),
        0x01 => EncryptionRequest(EncryptionRequest),
        0x02 => LoginSuccess(LoginSuccess),
        0x03 => SetCompression(SetCompression),
        0x04 => LoginPluginRequest(LoginPluginRequest)
    }
}

packets! {
    LoginStart(0x00) {
        name: String,
        sig_data: Option<SigData>
    };
    EncryptionResponse(0x01) {
        shared_secret: Vec<u8>,
        verify_token: Either<VerifyToken, SaltSignature>
    };
    LoginPluginResponse(0x02) {
        message_id: VarInt,
        data: Option<Vec<u8>>
    };

    LoginDisconnect(0x00) {
        reason: Chat
    };
    EncryptionRequest(0x01) {
        server_id: String,
        public_key: Vec<u8>,
        verify_token: Vec<u8>
    };
    LoginSuccess(0x02) {
        uuid: Uuid,
        username: String,
        properties: Vec<LoginSuccessProperty>
    };
    SetCompression(0x03) {
        threshold: VarInt
    };
    LoginPluginRequest(0x04) {
        message_id: VarInt,
        channel: String,
        data: Vec<u8>
    }
}

data! {
    SigData {
        timestamp: i64,
        public_key: Vec<u8>,
        signature: Vec<u8>
    };
    VerifyToken {
        verify_token: Vec<u8>
    };
    SaltSignature {
        salt: i64,
        signature: Vec<u8>
    };

    LoginSuccessProperty {
        name: String,
        value: String,
        signature: Option<String>
    }
}
