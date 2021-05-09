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

server.connect_player(user).unwrap();

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

#[macro_use]
mod packet;
mod encryption;
pub mod errors;
pub mod event;
pub mod server;
#[allow(dead_code)]
pub mod user;
mod versions;
