use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};
use uuid::Uuid;

use crate::{
    chat::Chat,
    data,
    error::{DeRes, DeserializeError, SerRes},
    packets, state,
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
        shared_secret: SharedSecret,
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
        public_key: RsaPublicKey,
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
    SharedSecret {
        encrypted_secret: Vec<u8>
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

impl SharedSecret {
    pub fn encrypt(secret: &[u8; 16], public_key: RsaPublicKey) -> SerRes<Self> {
        let mut rng = rand::thread_rng();

        let encrypted_secret =
            public_key.encrypt(&mut rng, PaddingScheme::PKCS1v15Encrypt, secret)?;

        Ok(Self { encrypted_secret })
    }

    pub fn decrypt(&self, private_key: RsaPrivateKey) -> DeRes<[u8; 16]> {
        let secret = private_key.decrypt(PaddingScheme::PKCS1v15Encrypt, &self.encrypted_secret)?;

        secret
            .try_into()
            .map_err(|_| DeserializeError::InvalidSharedSecretLength)
    }
}
