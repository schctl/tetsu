//! Basic server connection utilities.

use std::io;

pub use std::net::SocketAddr;
use std::net::TcpStream;

use crate::errors::Error;
use crate::event;
use crate::packet;
use crate::{encryption::DefaultStreamCipher, packet::PacketState};

/// Encrypted wrapper around a `TcpStream`.
pub(crate) struct EncryptedTcpStream<const KEY_LEN: usize> {
    /// TcpStream to read from.
    stream: TcpStream,
    /// Cipher algorithm.
    cipher: Option<DefaultStreamCipher<KEY_LEN>>,
}

impl<const KEY_LEN: usize> EncryptedTcpStream<KEY_LEN> {
    /// Create a new TCP connection to the `address`.
    #[inline]
    pub fn connect(address: &str, cipher: Option<DefaultStreamCipher<KEY_LEN>>) -> Self {
        Self {
            stream: TcpStream::connect(address).unwrap(),
            cipher,
        }
    }

    /// Set the key to encrypt with.
    #[inline]
    pub fn set_cipher(&mut self, key: &[u8; KEY_LEN]) {
        self.cipher = Some(DefaultStreamCipher::new(*key));
    }

    /// Get the current connected address.
    #[inline]
    pub fn get_address(&self) -> SocketAddr {
        self.stream.peer_addr().unwrap()
    }
}

impl<const KEY_LEN: usize> io::Read for EncryptedTcpStream<KEY_LEN> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match &mut self.cipher {
            None => self.stream.read(buf),
            Some(cipher) => {
                let read = self.stream.read(buf)?;
                cipher.decrypt(&mut buf[..read]);
                Ok(read)
            }
        }
    }
}

impl<const KEY_LEN: usize> io::Write for EncryptedTcpStream<KEY_LEN> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match &mut self.cipher {
            None => self.stream.write(buf),
            Some(cipher) => {
                let mut data = buf.to_owned();
                cipher.encrypt(&mut data);
                self.stream.write_all(&data).unwrap();
                Ok(data.len())
            }
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stream.flush()
    }
}

/// Encrypted connection to a Minecraft server.
pub struct EncryptedConnection {
    /// Internal TCP stream.
    stream: EncryptedTcpStream<16>,
    /// Current connection state (Status/Handshake/Login/Play).
    state: packet::PacketState,
    /// Protocol version used by the connection.
    protocol_version: event::ProtocolVersion,
    /// Compression threshold.
    compression_threshold: i32,
}

impl EncryptedConnection {
    /// Construct a new Encrypted Connection to a server.
    #[inline]
    pub fn new(address: &str, port: u16, protocol_version: event::ProtocolVersion) -> Self {
        Self {
            stream: EncryptedTcpStream::connect(&format!("{}:{}", address, port), None),
            state: packet::PacketState::Status,
            protocol_version,
            compression_threshold: 0,
        }
    }

    /// Set the current state of the the connection.
    #[inline]
    pub fn set_state(&mut self, state: &PacketState) {
        self.state = *state;
    }

    /// Set the packet compression threshold.
    #[inline]
    pub fn set_compression_threshold(&mut self, compression_threshold: i32) {
        self.compression_threshold = compression_threshold;
    }

    /// Read and parse a packet from the internal `TcpStream`.
    #[inline]
    pub fn read_event(&mut self) -> Result<event::Event, Error> {
        event::Event::read_from(
            &mut self.stream,
            &self.state,
            &self.protocol_version,
            self.compression_threshold,
        )
    }

    /// Send a packet to the internal `TcpStream`.
    #[inline]
    pub fn send_event(&mut self, _event: event::Event) -> Result<(), Error> {
        _event.write_to(
            &mut self.stream,
            &self.state,
            &self.protocol_version,
            self.compression_threshold,
        )
    }

    /// Set the key to encrypt with.
    #[inline]
    pub fn set_cipher(&mut self, key: &[u8; 16]) {
        self.stream.set_cipher(key);
    }

    /// Get the address of the internal `TcpStream`.
    #[inline]
    pub fn get_address(&self) -> SocketAddr {
        self.stream.get_address()
    }
}
