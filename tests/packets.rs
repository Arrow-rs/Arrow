use aes::cipher::KeyIvInit;
use arrow_protocol::{
    chat::Chat, handshake::NextState, types::varint::VarInt, Bound, Decryptor, Encryptor,
    PacketCompression, Protocol, State,
};
use bytes::{BufMut, Bytes, BytesMut};

macro_rules! test_packet {
    ($module:ident :: $packet:ident { $($field:ident : $value:expr),* } = $bound:ident($state:ident) $(; $state_name:ident)?) => {
        let packet = arrow_protocol::$module::$packet {
            $($field : $value),*
        };

        let compression = PacketCompression::default();

        let protocol = Protocol::$state(packet.clone().into());

        let mut bytes = Bytes::from(protocol.serialize(compression, None).unwrap().into_boxed_slice());

        let protocol2 =
            Protocol::deserialize(Bound::$bound, State::$state, compression, None, &mut bytes).unwrap();

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
        } = Clientbound(Login); LoginDisconnect
    }
}

#[test]
fn encryption() {
    let handshake = arrow_protocol::handshake::Handshake {
        version: VarInt(10),
        address: "foo".to_string(),
        port: 1337,
        next_state: NextState::Login,
    };
    let handshake2 = arrow_protocol::handshake::Handshake {
        version: VarInt(100),
        address: "foobar".to_string(),
        port: 42,
        next_state: NextState::Status,
    };

    let compression = PacketCompression::default();

    let protocol = Protocol::Handshake(handshake.clone().into());
    let protocol2 = Protocol::Handshake(handshake2.clone().into());

    let key = [0x42; 16];

    let mut encryptor = Encryptor::new(&key.into(), &key.into());
    let mut decryptor = Decryptor::new(&key.into(), &key.into());

    let mut bytes = BytesMut::from(
        protocol
            .serialize(compression, Some(&mut encryptor))
            .unwrap()
            .as_slice(),
    );
    bytes.put_slice(
        &protocol2
            .serialize(compression, Some(&mut encryptor))
            .unwrap(),
    );
    let mut bytes = bytes.freeze();

    let protocol1 = Protocol::deserialize(
        Bound::Serverbound,
        State::Handshake,
        compression,
        Some(&mut decryptor),
        &mut bytes,
    )
    .unwrap();
    let protocol2 = Protocol::deserialize(
        Bound::Serverbound,
        State::Handshake,
        compression,
        Some(&mut decryptor),
        &mut bytes,
    )
    .unwrap();

    if let (Protocol::Handshake(packet1), Protocol::Handshake(packet2)) = (protocol1, protocol2) {
        assert_eq!(packet1.version, handshake.version);
        assert_eq!(packet1.address, handshake.address);
        assert_eq!(packet1.port, handshake.port);
        assert_eq!(packet1.next_state, handshake.next_state);
        assert_eq!(packet2.version, handshake2.version);
        assert_eq!(packet2.address, handshake2.address);
        assert_eq!(packet2.port, handshake2.port);
        assert_eq!(packet2.next_state, handshake2.next_state);
    }
}

#[test]
fn compression() {
    let handshake = arrow_protocol::handshake::Handshake {
        version: VarInt(42),
        address: "foobarbaz".to_string(),
        port: 25565,
        next_state: NextState::Login,
    };

    let mut compression = PacketCompression::default();
    compression.enabled = true;
    compression.threshold = 5;

    let protocol = Protocol::Handshake(handshake.clone().into());

    let mut bytes = Bytes::from(
        protocol
            .serialize(compression, None)
            .unwrap()
            .into_boxed_slice(),
    );

    let protocol = Protocol::deserialize(
        Bound::Serverbound,
        State::Handshake,
        compression,
        None,
        &mut bytes,
    )
    .unwrap();

    if let Protocol::Handshake(packet) = protocol {
        assert_eq!(packet.version, handshake.version);
        assert_eq!(packet.address, handshake.address);
        assert_eq!(packet.port, handshake.port);
        assert_eq!(packet.next_state, handshake.next_state);
    }
}
