//! Protocol events. The client/server communicates by sending
//! and receiving events. These act as a sort of common interface
//! to all the major Minecraft versions' packet implementations.

use std::time;

use colorful::Colorful;
use serde::{Deserialize, Serialize};
use serde_repr::*;

use crate::packet::*;
use crate::versions;

#[allow(dead_code)]
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum ProtocolVersion {
    V47 = 47,
    V754 = 754,
}

/// All possible server events.
#[non_exhaustive]
#[derive(Debug)]
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
}

impl Event {
    /// Write an event to a buffer.
    #[inline]
    pub fn write_to<T: std::io::Write>(
        self,
        buf: &mut T,
        _state: &PacketState,
        protocol: &ProtocolVersion,
        compression_threshold: i32,
    ) {
        let start = time::Instant::now();
        match protocol {
            ProtocolVersion::V47 => versions::v47::write_event(self, buf, compression_threshold),
            ProtocolVersion::V754 => versions::v754::write_event(self, buf, compression_threshold),
        };
        println!(
            "{} {} {}",
            "Wrote event: Took:".bold().green(),
            format!("{}", start.elapsed().as_micros()).bold().red(),
            "us".bold().green()
        );
    }

    /// Read an event from a buffer.
    #[inline]
    pub fn read_from<T: std::io::Read>(
        buf: &mut T,
        state: &PacketState,
        protocol: &ProtocolVersion,
        compression_threshold: i32,
    ) -> Self {
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
        println!(
            "{} {} {}",
            "Read event: Took:".bold().green(),
            format!("{}", start.elapsed().as_micros()).bold().red(),
            "us".bold().green()
        );
        ev
    }
}

// Other types --------------------------

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerDescription {
    Short(String),
    Long(ServerDescriptionLong),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerDescriptionLong {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerPlayers {
    pub max: u32,
    pub online: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerVersion {
    pub name: String,
    pub protocol: ProtocolVersion,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerInformation {
    pub description: ServerDescription,
    pub players: ServerPlayers,
    pub version: ServerVersion,
}

// All possible server events -----------

// Status ----------

#[derive(Debug)]
pub struct Ping {
    pub payload: i64,
}

#[derive(Debug)]
pub struct Pong {
    pub payload: i64,
}

#[derive(Debug)]
pub struct StatusRequest {}

#[derive(Debug)]
pub struct StatusResponse {
    pub response: ServerInformation,
}

// Handshake -------

/// Handshake packet. This begins the server connection.
#[derive(Debug)]
pub struct Handshake {
    pub server_address: String,
    pub server_port: u16,
    pub next_state: PacketState,
}

// Login -----------

/// Start the login process.
#[derive(Debug)]
pub struct LoginStart {
    pub name: String,
}

#[derive(Debug)]
pub struct Disconnect {
    pub reason: Chat,
}

/// Encryption request to generate a shared key.
#[derive(Debug)]
pub struct EncryptionRequest {
    pub server_id: String,
    pub public_key: Vec<u8>,
    pub verify_token: Vec<u8>,
}

/// Send the shared key.
#[derive(Debug)]
pub struct EncryptionResponse {
    pub shared_secret: Vec<u8>,
    pub verify_token: Vec<u8>,
}

/// Check if the login process succeeded.
#[derive(Debug)]
pub struct LoginSuccess {
    pub uuid: Uuid,
    pub name: String,
}

/// Set the connection compression.
#[derive(Debug)]
pub struct SetCompression {
    pub threshold: i32,
}

// All possible server events -----------
