//! Useful macros.

/// Autoimplement packets for a protocol version.
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
        use std::io::prelude::*;
        use std::convert::TryFrom;
        use std::convert::TryInto;

        use flate2::{Compression, {write::ZlibEncoder, read::ZlibDecoder}};

        use $crate::event;
        use $crate::serialization::{Readable, Writable};

        pub fn read_event<__T: std::io::Read>(
            buf: &mut __T,
            state: &event::EventState,
            direction: &event::EventDirection,
            compression_threshold: i32
        ) -> $crate::TetsuResult<event::Event> {
            let mut bytes = vec![0; $crate::versions::common::VarInt::read_from(buf)?.0 as usize];

            buf.read_exact(&mut bytes)?;
            let mut bytes = std::io::Cursor::new(bytes);

            if compression_threshold > 0 {
                let uncompressed_size = $crate::versions::common::VarInt::read_from(&mut bytes)?.0;

                if uncompressed_size > 0 {
                    let mut new_bytes = Vec::with_capacity(uncompressed_size as usize);
                    let mut reader = ZlibDecoder::new(bytes);

                    reader.read_to_end(&mut new_bytes)?;
                    bytes = std::io::Cursor::new(new_bytes);
                }
            }

            let id = $crate::versions::common::VarInt::read_from(&mut bytes)?.0;

            #[allow(unreachable_patterns)]
            match (&id, direction, state) {
                $(
                    (&<$inherit>::ID, &<$inherit>::DIRECTION, &<$inherit>::STATE) => {
                        Ok(<$inherit>::read_from(&mut bytes)?.try_into()?)
                    },
                )*
                $(
                    (&$id, &event::EventDirection::$direction, &event::EventState::$state) => {
                        Ok($name::read_from(&mut bytes)?.try_into()?)
                    },
                )*
                _ => {
                    Err($crate::errors::Error::from($crate::errors::InvalidValue {
                        expected: format!("not packet: [{:x}]:{:?}:{:?}", id, direction, state)
                    }))
                }
            }
        }

        pub fn write_event<__T: std::io::Write>(
            buf: &mut __T,
            event: event::Event,
            _state: &event::EventState,
            _direction: &event::EventDirection,
            compression_threshold: i32
        ) -> $crate::TetsuResult<()> {
            let mut _buf = Vec::new();

            #[allow(unreachable_patterns)]
            match event {
                $(
                    event::Event::$inherit_event(origin) => {
                        <$inherit>::try_from(origin)?.write_to(&mut _buf)?
                    },
                )*
                $(
                    event::Event::$event_type(origin) => {
                        $name::try_from(origin)?.write_to(&mut _buf)?
                    },
                )*
                _ => {
                    return Err($crate::errors::Error::from($crate::errors::InvalidValue {
                        expected: format!("not event: {:?}", event)
                    }))
                }
            }

            Ok(if compression_threshold > 0 {
                let uncompressed_len = _buf.len();
                let mut compressed = Vec::new();
                let mut writer = ZlibEncoder::new(std::io::Cursor::new(_buf), Compression::default());

                $crate::versions::common::VarInt(uncompressed_len as i32).write_to(&mut compressed)?;
                writer.read_to_end(&mut compressed)?;
                $crate::versions::common::VarInt(compressed.len() as i32).write_to(buf)?;
                buf.write_all(&compressed)?;
            } else {
                $crate::versions::common::VarInt(_buf.len() as i32).write_to(buf)?;
                buf.write_all(&_buf)?;
            })
        }

        $(
            #[derive(Debug, PartialEq)]
            pub struct $name {
                $(
                    $field_name: $field_type,
                )*
            }

            impl $crate::serialization::Packet for $name {
                const ID: i32 = $id;
                const DIRECTION: event::EventDirection = event::EventDirection::$direction;
                const STATE: event::EventState = event::EventState::$state;
            }

            impl Readable for $name {
                #[inline]
                fn read_from<__T: std::io::Read>(_buf: &mut __T) -> $crate::TetsuResult<Self> {
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
                fn write_to<__T: std::io::Write>(&self, buf: &mut __T) -> $crate::TetsuResult<()> {
                    $crate::versions::common::VarInt(Self::ID).write_to(buf)?;
                    $(
                        self.$field_name.write_to(buf)?;
                    )*
                    Ok(())
                }
            }

            impl TryFrom<$event_type> for $name {
                type Error = $crate::errors::Error;

                #[inline]
                $from_event
            }

            impl TryFrom<$name> for event::Event {
                type Error = $crate::errors::Error;

                #[inline]
                $to_event
            }
        )*
    };
}
