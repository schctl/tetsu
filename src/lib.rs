/*!
Interface to Minecraft's server protocol.

# Examples
```no_run
use std::env;
use std::thread;
use std::time;

use tetsu::errors;
use tetsu::server;
use tetsu::user;

let user = user::User::authenticate(
    env::var("MOJANG_USER").unwrap(),
    env::var("MOJANG_USER_PWD").unwrap(),
);

let mut server = server::Server::new("127.0.0.1", None, None).unwrap();

server.connect_player(&user).unwrap();

loop {
    match server.read_event() {
        Ok(e) => println!("{:?}", e),
        Err(e) => match e {
            errors::ConnectionError::LockError(_) => {
                thread::sleep(time::Duration::from_millis(50));
                continue;
            }
            errors::ConnectionError::Error(e) => panic!("Error while reading event: {:?}", e),
        },
    }
}
```
*/

// #![warn(missing_docs)]

#[macro_use]
mod packet;
mod encryption;
pub mod errors;
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
