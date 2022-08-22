macro_rules! state {
    ($name:ident; serverbound { $($sbid:literal => $sbpacket:ident),* }; clientbound { $($cbid:literal => $cbpacket:ident),* } ) => {
        pub enum $name {
            $($sbpacket($sbpacket),)*
            $($cbpacket($cbpacket),)*
        }

        impl $name {
            pub fn serialize(&self) -> $crate::error::SerRes<(i32, Vec<u8>)> {
                match self {
                    $(Self::$sbpacket(variant) => variant.serialize(),)*
                    $(Self::$cbpacket(variant) => variant.serialize(),)*
                }
            }

            pub fn deserialize(bound: $crate::Bound, id: i32, bytes: &mut bytes::BytesMut) -> $crate::error::DeRes<Self> {
                match bound {
                    $crate::Bound::Serverbound => {
                        match id {
                            $($sbid => Ok(Self::$sbpacket(<$sbpacket>::deserialize(bytes)?)),)*
                            _ => Err($crate::error::DeserializeError::UnknownPacketId(bound, $crate::State::$name, id))
                        }
                    }
                    $crate::Bound::Clientbound => {
                        match id {
                            $($cbid => Ok(Self::$cbpacket(<$cbpacket>::deserialize(bytes)?)),)*
                            _ => Err($crate::error::DeserializeError::UnknownPacketId(bound, $crate::State::$name, id))
                        }
                    }
                }
            }
        }

        $(impl From<$sbpacket> for $name {
            fn from(p: $sbpacket) -> Self {
                Self::$sbpacket(p)
            }
        })*
        $(impl From<$cbpacket> for $name {
            fn from(p: $cbpacket) -> Self {
                Self::$cbpacket(p)
            }
        })*
    }
}

macro_rules! packets {
    ($($name:ident($id:literal) $({ $($field:ident: $ty:ty),+ })?);*) => {
        $(
            $crate::packet! { $name($id) $({ $($field: $ty),+ })? }
        )*
    }
}

macro_rules! packet {
    ($name:ident($id:literal)) => {
        #[derive(Debug, Clone)]
        pub struct $name;

        impl $name {
            pub fn serialize(&self) -> $crate::error::SerRes<(i32, Vec<u8>)> {
                Ok(($id, vec![]))
            }

            pub fn deserialize(_: &mut bytes::BytesMut) -> $crate::error::DeRes<Self> {
                Ok(Self)
            }

        }
    };

    ($name:ident($id:literal) { $($field:ident: $ty:ty),+ }) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            $(pub $field: $ty),*
        }

        impl $name {
            pub fn serialize(&self) -> $crate::error::SerRes<(i32, Vec<u8>)> {
                use $crate::types::Serialize;

                let mut data = bytes::BytesMut::new();

                $(self.$field.serialize(&mut data)?;)*

                Ok(($id, data.to_vec()))
            }

            pub fn deserialize(buf: &mut bytes::BytesMut) -> $crate::error::DeRes<Self> {
                use $crate::types::Serialize;

                $(let $field: $ty = dbg!(Serialize::deserialize(buf)?);)*

                Ok(Self {
                    $($field),*
                })
            }
        }
    };
}

macro_rules! varint_enum {
    ($($name:ident { $($variant:ident = $value:literal),* });*) => {
        $(
            #[repr(i32)]
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum $name {
                $($variant = $value),*
            }

            impl $crate::types::Serialize for $name {
                fn serialize(&self, buf: &mut bytes::BytesMut) -> $crate::error::SerRes<()> {
                    use $crate::types::varint::VarInt;

                    let val = *self as i32;

                    let varint = VarInt(val);

                    varint.serialize(buf)
                }

                fn deserialize(buf: &mut bytes::BytesMut) -> $crate::error::DeRes<Self> {
                    use $crate::{error::DeserializeError, types::varint::VarInt};

                    let varint = VarInt::deserialize(buf)?;

                    match varint.0 {
                        $($value => Ok(Self::$variant),)*
                        _ => Err(DeserializeError::InvalidEnumVariant(stringify!($name), varint.0 as isize))
                    }
                }
            }
        )*
    }
}

macro_rules! int_enum {
    ($($name:ident($int:ty) { $($variant:ident = $value:literal),* });*) => {
        $(
            #[repr($int)]
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum $name {
                $($variant = $value),*
            }

            impl $crate::types::Serialize for $name {
                fn serialize(&self, buf: &mut bytes::BytesMut) -> $crate::error::SerRes<()> {
                    let val = *self as $int;

                    val.serialize(buf)
                }

                fn deserialize(buf: &mut bytes::BytesMut) -> $crate::error::DeRes<Self> {
                    use $crate::error::DeserializeError;

                    let val = <$int>::deserialize(buf)?;

                    match val {
                        $($value => Ok(Self::$variant),)*
                        _ => Err(DeserializeError::InvalidEnumVariant(stringify!($name), val as isize))
                    }
                }
            }
        )*
    }
}

macro_rules! data {
    ($($name:ident { $($field:ident: $ty:ty),* });*) => {
        $(
            #[derive(Debug, Clone)]
            pub struct $name {
                $(pub $field: $ty),*
            }

            impl $crate::types::Serialize for $name {
                fn serialize(&self, buf: &mut bytes::BytesMut) -> $crate::error::SerRes<()> {
                    $(self.$field.serialize(buf)?;)*
                    Ok(())
                }

                fn deserialize(buf: &mut bytes::BytesMut) -> $crate::error::DeRes<Self> {
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

macro_rules! nbt_data {
    ($($name:ident { $($field:ident: $ty:ty),* });*) => {
        $(
            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
            pub struct $name {
                $(pub $field: $ty),*
            }

            impl $crate::types::Serialize for $name {
                fn serialize(&self, buf: &mut bytes::BytesMut) -> $crate::error::SerRes<()> {
                    nbt::to_writer(&mut bytes::BufMut::writer(buf), &self, None).map_err(Into::into)
                }

                fn deserialize(buf: &mut bytes::BytesMut) -> $crate::error::DeRes<Self> {
                    nbt::from_reader(&mut bytes::Buf::reader(buf)).map_err(Into::into)
                }
            }
        )*
    }
}

pub(crate) use {data, int_enum, nbt_data, packet, packets, state, varint_enum};

mod test {
    #[test]
    fn nbt_data() {
        use crate::types::Serialize;
        use bytes::BytesMut;

        nbt_data! {
            NbtData {
                s: String,
                num: i32
            }
        }

        let nbt = NbtData {
            s: "foo".to_string(),
            num: 42,
        };

        let mut buf = BytesMut::new();

        nbt.serialize(&mut buf).unwrap();

        let nbt2: NbtData = crate::types::Serialize::deserialize(&mut buf).unwrap();

        assert_eq!(nbt.s, nbt2.s);
        assert_eq!(nbt.num, nbt2.num);
    }
}
