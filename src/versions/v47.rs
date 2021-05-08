//! Event implementation for v47 of the protocol.
//! V47 covers server versions 1.8 - 1.8.9

use crate::event::*;
use crate::packet::*;

use uuid::Uuid;

mod internal {

    use super::*;

    use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
    use std::io;

    // Protocol specific types ---

    #[derive(Debug, PartialEq, Eq)]
    pub struct PositionXZY {
        pub x: i64,
        pub z: i64,
        pub y: i64,
    }

    impl From<Position> for PositionXZY {
        fn from(item: Position) -> Self {
            Self {
                x: item.x,
                y: item.y,
                z: item.z,
            }
        }
    }

    impl From<PositionXZY> for Position {
        fn from(item: PositionXZY) -> Self {
            Self {
                x: item.x,
                y: item.y,
                z: item.z,
            }
        }
    }

    impl Readable for PositionXZY {
        fn read_from<T: io::Read>(buf: &mut T) -> Result<Self, Error> {
            let val = buf.read_u64::<BigEndian>()?;
            Ok(Self {
                x: (val >> 38) as i64,
                y: ((val >> 26) & 0xFF) as i64,
                z: (val << 38 >> 38) as i64,
            })
        }
    }

    impl Writable for PositionXZY {
        fn write_to<T: io::Write>(&self, buf: &mut T) -> Result<(), Error> {
            let val = ((self.x as u64 & 0x3FFFFFF) << 38)
                | ((self.y as u64 & 0xFFF) << 26)
                | (self.z as u64 & 0x3FFFFFF);
            Ok(buf.write_u64::<BigEndian>(val)?)
        }
    }

    // Conversions ---

    pub fn byte_to_gamemode(byte: UnsignedByte) -> Gamemode {
        match byte {
            0 => Gamemode::Survival,
            1 => Gamemode::Creative,
            2 => Gamemode::Adventure,
            3 => Gamemode::Spectator,
            _ => panic!("Unknown packet"),
        }
    }

    pub fn gamemode_to_byte(gamemode: &Gamemode) -> UnsignedByte {
        match gamemode {
            Gamemode::Survival => 0,
            Gamemode::Creative => 1,
            Gamemode::Adventure => 2,
            Gamemode::Spectator => 3,
        }
    }

    pub fn byte_to_dimension(byte: Byte) -> Dimension {
        match byte {
            -1 => Dimension::Nether,
            0 => Dimension::Overworld,
            1 => Dimension::End,
            _ => panic!("Unknown packet"),
        }
    }

    pub fn dimension_to_byte(dimension: &Dimension) -> Byte {
        match dimension {
            Dimension::Nether => -1,
            Dimension::Overworld => 0,
            Dimension::End => 1,
        }
    }

    pub fn byte_to_difficulty(byte: UnsignedByte) -> Difficulty {
        match byte {
            0 => Difficulty::Peaceful,
            1 => Difficulty::Easy,
            2 => Difficulty::Normal,
            3 => Difficulty::Hard,
            _ => panic!("Unknown packet"),
        }
    }

    pub fn difficulty_to_byte(difficulty: &Difficulty) -> UnsignedByte {
        match difficulty {
            Difficulty::Peaceful => 0,
            Difficulty::Easy => 1,
            Difficulty::Normal => 2,
            Difficulty::Hard => 3,
        }
    }
}

use internal::*;

// ---------------

packet_impl! {

    inherit {
    }

    // Status ------------------

    (0x01) ServerBound Status StatusPingPacket: Ping {
        from_event {
            | origin: Ping | -> StatusPingPacket {
                StatusPingPacket {
                    payload: origin.payload
                }
            }
        }
        to_event {
            | origin: StatusPingPacket | -> Event {
                Event::Ping(Ping {
                    payload: origin.payload
                })
            }
        }
        fields {
            payload: Long,
        }
    }

    (0x01) ClientBound Status StatusPongPacket: Pong {
        from_event {
            | origin: Pong | -> StatusPongPacket {
                StatusPongPacket {
                    payload: origin.payload
                }
            }
        }
        to_event {
            | origin: StatusPongPacket | -> Event {
                Event::Pong(Pong {
                    payload: origin.payload
                })
            }
        }
        fields {
            payload: Long,
        }
    }

    (0x00) ServerBound Status StatusRequestPacket: StatusRequest {
        from_event {
            | _: StatusRequest | -> StatusRequestPacket {
                StatusRequestPacket {}
            }
        }
        to_event {
            | _: StatusRequestPacket | -> Event {
                Event::StatusRequest(StatusRequest {})
            }
        }
        fields {

        }
    }

    (0x00) ClientBound Status StatusResponsePacket: StatusResponse {
        from_event {
            | origin: StatusResponse | -> StatusResponsePacket {
                StatusResponsePacket {
                    response: serde_json::to_string(&origin.response).unwrap()
                }
            }
        }
        to_event {
            | origin: StatusResponsePacket | -> Event {
                Event::StatusResponse(StatusResponse {
                    response: serde_json::from_str(&origin.response[..]).unwrap()
                })
            }
        }
        fields {
            response: String,
        }
    }

    // Handshake ---------------

    (0x00) ServerBound Handshake HandshakePacket: Handshake {
        from_event {
            | origin: Handshake | -> HandshakePacket {
                HandshakePacket {
                    protocol_version: VarInt(47),
                    server_address: origin.server_address,
                    server_port: origin.server_port,
                    next_state: match origin.next_state {
                        PacketState::Status => VarInt(1),
                        PacketState::Login => VarInt(2),
                        _ => panic!("Invalid next state for handshake!")
                    }
                }
            }
        }
        to_event {
            | origin: HandshakePacket | -> Event {
                Event::Handshake(Handshake {
                    server_address: origin.server_address,
                    server_port: origin.server_port,
                    next_state: match origin.next_state.0 {
                        1 => PacketState::Status,
                        2 => PacketState::Login,
                        _ => panic!("Invalid next state for handshake!")
                    }
                })
            }
        }
        fields {
            protocol_version: VarInt,
            server_address: String,
            server_port: UnsignedShort,
            next_state: VarInt,
        }
    }

    // Login -------------------

    (0x00) ServerBound Login LoginStartPacket: LoginStart {
        from_event {
            | origin: LoginStart | -> LoginStartPacket {
                LoginStartPacket {
                    name: origin.name
                }
            }
        }
        to_event {
            | origin: LoginStartPacket | -> Event {
                Event::LoginStart(LoginStart {
                    name: origin.name
                })
            }
        }
        fields {
            name: String,
        }
    }

    (0x00) ClientBound Login DisconnectPacket: Disconnect {
        from_event {
            | origin: Disconnect | -> DisconnectPacket {
                DisconnectPacket {
                    reason: match origin.reason.text {
                        Some(t) => t,
                        None => panic!("Unknown reason")
                    }
                }
            }
        }
        to_event {
            | origin: DisconnectPacket | -> Event {
                Event::Disconnect(Disconnect {
                    reason: Chat {
                        text: Some(origin.reason),
                        translate: None,
                        bold: None,
                        italic: None,
                        underlined: None,
                        obfuscated: None,
                        strikethrough: None,
                        color: None,
                        click_event: None,
                        hover_event: None,
                        extra: None
                    }
                })
            }
        }
        fields {
            reason: String,
        }
    }

    (0x01) ClientBound Login EncryptionRequestVarIntPacket: EncryptionRequest {
        from_event {
            | origin: EncryptionRequest | -> EncryptionRequestVarIntPacket {
                EncryptionRequestVarIntPacket {
                    server_id: origin.server_id,
                    public_key: ByteArrayVarInt(origin.public_key.len(), origin.public_key),
                    verify_token: ByteArrayVarInt(origin.verify_token.len(), origin.verify_token)
                }
            }
        }
        to_event {
            | origin: EncryptionRequestVarIntPacket | -> Event {
                Event::EncryptionRequest(EncryptionRequest {
                    server_id: origin.server_id,
                    public_key: origin.public_key.1,
                    verify_token: origin.verify_token.1
                })
            }
        }
        fields {
            server_id: String,
            public_key: ByteArrayVarInt,
            verify_token: ByteArrayVarInt,
        }
    }

    (0x01) ServerBound Login EncryptionResponseVarIntPacket: EncryptionResponse {
        from_event {
            | origin: EncryptionResponse | -> EncryptionResponseVarIntPacket {
                EncryptionResponseVarIntPacket {
                    shared_secret: ByteArrayVarInt(origin.shared_secret.len(), origin.shared_secret),
                    verify_token: ByteArrayVarInt(origin.verify_token.len(), origin.verify_token)
                }
            }
        }
        to_event {
            | origin: EncryptionResponseVarIntPacket | -> Event {
                Event::EncryptionResponse(EncryptionResponse {
                    shared_secret: origin.shared_secret.1,
                    verify_token: origin.verify_token.1
                })
            }
        }
        fields {
            shared_secret: ByteArrayVarInt,
            verify_token: ByteArrayVarInt,
        }
    }

    (0x02) ClientBound Login LoginSuccessPacket: LoginSuccess {
        from_event {
            | origin: LoginSuccess | -> LoginSuccessPacket {
                LoginSuccessPacket {
                    uuid: origin.uuid.to_hyphenated().to_string(),
                    name: origin.name
                }
            }
        }
        to_event {
            | origin: LoginSuccessPacket | -> Event {
                Event::LoginSuccess(LoginSuccess {
                    uuid: Uuid::parse_str(&origin.uuid[..]).unwrap(),
                    name: origin.name,
                })
            }
        }
        fields {
            uuid: String,
            name: String,
        }
    }

    (0x03) ClientBound Login SetCompressionPacket: SetCompression {
        from_event {
            | origin: SetCompression | -> SetCompressionPacket {
                SetCompressionPacket {
                    threshold: VarInt(origin.threshold)
                }
            }
        }
        to_event {
            | origin: SetCompressionPacket | -> Event {
                Event::SetCompression(SetCompression {
                    threshold: origin.threshold.0
                })
            }
        }
        fields {
            threshold: VarInt,
        }
    }

    // Play --------------------

    (0x00) ClientBound Play KeepAlivePacket: KeepAlive {
        from_event {
            | origin: KeepAlive | -> KeepAlivePacket {
                KeepAlivePacket {
                    id: VarInt(origin.id as i32)
                }
            }
        }
        to_event {
            | origin: KeepAlivePacket | -> Event {
                Event::KeepAlive(KeepAlive {
                    id: origin.id.0 as i64
                })
            }
        }
        fields {
            id: VarInt,
        }
    }

    (0x01) ClientBound Play JoinGamePacket: JoinGame {
        from_event {
            | origin: JoinGame | -> JoinGamePacket {
                JoinGamePacket {
                    id: origin.id,
                    gamemode: gamemode_to_byte(&origin.gamemode) | (if origin.is_hardcore { 0x80 } else { 0x00 }),
                    dimension: dimension_to_byte(&origin.dimension),
                    difficulty: difficulty_to_byte(&origin.difficulty),
                    max_players: origin.max_players as u8,
                    level_type: origin.world_type,
                    reduced_debug: origin.reduced_debug
                }
            }
        }
        to_event {
            | origin: JoinGamePacket | -> Event {
                Event::JoinGame(JoinGame {
                    id: origin.id,
                    gamemode: byte_to_gamemode(origin.gamemode),
                    is_hardcore: origin.gamemode & 0x80 == 0x80,
                    dimension: byte_to_dimension(origin.dimension),
                    difficulty: byte_to_difficulty(origin.difficulty),
                    max_players: origin.max_players as u32,
                    world_type: origin.level_type,
                    reduced_debug: origin.reduced_debug
                })
            }
        }
        fields {
            id: Int,
            gamemode: UnsignedByte,
            dimension: Byte,
            difficulty: UnsignedByte,
            max_players: UnsignedByte,
            level_type: String,
            reduced_debug: bool,
        }
    }

    (0x05) ClientBound Play SpawnPositionPacket: SpawnPosition {
        from_event {
            | origin: SpawnPosition | -> SpawnPositionPacket {
                SpawnPositionPacket {
                    location: origin.location.into()
                }
            }
        }
        to_event {
            | origin: SpawnPositionPacket | -> Event {
                Event::SpawnPosition(SpawnPosition {
                    location: origin.location.into()
                })
            }
        }
        fields {
            location: PositionXZY,
        }
    }

    (0x3F) ClientBound Play PluginMessagePacket: PluginMessage {
        from_event {
            | origin: PluginMessage | -> PluginMessagePacket {
                PluginMessagePacket {
                    channel: origin.channel,
                    data: origin.data
                }
            }
        }
        to_event {
            | origin: PluginMessagePacket | -> Event {
                Event::PluginMessage(PluginMessage {
                    channel: origin.channel,
                    data: origin.data
                })
            }
        }
        fields {
            channel: String,
            data: Vec<u8>,
        }
    }

    (0x41) ClientBound Play ServerDifficultyUpdatePacket: ServerDifficultyUpdate {
        from_event {
            | origin: ServerDifficultyUpdate | -> ServerDifficultyUpdatePacket {
                ServerDifficultyUpdatePacket {
                    difficulty: difficulty_to_byte(&origin.difficulty)
                }
            }
        }
        to_event {
            | origin: ServerDifficultyUpdatePacket | -> Event {
                Event::ServerDifficultyUpdate(ServerDifficultyUpdate {
                    difficulty: byte_to_difficulty(origin.difficulty),
                    difficulty_locked: false
                })
            }
        }
        fields {
            difficulty: UnsignedByte,
        }
    }

}
