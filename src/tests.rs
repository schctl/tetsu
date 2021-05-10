#[test]
fn test_event_serialization() {
    use crate::event::*;
    use std::io::Cursor;

    let mut connection = Cursor::new(Vec::new());

    let write_handshake = Event::Handshake(Handshake {
        server_address: "127.0.0.1".to_owned(),
        server_port: 25565,
        next_state: EventState::Login,
    });
    write_handshake
        .clone()
        .write_to(
            &mut connection,
            &EventState::Handshake,
            &EventDirection::ServerBound,
            &ProtocolVersion::V47,
            0,
        )
        .unwrap();

    connection.set_position(0);

    let read_handshake = Event::read_from(
        &mut connection,
        &EventState::Handshake,
        &EventDirection::ServerBound,
        &ProtocolVersion::V47,
        0,
    )
    .unwrap();

    assert_eq!(write_handshake, read_handshake)
}
