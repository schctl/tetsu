# Implementing packet types

All protocol version must be able to convert native types to corresponding `event::types`.

- If all protocol versions share the same implementation for a type in `event::types`, define its implementations directly in `common.rs`.
- If an `event::types` type has different implementations accross different versions, define a private "intermediate type" for the type in each protocol implementation and implement `From<"intermediate_type">` for the `"event_type"`, and vice-versa.
