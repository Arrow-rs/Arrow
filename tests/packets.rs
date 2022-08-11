use aes::cipher::KeyIvInit;
use arrow_protocol::{
    chat::Chat,
    handshake::NextState,
    types::{varint::VarInt, Serialize},
    Bound, Decryptor, Encryptor, PacketCompression, Protocol, State,
};
use bytes::Bytes;

macro_rules! test_packet {
    ($module:ident :: $packet:ident { $($field:ident : $value:expr),* } = $bound:ident($state:ident) $(; $state_name:ident)?) => {
        let packet = arrow_protocol::$module::$packet {
            $($field : $value),*
        };

        let compression = PacketCompression::default();

        let protocol = Protocol::$state(packet.clone().into());

        let key = [0x42; 16];

        let mut encryptor = Encryptor::new(&key.into(), &key.into());
        let mut decryptor = Decryptor::new(&key.into(), &key.into());

        let mut bytes = Bytes::from(protocol.serialize(compression, Some(&mut encryptor)).unwrap().into_boxed_slice());

        let protocol2 =
            Protocol::deserialize(Bound::$bound, State::$state, compression, Some(&mut decryptor), &mut bytes).unwrap();

        #[allow(unused_parens)]
        if let Protocol::$state($(arrow_protocol::$module::$state::$state_name)?(packet2)) = protocol2 {
            $(
                assert_eq!(packet.$field, packet2.$field);
            )*
        }
    }
}

#[test]
fn handshake() {
    test_packet! {
        handshake::Handshake {
            version: 759.into(),
            address: "localhost".to_string(),
            port: 25565,
            next_state: NextState::Login
        } = Serverbound(Handshake)
    }
}

#[test]
fn login_disconnect() {
    test_packet! {
        login::LoginDisconnect {
            reason: Chat::default().with_bold(true)
        } = Clientbound(Login); Disconnect
    }
}
