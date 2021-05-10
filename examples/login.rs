//! Log into a 1.8.*/1.16.[4/5] server.

use std::env;
use std::thread;
use std::time;

use tetsu::errors;
use tetsu::mojang;
use tetsu::server;

fn main() {
    let user = mojang::User::authenticate(
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
}
