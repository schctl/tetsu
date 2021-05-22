//! Log into a 1.8.*/1.16.[4/5] server.

use std::env;
use std::thread;
use std::time;

use tetsu::client;
use tetsu::errors;

fn main() {
    env_logger::builder()
        .filter(Some("tetsu"), log::LevelFilter::Debug)
        .init();

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
                errors::ConnectionError::Error(e) => panic!("Error while reading event: {:?}", e),
            },
        }
    }
}
