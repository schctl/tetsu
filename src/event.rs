//! Protocol events. The client/server communicates by sending
//! and receiving events. These act as a sort of common interface
//! to all the major Minecraft versions' packet implementations.

use std::time;

#[allow(unused_imports)]
use log::{debug, error, info, warn};

use serde::{Deserialize, Serialize};
use serde_repr::*;

use crate::errors::Error;
use crate::packet::*;
use crate::versions;

/// All supported protocol versions.
#[non_exhaustive]
#[allow(dead_code)]
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Eq, Clone)]
#[repr(u16)]
pub enum ProtocolVersion {
    /// Server versions 1.8-1.8.9
    V47 = 47,
    /// Server versions 1.16.4 and 1.16.5
    V754 = 754,
}

/// Different connection states.
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EventState {
    Test,
    Status,
    Handshake,
    Login,
    Play,
}

/**
All possible server events.

# Examples
```
use std::io::Cursor;
use tetsu::event::{Event, Handshake, ProtocolVersion, EventState};

let mut connection = Cursor::new(Vec::new());

let write_handshake = Event::Handshake(Handshake {
    server_address: "127.0.0.1".to_owned(),
    server_port: 25565,
    next_state: EventState::Login
});
write_handshake.clone().write_to(&mut connection, &EventState::Login, &ProtocolVersion::V47, 0);

connection.set_position(0);
println!("{:?}", connection);

let read_handshake = Event::read_from(&mut connection, &EventState::Login, &ProtocolVersion::V47, 0).unwrap();

assert_eq!(write_handshake, read_handshake)
```
*/
#[allow(missing_docs)]
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone)]
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
    PluginMessage(PluginMessage),
    ServerDifficultyUpdate(ServerDifficultyUpdate),
}

impl Event {
    /// Write an event to a buffer.
    #[inline]
    pub fn write_to<T: std::io::Write>(
        self,
        buf: &mut T,
        _state: &EventState,
        protocol: &ProtocolVersion,
        compression_threshold: i32,
    ) -> Result<(), Error> {
        let start = time::Instant::now();
        match protocol {
            ProtocolVersion::V47 => versions::v47::write_event(buf, self, compression_threshold),
            ProtocolVersion::V754 => versions::v754::write_event(buf, self, compression_threshold),
        }?;
        debug!("Wrote event: Took: {} us", start.elapsed().as_micros());
        Ok(())
    }

    /// Read an event from a buffer.
    #[inline]
    pub fn read_from<T: std::io::Read>(
        buf: &mut T,
        state: &EventState,
        protocol: &ProtocolVersion,
        compression_threshold: i32,
    ) -> Result<Self, Error> {
        let start = time::Instant::now();
        let ev = match protocol {
            ProtocolVersion::V47 => versions::v47::read_event(
                buf,
                state,
                &PacketDirection::ClientBound,
                compression_threshold,
            ),
            ProtocolVersion::V754 => versions::v754::read_event(
                buf,
                state,
                &PacketDirection::ClientBound,
                compression_threshold,
            ),
        };
        debug!("Read event: Took: {} us", start.elapsed().as_micros());
        ev
    }
}

// Other types --------------------------

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Gamemode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Dimension {
    Nether,
    Overworld,
    End,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Difficulty {
    Peaceful,
    Easy,
    Normal,
    Hard,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum ServerDescription {
    Short(String),
    Long(ServerDescriptionLong),
}

/// Long server description. All fields haven't been covered yet.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ServerDescriptionLong {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ServerPlayers {
    pub max: u32,
    pub online: u16,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ServerVersion {
    pub name: String,
    pub protocol: ProtocolVersion,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ServerInformation {
    pub description: ServerDescription,
    pub players: ServerPlayers,
    pub version: ServerVersion,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Position {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

// All possible server events -----------

// Status ----------

/// Ping the server to make sure its alive.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ping {
    /// Verify payload.
    pub payload: i64,
}

/// Verify server response.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Pong {
    /// Verify payload.
    pub payload: i64,
}

/// Request for server information.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StatusRequest {}

/// Server information response to `StatusRequest`.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StatusResponse {
    /// Server information.
    pub response: ServerInformation,
}

// Handshake -------

/// Handshake packet. This begins the server connection.
#[derive(Debug, PartialEq, Eq, Clone)]
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
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LoginStart {
    /// Username to log in with.
    pub name: String,
}

/// Client disconnect reason.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Disconnect {
    /// Reason field.
    pub reason: Chat,
}

/// Encryption request to generate a shared key. Note that
/// none of the fields are encrypted.
#[derive(Debug, PartialEq, Eq, Clone)]
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
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EncryptionResponse {
    /// Shared secret. Key used to encrypt all packets with.
    pub shared_secret: Vec<u8>,
    /// Encrypted verification token to verify that the shared key was
    /// encrypted correctly.
    pub verify_token: Vec<u8>,
}

/// Check if the login process succeeded.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LoginSuccess {
    /// UUID of the user profile logged in.
    pub uuid: Uuid,
    /// Name of the user profile logged in.
    pub name: String,
}

/// Set the connection compression.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SetCompression {
    /// Maximum packet size to need compression.
    pub threshold: i32,
}

// Play ------------

/// Sent often to make sure the client is still connected.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct KeepAlive {
    /// Payload.
    pub id: i64,
}

/// Sent when a player joins a server.
#[derive(Debug, PartialEq, Eq, Clone)]
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
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SpawnPosition {
    /// Spawn position coordinates.
    pub location: Position,
}

// TODO: serialize data as `enum` based on namespace.

/// Plugin channel message.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PluginMessage {
    /// Channel name.
    pub channel: String,
    /// Raw bytes the channel sent.
    pub data: Vec<u8>,
}

/// Sent when the server changes its difficulty.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ServerDifficultyUpdate {
    pub difficulty: Difficulty,
    pub difficulty_locked: bool,
}
