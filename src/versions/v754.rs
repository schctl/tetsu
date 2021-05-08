//! Event implementation for v754 of the protocol.
//! V754 covers server versions 1.16.4-1.16.5.
//! This implementation is not given priority to as
//! v47 will be implemented first.

use crate::event::*;
use crate::packet::*;

pub use crate::versions::v47::{
    EncryptionRequestVarIntPacket, EncryptionResponseVarIntPacket, LoginStartPacket,
    SetCompressionPacket, StatusPingPacket, StatusPongPacket, StatusRequestPacket,
    StatusResponsePacket,
};

packet_impl! {

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
            | origin: Handshake | -> HandshakePacket {
                HandshakePacket {
                    protocol_version: VarInt(754),
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

    (0x02) ClientBound Login LoginSuccessPacket: LoginSuccess {
        from_event {
            | origin: LoginSuccess | -> LoginSuccessPacket {
                LoginSuccessPacket {
                    uuid: origin.uuid,
                    name: origin.name
                }
            }
        }
        to_event {
            | origin: LoginSuccessPacket | -> Event {
                Event::LoginSuccess(LoginSuccess {
                    uuid: origin.uuid,
                    name: origin.name,
                })
            }
        }
        fields {
            uuid: Uuid,
            name: String,
        }
    }

    (0x00) ClientBound Login DisconnectPacket: Disconnect {
        from_event {
            | origin: Disconnect | -> DisconnectPacket {
                DisconnectPacket {
                    reason: origin.reason
                }
            }
        }
        to_event {
            | origin: DisconnectPacket | -> Event {
                Event::Disconnect(Disconnect {
                    reason: origin.reason
                })
            }
        }
        fields {
            reason: Chat,
        }
    }

    // Play --------------------

}
