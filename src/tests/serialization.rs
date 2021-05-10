#![allow(unused_imports)]

use std::io::prelude::*;
use std::{fs::File, io, time};
use flate2::write;
use serde_json::json;

use crate::{
    event::{
        self, Disconnect, Event, EventDirection, EventState, Handshake, Position, SpawnPosition,
    },
    packet::Chat,
};

const SER_RUNS: u128 = 50_000;

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
        ),
        (
            Event::Handshake(Handshake {
                server_address: "127.0.0.1".to_owned(),
                server_port: 25565,
                next_state: EventState::Status,
            }),
            EventState::Handshake,
            EventDirection::ServerBound,
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
        ),
    ];

    let mut writes = vec![];
    let mut reads = vec![];

    for _ in 0..SER_RUNS {
        let mut buf = io::Cursor::new(Vec::new());
        // Write
        for (e, s, d) in events.iter() {
            let event_w = e.clone();

            let start = time::Instant::now();
            event_w
                .write_to(&mut buf, s, d, &event::ProtocolVersion::V47, 0)
                .unwrap();
            writes.push(start.elapsed().as_nanos() as u64);
        }
        buf.set_position(0);
        // Read
        for (e, s, d) in events.iter() {
            let start = time::Instant::now();
            let event_r =
                Event::read_from(&mut buf, s, d, &event::ProtocolVersion::V47, 0).unwrap();
            reads.push(start.elapsed().as_nanos() as u64);
            assert_eq!(e, &event_r);
        }
    }

    let res = json!([reads, writes]);
    let mut f = File::create("target/rw.json").unwrap();
    write!(f, "{}", res).unwrap();

    println!(
        "Write avg: {}",
        writes.iter().sum::<u64>() as f32 / writes.len() as f32
    );
    println!(
        "Read avg: {}",
        reads.iter().sum::<u64>() as f32 / writes.len() as f32
    );
}
