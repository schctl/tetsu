# Tetsu

[<img alt="docs.rs" height=26 src="https://img.shields.io/docsrs/tetsu?style=for-the-badge&color=66c192&logo=Rust" />](https://docs.rs/tetsu)
[<img alt="License" height=26 src="https://img.shields.io/crates/l/tetsu?style=for-the-badge&color=66a7c1" />](LICENSE)
[<img alt="Workflow Status" height=26 src="https://img.shields.io/github/workflow/status/schctl/tetsu/Test?style=for-the-badge&logo=Github" />](https://github.com/schctl/tetsu/actions/workflows/test.yml)

`Tetsu` is a highly experimental version agnostic implementation of Minecraft's [server protocols](https://wiki.vg/Protocol) written in Rust. I'm currently trying to make this work with server versions `1.8.*` and `1.16.*`.

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
