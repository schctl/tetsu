# Tetsu

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/schctl/tetsu/Test?style=for-the-badge)](https://github.com/schctl/tetsu/actions/workflows/test.yml)
[![Crates.io](https://img.shields.io/crates/v/tetsu?style=for-the-badge)](https://crates.io/crates/tetsu)
[![Crates.io](https://img.shields.io/crates/l/tetsu?style=for-the-badge)](LICENSE)

`Tetsu` is a version agnostic implementation of Minecraft's [server protocols](https://wiki.vg/Protocol) written in Rust. Currently supports logging into server versions `1.8.*` and `1.16.[4/5]`.

## Plans

The next few immediate goals for `Tetsu` are:

- Implementing all `Play` packets for version 47 of the protocol.
- Minimizing the dependency list.
- A more robust way to implement packets.

<p><sup>
    <b>
        Note:
    </b>
    Some of the packet serialization code was referenced from
    <a href="https://github.com/iceiix/stevenarella">
        Stevenarella
    </a>.
</sup></p>
