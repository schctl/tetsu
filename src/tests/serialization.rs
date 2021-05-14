use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use serde_json::json;

use crate::event::Chat;
use crate::event::{
    self, dispatcher::EventDispatcher, Disconnect, Event, EventDirection, EventState, Handshake,
    KeepAlive, PlayerAbility, Position, SpawnPosition, Statistic, Statistics,
};

const SER_RUNS: usize = 4_096;

#[test]
fn test_event_serialization() {
    env_logger::builder()
        .filter(Some("tetsu"), log::LevelFilter::Debug)
        .init();

    let events = [
        (
            Event::SpawnPosition(SpawnPosition {
                location: Position {
                    x: -120,
                    y: -120,
                    z: 1920,
                },
            }),
            EventState::Play,
            EventDirection::ClientBound,
            "SpawnPosition",
        ),
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
        (
            Event::KeepAlive(KeepAlive { id: 120 }),
            EventState::Play,
            EventDirection::ClientBound,
            "KeepAlive",
        ),
        (
            Event::PlayerAbility(PlayerAbility {
                invulnerable: false,
                is_flying: false,
                allow_flying: false,
                creative_mode: true,
                flying_speed: 1.2,
                walking_speed: 0.7,
            }),
            EventState::Play,
            EventDirection::ClientBound,
            "PlayerAbility",
        ),
        (
            Event::Statistics(Statistics {
                values: vec![Statistic {
                    name: "minecraft:something".to_owned(),
                    value: 128,
                }],
            }),
            EventState::Play,
            EventDirection::ClientBound,
            "Statistics",
        ),
    ];

    let mut times: HashMap<String, (Vec<u64>, Vec<u64>)> = HashMap::new();

    for (_, _, _, s) in events.iter() {
        times.insert(s.to_string(), (vec![], vec![]));
    }

    let event_dispatcher = EventDispatcher::new(&event::ProtocolVersion::V47);

    for _ in 0..SER_RUNS {
        let mut buf = io::Cursor::new(Vec::new());

        // Write
        for (e, s, d, name) in events.iter() {
            let event_w = e.clone();
            let start = std::time::Instant::now();

            event_dispatcher
                .write_event(&mut buf, event_w, s, d, 0)
                .unwrap();

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

            let event_r = event_dispatcher.read_event(&mut buf, s, d, 0).unwrap();

            times
                .get_mut(&name.to_string())
                .unwrap()
                .1
                .push(start.elapsed().as_nanos() as u64);

            assert_eq!(e, &event_r);
        }
    }

    write!(File::create("target/rw.json").unwrap(), "{}", json!(times)).unwrap();
}
