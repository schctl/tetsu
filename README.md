# Tetsu

`Tetsu` is a prototype interface to Minecraft's [server protocols](https://wiki.vg/Protocol) written in Rust. It allows for server/client communication across multiple versions using a common API. Currently supports logging into server versions `1.8.*` and `1.16.[4/5]`.

## Plans

The next few immediate goals for `Tetsu` are:

- Implementing `Play` packets for version 47 of the protocol.
- Better error propogation.
- Minimizing the dependency list.
- Better logging.
- A more robust way to implement packets.
