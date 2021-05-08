//! Packet specific tools.

use crate::event::Event;

mod serializable;
pub use serializable::*;

pub(crate) trait Packet {
    const ID: i32;
    const DIRECTION: PacketDirection;
    const STATE: PacketState;

    type Item;

    fn into_event(self) -> Event;
    fn from_event(event: Self::Item) -> Self;
}

/// Different connection states.
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PacketState {
    Test,
    Status,
    Handshake,
    Login,
    Play,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PacketDirection {
    ClientBound,
    ServerBound,
}

#[macro_export]
macro_rules! packet_impl {
    (
        inherit {
            $(
                $inherit:ty: $inherit_event:ident;
            )*
        }

        $(
            ($id:expr) $direction:ident $state:ident $name:ident: $event_type:ident {
                from_event {
                    $from_event:expr
                }
                to_event {
                    $to_event:expr
                }

                fields {
                    $($field_name:ident: $field_type:ty,)*
                }
            }
        )*
    ) => {
        use std::io::prelude::*;

        use flate2::Compression;
        use flate2::{write::ZlibEncoder, read::ZlibDecoder};

        #[allow(unused_imports)]
        use crate::packet::*;
        use crate::errors::Error;

        /// Implementation for converting protocol-specific calls to `Event`s.
        pub fn read_event
            <T: std::io::Read>
            (buf: &mut T, state: &PacketState, direction: &PacketDirection, compression_threshold: i32)
            -> Result<Event, Error>
        {
            let mut bytes = vec![0; VarInt::read_from(buf)?.0 as usize];
            buf.read_exact(&mut bytes)?;

            let mut bytes = std::io::Cursor::new(bytes);

            if compression_threshold > 0 {
                let uncompressed_size = VarInt::read_from(&mut bytes)?.0;

                if uncompressed_size > 0 {
                    let mut reader = ZlibDecoder::new(bytes);
                    let mut new_bytes = Vec::with_capacity(uncompressed_size as usize);
                    reader.read_to_end(&mut new_bytes)?;
                    bytes = std::io::Cursor::new(new_bytes);
                }
            }

            let id = VarInt::read_from(&mut bytes)?.0;

            #[allow(unreachable_pattern)]
            match (&id, direction, state) {
                $(
                    (&<$inherit>::ID, &<$inherit>::DIRECTION, &<$inherit>::STATE) => {
                        Ok(<$inherit>::read_from(&mut bytes)?.into_event())
                    },
                )*
                $(
                    (&$id, &PacketDirection::$direction, &PacketState::$state) => {
                        Ok($name::read_from(&mut bytes)?.into_event())
                    },
                )*
                _ => panic!("Unknown packet: [{:x}]:{:?}:{:?}", id, direction, state),
            }
        }

        /// Implementation for converting `Event`s to protocol-specific calls.
        pub fn write_event
            <T: std::io::Write>
            (event: Event, buf: &mut T, compression_threshold: i32)
            -> Result<(), Error>
        {
            let mut _buf = Vec::new();

            #[allow(unreachable_patterns)]
            match event {
                $(
                    Event::$inherit_event(origin) => {
                        <$inherit>::from_event(origin).write_to(&mut _buf)?
                    },
                )*
                $(
                    Event::$event_type(origin) => {
                        $name::from_event(origin).write_to(&mut _buf)?
                    },
                )*
                _ => panic!("Unknown packet"),
            }

            if compression_threshold <= 0 {
                VarInt(_buf.len() as i32).write_to(buf)?;
                buf.write_all(&_buf)?;
            } else {
                let uncompressed_len = _buf.len();
                let mut compressed = vec![];
                VarInt(uncompressed_len as i32).write_to(&mut compressed)?;
                let mut writer = ZlibEncoder::new(std::io::Cursor::new(_buf), Compression::default());
                writer.read_to_end(&mut compressed)?;
                VarInt(compressed.len() as i32).write_to(buf)?;
                buf.write_all(&compressed)?;
            }

            Ok(())
        }

        $(
            /// Protocol implementation for packet $name.
            #[derive(Debug, PartialEq)]
            pub struct $name {
                $(
                    pub $field_name: $field_type,
                )*
            }

            impl Packet for $name {
                const ID: i32 = $id;
                const DIRECTION: PacketDirection = PacketDirection::$direction;
                const STATE: PacketState = PacketState::$state;

                type Item = $event_type;

                #[inline]
                fn into_event(self) -> Event {
                    $to_event(self)
                }

                #[inline]
                fn from_event(event: $event_type) -> Self {
                    $from_event(event)
                }
            }

            impl Readable for $name {
                #[inline]
                #[allow(unused_variables)]
                fn read_from<T: std::io::Read>(buf: &mut T) -> Result<Self, Error> {
                    Ok(Self {
                        $(
                            $field_name: <$field_type>::read_from(buf)?,
                        )*
                    })
                }
            }

            impl Writable for $name {
                #[inline]
                #[allow(unused_variables)]
                fn write_to<T: std::io::Write>(&self, buf: &mut T) -> Result<(), Error> {
                    VarInt(Self::ID).write_to(buf)?;
                    $(
                        self.$field_name.write_to(buf)?;
                    )*
                    Ok(())
                }
            }
        )*
    };
}
