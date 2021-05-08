//! Event implementation for v47 of the protocol.
//! V47 covers server versions 1.8 - 1.8.9

use crate::errors::*;
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

protocol_impl! {

    inherit {
    }

    // Status ------------------

    (0x01) ServerBound Status StatusPingPacket: Ping {
        from_event {
            | origin: Ping | -> TetsuResult<StatusPingPacket> {
                Ok(StatusPingPacket {
                    payload: origin.payload
                })
            }
        }
        to_event {
            | origin: StatusPingPacket | -> Result<Event, Error> {
                Ok(Event::Ping(Ping {
                    payload: origin.payload
                }))
            }
        }
        fields {
            payload: Long,
        }
    }

    (0x01) ClientBound Status StatusPongPacket: Pong {
        from_event {
            | origin: Pong | -> TetsuResult<StatusPongPacket> {
                Ok(StatusPongPacket {
                    payload: origin.payload
                })
            }
        }
        to_event {
            | origin: StatusPongPacket | -> TetsuResult<Event> {
                Ok(Event::Pong(Pong {
                    payload: origin.payload
                }))
            }
        }
        fields {
            payload: Long,
        }
    }

    (0x00) ServerBound Status StatusRequestPacket: StatusRequest {
        from_event {
            | _: StatusRequest | -> TetsuResult<StatusRequestPacket> {
                Ok(StatusRequestPacket {})
            }
        }
        to_event {
            | _: StatusRequestPacket | -> TetsuResult<Event> {
                Ok(Event::StatusRequest(StatusRequest {}))
            }
        }
        fields {

        }
    }

    (0x00) ClientBound Status StatusResponsePacket: StatusResponse {
        from_event {
            | origin: StatusResponse | -> TetsuResult<StatusResponsePacket> {
                Ok(StatusResponsePacket {
                    response: serde_json::to_string(&origin.response).unwrap()
                })
            }
        }
        to_event {
            | origin: StatusResponsePacket | -> TetsuResult<Event> {
                Ok(Event::StatusResponse(StatusResponse {
                    response: serde_json::from_str(&origin.response[..]).unwrap()
                }))
            }
        }
        fields {
            response: String,
        }
    }

    // Handshake ---------------

    (0x00) ServerBound Handshake HandshakePacket: Handshake {
        from_event {
            | origin: Handshake | -> TetsuResult<HandshakePacket> {
                Ok(HandshakePacket {
                    protocol_version: VarInt(47),
                    server_address: origin.server_address,
                    server_port: origin.server_port,
                    next_state: match origin.next_state {
                        EventState::Status => VarInt(1),
                        EventState::Login => VarInt(2),
                        _ => return Err(Error::from(InvalidValue { expected: "Status or Login".to_owned() }))
                    }
                })
            }
        }
        to_event {
            | origin: HandshakePacket | -> TetsuResult<Event> {
                Ok(Event::Handshake(Handshake {
                    server_address: origin.server_address,
                    server_port: origin.server_port,
                    next_state: match origin.next_state.0 {
                        1 => EventState::Status,
                        2 => EventState::Login,
                        _ => return Err(Error::from(InvalidValue { expected: "1 or 2".to_owned() }))
                    }
                }))
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
            | origin: LoginStart | -> TetsuResult<LoginStartPacket> {
                Ok(LoginStartPacket {
                    name: origin.name
                })
            }
        }
        to_event {
            | origin: LoginStartPacket | -> TetsuResult<Event> {
                Ok(Event::LoginStart(LoginStart {
                    name: origin.name
                }))
            }
        }
        fields {
            name: String,
        }
    }

    (0x00) ClientBound Login DisconnectPacket: Disconnect {
        from_event {
            | origin: Disconnect | -> TetsuResult<DisconnectPacket> {
                Ok(DisconnectPacket {
                    reason: match origin.reason.text {
                        Some(t) => t,
                        None => panic!("Unknown reason")
                    }
                })
            }
        }
        to_event {
            | origin: DisconnectPacket | -> TetsuResult<Event> {
                Ok(Event::Disconnect(Disconnect {
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
                }))
            }
        }
        fields {
            reason: String,
        }
    }

    (0x01) ClientBound Login EncryptionRequestVarIntPacket: EncryptionRequest {
        from_event {
            | origin: EncryptionRequest | -> TetsuResult<EncryptionRequestVarIntPacket> {
               Ok( EncryptionRequestVarIntPacket {
                    server_id: origin.server_id,
                    public_key: ByteArrayVarInt(origin.public_key.len(), origin.public_key),
                    verify_token: ByteArrayVarInt(origin.verify_token.len(), origin.verify_token)
                })
            }
        }
        to_event {
            | origin: EncryptionRequestVarIntPacket | -> TetsuResult<Event> {
                Ok(Event::EncryptionRequest(EncryptionRequest {
                    server_id: origin.server_id,
                    public_key: origin.public_key.1,
                    verify_token: origin.verify_token.1
                }))
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
            | origin: EncryptionResponse | -> TetsuResult<EncryptionResponseVarIntPacket> {
                Ok(EncryptionResponseVarIntPacket {
                    shared_secret: ByteArrayVarInt(origin.shared_secret.len(), origin.shared_secret),
                    verify_token: ByteArrayVarInt(origin.verify_token.len(), origin.verify_token)
                })
            }
        }
        to_event {
            | origin: EncryptionResponseVarIntPacket | -> TetsuResult<Event> {
                Ok(Event::EncryptionResponse(EncryptionResponse {
                    shared_secret: origin.shared_secret.1,
                    verify_token: origin.verify_token.1
                }))
            }
        }
        fields {
            shared_secret: ByteArrayVarInt,
            verify_token: ByteArrayVarInt,
        }
    }

    (0x02) ClientBound Login LoginSuccessPacket: LoginSuccess {
        from_event {
            | origin: LoginSuccess | -> TetsuResult<LoginSuccessPacket> {
                Ok(LoginSuccessPacket {
                    uuid: origin.uuid.to_hyphenated().to_string(),
                    name: origin.name
                })
            }
        }
        to_event {
            | origin: LoginSuccessPacket | -> TetsuResult<Event> {
                Ok(Event::LoginSuccess(LoginSuccess {
                    uuid: Uuid::parse_str(&origin.uuid[..]).unwrap(),
                    name: origin.name,
                }))
            }
        }
        fields {
            uuid: String,
            name: String,
        }
    }

    (0x03) ClientBound Login SetCompressionPacket: SetCompression {
        from_event {
            | origin: SetCompression | -> TetsuResult<SetCompressionPacket> {
                Ok(SetCompressionPacket {
                    threshold: VarInt(origin.threshold)
                })
            }
        }
        to_event {
            | origin: SetCompressionPacket | -> TetsuResult<Event> {
                Ok(Event::SetCompression(SetCompression {
                    threshold: origin.threshold.0
                }))
            }
        }
        fields {
            threshold: VarInt,
        }
    }

    // Play --------------------

    (0x00) ClientBound Play KeepAlivePacket: KeepAlive {
        from_event {
            | origin: KeepAlive | -> TetsuResult<KeepAlivePacket> {
                Ok(KeepAlivePacket {
                    id: VarInt(origin.id as i32)
                })
            }
        }
        to_event {
            | origin: KeepAlivePacket | -> TetsuResult<Event> {
                Ok(Event::KeepAlive(KeepAlive {
                    id: origin.id.0 as i64
                }))
            }
        }
        fields {
            id: VarInt,
        }
    }

    (0x01) ClientBound Play JoinGamePacket: JoinGame {
        from_event {
            | origin: JoinGame | -> TetsuResult<JoinGamePacket> {
                Ok(JoinGamePacket {
                    id: origin.id,
                    gamemode: gamemode_to_byte(&origin.gamemode) | (if origin.is_hardcore { 0x80 } else { 0x00 }),
                    dimension: dimension_to_byte(&origin.dimension),
                    difficulty: difficulty_to_byte(&origin.difficulty),
                    max_players: origin.max_players as u8,
                    level_type: origin.world_type,
                    reduced_debug: origin.reduced_debug
                })
            }
        }
        to_event {
            | origin: JoinGamePacket | -> TetsuResult<Event> {
                Ok(Event::JoinGame(JoinGame {
                    id: origin.id,
                    gamemode: byte_to_gamemode(origin.gamemode),
                    is_hardcore: origin.gamemode & 0x80 == 0x80,
                    dimension: byte_to_dimension(origin.dimension),
                    difficulty: byte_to_difficulty(origin.difficulty),
                    max_players: origin.max_players as u32,
                    world_type: origin.level_type,
                    reduced_debug: origin.reduced_debug
                }))
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
            | origin: SpawnPosition | -> TetsuResult<SpawnPositionPacket> {
                Ok(SpawnPositionPacket {
                    location: origin.location.into()
                })
            }
        }
        to_event {
            | origin: SpawnPositionPacket | -> TetsuResult<Event> {
                Ok(Event::SpawnPosition(SpawnPosition {
                    location: origin.location.into()
                }))
            }
        }
        fields {
            location: PositionXZY,
        }
    }

    (0x3F) ClientBound Play PluginMessagePacket: PluginMessage {
        from_event {
            | origin: PluginMessage | -> TetsuResult<PluginMessagePacket> {
                Ok(PluginMessagePacket {
                    channel: origin.channel,
                    data: origin.data
                })
            }
        }
        to_event {
            | origin: PluginMessagePacket | -> TetsuResult<Event> {
                Ok(Event::PluginMessage(PluginMessage {
                    channel: origin.channel,
                    data: origin.data
                }))
            }
        }
        fields {
            channel: String,
            data: Vec<u8>,
        }
    }

    (0x41) ClientBound Play ServerDifficultyUpdatePacket: ServerDifficultyUpdate {
        from_event {
            | origin: ServerDifficultyUpdate | -> TetsuResult<ServerDifficultyUpdatePacket> {
                Ok(ServerDifficultyUpdatePacket {
                    difficulty: difficulty_to_byte(&origin.difficulty)
                })
            }
        }
        to_event {
            | origin: ServerDifficultyUpdatePacket | -> TetsuResult<Event> {
                Ok(Event::ServerDifficultyUpdate(ServerDifficultyUpdate {
                    difficulty: byte_to_difficulty(origin.difficulty),
                    difficulty_locked: false
                }))
            }
        }
        fields {
            difficulty: UnsignedByte,
        }
    }

}
