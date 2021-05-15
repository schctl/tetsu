use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use serde_json::json;

use crate::event::Chat;
use crate::event::{
    dispatcher::EventDispatcher, Disconnect, Event, EventDirection, EventState, Handshake,
    ProtocolVersion,
};

const SER_RUNS: usize = 12_000;

fn test_protocol_version(
    version: String,
    dispatcher: EventDispatcher<io::Cursor<Vec<u8>>, io::Cursor<Vec<u8>>>,
    events: &[(Event, EventState, EventDirection, &'static str); 2],
) {
    let mut times: HashMap<String, (Vec<u64>, Vec<u64>)> = HashMap::new();

    for (_, _, _, s) in events.iter() {
        times.insert(s.to_string(), (vec![], vec![]));
    }

    for _ in 0..SER_RUNS {
        let mut buf = io::Cursor::new(Vec::new());

        // Write
        for (e, s, d, name) in events.iter() {
            let event_w = e.clone();
            let start = std::time::Instant::now();

            dispatcher.write_event(&mut buf, event_w, s, d, 0).unwrap();

            times
                .get_mut(&name.to_string())
                .unwrap()
                .0
                .push(start.elapsed().as_nanos() as u64);
        }

        buf.set_position(0);

        // Read
        for (e, s, d, name) in events.iter() {
            let start = std::time::Instant::now();

            let event_r = dispatcher.read_event(&mut buf, s, d, 0).unwrap();

            times
                .get_mut(&name.to_string())
                .unwrap()
                .1
                .push(start.elapsed().as_nanos() as u64);

            assert_eq!(e, &event_r);
        }
    }

    write!(
        File::create(format!("target/protocol-ser-test-{}.json", version)).unwrap(),
        "{}",
        json!(times)
    )
    .unwrap();
}

#[test]
fn test_event_serialization() {
    env_logger::builder()
        .filter(Some("tetsu"), log::LevelFilter::Debug)
        .init();

    let events = [
        (
            Event::Handshake(Handshake {
                server_address: "127.0.0.1".to_owned(),
                server_port: 25565,
                next_state: EventState::Status,
            }),
            EventState::Handshake,
            EventDirection::ServerBound,
            "Handshake",
        ),
        (
            Event::Disconnect(Disconnect {
                reason: Chat {
                    text: Some("None".to_owned()),
                    ..Default::default()
                },
            }),
            EventState::Login,
            EventDirection::ClientBound,
            "Disconnect",
        ),
    ];

    test_protocol_version(
        "47".to_owned(),
        EventDispatcher::new(&ProtocolVersion::V47),
        &events,
    );
    test_protocol_version(
        "754".to_owned(),
        EventDispatcher::new(&ProtocolVersion::V754),
        &events,
    );
}
