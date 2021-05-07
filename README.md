# Tetsu

`Tetsu` is a prototype interface to Minecraft's [server protocols](https://wiki.vg/Protocol) written in Rust. It allows for server/client communication across multiple versions using a common API.

## Support

Currently supported features:

- Currently supports logging into server versions `1.8.*` and `1.16.[4/5]`.
- A high level wrapper around a Minecraft server connection.

## Plans

The next few immediate goals for `Tetsu` are:

- Implementing `Play` packets for version 47 of the protocol.
- Error propogation.
- Minimizing the dependency list.
- Logging.
- A more robust way to implement packets.
