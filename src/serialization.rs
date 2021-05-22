//! Serialization utils.

use crate::errors::*;
use crate::event::*;

/// Something that can be read from a buffer.
pub trait Readable: Sized {
    fn read_from<T: std::io::Read>(buf: &mut T) -> TetsuResult<Self>;
}

/// Something that can be written to a buffer.
pub trait Writable: Sized {
    fn write_to<T: std::io::Write>(&self, buf: &mut T) -> TetsuResult<()>;
}

/// Packet information.
pub trait Packet: Readable + Writable {
    const ID: i32;
    const DIRECTION: EventDirection;
    const STATE: EventState;
}
