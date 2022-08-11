use bytes::BytesMut;

use arrow_protocol::types::{position::Position, Serialize};

#[test]
fn position() {
    let pos = Position {
        x: -100,
        y: -10,
        z: -69420,
    };

    let mut buf = BytesMut::new();

    pos.serialize(&mut buf).unwrap();

    let pos2 = Position::deserialize(&mut buf.freeze()).unwrap();

    assert_eq!(pos, pos2);
}
