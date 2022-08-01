use bytes::BufMut;
use uuid::Uuid;

use crate::{
    data,
    error::Res,
    packets, state,
    types::{varint::VarInt, Either, Serialize},
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

    LoginDisconnect(0x00) {
        reason: String
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

pub struct LoginPluginResponse {
    message_id: VarInt,
    data: Option<Vec<u8>>,
}

impl LoginPluginResponse {
    pub fn serialize(&self) -> Vec<u8> {
        let mut packet = bytes::BytesMut::new();

        let mut buf = bytes::BytesMut::new();

        0x02u8.serialize(&mut buf);

        self.message_id.serialize(&mut buf);

        self.data.is_some().serialize(&mut buf);

        if self.data.is_some() {
            buf.put(self.data.as_ref().unwrap().as_slice());
        }

        VarInt(buf.len() as i32).serialize(&mut packet);

        let mut packet = packet.to_vec();
        packet.extend_from_slice(&buf);

        packet
    }

    pub fn deserialize(buf: &mut bytes::Bytes) -> Res<Self> {
        let message_id = VarInt::deserialize(buf)?;
        let data = Option::<_>::deserialize(buf)?;

        Ok(Self { message_id, data })
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
