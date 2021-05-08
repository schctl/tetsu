//! Interface to Minecraft's server protocol.

#![warn(missing_docs)]

#[macro_use]
mod packet;
mod encryption;
mod errors;
pub mod event;
pub mod server;
#[allow(dead_code)]
pub mod user;
mod versions;

#[cfg(test)]
mod type_tests {
    use std::io::Cursor;

    use super::*;
    use packet::{Readable, Writable};

    macro_rules! test_type_serialization {
        ( $type:ty, $val:expr ) => {
            let x = $val;
            // ----------------
            let mut f = Cursor::new(vec![]);
            x.write_to(&mut f).unwrap();
            println!("{:?}", f);
            // ----------------
            f.set_position(0);
            let y = <$type>::read_from(&mut f).unwrap();
            // ----------------
            assert!(x == y)
        };
    }

    #[test]
    fn test_serialization() {
        test_type_serialization!(packet::VarInt, packet::VarInt(10));
        test_type_serialization!(u16, 10);
        test_type_serialization!(String, String::from("10"));
    }
}
