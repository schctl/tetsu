//! Event implementation for v754 of the protocol.
//! V754 covers server versions 1.16.4-1.16.5.
//! This implementation is not given priority to as
//! v47 will be implemented first.

use super::common::*;

use crate::errors::*;
use crate::event::*;
use crate::serialization::*;

use crate::versions::v47::{
    self, EncryptionRequestVarIntPacket, EncryptionResponseVarIntPacket, LoginStartPacket,
    SetCompressionPacket, StatusPingPacket, StatusPongPacket, StatusRequestPacket,
    StatusResponsePacket,
};

pub mod internal {}

protocol_impl! {

    inherit {
        StatusPingPacket: Ping;
        StatusPongPacket: Pong;
        StatusRequestPacket: StatusRequest;
        StatusResponsePacket: StatusResponse;

        LoginStartPacket: LoginStart;
        EncryptionRequestVarIntPacket: EncryptionRequest;
        EncryptionResponseVarIntPacket: EncryptionResponse;
        SetCompressionPacket: SetCompression;
    }

    // Handshake ---------------

    (0x00) ServerBound Handshake HandshakePacket: Handshake {
        from_event {
            fn try_from(item: Handshake) -> TetsuResult<HandshakePacket> {
                Ok(HandshakePacket {
                    protocol_version: VarInt(754),
                    server_address: item.server_address,
                    server_port: item.server_port,
                    next_state: match item.next_state {
                        EventState::Status => VarInt(1),
                        EventState::Login => VarInt(2),
                        _ => return Err(Error::from(InvalidValue { expected: "Status or Login".to_owned() }))
                    }
                })
            }
        }
        to_event {
            fn try_from(item: HandshakePacket) -> TetsuResult<Event> {
                Ok(Event::Handshake(Handshake {
                    server_address: item.server_address,
                    server_port: item.server_port,
                    next_state: match item.next_state.0 {
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

    (0x02) ClientBound Login LoginSuccessPacket: LoginSuccess {
        from_event {
            fn try_from(item: LoginSuccess) -> TetsuResult<LoginSuccessPacket> {
                Ok(LoginSuccessPacket {
                    uuid: item.uuid,
                    name: item.name
                })
            }
        }
        to_event {
            fn try_from(item: LoginSuccessPacket) -> TetsuResult<Event> {
                Ok(Event::LoginSuccess(LoginSuccess {
                    uuid: item.uuid,
                    name: item.name,
                }))
            }
        }
        fields {
            uuid: Uuid,
            name: String,
        }
    }

    (0x00) ClientBound Login DisconnectPacket: Disconnect {
        from_event {
            fn try_from(item: Disconnect) -> TetsuResult<DisconnectPacket> {
                Ok(DisconnectPacket {
                    reason: item.reason
                })
            }
        }
        to_event {
            fn try_from(item: DisconnectPacket) -> TetsuResult<Event> {
                Ok(Event::Disconnect(Disconnect {
                    reason: item.reason
                }))
            }
        }
        fields {
            reason: Chat,
        }
    }

    // Play --------------------

    (0x0D) ClientBound Play ServerDifficultyUpdatePacket: ServerDifficultyUpdate {
        from_event {
            fn try_from(item: ServerDifficultyUpdate) -> TetsuResult<ServerDifficultyUpdatePacket> {
                Ok(ServerDifficultyUpdatePacket {
                    difficulty: v47::internal::difficulty_to_byte(&item.difficulty),
                    difficulty_locked: item.difficulty_locked,
                })
            }
        }
        to_event {
            fn try_from(item: ServerDifficultyUpdatePacket) -> TetsuResult<Event> {
                Ok(Event::ServerDifficultyUpdate(ServerDifficultyUpdate {
                    difficulty: v47::internal::byte_to_difficulty(item.difficulty),
                    difficulty_locked: item.difficulty_locked
                }))
            }
        }
        fields {
            difficulty: UnsignedByte,
            difficulty_locked: bool,
        }
    }

    (0x17) ClientBound Play PluginMessagePacket: PluginMessage {
        from_event {
            fn try_from(item: PluginMessage) -> TetsuResult<PluginMessagePacket> {
                Ok(PluginMessagePacket {
                    channel: item.channel,
                    data: item.data
                })
            }
        }
        to_event {
            fn try_from(item: PluginMessagePacket) -> TetsuResult<Event> {
                Ok(Event::PluginMessage(PluginMessage {
                    channel: item.channel,
                    data: item.data
                }))
            }
        }
        fields {
            channel: String,
            data: Vec<u8>,
        }
    }

    (0x24) ClientBound Play JoinGamePacket: JoinGame {
        from_event {
            fn try_from(item: JoinGame) -> TetsuResult<JoinGamePacket> {
                Ok(JoinGamePacket {
                    id: item.id,
                    is_hardcore: item.is_hardcore,
                    gamemode: v47::internal::gamemode_to_byte(&item.gamemode) | (if item.is_hardcore { 0x80 } else { 0x00 }),
                    worlds: item.worlds.unwrap().into(),
                    previous_gamemode: -1, // for now, TODO: fix
                    dimension_registry: item.dimension_registry.unwrap(),
                    dimension_codec: item.dimension_codec.unwrap(),
                    world_name: item.world_name.unwrap(),
                    hashed_seed: item.hashed_seed.unwrap(),
                    max_players: item.max_players as u8,
                    view_distance: item.view_distance.unwrap().into(),
                    reduced_debug: item.reduced_debug,
                    enable_respawn: item.enable_respawn.unwrap(),
                    is_debug: item.is_debug.unwrap(),
                    is_flat: item.is_flat.unwrap(),
                })
            }
        }
        to_event {
            fn try_from(item: JoinGamePacket) -> TetsuResult<Event> {
                Ok(Event::JoinGame(JoinGame {
                    id: item.id,
                    is_hardcore: item.is_hardcore,
                    gamemode: v47::internal::byte_to_gamemode(item.gamemode),
                    worlds: Some(item.worlds.into()),
                    dimension: None,
                    dimension_registry: Some(item.dimension_registry),
                    dimension_codec: Some(item.dimension_codec),
                    world_name: Some(item.world_name),
                    difficulty: None,
                    hashed_seed: Some(item.hashed_seed),
                    max_players: item.max_players as u32,
                    level_type: None,
                    view_distance: Some(item.view_distance.into()),
                    reduced_debug: item.reduced_debug,
                    enable_respawn: Some(item.enable_respawn),
                    is_debug: Some(item.is_debug),
                    is_flat: Some(item.is_flat),
                }))
            }
        }
        fields {
            id: Int,
            is_hardcore: bool,
            gamemode: UnsignedByte,
            previous_gamemode: Byte,
            worlds: GenericArray<VarInt, String>,
            dimension_registry: NbtBlob,
            dimension_codec: NbtBlob,
            world_name: String,
            hashed_seed: Long,
            max_players: UnsignedByte,
            view_distance: VarInt,
            reduced_debug: bool,
            enable_respawn: bool,
            is_debug: bool,
            is_flat: bool,
        }
    }

}
