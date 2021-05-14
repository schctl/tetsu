# 鉄 Tetsu

[<img alt="docs.rs" height=26 src="https://img.shields.io/crates/v/tetsu?style=for-the-badge&color=9a7155&logo=Rust&label=Docs" />](https://docs.rs/tetsu)
[<img alt="License" height=26 src="https://img.shields.io/crates/l/tetsu?style=for-the-badge&color=69868e" />](LICENSE)
[<img alt="Workflow Status" height=26 src="https://img.shields.io/github/workflow/status/schctl/tetsu/Test?style=for-the-badge&logo=Github" />](https://github.com/schctl/tetsu/actions/workflows/test.yml)

`Tetsu` is a highly experimental version agnostic implementation of Minecraft's [server protocols](https://wiki.vg/Protocol) written in Rust. I'm currently trying to make this work with server versions `1.8.*` and `1.16.*`.

## Plans

The next few immediate goals for `Tetsu` are:

- More tests.
- Optimizing impls.
- Implementing all `Play` packets for version 47 of the protocol before `v0.1.0`.
- Minimizing the dependency list.
- A macro crate for `derive(Packet)` and `protocol_impl`.

## Examples

### Logging into a server

```rust
use std::env;

use tetsu::errors;
use tetsu::mojang;
use tetsu::server;

fn main() {
    let user = mojang::User::authenticate(
        env::var("MOJANG_USER").unwrap(),
        env::var("MOJANG_USER_PWD").unwrap(),
    );

    let mut server = server::Server::new("127.0.0.1", None, None).unwrap();

    server.connect_user(user).unwrap();

    loop {
        match server.read_event() {
            Ok(e) => println!("{:?}", e),
            _ => {}
        }
    }
}
```

<sup>
<p>
    <b>
        Note:
    </b>
    Some of the packet serialization code was referenced from
    <a href="https://github.com/iceiix/stevenarella">
        iceiix/Stevenarella
    </a>
</p>
</sup>
