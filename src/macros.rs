#[macro_export]
macro_rules! state {
    ($name:ident; serverbound { $($sbid:literal => $sbvariant:ident($sbpacket:ty)),* }; clientbound { $($cbid:literal => $cbvariant:ident($cbpacket:ty)),* } ) => {
        pub enum $name {
            $($sbvariant($sbpacket),)*
            $($cbvariant($cbpacket),)*
        }

        impl $name {
            pub fn serialize(&self) -> Vec<u8> {
                match self {
                    $(Self::$sbvariant(variant) => variant.serialize(),)*
                    $(Self::$cbvariant(variant) => variant.serialize(),)*
                }
            }

            pub fn deserialize(bound: $crate::Bound, id: i32, bytes: &mut bytes::Bytes) -> $crate::error::Res<Self> {
                match bound {
                    $crate::Bound::Serverbound => {
                        match id {
                            $($sbid => Ok(Self::$sbvariant(<$sbpacket>::deserialize(bytes)?)),)*
                            _ => Err($crate::error::ProtocolError::UnknownPacketId(bound, $crate::State::$name, id))
                        }
                    }
                    $crate::Bound::Clientbound => {
                        match id {
                            $($cbid => Ok(Self::$cbvariant(<$cbpacket>::deserialize(bytes)?)),)*
                            _ => Err($crate::error::ProtocolError::UnknownPacketId(bound, $crate::State::$name, id))
                        }
                    }
                }

            }
        }
    }
}

#[macro_export]
macro_rules! packets {
    ($($name:ident($id:literal) $({ $($field:ident: $ty:ty),+ })?);*) => {
        $(
            $crate::packet! { $name($id) $({ $($field: $ty),+ })? }
        )*
    }
}

#[macro_export]
macro_rules! packet {
    ($name:ident($id:literal)) => {
        pub struct $name;

        impl $name {
            pub fn serialize(&self) -> Vec<u8> {
                use $crate::types::Serialize;

                let mut packet = bytes::BytesMut::new();

                let mut buf = bytes::BytesMut::new();

                $crate::types::varint::VarInt($id).serialize(&mut buf);

                $crate::types::varint::VarInt(buf.len() as i32).serialize(&mut packet);

                let mut packet = packet.to_vec();
                packet.extend_from_slice(&buf);

                packet
            }

            pub fn deserialize(_: &mut bytes::Bytes) -> $crate::error::Res<Self> {
                Ok(Self)
            }

        }
    };

    ($name:ident($id:literal) { $($field:ident: $ty:ty),+ }) => {
        pub struct $name {
            $(pub $field: $ty),*
        }

        impl $name {
            pub fn serialize(&self) -> Vec<u8> {
                use $crate::types::Serialize;

                let mut packet = bytes::BytesMut::new();

                let mut buf = bytes::BytesMut::new();

                $crate::types::varint::VarInt($id).serialize(&mut buf);

                $(self.$field.serialize(&mut buf);)*

                $crate::types::varint::VarInt(buf.len() as i32).serialize(&mut packet);

                let mut packet = packet.to_vec();
                packet.extend_from_slice(&buf);

                packet
            }

            pub fn deserialize(buf: &mut bytes::Bytes) -> $crate::error::Res<Self> {
                use $crate::types::Serialize;

                $(let $field: $ty = Serialize::deserialize(buf)?;)*

                Ok(Self {
                    $($field),*
                })
            }
        }
    };
}

#[macro_export]
macro_rules! varint_enum {
    ($($name:ident { $($variant:ident = $value:literal),* });*) => {
        $(
            #[repr(i32)]
            #[derive(Debug, Clone, Copy, PartialEq)]
            pub enum $name {
                $($variant = $value),*
            }

            impl $crate::types::Serialize for $name {
                fn serialize(&self, buf: &mut bytes::BytesMut) {
                    use $crate::types::varint::VarInt;

                    let val = *self as i32;

                    let varint = VarInt(val);

                    varint.serialize(buf);
                }

                fn deserialize(buf: &mut bytes::Bytes) -> $crate::error::Res<Self> {
                    use $crate::{error::ProtocolError, types::varint::VarInt};

                    let varint = VarInt::deserialize(buf)?;

                    match varint.0 {
                        $($value => Ok(Self::$variant),)*
                        _ => Err(ProtocolError::InvalidEnumVariant(stringify!($name), varint.0 as isize))
                    }
                }
            }
        )*
    }
}

#[macro_export]
macro_rules! int_enum {
    ($($name:ident($int:ty) { $($variant:ident = $value:literal),* });*) => {
        $(
            #[repr($int)]
            #[derive(Debug, Clone, Copy, PartialEq)]
            pub enum $name {
                $($variant = $value),*
            }

            impl $crate::types::Serialize for $name {
                fn serialize(&self, buf: &mut bytes::BytesMut) {
                    let val = *self as $int;

                    val.serialize(buf);
                }

                fn deserialize(buf: &mut bytes::Bytes) -> $crate::error::Res<Self> {
                    use $crate::error::ProtocolError;

                    let val = <$int>::deserialize(buf)?;

                    match val {
                        $($value => Ok(Self::$variant),)*
                        _ => Err(ProtocolError::InvalidEnumVariant(stringify!($name), val as isize))
                    }
                }
            }
        )*
    }
}

#[macro_export]
macro_rules! data {
    ($($name:ident { $($field:ident: $ty:ty),* });*) => {
        $(
            pub struct $name {
                $(pub $field: $ty),*
            }

            impl $crate::types::Serialize for $name {
                fn serialize(&self, buf: &mut bytes::BytesMut) {
                    $(self.$field.serialize(buf);)*
                }

                fn deserialize(buf: &mut bytes::Bytes) -> $crate::error::Res<Self> {
                    use $crate::types::Serialize;

                    $(let $field: $ty = Serialize::deserialize(buf)?;)*

                    Ok(Self {
                        $($field),*
                    })
                }
            }
        )*
    }
}
