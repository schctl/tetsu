/*!
High level interface to Minecraft's server protocols.

# Examples
```no_run
use std::env;
use std::thread;
use std::time;

use tetsu::errors;
use tetsu::client;

let user = client::mojang::User::authenticate(
    env::var("MOJANG_USER").unwrap(),
    env::var("MOJANG_USER_PWD").unwrap(),
);

let mut client = client::Client::new("127.0.0.1", None, None).unwrap();
client.connect_user(user).unwrap();

loop {
    match client.read_event() {
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
#![doc(html_favicon_url = "https://raw.githubusercontent.com/schctl/tetsu/master/res/favicon.ico")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/schctl/tetsu/master/res/logo.png")]

pub mod client;
pub mod crypto;
pub mod errors;
pub mod event;
pub mod serialization;
mod versions;

pub use errors::TetsuResult;

#[cfg(test)]
mod tests;
