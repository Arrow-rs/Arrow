use arrow_protocol::{
    chat::Chat,
    handshake::{Handshake, NextState},
    types::{varint::VarInt, Serialize},
    Bound, PacketCompression, Protocol, State,
};
use bytes::Bytes;

macro_rules! test_packet {
    ($module:ident :: $packet:ident { $($field:ident : $value:expr),* } = $bound:ident($state:ident) $(; $state_name:ident)?) => {
        let packet = arrow_protocol::$module::$packet {
            $($field : $value),*
        };

        let compression = PacketCompression::default();

        let protocol = Protocol::$state(packet.clone().into());

        let mut bytes = Bytes::from(protocol.serialize(compression).into_boxed_slice());
        let _len = VarInt::deserialize(&mut bytes);

        let protocol2 =
            Protocol::deserialize(Bound::$bound, State::$state, compression, bytes).unwrap();

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
