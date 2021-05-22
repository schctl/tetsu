use std::convert::{TryFrom, TryInto};

use super::common::*;
use crate::errors::*;
use crate::event::*;

use super::v47::{V47Readable, V47Writable};

pub trait V754Readable<F>: Sized {
    fn v754_read<T: std::io::Read>(buf: &mut T) -> TetsuResult<F>;
}

pub trait V754Writable: Sized {
    fn v754_write<T: std::io::Write>(&self, buf: &mut T) -> TetsuResult<()>;
}

auto_read_and_write_impl! {
    (read: V754Readable<Event>, v754_read;
    write: V754Writable, v754_write) => {
        // Handshake =====================================
        // Server bound ----------------------------------

        // Status ========================================
        // Client bound ----------------------------------
        // Server bound ----------------------------------

        // Login =========================================
        // Client bound ----------------------------------
        {
            Disconnect,
            reason: Chat,
        }
        {
            LoginSuccess,
            uuid: Uuid,
            name: String,
        }
        // Server bound ----------------------------------

        // Play ==========================================
        // Client bound ----------------------------------
        {
            KeepAlive,
            id: Long,
        }
        {
            ServerDifficultyUpdate,
            difficulty: Difficulty,
            difficulty_locked: bool,
        }
    }
}

new_protocol_impl! {
    (read: V47Readable, v47_read;
    write: V47Writable, v47_write) => {

        // Status ========================================
        // Client bound ----------------------------------
        (0x00, ClientBound, Status) => StatusResponse,
        (0x01, ClientBound, Status) => Pong,
        // Server bound ----------------------------------
        (0x00, ServerBound, Status) => StatusRequest,
        (0x01, ServerBound, Status) => Ping,

        // Login =========================================
        // Client bound ----------------------------------
        (0x01, ClientBound, Login) => EncryptionRequest,
        (0x03, ClientBound, Login) => SetCompression,
        // Server bound ----------------------------------
        (0x00, ServerBound, Login) => LoginStart,
        (0x01, ServerBound, Login) => EncryptionResponse,

        // Play ==========================================
        // Client bound ----------------------------------
    }

    (read: V754Readable, v754_read;
    write: V754Writable, v754_write) => {
        // Handshake =====================================
        // Server bound ----------------------------------
        (0x00, ServerBound, Handshake) => Handshake,

        // Login =========================================
        // Client bound ----------------------------------
        (0x00, ClientBound, Login) => Disconnect,
        (0x02, ClientBound, Login) => LoginSuccess,

        // Play ==========================================
        // Client bound ----------------------------------
        (0x0D, ClientBound, Play) => ServerDifficultyUpdate,
        (0x1F, ClientBound, Play) => KeepAlive,
        (0x24, ClientBound, Play) => JoinGame,
    }
}

// =========== Manual Implementations ============

// Handshake =====================================
// Server bound ----------------------------------

// probably should also add the protocol version field for servers.

impl V754Readable<Event> for Handshake {
    fn v754_read<T: std::io::Read>(buf: &mut T) -> TetsuResult<Event> {
        let _ = VarInt::read_from(buf)?;
        Ok(Event::Handshake(Handshake {
            server_address: String::read_from(buf)?,
            server_port: UnsignedShort::read_from(buf)?,
            next_state: EventState::read_from(buf)?,
        }))
    }
}

impl V754Writable for Handshake {
    fn v754_write<T: std::io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        VarInt(754).write_to(buf)?;
        self.server_address.write_to(buf)?;
        self.server_port.write_to(buf)?;
        self.next_state.write_to(buf)
    }
}

// Play ==========================================
// Client bound ----------------------------------

// ----------------------------------

impl V754Readable<Event> for JoinGame {
    fn v754_read<T: std::io::Read>(buf: &mut T) -> TetsuResult<Event> {
        let id = Int::read_from(buf)?;
        let is_hardcore = Bool::read_from(buf)?;
        let gamemode = UnsignedByte::read_from(buf)?;
        let _ = Byte::read_from(buf)?;
        let worlds: GenericArray<VarInt, String> = GenericArray::read_from(buf)?;

        Ok(Event::JoinGame(Self {
            id,
            is_hardcore,
            gamemode: Gamemode::try_from(gamemode as i32)?,
            worlds: Some(worlds.into()),
            dimension_registry: Some(NbtBlob::read_from(buf)?),
            dimension_codec: Some(NbtBlob::read_from(buf)?),
            world_name: Some(String::read_from(buf)?),
            hashed_seed: Some(Long::read_from(buf)?),
            max_players: UnsignedByte::read_from(buf)? as u32,
            view_distance: Some(VarInt::read_from(buf)?.0),
            reduced_debug: Bool::read_from(buf)?,
            enable_respawn: Some(Bool::read_from(buf)?),
            is_debug: Some(Bool::read_from(buf)?),
            is_flat: Some(Bool::read_from(buf)?),
            ..Default::default()
        }))
    }
}

impl V754Writable for JoinGame {
    fn v754_write<T: std::io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        self.id.write_to(buf)?;
        self.is_hardcore.write_to(buf)?;
        let gamemode: i32 = self.gamemode.clone().try_into()?;
        (gamemode as u8).write_to(buf)?;
        (-1i8).write_to(buf)?; // TODO: move
        let worlds: GenericArray<VarInt, String> = GenericArray::from(self.worlds.clone().unwrap());
        worlds.write_to(buf)?;
        self.dimension_registry.clone().unwrap().write_to(buf)?;
        self.dimension_codec.clone().unwrap().write_to(buf)?;
        self.world_name.clone().unwrap().write_to(buf)?;
        self.max_players.write_to(buf)?;
        self.view_distance.unwrap().write_to(buf)?;
        self.reduced_debug.write_to(buf)?;
        self.enable_respawn.unwrap().write_to(buf)?;
        self.is_debug.unwrap().write_to(buf)?;
        self.is_flat.unwrap().write_to(buf)
    }
}
