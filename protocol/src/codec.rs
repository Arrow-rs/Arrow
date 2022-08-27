use aes::cipher::KeyIvInit;
use tokio_util::codec::{Decoder, Encoder};

use crate::{
    error::{DeserializeError, SerializeError},
    Bound, Decryptor, Encryptor, PacketCompression, Protocol, State,
};

#[derive(Debug)]
pub struct Codec {
    bound: Bound,
    state: State,
    compression: PacketCompression,
    encryptor: Option<Encryptor>,
    decryptor: Option<Decryptor>,
}

impl Codec {
    pub fn new(bound: Bound, state: State) -> Self {
        Self {
            bound,
            state,
            compression: Default::default(),
            encryptor: None,
            decryptor: None,
        }
    }

    pub fn enable_compression(&mut self, threshold: usize) {
        self.compression.enabled = true;
        self.compression.threshold = threshold;
    }

    pub fn enable_encyption(&mut self, key: [u8; 16]) {
        self.encryptor = Some(Encryptor::new(&key.into(), &key.into()));
        self.decryptor = Some(Decryptor::new(&key.into(), &key.into()));
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }
}

impl Decoder for Codec {
    type Item = Protocol;

    type Error = DeserializeError;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match Protocol::deserialize(
            self.bound,
            self.state,
            self.compression,
            self.decryptor.as_mut(),
            src,
        ) {
            Ok(packet) => Ok(Some(packet)),
            Err(DeserializeError::UnexpectedEof) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

impl Encoder<Protocol> for Codec {
    type Error = SerializeError;

    fn encode(&mut self, item: Protocol, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        dst.copy_from_slice(
            item.serialize(self.compression, self.encryptor.as_mut())?
                .as_slice(),
        );

        Ok(())
    }
}
