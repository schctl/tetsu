use std::convert::{TryFrom, TryInto};

use super::common::*;
use crate::errors::*;
use crate::event::*;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

pub trait V47Readable<F>: Sized {
    fn v47_read<T: std::io::Read>(buf: &mut T) -> TetsuResult<F>;
}

pub trait V47Writable: Sized {
    fn v47_write<T: std::io::Write>(&self, buf: &mut T) -> TetsuResult<()>;
}

// ----- Other types -----------------------------

impl V47Readable<Position> for Position {
    fn v47_read<T: std::io::Read>(buf: &mut T) -> TetsuResult<Position> {
        let val = buf.read_u64::<BigEndian>()?;

        let x = (val as i64) >> 38;
        let y = ((val as i64) >> 26) & 0xFFF;
        let z = ((val as i64) << 38) >> 38;

        Ok(Self {
            x: if x >= (2 << 24) { x - (2 << 25) } else { x },
            y: if y >= (2 << 10) { y - (2 << 11) } else { y },
            z: if z >= (2 << 24) { z - (2 << 25) } else { z },
        })
    }
}

impl V47Writable for Position {
    fn v47_write<T: std::io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        Ok(buf.write_i64::<BigEndian>(
            ((self.x & 0x3FFFFFF) << 38) | ((self.y & 0xFFF) << 26) | (self.z & 0x3FFFFFF),
        )?)
    }
}

// ----------------------------------

impl V47Readable<Dimension> for Dimension {
    fn v47_read<T: std::io::Read>(_buf: &mut T) -> TetsuResult<Dimension> {
        Ok(match Byte::read_from(_buf)? {
            -1 => Dimension::Nether,
            0 => Dimension::Overworld,
            1 => Dimension::End,
            _ => {
                return Err(Error::from(InvalidValue {
                    expected: "-1, 0, 1".to_owned(),
                }))
            }
        })
    }
}

impl V47Writable for Dimension {
    fn v47_write<T: std::io::Write>(&self, _buf: &mut T) -> TetsuResult<()> {
        (match self {
            Dimension::Nether => -1,
            Dimension::Overworld => 0,
            Dimension::End => 1,
        } as Byte)
            .write_to(_buf)
    }
}

// ----------------------------------

impl V47Readable<Statistic> for Statistic {
    fn v47_read<T: std::io::Read>(buf: &mut T) -> TetsuResult<Statistic> {
        Ok(Self {
            name: String::read_from(buf)?,
            value: VarInt::read_from(buf)?.0,
        })
    }
}

impl V47Writable for Statistic {
    fn v47_write<T: std::io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        self.name.write_to(buf)?;
        VarInt(self.value).write_to(buf)
    }
}

// Auto implemented ------------------------------

auto_read_and_write_impl! {
    (read: V47Readable<Event>, v47_read;
    write: V47Writable, v47_write) => {
        // Status ========================================
        // Client bound ----------------------------------
        {
            Pong,
            payload: Long,
        }
        // Server bound ----------------------------------
        {
            StatusRequest,
        }
        {
            Ping,
            payload: Long,
        }

        // Login =========================================
        // Client bound ----------------------------------
        {
            EncryptionRequest,
            server_id: String,
            public_key: ByteArrayVarInt,
            verify_token: ByteArrayVarInt,
        }
        {
            SetCompression,
            threshold: VarInt,
        }
        // Server bound ----------------------------------
        {
            LoginStart,
            name: String,
        }
        {
            EncryptionResponse,
            shared_secret: ByteArrayVarInt,
            verify_token: ByteArrayVarInt,
        }
        // Play ==========================================
        // Client bound ----------------------------------
        {
            TimeUpdate,
            world_age: Long,
            time_of_day: Long,
        }
        {
            HeldItemChange,
            slot: i8,
        }
        {
            PluginMessage,
            channel: String,
            data: Vec<u8>,
        }
    }
}

new_protocol_impl! {
    (read: V47Readable, v47_read;
    write: V47Writable, v47_write) => {
        // Handshake =====================================
        // Server bound ----------------------------------
        (0x00, ServerBound, Handshake) => Handshake,

        // Status ========================================
        // Client bound ----------------------------------
        (0x00, ClientBound, Status) => StatusResponse,
        (0x01, ClientBound, Status) => Pong,
        // Server bound ----------------------------------
        (0x00, ServerBound, Status) => StatusRequest,
        (0x01, ServerBound, Status) => Ping,

        // Login =========================================
        // Client bound ----------------------------------
        (0x00, ClientBound, Login) => Disconnect,
        (0x01, ClientBound, Login) => EncryptionRequest,
        (0x02, ClientBound, Login) => LoginSuccess,
        (0x03, ClientBound, Login) => SetCompression,
        // Server bound ----------------------------------
        (0x00, ServerBound, Login) => LoginStart,
        (0x01, ServerBound, Login) => EncryptionResponse,

        // Play ==========================================
        // Client bound ----------------------------------
        (0x00, ClientBound, Play) => KeepAlive,
        (0x01, ClientBound, Play) => JoinGame,
        (0x03, ClientBound, Play) => TimeUpdate,
        (0x05, ClientBound, Play) => SpawnPosition,
        (0x08, ClientBound, Play) => PlayerPositionAndLook,
        (0x09, ClientBound, Play) => HeldItemChange,
        (0x30, ClientBound, Play) => WindowItemsUpdate,
        (0x37, ClientBound, Play) => Statistics,
        (0x38, ClientBound, Play) => PlayerInfoUpdate,
        (0x39, ClientBound, Play) => PlayerAbility,
        (0x3F, ClientBound, Play) => PluginMessage,
        (0x41, ClientBound, Play) => ServerDifficultyUpdate,
        (0x44, ClientBound, Play) => WorldBorder,
    }
}

// =========== Manual Implementations ============

// Handshake =====================================
// Server bound ----------------------------------

impl V47Readable<Event> for Handshake {
    fn v47_read<T: std::io::Read>(buf: &mut T) -> TetsuResult<Event> {
        let _ = VarInt::read_from(buf)?;
        Ok(Event::Handshake(Handshake {
            server_address: String::read_from(buf)?,
            server_port: UnsignedShort::read_from(buf)?,
            next_state: EventState::read_from(buf)?,
        }))
    }
}

impl V47Writable for Handshake {
    fn v47_write<T: std::io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        VarInt(47).write_to(buf)?;
        self.server_address.write_to(buf)?;
        self.server_port.write_to(buf)?;
        self.next_state.write_to(buf)
    }
}

// Status ========================================
// Client bound ----------------------------------

impl V47Readable<Event> for StatusResponse {
    fn v47_read<T: std::io::Read>(buf: &mut T) -> TetsuResult<Event> {
        Ok(Event::StatusResponse(Self {
            response: serde_json::from_str(&String::read_from(buf)?[..])?,
        }))
    }
}

impl V47Writable for StatusResponse {
    fn v47_write<T: std::io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        serde_json::to_string(&self.response)?.write_to(buf)
    }
}

// Server bound ----------------------------------

// Login =========================================
// Client bound ----------------------------------

impl V47Readable<Event> for Disconnect {
    fn v47_read<T: std::io::Read>(buf: &mut T) -> TetsuResult<Event> {
        Ok(Event::Disconnect(Disconnect {
            reason: Chat {
                text: Some(String::read_from(buf)?),
                ..Default::default()
            },
        }))
    }
}

impl V47Writable for Disconnect {
    fn v47_write<T: std::io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        match &self.reason.text {
            Some(t) => t.write_to(buf),
            _ => Err(Error::from(InvalidValue {
                expected: "Text field".to_owned(),
            })),
        }
    }
}

// ----------------------------------

impl V47Readable<Event> for LoginSuccess {
    fn v47_read<T: std::io::Read>(buf: &mut T) -> TetsuResult<Event> {
        Ok(Event::LoginSuccess(LoginSuccess {
            uuid: Uuid::parse_str(&String::read_from(buf)?[..]).unwrap(),
            name: String::read_from(buf)?,
        }))
    }
}

impl V47Writable for LoginSuccess {
    fn v47_write<T: std::io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        self.uuid.to_hyphenated().to_string().write_to(buf)?;
        self.name.write_to(buf)
    }
}

// Play ==========================================
// Client bound ----------------------------------

impl V47Readable<Event> for KeepAlive {
    fn v47_read<T: std::io::Read>(buf: &mut T) -> TetsuResult<Event> {
        Ok(Event::KeepAlive(KeepAlive {
            id: VarInt::read_from(buf)?.0 as i64,
        }))
    }
}

impl V47Writable for KeepAlive {
    fn v47_write<T: std::io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        VarInt(self.id as i32).write_to(buf)
    }
}

// ----------------------------------

impl V47Readable<Event> for JoinGame {
    fn v47_read<T: std::io::Read>(buf: &mut T) -> TetsuResult<Event> {
        let id = Int::read_from(buf)?;
        let gamemode = UnsignedByte::read_from(buf)?;
        let dimension = Dimension::v47_read(buf)?;
        let difficulty = Difficulty::read_from(buf)?;
        let max_players = UnsignedByte::read_from(buf)?;
        let level_type = String::read_from(buf)?;
        let reduced_debug = bool::read_from(buf)?;

        Ok(Event::JoinGame(Self {
            id,
            gamemode: (gamemode as i32).try_into()?,
            is_hardcore: gamemode & 0x80 == 0x80,
            worlds: None,
            dimension: Some(dimension),
            dimension_registry: None,
            dimension_codec: None,
            world_name: None,
            difficulty: Some(difficulty),
            hashed_seed: None,
            max_players: max_players as u32,
            level_type: Some(level_type),
            view_distance: None,
            reduced_debug,
            enable_respawn: None,
            is_debug: None,
            is_flat: None,
        }))
    }
}

impl V47Writable for JoinGame {
    fn v47_write<T: std::io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        self.id.write_to(buf)?;

        (i32::try_from(self.gamemode.clone())? | if self.is_hardcore { 0x80 } else { 0x00 })
            .write_to(buf)?;

        match &self.dimension {
            Some(d) => d.v47_write(buf)?,
            _ => panic!("Expected dimension"),
        }
        match &self.difficulty {
            Some(d) => d.write_to(buf)?,
            _ => panic!("Expected difficulty"),
        }

        self.max_players.write_to(buf)?;

        match &self.level_type {
            Some(l) => l.write_to(buf)?,
            _ => panic!("Level Expected."),
        }

        self.reduced_debug.write_to(buf)
    }
}

// ----------------------------------

impl V47Readable<Event> for SpawnPosition {
    fn v47_read<T: std::io::Read>(buf: &mut T) -> TetsuResult<Event> {
        Ok(Event::SpawnPosition(Self {
            location: Position::v47_read(buf)?,
        }))
    }
}

impl V47Writable for SpawnPosition {
    fn v47_write<T: std::io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        self.location.v47_write(buf)
    }
}

// ----------------------------------

impl V47Readable<Event> for PlayerPositionAndLook {
    fn v47_read<T: std::io::Read>(buf: &mut T) -> TetsuResult<Event> {
        let x = Double::read_from(buf)?;
        let y = Double::read_from(buf)?;
        let z = Double::read_from(buf)?;
        let yaw = Float::read_from(buf)?;
        let pitch = Float::read_from(buf)?;
        let flags = Byte::read_from(buf)?;

        let x = if flags | 0x01 == 0x01 {
            RelativeOrAbsolute::Relative(x)
        } else {
            RelativeOrAbsolute::Absolute(x)
        };
        let y = if flags | 0x02 == 0x02 {
            RelativeOrAbsolute::Relative(y)
        } else {
            RelativeOrAbsolute::Absolute(y)
        };
        let z = if flags | 0x04 == 0x04 {
            RelativeOrAbsolute::Relative(z)
        } else {
            RelativeOrAbsolute::Absolute(z)
        };

        let yaw = if flags | 0x08 == 0x08 {
            RelativeOrAbsolute::Relative(yaw)
        } else {
            RelativeOrAbsolute::Absolute(yaw)
        };
        let pitch = if flags | 0x10 == 0x10 {
            RelativeOrAbsolute::Relative(pitch)
        } else {
            RelativeOrAbsolute::Absolute(pitch)
        };

        Ok(Event::PlayerPositionAndLook(Self {
            x,
            y,
            z,
            yaw,
            pitch,
            teleport_id: None,
        }))
    }
}

impl V47Writable for PlayerPositionAndLook {
    fn v47_write<T: std::io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        let mut flags = 0;

        let x = match self.x {
            RelativeOrAbsolute::Relative(x) => {
                flags |= 0x01;
                x
            }
            RelativeOrAbsolute::Absolute(x) => x,
        };
        let y = match self.y {
            RelativeOrAbsolute::Relative(x) => {
                flags |= 0x02;
                x
            }
            RelativeOrAbsolute::Absolute(x) => x,
        };
        let z = match self.z {
            RelativeOrAbsolute::Relative(x) => {
                flags |= 0x04;
                x
            }
            RelativeOrAbsolute::Absolute(x) => x,
        };

        let yaw = match self.yaw {
            RelativeOrAbsolute::Relative(x) => {
                flags |= 0x08;
                x
            }
            RelativeOrAbsolute::Absolute(x) => x,
        };
        let pitch = match self.pitch {
            RelativeOrAbsolute::Relative(x) => {
                flags |= 0x10;
                x
            }
            RelativeOrAbsolute::Absolute(x) => x,
        };

        x.write_to(buf)?;
        y.write_to(buf)?;
        z.write_to(buf)?;
        yaw.write_to(buf)?;
        pitch.write_to(buf)?;
        flags.write_to(buf)
    }
}

// ----------------------------------

impl V47Readable<Event> for ChangeGameState {
    fn v47_read<T: std::io::Read>(buf: &mut T) -> TetsuResult<Event> {
        Ok(Event::ChangeGameState(
            match UnsignedByte::read_from(buf)? {
                0 => ChangeGameState::NoRespawnBlock,
                1 => ChangeGameState::EndRaining,
                2 => ChangeGameState::BeginRaining,
                3 => ChangeGameState::GamemodeUpdate(Gamemode::try_from(
                    Float::read_from(buf)? as i32
                )?),
                4 => ChangeGameState::WinGame(AfterGameWin::CreditsAndRespawn),
                5 => ChangeGameState::DemoEvent(match Float::read_from(buf)? as i32 {
                    0 => DemoEventAction::Show,
                    101 => DemoEventAction::ShowMovementControls,
                    102 => DemoEventAction::ShowJumpControl,
                    103 => DemoEventAction::ShowInventoryControl,
                    _ => DemoEventAction::Over, // for now
                }),
                6 => ChangeGameState::ArrowHitPlayer,
                7 => ChangeGameState::FadeValue(Float::read_from(buf)?),
                8 => ChangeGameState::FadeTime(Float::read_from(buf)?),
                10 => ChangeGameState::MobAppear,
                _ => {
                    return Err(Error::from(InvalidValue {
                        expected: "1-9 or 10".to_owned(),
                    }))
                }
            },
        ))
    }
}

impl V47Writable for ChangeGameState {
    fn v47_write<T: std::io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        match &self {
            ChangeGameState::NoRespawnBlock => (0 as UnsignedByte).write_to(buf)?,
            ChangeGameState::EndRaining => (1 as UnsignedByte).write_to(buf)?,
            ChangeGameState::BeginRaining => (2 as UnsignedByte).write_to(buf)?,
            ChangeGameState::GamemodeUpdate(g) => {
                (3 as UnsignedByte).write_to(buf)?;
                (i32::try_from(g.clone())? as Float).write_to(buf)?
            }
            ChangeGameState::WinGame(_) => (4 as UnsignedByte).write_to(buf)?,
            ChangeGameState::DemoEvent(d) => {
                (5 as UnsignedByte).write_to(buf)?;
                match d {
                    DemoEventAction::Show => 0f32.write_to(buf)?,
                    DemoEventAction::ShowMovementControls => 101f32.write_to(buf)?,
                    DemoEventAction::ShowJumpControl => 102f32.write_to(buf)?,
                    DemoEventAction::ShowInventoryControl => 103f32.write_to(buf)?,
                    DemoEventAction::Over => 104f32.write_to(buf)?,
                }
            }
            ChangeGameState::ArrowHitPlayer => (6 as UnsignedByte).write_to(buf)?,
            ChangeGameState::FadeValue(v) => {
                (7 as UnsignedByte).write_to(buf)?;
                v.write_to(buf)?
            }
            ChangeGameState::FadeTime(v) => {
                (8 as UnsignedByte).write_to(buf)?;
                v.write_to(buf)?
            }
            ChangeGameState::MobAppear => (10 as UnsignedByte).write_to(buf)?,
            _ => {
                return Err(Error::from(InvalidValue {
                    expected: "not supported for this protocol version".to_owned(),
                }))
            }
        }

        Ok(())
    }
}

// ----------------------------------

// Untested

impl V47Readable<Slot> for Slot {
    fn v47_read<T: std::io::Read>(buf: &mut T) -> TetsuResult<Slot> {
        let id = Byte::read_from(buf)?;

        if id == -1 {
            return Ok(Self {
                item_id: None,
                item_count: 0,
                damage: None,
                nbt: None,
            });
        }

        let item_count = Byte::read_from(buf)?;
        let damage = Short::read_from(buf)?;

        let nbt = NbtBlob::read_from(buf)?;

        Ok(Self {
            item_id: Some(id as i32),
            item_count,
            damage: Some(damage),
            nbt: Some(nbt),
        })
    }
}

impl V47Writable for Slot {
    fn v47_write<T: std::io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        match &self.item_id {
            None => {
                (-1 as Byte).write_to(buf)?;
                return Ok(());
            }
            Some(id) => id.write_to(buf)?,
        }

        self.item_count.write_to(buf)?;
        self.damage.unwrap().write_to(buf)?;
        self.nbt.clone().unwrap().write_to(buf)
    }
}

impl V47Readable<Event> for WindowItemsUpdate {
    fn v47_read<T: std::io::Read>(buf: &mut T) -> TetsuResult<Event> {
        let window_id = UnsignedByte::read_from(buf)?;
        let slots_len = Short::read_from(buf)?;
        let mut slots = Vec::with_capacity(slots_len as usize);
        for _ in 0..slots_len {
            slots.push(Slot::v47_read(buf)?);
        }
        Ok(Event::WindowItemsUpdate(WindowItemsUpdate {
            window_id,
            slots: slots,
        }))
    }
}

impl V47Writable for WindowItemsUpdate {
    fn v47_write<T: std::io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        self.window_id.write_to(buf)?;
        (self.slots.len() as Short).write_to(buf)?;
        for i in &self.slots {
            i.v47_write(buf)?;
        }
        Ok(())
    }
}

// ----------------------------------

impl V47Readable<Event> for Statistics {
    fn v47_read<T: std::io::Read>(buf: &mut T) -> TetsuResult<Event> {
        let len = VarInt::read_from(buf)?.0 as usize;
        let mut values = Vec::with_capacity(len);

        for _ in 0..len {
            values.push(Statistic::v47_read(buf)?)
        }

        Ok(Event::Statistics(Self { values }))
    }
}

impl V47Writable for Statistics {
    fn v47_write<T: std::io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        VarInt(self.values.len() as i32).write_to(buf)?;
        for i in &self.values {
            i.v47_write(buf)?;
        }
        Ok(())
    }
}

// ----------------------------------

impl V47Readable<Event> for PlayerInfoUpdate {
    fn v47_read<T: std::io::Read>(buf: &mut T) -> TetsuResult<Event> {
        let action = VarInt::read_from(buf)?.0;
        let player_len = VarInt::read_from(buf)?.0;

        let mut players = vec![];

        for _ in 0..player_len {
            let uuid = Uuid::read_from(buf)?;
            let p_action = match action {
                0 => PlayerInfoAction::Add(PlayerInfoAdd::read_from(buf)?),
                1 => PlayerInfoAction::GamemodeUpdate(PlayerGamemodeUpdate::read_from(buf)?),
                2 => PlayerInfoAction::LatencyUpdate(PlayerLatencyUpdate::read_from(buf)?),
                3 => PlayerInfoAction::DisplayNameUpdate(PlayerDisplayNameUpdate::read_from(buf)?),
                4 => PlayerInfoAction::Remove(RemovePlayer {}),
                _ => panic!("Unknown"),
            };
            players.push(PlayerListInfo {
                uuid,
                action: p_action,
            });
        }

        Ok(Event::PlayerInfoUpdate(PlayerInfoUpdate { players }))
    }
}

impl V47Writable for PlayerInfoUpdate {
    fn v47_write<T: std::io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        let player_len = self.players.len();

        match self.players[0].action {
            PlayerInfoAction::Add(_) => VarInt(0).write_to(buf)?,
            PlayerInfoAction::GamemodeUpdate(_) => VarInt(1).write_to(buf)?,
            PlayerInfoAction::LatencyUpdate(_) => VarInt(2).write_to(buf)?,
            PlayerInfoAction::DisplayNameUpdate(_) => VarInt(3).write_to(buf)?,
            PlayerInfoAction::Remove(_) => VarInt(4).write_to(buf)?,
        };

        VarInt(player_len as i32).write_to(buf)?;

        for i in self.players.iter() {
            i.uuid.write_to(buf)?;
            match &i.action {
                PlayerInfoAction::Add(e) => e.write_to(buf)?,
                PlayerInfoAction::GamemodeUpdate(e) => e.write_to(buf)?,
                PlayerInfoAction::LatencyUpdate(e) => e.write_to(buf)?,
                PlayerInfoAction::DisplayNameUpdate(e) => e.write_to(buf)?,
                PlayerInfoAction::Remove(_) => {}
            };
        }

        Ok(())
    }
}

// ----------------------------------

impl V47Readable<Event> for PlayerAbility {
    fn v47_read<T: std::io::Read>(buf: &mut T) -> TetsuResult<Event> {
        let flags = Byte::read_from(buf)?;
        let flying_speed = Float::read_from(buf)?;
        let walking_speed = Float::read_from(buf)?;

        Ok(Event::PlayerAbility(Self {
            invulnerable: flags & 0x01 == 0x01,
            is_flying: flags & 0x02 == 0x02,
            allow_flying: flags & 0x04 == 0x04,
            creative_mode: flags & 0x08 == 0x08,
            flying_speed,
            walking_speed,
        }))
    }
}

impl V47Writable for PlayerAbility {
    fn v47_write<T: std::io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        ((if self.invulnerable { 0x01 } else { 0x00 })
            | (if self.is_flying { 0x02 } else { 0x00 })
            | (if self.allow_flying { 0x04 } else { 0x00 })
            | (if self.creative_mode { 0x08 } else { 0x00 }) as Byte)
            .write_to(buf)?;
        self.flying_speed.write_to(buf)?;
        self.walking_speed.write_to(buf)
    }
}

// ----------------------------------

impl V47Readable<Event> for ServerDifficultyUpdate {
    fn v47_read<T: std::io::Read>(buf: &mut T) -> TetsuResult<Event> {
        Ok(Event::ServerDifficultyUpdate(Self {
            difficulty: Difficulty::read_from(buf)?,
            difficulty_locked: false,
        }))
    }
}

impl V47Writable for ServerDifficultyUpdate {
    fn v47_write<T: std::io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        self.difficulty.write_to(buf)
    }
}

// ----------------------------------

impl V47Readable<Event> for WorldBorder {
    fn v47_read<T: std::io::Read>(buf: &mut T) -> TetsuResult<Event> {
        Ok(Event::WorldBorder(match VarInt::read_from(buf)?.0 {
            0 => Self::SetSize {
                diameter: Double::read_from(buf)? / 2.0,
            },
            1 => Self::LerpSize {
                old_diameter: Double::read_from(buf)? / 2.0,
                new_diameter: Double::read_from(buf)? / 2.0,
                speed: VarLong::read_from(buf)?.0,
            },
            2 => Self::SetCenter {
                x: Double::read_from(buf)?,
                y: Double::read_from(buf)?,
            },
            3 => Self::Initialize {
                x: Double::read_from(buf)?,
                y: Double::read_from(buf)?,
                old_diameter: Double::read_from(buf)? / 2.0,
                new_diameter: Double::read_from(buf)? / 2.0,
                speed: VarLong::read_from(buf)?.0,
                portal_teleport_boundary: VarInt::read_from(buf)?.0,
                warning_time: VarInt::read_from(buf)?.0,
                warning_blocks: VarInt::read_from(buf)?.0,
            },
            4 => Self::SetWarnTime {
                warning_time: VarInt::read_from(buf)?.0,
            },
            5 => Self::SetWarnBlocks {
                warning_blocks: VarInt::read_from(buf)?.0,
            },
            _ => panic!("Unknown type"),
        }))
    }
}

impl V47Writable for WorldBorder {
    fn v47_write<T: std::io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        match &self {
            Self::SetSize { diameter } => {
                (diameter / 2.0).write_to(buf)?;
            }
            Self::LerpSize {
                old_diameter,
                new_diameter,
                speed,
            } => {
                (old_diameter / 2.0).write_to(buf)?;
                (new_diameter / 2.0).write_to(buf)?;
                speed.write_to(buf)?;
            }
            Self::SetCenter { x, y } => {
                x.write_to(buf)?;
                y.write_to(buf)?;
            }
            Self::Initialize {
                x,
                y,
                old_diameter,
                new_diameter,
                speed,
                portal_teleport_boundary,
                warning_time,
                warning_blocks,
            } => {
                x.write_to(buf)?;
                y.write_to(buf)?;
                (old_diameter / 2.0).write_to(buf)?;
                (new_diameter / 2.0).write_to(buf)?;
                speed.write_to(buf)?;
                portal_teleport_boundary.write_to(buf)?;
                warning_time.write_to(buf)?;
                warning_blocks.write_to(buf)?;
            }
            Self::SetWarnTime { warning_time } => {
                warning_time.write_to(buf)?;
            }
            Self::SetWarnBlocks { warning_blocks } => {
                warning_blocks.write_to(buf)?;
            }
        }

        Ok(())
    }
}
