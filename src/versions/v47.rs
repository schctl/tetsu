//! Event implementation for v47 of the protocol.
//! V47 covers server versions 1.8 - 1.8.9

use crate::event::*;
use crate::packet::*;

use uuid::Uuid;

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

}
