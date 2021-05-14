//! Event implementation for v754 of the protocol.
//! V754 covers server versions 1.16.4-1.16.5.
//! This implementation is not given priority to as
//! v47 will be implemented first.

use super::common::*;

use crate::errors::*;
use crate::event::*;
use crate::serialization::*;

use crate::versions::v47::{
    EncryptionRequestVarIntPacket, EncryptionResponseVarIntPacket, LoginStartPacket,
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

}
