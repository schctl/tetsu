//! Packet specific tools.

use crate::event::*;

mod types;
pub use types::*;

pub trait Packet: Readable + Writable {
    const ID: i32;
    const DIRECTION: EventDirection;
    const STATE: EventState;
}

/// Autoimplement packets for a protocol version.
#[macro_export]
macro_rules! protocol_impl {
    (
        inherit {
            $(
                $inherit:ty: $inherit_event:ident;
            )*
        }

        $(
            ($id:expr) $direction:ident $state:ident $name:ident: $event_type:ident {
                from_event {
                    $from_event:item
                }
                to_event {
                    $to_event:item
                }

                fields {
                    $($field_name:ident: $field_type:ty,)*
                }
            }
        )*
    ) => {
        #[allow(unused_imports)]
        use std::io::prelude::*;
        #[allow(unused_imports)]
        use flate2::{Compression, {write::ZlibEncoder, read::ZlibDecoder}};
        #[allow(unused_imports)]
        use crate::{packet::*, errors::*};
        #[allow(unused_imports)]
        use log::{debug, error, info, warn};

        use std::convert::TryFrom;
        use std::convert::TryInto;

        /// Implementation for converting protocol-specific calls to `Event`s
        /// for protocol version $version.
        pub fn read_event<T: std::io::Read>(
            buf: &mut T,
            state: &EventState,
            direction: &EventDirection,
            compression_threshold: i32
        ) -> TetsuResult<Event> {
            let mut bytes = vec![0; VarInt::read_from(buf)?.0 as usize];

            buf.read_exact(&mut bytes)?;
            let mut bytes = std::io::Cursor::new(bytes);

            if compression_threshold > 0 {
                let uncompressed_size = VarInt::read_from(&mut bytes)?.0;

                if uncompressed_size > 0 {
                    let mut new_bytes = Vec::with_capacity(uncompressed_size as usize);
                    let mut reader = ZlibDecoder::new(bytes);

                    reader.read_to_end(&mut new_bytes)?;
                    bytes = std::io::Cursor::new(new_bytes);
                }
            }

            let id = VarInt::read_from(&mut bytes)?.0;

            #[allow(unreachable_patterns)]
            match (&id, direction, state) {
                $(
                    (&<$inherit>::ID, &<$inherit>::DIRECTION, &<$inherit>::STATE) => {
                        Ok(<$inherit>::read_from(&mut bytes)?.try_into()?)
                    },
                )*
                $(
                    (&$id, &EventDirection::$direction, &EventState::$state) => {
                        Ok($name::read_from(&mut bytes)?.try_into()?)
                    },
                )*
                _ => {
                    Err(Error::from(InvalidValue {
                        expected: format!("not packet: [{:x}]:{:?}:{:?}", id, direction, state)
                    }))
                }
            }
        }

        /// Implementation for converting `Event`s to protocol-specific calls.
        pub fn write_event<T: std::io::Write>(
            buf: &mut T,
            event: Event,
            _state: &EventState,
            _direction: &EventDirection,
            compression_threshold: i32
        ) -> TetsuResult<()> {
            let mut _buf = Vec::new();

            #[allow(unreachable_patterns)]
            match event {
                $(
                    Event::$inherit_event(origin) => {
                        <$inherit>::try_from(origin)?.write_to(&mut _buf)?
                    },
                )*
                $(
                    Event::$event_type(origin) => {
                        $name::try_from(origin)?.write_to(&mut _buf)?
                    },
                )*
                _ => {
                    return Err(Error::from(InvalidValue {
                        expected: format!("not event: {:?}", event)
                    }))
                }
            }

            Ok(if compression_threshold > 0 {
                let uncompressed_len = _buf.len();
                let mut compressed = Vec::new();
                let mut writer = ZlibEncoder::new(std::io::Cursor::new(_buf), Compression::default());

                VarInt(uncompressed_len as i32).write_to(&mut compressed)?;
                writer.read_to_end(&mut compressed)?;
                VarInt(compressed.len() as i32).write_to(buf)?;
                buf.write_all(&compressed)?;
            } else {
                VarInt(_buf.len() as i32).write_to(buf)?;
                buf.write_all(&_buf)?;
            })
        }

        $(
            /// Protocol implementation for packet $name.
            #[derive(Debug, PartialEq)]
            pub struct $name {
                $(
                    $field_name: $field_type,
                )*
            }

            impl Packet for $name {
                const ID: i32 = $id;
                const DIRECTION: EventDirection = EventDirection::$direction;
                const STATE: EventState = EventState::$state;
            }

            impl Readable for $name {
                #[inline]
                fn read_from<T: std::io::Read>(_buf: &mut T) -> Result<Self, Error> {
                    Ok(Self {
                        $(
                            $field_name: <$field_type>::read_from(_buf)?,
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

            impl TryFrom<$event_type> for $name {
                type Error = Error;

                #[inline]
                $from_event
            }

            impl TryFrom<$name> for Event {
                type Error = Error;

                #[inline]
                $to_event
            }
        )*
    };
}
