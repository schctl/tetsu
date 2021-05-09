# Tetsu

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/schctl/tetsu/Test?style=for-the-badge)
![Crates.io](https://img.shields.io/crates/v/tetsu?style=for-the-badge)
![Crates.io](https://img.shields.io/crates/l/tetsu?style=for-the-badge)

`Tetsu` is a version agnostic implementation of Minecraft's [server protocols](https://wiki.vg/Protocol) written in Rust. Currently supports logging into server versions `1.8.*` and `1.16.[4/5]`.

## Plans

The next few immediate goals for `Tetsu` are:

- Implementing `Play` packets for version 47 of the protocol.
- Better error propogation.
- Minimizing the dependency list.
- Better logging.
- A more robust way to implement packets.
