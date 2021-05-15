//! Types used by events.

use serde::{Deserialize, Serialize};
use serde_repr::*;
use uuid::Uuid;

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
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Gamemode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

/// Dimension of a world.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Dimension {
    Nether,
    Overworld,
    End,
}

/// Difficulty of a level.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Difficulty {
    Peaceful,
    Easy,
    Normal,
    Hard,
}

/// General server description.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum ServerDescription {
    Short(String),
    Long(ServerDescriptionLong),
}

/// Long server description.
///
/// **Warning:** All fields haven't been covered yet.
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
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Position {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

// ---- Chat ---------------

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Action {
    action: String,
    value: String,
}

/// Information that defines contents/style of a chat message.
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    pub text: Option<String>,
    pub translate: Option<String>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underlined: Option<bool>,
    pub strikethrough: Option<bool>,
    pub obfuscated: Option<bool>,
    pub color: Option<String>,
    pub click_event: Option<Action>,
    pub hover_event: Option<Action>,
    pub extra: Option<Vec<Self>>,
}

impl Default for Chat {
    fn default() -> Self {
        Self {
            text: None,
            translate: None,
            bold: None,
            italic: None,
            underlined: None,
            strikethrough: None,
            obfuscated: None,
            color: None,
            click_event: None,
            hover_event: None,
            extra: None,
        }
    }
}

// ---- Player Infos -------

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerProperty {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerInfoAdd {
    pub name: String,
    pub properties: Vec<PlayerProperty>,
    pub gamemode: Gamemode,
    pub ping: i32,
    pub display: Option<Chat>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerGamemodeUpdate {
    pub gamemode: Gamemode,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerLatencyUpdate {
    pub ping: i32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerDisplayNameUpdate {
    pub display: Option<Chat>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RemovePlayer {}

#[derive(Debug, PartialEq, Clone)]
pub enum PlayerInfoAction {
    Add(PlayerInfoAdd),
    GamemodeUpdate(PlayerGamemodeUpdate),
    LatencyUpdate(PlayerLatencyUpdate),
    DisplayNameUpdate(PlayerDisplayNameUpdate),
    Remove(RemovePlayer),
}

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerListInfo {
    pub uuid: Uuid,
    pub action: PlayerInfoAction,
}

// ---- Position and Look ---

#[derive(Debug, PartialEq, Clone)]
pub enum RelativeOrAbsolute<T> {
    Relative(T),
    Absolute(T),
}

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerPositionAndLook {
    pub x: RelativeOrAbsolute<f64>,
    pub y: RelativeOrAbsolute<f64>,
    pub z: RelativeOrAbsolute<f64>,
    pub yaw: RelativeOrAbsolute<f32>,
    pub pitch: RelativeOrAbsolute<f32>,
    pub teleport_id: Option<i32>,
}

// ---- Slot ------

#[derive(Debug, PartialEq, Clone)]
pub struct Slot {
    pub item_id: Option<i32>,
    pub item_count: i8,
    pub damage: Option<i16>,
    pub nbt: Option<nbt::Blob>,
}
