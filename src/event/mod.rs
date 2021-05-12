/*!
Server-client communication types.

# Examples

## Using an `EventDispatcher` to send events.
```
use std::io::Cursor;
use tetsu::event::*;

let mut connection = Cursor::new(Vec::new());
let dispatcher: dispatcher::EventDispatcher<Cursor<Vec<u8>>, Cursor<Vec<u8>>> =
    dispatcher::EventDispatcher::new(&ProtocolVersion::V47);

// ...

let handshake = Event::Handshake(Handshake {
    server_address: "127.0.0.1".to_owned(),
    server_port: 25565,
    next_state: EventState::Login,
});

dispatcher.write_event(
    &mut connection,
     handshake,
    &EventState::Handshake,
    &EventDirection::ServerBound,
    0,
);
```
*/

#[allow(unused_imports)]
use log::{debug, error, info, warn};

use crate::errors::*;
use crate::packet::*;
use crate::versions;

pub mod dispatcher;
mod types;

pub use types::*;

/**
Non exhaustive list of all events that can be sent and received.

# Examples
```no_run
use std::io::Cursor;
use tetsu::event::*;

let mut buf = Cursor::new(Vec::new());
let dispatcher: dispatcher::EventDispatcher<Cursor<Vec<u8>>, Cursor<Vec<u8>>> = dispatcher::EventDispatcher::new(&ProtocolVersion::V47);

// ...

let event = dispatcher.read_event(
    &mut buf,
    &EventState::Status,
    &EventDirection::ClientBound,
    0,
).unwrap();

match event {
    Event::Pong(e) => {},
    Event::StatusResponse(e) => {},
    _ => {}
}
```
*/
#[non_exhaustive]
#[derive(Debug, PartialEq, Clone)]
pub enum Event {
    Ping(Ping),
    Pong(Pong),
    StatusRequest(StatusRequest),
    StatusResponse(StatusResponse),

    Handshake(Handshake),

    LoginStart(LoginStart),
    Disconnect(Disconnect),
    EncryptionRequest(EncryptionRequest),
    EncryptionResponse(EncryptionResponse),
    LoginSuccess(LoginSuccess),
    SetCompression(SetCompression),

    KeepAlive(KeepAlive),
    JoinGame(JoinGame),
    SpawnPosition(SpawnPosition),
    HeldItemChange(HeldItemChange),
    Statistics(Statistics),
    PlayerAbility(PlayerAbility),
    PluginMessage(PluginMessage),
    ServerDifficultyUpdate(ServerDifficultyUpdate),
}

unsafe impl Send for Event {}
unsafe impl Sync for Event {}

// Status ----------

/// Ping the server to make sure its alive.
#[derive(Debug, PartialEq, Clone)]
pub struct Ping {
    /// Verify payload.
    pub payload: i64,
}

/// Verify server response.
#[derive(Debug, PartialEq, Clone)]
pub struct Pong {
    /// Verify payload.
    pub payload: i64,
}

/// Request for server information.
#[derive(Debug, PartialEq, Clone)]
pub struct StatusRequest {}

/// Server information response to `StatusRequest`.
#[derive(Debug, PartialEq, Clone)]
pub struct StatusResponse {
    /// Server information.
    pub response: ServerInformation,
}

// Handshake -------

/// Handshake packet. This begins the server connection.
#[derive(Debug, PartialEq, Clone)]
pub struct Handshake {
    /// Server IP string.
    pub server_address: String,
    /// Server port.
    pub server_port: u16,
    /// Next connection state.
    pub next_state: EventState,
}

// Login -----------

/// Start the login process.
#[derive(Debug, PartialEq, Clone)]
pub struct LoginStart {
    /// Username to log in with.
    pub name: String,
}

/// Client disconnect reason.
#[derive(Debug, PartialEq, Clone)]
pub struct Disconnect {
    /// Reason field.
    pub reason: Chat,
}

/// Encryption request to generate a shared key. Note that
/// none of the fields are encrypted.
#[derive(Debug, PartialEq, Clone)]
pub struct EncryptionRequest {
    /// Server ID. Usually empty.
    pub server_id: String,
    /// Server's public key. Part of an RSA keypair generated
    /// on the server and only used once - to encrypt the shared key.
    pub public_key: Vec<u8>,
    /// Verify that the shared key was encrypted correctly.
    pub verify_token: Vec<u8>,
}

/// Send the shared key.
#[derive(Debug, PartialEq, Clone)]
pub struct EncryptionResponse {
    /// Shared secret. Key used to encrypt all packets with.
    pub shared_secret: Vec<u8>,
    /// Encrypted verification token to verify that the shared key was
    /// encrypted correctly.
    pub verify_token: Vec<u8>,
}

/// Check if the login process succeeded.
#[derive(Debug, PartialEq, Clone)]
pub struct LoginSuccess {
    /// UUID of the user profile logged in.
    pub uuid: Uuid,
    /// Name of the user profile logged in.
    pub name: String,
}

/// Set the connection compression.
#[derive(Debug, PartialEq, Clone)]
pub struct SetCompression {
    /// Maximum packet size to need compression.
    pub threshold: i32,
}

// Play ------------

/// Sent often to make sure the client is still connected.
#[derive(Debug, PartialEq, Clone)]
pub struct KeepAlive {
    /// Payload.
    pub id: i64,
}

/// Sent when a player joins a server.
#[derive(Debug, PartialEq, Clone)]
pub struct JoinGame {
    pub id: i32,
    pub gamemode: Gamemode,
    pub is_hardcore: bool,
    pub dimension: Dimension,
    pub difficulty: Difficulty,
    pub max_players: u32,
    pub world_type: String,
    pub reduced_debug: bool,
}

/// Spawn position of a player.
#[derive(Debug, PartialEq, Clone)]
pub struct SpawnPosition {
    /// Spawn position coordinates.
    pub location: Position,
}

/// Change player's selected slot.
#[derive(Debug, PartialEq, Clone)]
pub struct HeldItemChange {
    /// Spawn position coordinates.
    pub slot: i8,
}

/// A single stat value.
#[derive(Debug, PartialEq, Clone)]
pub struct Statistic {
    pub name: String,
    pub value: i32,
}

/// Player statistics.
#[derive(Debug, PartialEq, Clone)]
pub struct Statistics {
    pub values: Vec<Statistic>,
}

/// Sent to update player abilities.
#[derive(Debug, PartialEq, Clone)]
pub struct PlayerAbility {
    /// "God mode".
    pub invulnerable: bool,
    pub is_flying: bool,
    pub allow_flying: bool,
    pub creative_mode: bool,
    pub flying_speed: f32,
    pub walking_speed: f32,
}

// TODO: serialize data as `enum` based on namespace.

/// Plugin channel message.
#[derive(Debug, PartialEq, Clone)]
pub struct PluginMessage {
    /// Channel name.
    pub channel: String,
    /// Raw bytes the channel sent.
    pub data: Vec<u8>,
}

/// Sent when the server changes its difficulty.
#[derive(Debug, PartialEq, Clone)]
pub struct ServerDifficultyUpdate {
    pub difficulty: Difficulty,
    pub difficulty_locked: bool,
}
