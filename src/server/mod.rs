//! High level server connection.

pub mod connection;
use connection::EncryptedConnection;

use std::sync::Mutex;
use std::time;

#[allow(unused_imports)]
use log::{debug, error, info, warn};

use crate::crypto;
use crate::errors::*;
use crate::event::{self, Event};
use crate::{event::ProtocolVersion, mojang::User};

/// High level wrapper around a Minecraft server connection.
pub struct Server {
    // Mutex here is for interior mutability ->
    // allows server methods such as `read_event` to be called without passing a mutable reference to self.
    connection: Mutex<EncryptedConnection>,
    connected_address: String,
    connected_user: Option<User>,
}

impl Server {
    /// Constructs a new server object.
    /// The connection will use port `25565` if the `port` argument is `None`.
    /// The protocol version will be auto-detected if the `protocol` argument is `None`.
    #[inline]
    pub fn new(
        address: &str,
        port: Option<u16>,
        protocol: Option<ProtocolVersion>,
    ) -> Result<Self, Error> {
        let port = match port {
            Some(p) => p,
            _ => 25565,
        };

        Ok(Self {
            connection: Mutex::new(EncryptedConnection::new(
                address,
                port,
                match protocol {
                    Some(p) => p,
                    _ => Self::get_version(&address, port)?,
                },
            )?),
            connected_address: format!("{}:{}", address, port),
            connected_user: None,
        })
    }

    /// Get the address with which the server was connected to,
    #[inline]
    pub fn get_address(&self) -> &String {
        &self.connected_address
    }

    /// Get the ip address and port of the server.
    #[inline]
    pub fn get_connection_address(&self) -> connection::SocketAddr {
        self.connection.lock().unwrap().get_address()
    }

    /// Get the currently connected user.
    #[inline]
    pub fn get_connected_user(&self) -> &Option<User> {
        &self.connected_user
    }

    /// Read incoming server events.
    #[inline]
    pub fn read_event(&self) -> Result<Event, ConnectionError<EncryptedConnection>> {
        Ok(self.connection.lock()?.read_event()?)
    }

    /// Send an event to the server.
    #[inline]
    pub fn send_event(&self, _event: Event) -> Result<(), ConnectionError<EncryptedConnection>> {
        self.connection.lock()?.send_event(_event)?;
        Ok(())
    }

    /// Attempt to get the protocol version of a server.
    pub fn get_version(address: &str, port: u16) -> Result<ProtocolVersion, Error> {
        let mut connection = EncryptedConnection::new(address, port, event::ProtocolVersion::V47)?;

        connection.set_state(&event::EventState::Handshake);

        connection
            .send_event(Event::Handshake(event::Handshake {
                server_address: address.to_owned(),
                server_port: port,
                next_state: event::EventState::Status,
            }))
            .unwrap();

        connection.set_state(&event::EventState::Status);

        connection
            .send_event(Event::StatusRequest(event::StatusRequest {}))
            .unwrap();

        Ok(match connection.read_event()? {
            Event::StatusResponse(e) => e.response.version.protocol,
            _ => {
                return Err(Error::from(InvalidValue {
                    expected: "StatusResponse".to_owned(),
                }))
            }
        })
    }

    /// Connect a user to the server. Only one user can be connected at a time.
    pub fn connect_player(
        &mut self,
        user: User,
    ) -> Result<(), ConnectionError<EncryptedConnection>> {
        let start = time::Instant::now();

        if let Some(p) = &self.connected_user {
            return Err(ConnectionError::from(Error::from(InvalidValue {
                expected: format!("User {} already connected.", p.selected_profile.name),
            })));
        }

        let (address, port) = match self.get_connection_address() {
            connection::SocketAddr::V4(p) => (format!("{}", p.ip()), p.port()),
            connection::SocketAddr::V6(p) => (format!("{}", p.ip()), p.port()),
        };

        self.connection
            .lock()?
            .set_state(&event::EventState::Handshake);

        self.connection
            .lock()?
            .send_event(Event::Handshake(event::Handshake {
                server_address: address,
                server_port: port,
                next_state: event::EventState::Login,
            }))
            .unwrap();

        self.connection.lock()?.set_state(&event::EventState::Login);

        self.connection
            .lock()?
            .send_event(Event::LoginStart(event::LoginStart {
                name: user.selected_profile.name.clone(),
            }))
            .unwrap();

        let encryption_request = match self.connection.lock().unwrap().read_event()? {
            Event::EncryptionRequest(e) => e,
            Event::LoginSuccess(_) => {
                warn!("Server running in offline mode. Logging in.");
                info!("Login success at: {} ms!", start.elapsed().as_millis());
                self.connected_user = Some(user);
                self.connection.lock()?.set_state(&event::EventState::Play);
                return Ok(());
            }
            _ => {
                return Err(ConnectionError::from(Error::from(InvalidValue {
                    expected: "EncryptionRequest".to_owned(),
                })))
            }
        };

        let mut encryption_response = event::EncryptionResponse {
            shared_secret: vec![],
            verify_token: vec![],
        };

        let mut shared = [0; 16];

        {
            crypto::generate_key(&mut shared);

            let pkey = match crypto::Rsa::public_key_from_der(&encryption_request.public_key) {
                Ok(p) => p,
                Err(e) => return Err(Error::from(e).into()),
            };
            encryption_response.shared_secret = crypto::public_encrypt(&pkey, &shared)?;
            encryption_response.verify_token =
                crypto::public_encrypt(&pkey, &encryption_request.verify_token)?;

            user.join_server(
                &encryption_request.server_id,
                &shared,
                &encryption_request.public_key,
            );
        }

        self.connection
            .lock()?
            .send_event(Event::EncryptionResponse(encryption_response))
            .unwrap();

        self.connection.lock()?.set_cipher(&shared)?;

        loop {
            let event = self.connection.lock()?.read_event()?;
            match event {
                Event::SetCompression(c) => self
                    .connection
                    .lock()
                    .unwrap()
                    .set_compression_threshold(c.threshold),
                Event::LoginSuccess(_) => {
                    info!("Login success at: {} ms!", start.elapsed().as_millis());
                    self.connected_user = Some(user);
                    break;
                }
                Event::Disconnect(c) => panic!("Disconnected!: {:?}", c),
                _ => panic!("Unknown event!"),
            };
        }

        self.connection.lock()?.set_state(&event::EventState::Play);

        Ok(())
    }
}
