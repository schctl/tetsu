//! Basic server connection utilities.

use crate::crypto::*;
use crate::errors::*;
use crate::event::*;

pub use std::net::SocketAddr;

#[allow(unused_imports)]
use log::{debug, error, info, warn};

/// Encrypted connection to a Minecraft server.
pub struct EncryptedConnection {
    /// Internal TCP stream.
    stream: EncryptedTcpStream,
    /// Current connection state (Status/Handshake/Login/Play).
    state: EventState,
    /// Protocol version used by the connection.
    pub protocol_version: ProtocolVersion,
    /// Compression threshold.
    compression_threshold: i32,
    dispatcher: EventDispatcher<EncryptedTcpStream, EncryptedTcpStream>,
}

impl EncryptedConnection {
    /// Construct a new Encrypted Connection to a server.
    #[inline]
    pub fn new(address: &str, port: u16, protocol_version: ProtocolVersion) -> TetsuResult<Self> {
        Ok(Self {
            stream: EncryptedTcpStream::connect(&format!("{}:{}", address, port), None)?,
            state: EventState::Status,
            protocol_version,
            compression_threshold: 0,
            dispatcher: EventDispatcher::new(&protocol_version),
        })
    }

    /// Set the current state of the the connection.
    #[inline]
    pub fn set_state(&mut self, state: &EventState) {
        info!(
            "Switching connection state from {:?} -> {:?}",
            self.state, state
        );
        self.state = *state;
    }

    /// Set the packet compression threshold.
    #[inline]
    pub fn set_compression_threshold(&mut self, compression_threshold: i32) {
        self.compression_threshold = compression_threshold;
    }

    /// Read and parse a packet from the internal `TcpStream`.
    #[inline]
    pub fn read_event(&mut self) -> TetsuResult<Event> {
        self.dispatcher.read_event(
            &mut self.stream,
            &self.state,
            &EventDirection::ClientBound,
            self.compression_threshold,
        )
    }

    /// Send a packet to the internal `TcpStream`.
    #[inline]
    pub fn send_event(&mut self, event: Event) -> TetsuResult<()> {
        self.dispatcher.write_event(
            &mut self.stream,
            event,
            &self.state,
            &EventDirection::ServerBound,
            self.compression_threshold,
        )
    }

    /// Set the key to encrypt with.
    #[inline]
    pub fn set_cipher(&mut self, key: &[u8; 16]) -> TetsuResult<()> {
        self.stream.set_cipher(key)
    }

    /// Get the address of the internal `TcpStream`.
    #[inline]
    pub fn get_address(&self) -> SocketAddr {
        self.stream.get_address()
    }
}

impl std::fmt::Debug for EncryptedConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&self.state)
            .field(&self.protocol_version)
            .finish()
    }
}
