//! Commonly used macros.

macro_rules! auto_read_and_write_impl {
    (
        (read: $read_trait:path, $read_fn:ident;
        write: $write_trait:path, $write_fn:ident) => {
            $(
                {
                    $name:ident,
                    $(
                        $field:ident: $type:ty,
                    )*
                }
            )*
        }
    ) => {
        #[allow(unused_imports)]
        use crate::errors::*;
        use $crate::serialization::{
            Readable as __auto_read_write_Readable,
            Writable as __auto_read_write_Writable
        };

        $(
            impl $read_trait for $name {
                #[inline]
                fn $read_fn<__T: std::io::Read>(_buf: &mut __T) -> TetsuResult<Event> {
                    Ok(Event::$name(Self {
                        $($field: <$type>::read_from(_buf)?.into(),)*
                    }))
                }
            }

            impl $write_trait for $name {
                #[inline]
                fn $write_fn<T: std::io::Write>(&self, _buf: &mut T) -> TetsuResult<()> {
                    $(
                        <$type>::from(self.$field.clone()).write_to(_buf)?;
                    )*
                    Ok(())
                }
            }
        )*
    }
}

macro_rules! new_protocol_impl {
    (
        $(
            (read: $read_trait:path, $read_fn:ident;
            write: $write_trait:path, $write_fn:ident) => {
                $(
                    ($id:expr, $direction:ident, $state:ident) => $name:ident,
                )*
            }
        )*
    ) => {
        #[allow(unused_imports)]
        use std::io::prelude::*;

        use flate2::{
            Compression as __new_protocol_impl_Compression,
            {
                write::ZlibEncoder as __new_protocol_impl_ZlibEncoder,
                read::ZlibDecoder as __new_protocol_impl_ZlibDecoder,
            }
        };

        #[allow(unused_imports)]
        use $crate::event::{
            EventDirection as __new_protocol_impl_EventDirection,
            EventState as __new_protocol_impl_EventState
        };
        #[allow(unused_imports)]
        use $crate::serialization::{
            Readable as __new_protocol_impl_Readable,
            Writable as __new_protocol_impl_Writable
        };

        /// Read event implementation for this protocol version.
        pub fn read_event<__T: std::io::Read>(
            buf: &mut __T,
            state: &__new_protocol_impl_EventState,
            direction: &__new_protocol_impl_EventDirection,
            compression_threshold: i32
        ) -> TetsuResult<Event> {

            $(use $read_trait;)*

            let total_len = VarInt::read_from(buf)?.0 as usize;

            let mut bytes = vec![0; total_len];
            buf.read_exact(&mut bytes)?;
            let mut bytes = std::io::Cursor::new(bytes);

            if compression_threshold > 0 {
                let uncompressed_size = VarInt::read_from(&mut bytes)?.0;

                if uncompressed_size > 0 {
                    let mut new_bytes = vec![0; uncompressed_size as usize];
                    let mut reader = __new_protocol_impl_ZlibDecoder::new(bytes);

                    reader.read_exact(&mut new_bytes)?;
                    bytes = std::io::Cursor::new(new_bytes);
                }
            } else {
                let mut new_bytes = vec![0; total_len];
                bytes.read_exact(&mut new_bytes)?;
                bytes = std::io::Cursor::new(new_bytes);
            }

            let id = VarInt::read_from(&mut bytes)?.0;

            Ok(match (&id, &direction, &state) {
                $($(
                    ($id, __new_protocol_impl_EventDirection::$direction, __new_protocol_impl_EventState::$state)
                    => $name::$read_fn(&mut bytes)?,
                )*)*
                _ => return Err(
                    Error::from(InvalidValue { expected: format!("Unknown packet [{:#x}]:[{:?}]:[{:?}]", id, direction, state) })
                )
            })

        }

        pub fn write_event<__T: std::io::Write>(
            buf: &mut __T,
            event: Event,
            _state: &__new_protocol_impl_EventState,
            _direction: &__new_protocol_impl_EventDirection,
            compression_threshold: i32
        ) -> TetsuResult<()> {
            $(use $write_trait;)*

            let mut _buf = Vec::new();

            match event {
                $($(
                    Event::$name(e) => {
                        VarInt($id as i32).write_to(&mut _buf)?;
                        e.$write_fn(&mut _buf)?
                    },
                )*)*
                _ => return Err(
                    Error::from(InvalidValue { expected: format!("Event: {:#?} is unimplemented", event) })
                )
            };

            Ok(if compression_threshold > 0 {
                let uncompressed_len = _buf.len();
                let mut compressed = Vec::new();
                let mut writer = __new_protocol_impl_ZlibEncoder::new(std::io::Cursor::new(_buf), __new_protocol_impl_Compression::default());

                VarInt(uncompressed_len as i32).write_to(&mut compressed)?;
                writer.read_to_end(&mut compressed)?;
                VarInt(compressed.len() as i32).write_to(buf)?;
                buf.write_all(&compressed)?;
            } else {
                VarInt(_buf.len() as i32).write_to(buf)?;
                buf.write_all(&_buf)?;
            })
        }
    }
}
