use serde::{Deserialize, Serialize};
use serde_repr::*;

/// All supported protocol versions.
#[non_exhaustive]
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u16)]
pub enum ProtocolVersion {
    /// Server versions 1.8-1.8.9
    V47 = 47,
    /// Server versions 1.16.4 and 1.16.5
    V754 = 754,
}

/// Types of events being sent and received.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EventState {
    Test,
    Status,
    Handshake,
    Login,
    Play,
}

/// Sender of the Event.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EventDirection {
    /// Server sent event.
    ClientBound,
    /// Client sent event.
    ServerBound,
}

// Event field types ------------

/// Gamemode of a level.
#[derive(Debug, PartialEq, Clone)]
pub enum Gamemode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

/// Dimension of a world.
#[derive(Debug, PartialEq, Clone)]
pub enum Dimension {
    Nether,
    Overworld,
    End,
}

/// Difficulty of a level.
#[derive(Debug, PartialEq, Clone)]
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

/// General server player information.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ServerPlayers {
    pub max: u32,
    pub online: u16,
}

/// Version the server is running on.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ServerVersion {
    pub name: String,
    pub protocol: ProtocolVersion,
}
/// Server information such as version, online players, etc.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ServerInformation {
    pub description: ServerDescription,
    pub players: ServerPlayers,
    pub version: ServerVersion,
}

/// Coordinates in a world.
#[derive(Debug, PartialEq, Clone)]
pub struct Position {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}
