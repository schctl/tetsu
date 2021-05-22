//! Event read/write tools.

use crate::event::*;

/// Wrapper around protocol specific event read/write impls.
pub struct EventDispatcher<R: std::io::Read, W: std::io::Write> {
    reader: Box<dyn Fn(&mut R, &EventState, &EventDirection, i32) -> TetsuResult<Event>>,
    writer: Box<dyn Fn(&mut W, Event, &EventState, &EventDirection, i32) -> TetsuResult<()>>,
}

unsafe impl<R: std::io::Read, W: std::io::Write> Send for EventDispatcher<R, W> {}
unsafe impl<R: std::io::Read, W: std::io::Write> Sync for EventDispatcher<R, W> {}

impl<R: std::io::Read, W: std::io::Write> EventDispatcher<R, W> {
    /// Create a new event dispatcher using protocol `version`.
    #[inline]
    pub fn new(version: &ProtocolVersion) -> Self {
        match version {
            ProtocolVersion::V47 => Self {
                reader: versions::v47::get_read_callback(),
                writer: versions::v47::get_write_callback(),
            },
            ProtocolVersion::V754 => Self {
                reader: versions::v754::get_read_callback(),
                writer: versions::v754::get_write_callback(),
            },
        }
    }

    /// Create a new [`EventDispatcher`] from any read/write functions.
    #[inline]
    pub fn new_from_raw(
        reader: Box<dyn Fn(&mut R, &EventState, &EventDirection, i32) -> TetsuResult<Event>>,
        writer: Box<dyn Fn(&mut W, Event, &EventState, &EventDirection, i32) -> TetsuResult<()>>,
    ) -> TetsuResult<Self> {
        Ok(Self { reader, writer })
    }

    /// Read an event from the buffer.
    #[inline]
    pub fn read_event(
        &self,
        buf: &mut R,
        state: &EventState,
        direction: &EventDirection,
        compression_threshold: i32,
    ) -> TetsuResult<Event> {
        (self.reader)(buf, state, direction, compression_threshold)
    }

    /// Write an event to the buffer.
    #[inline]
    pub fn write_event(
        &self,
        buf: &mut W,
        event: Event,
        state: &EventState,
        direction: &EventDirection,
        compression_threshold: i32,
    ) -> TetsuResult<()> {
        (self.writer)(buf, event, state, direction, compression_threshold)
    }
}
