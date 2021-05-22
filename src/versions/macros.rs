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
            Readable as _auto_rw_Readable,
            Writable as _auto_rw_Writable
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
            Compression as _p_impl_Compression,
            {
                write::ZlibEncoder as _p_impl_ZlibEncoder,
                read::ZlibDecoder as _p_impl_ZlibDecoder,
            }
        };

        #[allow(unused_imports)]
        use $crate::event::{
            Event as _p_impl_Event,
            EventDirection as _p_impl_EventDirection,
            EventState as _p_impl_EventState
        };
        #[allow(unused_imports)]
        use $crate::serialization::{
            Readable as _p_impl_Readable,
            Writable as _p_impl_Writable
        };

        /// Get the read event callback.
        #[inline]
        pub fn get_read_callback<__T: std::io::Read>() -> Box<dyn Fn(&mut __T, &EventState, &EventDirection, i32) -> TetsuResult<Event>> {
            $(use $read_trait;)*

            Box::new(| buf: &mut __T, state: &_p_impl_EventState, direction: &_p_impl_EventDirection, compression_threshold: i32 | {
                let total_len = VarInt::read_from(buf)?.0 as usize;

                let mut bytes = vec![0; total_len];
                buf.read_exact(&mut bytes)?;
                let mut bytes = &bytes[..];

                let mut uncompressed_bytes;

                if compression_threshold > 0 {
                    let uncompressed_len = VarInt::read_from(&mut bytes)?.0;

                    if uncompressed_len > 0 {
                        uncompressed_bytes = vec![0; uncompressed_len as usize];
                        let mut reader = _p_impl_ZlibDecoder::new(bytes);
                        reader.read_exact(&mut uncompressed_bytes)?;
                        bytes = &uncompressed_bytes[..];
                    }
                }

                let id = VarInt::read_from(&mut bytes)?.0;

                Ok(match (&id, &direction, &state) {
                    $($(
                        ($id, _p_impl_EventDirection::$direction, _p_impl_EventState::$state)
                        => $name::$read_fn(&mut bytes)?,
                    )*)*
                    _ => return Err(
                        Error::from(InvalidValue { expected: format!("Unknown packet [{:#x}]:[{:?}]:[{:?}]", id, direction, state) })
                    )
                })
            })
        }

        /// Get the write event callback
        #[inline]
        pub fn get_write_callback<__T: std::io::Write>() -> Box<dyn Fn(&mut __T, Event, &_p_impl_EventState, &_p_impl_EventDirection, i32) -> TetsuResult<()>> {
            $(use $write_trait;)*

            Box::new(| buf: &mut __T, event: _p_impl_Event, _state: &_p_impl_EventState, _direction: &_p_impl_EventDirection, compression_threshold: i32 | {
                let mut bytes = vec![];

                match event {
                    $($(
                        Event::$name(e) => {
                            VarInt($id).write_to(&mut bytes)?;
                            e.$write_fn(&mut bytes)?
                        },
                    )*)*
                    _ => return Err(
                        Error::from(InvalidValue { expected: format!("Event: {:#?} is unimplemented", event) })
                    )
                };

                if compression_threshold > 0 {
                    let uncompressed_len = bytes.len() as i32;
                    let mut uncompressed_buf = bytes.clone();

                    bytes.clear();

                    let mut writer = _p_impl_ZlibEncoder::new(&mut uncompressed_buf[..], _p_impl_Compression::default());

                    VarInt(uncompressed_len).write_to(&mut bytes)?;
                    writer.write_all(&mut bytes)?;
                }

                VarInt(bytes.len() as i32).write_to(buf)?;
                Ok(buf.write_all(&bytes)?)
            })
        }
    }
}
