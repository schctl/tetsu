/*!
High level interface to Minecraft's server protocols.

# Examples
```no_run
use std::env;
use std::thread;
use std::time;

use tetsu::errors;
use tetsu::server;
use tetsu::mojang;

let user = mojang::User::authenticate(
    env::var("MOJANG_USER").unwrap(),
    env::var("MOJANG_USER_PWD").unwrap(),
);

let mut server = server::Server::new("127.0.0.1", None, None).unwrap();
server.connect_user(user).unwrap();

loop {
    match server.read_event() {
        Ok(e) => println!("{:?}", e),
        Err(e) => match e {
            errors::ConnectionError::LockError(_) => {
                thread::sleep(time::Duration::from_millis(50));
                continue;
            }
            errors::ConnectionError::Error(e) =>
                panic!("Error while reading event: {:?}", e),
        },
    }
}
```
*/

#![allow(dead_code)]

#[macro_use]
mod packet;
pub mod crypto;
pub mod errors;
pub mod event;
pub mod mojang;
pub mod server;
mod versions;

pub use errors::TetsuResult;

#[cfg(test)]
mod tests;
