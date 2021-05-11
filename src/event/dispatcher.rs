use crate::event::*;

/// Wrapper around protocol specific event read/write impls.
pub struct EventDispatcher<R: std::io::Read, W: std::io::Write> {
    reader: fn(&mut R, &EventState, &EventDirection, i32) -> TetsuResult<Event>,
    writer: fn(&mut W, Event, &EventState, &EventDirection, i32) -> TetsuResult<()>,
}

impl<R: std::io::Read, W: std::io::Write> EventDispatcher<R, W> {
    /// Create a new event dispatcher using protocol `version`.
    #[inline]
    pub fn new(version: &ProtocolVersion) -> Self {
        match version {
            ProtocolVersion::V47 => Self {
                reader: versions::v47::read_event,
                writer: versions::v47::write_event,
            },
            ProtocolVersion::V754 => Self {
                reader: versions::v754::read_event,
                writer: versions::v754::write_event,
            },
        }
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
