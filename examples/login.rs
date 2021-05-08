//! Log into a 1.8.*/1.16.[4/5] server.

use std::env;

use tetsu::server;
use tetsu::user;

fn main() {
    let user = user::User::authenticate(
        env::var("MOJANG_USER").unwrap(),
        env::var("MOJANG_USER_PWD").unwrap(),
    );

    let mut server = server::Server::new("127.0.0.1", None, None).unwrap();

    server.connect_player(&user).unwrap();

    loop {
        println!("{:?}", server.read_event().unwrap());
    }
}
