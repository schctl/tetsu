//! Common implementations for all protocol versions.

use crate::errors::*;
use crate::event::*;
use crate::serialization::*;

use std::marker::PhantomData;
use std::{
    convert::TryFrom,
    io::{self, prelude::*},
};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

pub use nbt::Blob as NbtBlob;
pub use uuid::Uuid;

// -----------------------------------
// https://wiki.vg/Protocol#Data_types
// -----------------------------------

// ---- Bool ---------------

pub type Bool = bool;

impl Readable for Bool {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> TetsuResult<Self> {
        Ok(buf.read_u8()? == 0x01)
    }
}

impl Writable for Bool {
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        Ok(buf.write_u8(if *self { 0x01 } else { 0x00 })?)
    }
}

// ---- Byte ---------------

pub type Byte = i8;

impl Readable for Byte {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> TetsuResult<Self> {
        Ok(buf.read_i8()?)
    }
}

impl Writable for Byte {
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        Ok(buf.write_i8(*self)?)
    }
}

// ---- Unsigned Byte ------

pub type UnsignedByte = u8;

impl Readable for UnsignedByte {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> TetsuResult<Self> {
        Ok(buf.read_u8()?)
    }
}

impl Writable for UnsignedByte {
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        Ok(buf.write_u8(*self)?)
    }
}

// ---- Short --------------

pub type Short = i16;

impl Readable for Short {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> TetsuResult<Self> {
        Ok(buf.read_i16::<BigEndian>()?)
    }
}

impl Writable for Short {
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        Ok(buf.write_i16::<BigEndian>(*self)?)
    }
}

// ---- Unsigned Short -----

pub type UnsignedShort = u16;

impl Readable for UnsignedShort {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> TetsuResult<Self> {
        Ok(buf.read_u16::<BigEndian>()?)
    }
}

impl Writable for UnsignedShort {
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        Ok(buf.write_u16::<BigEndian>(*self)?)
    }
}

// ---- Int ----------------

pub type Int = i32;

impl Readable for Int {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> TetsuResult<Self> {
        Ok(buf.read_i32::<BigEndian>()?)
    }
}

impl Writable for Int {
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        Ok(buf.write_i32::<BigEndian>(*self)?)
    }
}

// ---- Unsigned Int -------

pub type UnsignedInt = u32;

impl Readable for UnsignedInt {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> TetsuResult<Self> {
        Ok(buf.read_u32::<BigEndian>()?)
    }
}

impl Writable for UnsignedInt {
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        Ok(buf.write_u32::<BigEndian>(*self)?)
    }
}

// ---- Long ---------------

// This type isn't actually used
// but is implemented anyway.

pub type Long = i64;

impl Readable for Long {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> TetsuResult<Self> {
        Ok(buf.read_i64::<BigEndian>()?)
    }
}

impl Writable for Long {
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        Ok(buf.write_i64::<BigEndian>(*self)?)
    }
}

// ---- Unsigned Long ------

// This type isn't actually used
// but is implemented anyway.

pub type UnsignedLong = u64;

impl Readable for UnsignedLong {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> TetsuResult<Self> {
        Ok(buf.read_u64::<BigEndian>()?)
    }
}

impl Writable for UnsignedLong {
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        Ok(buf.write_u64::<BigEndian>(*self)?)
    }
}

// ---- Float --------------

pub type Float = f32;

impl Readable for Float {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> TetsuResult<Self> {
        Ok(buf.read_f32::<BigEndian>()?)
    }
}

impl Writable for Float {
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        Ok(buf.write_f32::<BigEndian>(*self)?)
    }
}

// ---- Double -------------

pub type Double = f64;

impl Readable for Double {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> TetsuResult<Self> {
        Ok(buf.read_f64::<BigEndian>()?)
    }
}

impl Writable for Double {
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        Ok(buf.write_f64::<BigEndian>(*self)?)
    }
}

// ---- String -------------

impl Readable for String {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> TetsuResult<Self> {
        let len = VarInt::read_from(buf)?.0;
        let mut bytes = vec![0; len as usize];
        buf.read_exact(&mut bytes)?;
        Ok(Self::from_utf8(bytes)?)
    }
}

impl Writable for String {
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        let bytes = self.as_bytes();
        VarInt(bytes.len() as i32).write_to(buf)?;
        Ok(buf.write_all(bytes)?)
    }
}

// ---- Chat ---------------

impl Readable for Chat {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> TetsuResult<Self> {
        Ok(serde_json::from_str(&String::read_from(buf)?[..])?)
    }
}

impl Writable for Chat {
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        serde_json::to_string(&self)?.write_to(buf)
    }
}

// ---- Identifier ---------

// Same as String

// ---- VarInt -------------

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VarInt(pub i32);

impl Readable for VarInt {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> TetsuResult<Self> {
        let mut res: u32 = 0;
        let mut byte;

        for byte_index in 0..6 {
            byte = buf.read_u8()? as u32;

            res |= (byte & 0x7F) << (byte_index * 7);

            if (byte & 0x80) == 0 {
                break;
            }
        }

        Ok(Self(res as i32))
    }
}

impl Writable for VarInt {
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        let mut val = self.0 as u32;

        for _ in 0..6 {
            let byte = val & 0x7F;

            val >>= 7;

            if val == 0 {
                buf.write_u8(byte as u8)?;
                return Ok(());
            }

            buf.write_u8((byte | 0x80) as u8)?;
        }

        Ok(())
    }
}

impl From<i32> for VarInt {
    #[inline]
    fn from(item: i32) -> Self {
        Self(item)
    }
}

impl From<VarInt> for i32 {
    #[inline]
    fn from(item: VarInt) -> Self {
        item.0
    }
}

impl From<usize> for VarInt {
    #[inline]
    fn from(item: usize) -> Self {
        Self(item as i32)
    }
}

impl From<VarInt> for usize {
    #[inline]
    fn from(item: VarInt) -> Self {
        item.0 as usize
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VarLong(pub i64);

impl Readable for VarLong {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> TetsuResult<Self> {
        let mut res: u32 = 0;
        let mut byte;

        for byte_index in 0..11 {
            byte = buf.read_u8()? as u32;

            res |= (byte & 0x7F) << (byte_index * 7);

            if (byte & 0x80) == 0 {
                break;
            }
        }

        Ok(Self(res as i64))
    }
}

impl Writable for VarLong {
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        let mut val = self.0 as u32;

        for _ in 0..11 {
            let byte = val & 0x7F;

            val >>= 7;

            if val == 0 {
                buf.write_u8(byte as u8)?;
                return Ok(());
            }

            buf.write_u8((byte | 0x80) as u8)?;
        }

        Ok(())
    }
}

// ---- UUID ---------------

impl Readable for Uuid {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> TetsuResult<Self> {
        Ok(Self::from_u128(buf.read_u128::<BigEndian>()?))
    }
}

impl Writable for Uuid {
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        Ok(buf.write_u128::<BigEndian>(self.as_u128())?)
    }
}

// ---- Byte Arrays --------

#[derive(Debug, PartialEq)]
pub struct ByteArrayVarInt(pub usize, pub Vec<u8>);

impl Readable for ByteArrayVarInt {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> TetsuResult<Self> {
        let len = VarInt::read_from(buf)?.0 as usize;
        let mut data = Vec::with_capacity(len);
        buf.take(len as u64).read_to_end(&mut data)?;
        Ok(Self(len, data))
    }
}

impl Writable for ByteArrayVarInt {
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        VarInt(self.1.len() as i32).write_to(buf)?;
        Ok(buf.write_all(&self.1[..])?)
    }
}

impl From<Vec<u8>> for ByteArrayVarInt {
    fn from(item: Vec<u8>) -> Self {
        Self(item.len(), item)
    }
}

impl From<ByteArrayVarInt> for Vec<u8> {
    fn from(item: ByteArrayVarInt) -> Self {
        item.1
    }
}

// ---- Arrays -------------

#[derive(Debug, PartialEq)]
pub struct GenericArray<L: Into<usize> + From<usize> + Readable + Writable, C: Readable + Writable>(
    pub usize,
    pub Vec<C>,
    PhantomData<L>,
);

impl<L, C> Readable for GenericArray<L, C>
where
    L: Into<usize> + From<usize> + Readable + Writable,
    C: Readable + Writable,
{
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> TetsuResult<Self> {
        let len = L::read_from(buf)?.into();
        let mut data = Vec::with_capacity(len);
        for _ in 0..len {
            data.push(C::read_from(buf)?);
        }
        Ok(Self(len, data, PhantomData))
    }
}

impl<L, C> Writable for GenericArray<L, C>
where
    L: Into<usize> + From<usize> + Readable + Writable,
    C: Readable + Writable,
{
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        let len: L = self.1.len().into();
        len.write_to(buf)?;
        for i in &self.1 {
            i.write_to(buf)?;
        }
        Ok(())
    }
}

impl<L, C> From<Vec<C>> for GenericArray<L, C>
where
    L: Into<usize> + From<usize> + Readable + Writable,
    C: Readable + Writable,
{
    #[inline]
    fn from(item: Vec<C>) -> Self {
        Self(item.len(), item, PhantomData)
    }
}

impl<L, C> From<GenericArray<L, C>> for Vec<C>
where
    L: Into<usize> + From<usize> + Readable + Writable,
    C: Readable + Writable,
{
    #[inline]
    fn from(item: GenericArray<L, C>) -> Self {
        item.1
    }
}

// ---- Vec ----------------

impl Readable for Vec<UnsignedByte> {
    #[inline]
    fn read_from<R: io::Read>(buf: &mut R) -> TetsuResult<Self> {
        let mut v = Vec::new();
        buf.read_to_end(&mut v)?;
        Ok(v)
    }
}

impl Writable for Vec<UnsignedByte> {
    #[inline]
    fn write_to<W: io::Write>(&self, buf: &mut W) -> TetsuResult<()> {
        Ok(buf.write_all(&self[..])?)
    }
}

// ---- Named Binary Tags --

impl Readable for NbtBlob {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> TetsuResult<Self> {
        Ok(Self::from_reader(buf)?)
    }
}

impl Writable for NbtBlob {
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        Ok(self.to_writer(buf)?)
    }
}

// ---- Option --------------

#[derive(Debug, PartialEq, Clone)]
pub struct GenericOption<C: Readable + Writable>(pub Option<C>);

impl<C: Readable + Writable> Readable for GenericOption<C> {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> TetsuResult<Self> {
        let exists = bool::read_from(buf)?;
        let internal;
        if exists {
            internal = Some(C::read_from(buf)?)
        } else {
            internal = None
        }

        Ok(Self(internal))
    }
}

impl<C: Readable + Writable> Writable for GenericOption<C> {
    #[inline]
    fn write_to<W: io::Write>(&self, buf: &mut W) -> TetsuResult<()> {
        match &self.0 {
            Some(s) => {
                true.write_to(buf)?;
                s.write_to(buf)
            }
            _ => false.write_to(buf),
        }
    }
}

// ----- Other types -----

impl Readable for Gamemode {
    fn read_from<T: std::io::Read>(buf: &mut T) -> TetsuResult<Self> {
        Ok(match VarInt::read_from(buf)?.0 {
            0 => Gamemode::Survival,
            1 => Gamemode::Creative,
            2 => Gamemode::Adventure,
            3 => Gamemode::Spectator,
            _ => {
                return Err(Error::from(InvalidValue {
                    expected: "0-4".to_owned(),
                }))
            }
        })
    }
}

impl Writable for Gamemode {
    fn write_to<T: std::io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        match self {
            Gamemode::Survival => VarInt(0).write_to(buf),
            Gamemode::Creative => VarInt(1).write_to(buf),
            Gamemode::Adventure => VarInt(2).write_to(buf),
            Gamemode::Spectator => VarInt(3).write_to(buf),
        }
    }
}

impl TryFrom<i32> for Gamemode {
    type Error = Error;

    fn try_from(item: i32) -> TetsuResult<Gamemode> {
        Ok(match item {
            0 => Gamemode::Survival,
            1 => Gamemode::Creative,
            2 => Gamemode::Adventure,
            3 => Gamemode::Spectator,
            _ => {
                return Err(Error::from(InvalidValue {
                    expected: "0-4".to_owned(),
                }))
            }
        })
    }
}

impl TryFrom<Gamemode> for i32 {
    type Error = Error;

    fn try_from(item: Gamemode) -> TetsuResult<i32> {
        Ok(match item {
            Gamemode::Survival => 0,
            Gamemode::Creative => 1,
            Gamemode::Adventure => 2,
            Gamemode::Spectator => 3,
        })
    }
}

impl Readable for EventState {
    fn read_from<T: std::io::Read>(buf: &mut T) -> TetsuResult<Self> {
        Ok(match VarInt::read_from(buf)?.0 {
            1 => EventState::Status,
            2 => EventState::Login,
            _ => {
                return Err(Error::from(InvalidValue {
                    expected: "1 or 2".to_owned(),
                }))
            }
        })
    }
}

impl Writable for EventState {
    fn write_to<T: std::io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        VarInt(match self {
            EventState::Status => 1,
            EventState::Login => 2,
            _ => {
                return Err(Error::from(InvalidValue {
                    expected: "1 or 2".to_owned(),
                }))
            }
        })
        .write_to(buf)
    }
}

impl Readable for PlayerProperty {
    fn read_from<T: std::io::Read>(buf: &mut T) -> TetsuResult<Self> {
        Ok(Self {
            name: String::read_from(buf)?,
            value: String::read_from(buf)?,
            signature: GenericOption::read_from(buf)?.0,
        })
    }
}

impl Writable for PlayerProperty {
    fn write_to<T: std::io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        self.name.write_to(buf)?;
        self.value.write_to(buf)?;
        GenericOption(self.signature.clone()).write_to(buf)
    }
}

impl Readable for PlayerInfoAdd {
    fn read_from<T: std::io::Read>(buf: &mut T) -> TetsuResult<Self> {
        let name = String::read_from(buf)?;
        let properties: GenericArray<VarInt, PlayerProperty> = GenericArray::read_from(buf)?;
        Ok(Self {
            name,
            properties: properties.into(),
            gamemode: Gamemode::read_from(buf)?,
            ping: VarInt::read_from(buf)?.0,
            display: GenericOption::read_from(buf)?.0,
        })
    }
}

impl Writable for PlayerInfoAdd {
    fn write_to<T: std::io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        self.name.write_to(buf)?;
        let properites: GenericArray<VarInt, PlayerProperty> =
            GenericArray::from(self.properties.clone());
        properites.write_to(buf)?;
        self.gamemode.write_to(buf)?;
        VarInt(self.ping).write_to(buf)?;
        GenericOption(self.display.clone()).write_to(buf)
    }
}

impl Readable for PlayerGamemodeUpdate {
    fn read_from<T: std::io::Read>(buf: &mut T) -> TetsuResult<Self> {
        Ok(Self {
            gamemode: Gamemode::read_from(buf)?,
        })
    }
}

impl Writable for PlayerGamemodeUpdate {
    fn write_to<T: std::io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        self.gamemode.write_to(buf)
    }
}

impl Readable for PlayerLatencyUpdate {
    fn read_from<T: std::io::Read>(buf: &mut T) -> TetsuResult<Self> {
        Ok(Self {
            ping: VarInt::read_from(buf)?.0,
        })
    }
}

impl Writable for PlayerLatencyUpdate {
    fn write_to<T: std::io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        VarInt(self.ping).write_to(buf)
    }
}

impl Readable for PlayerDisplayNameUpdate {
    fn read_from<T: std::io::Read>(buf: &mut T) -> TetsuResult<Self> {
        Ok(Self {
            display: GenericOption::read_from(buf)?.0,
        })
    }
}

impl Writable for PlayerDisplayNameUpdate {
    fn write_to<T: std::io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
        GenericOption(self.display.clone()).write_to(buf)
    }
}

impl Readable for Difficulty {
    fn read_from<T: std::io::Read>(_buf: &mut T) -> TetsuResult<Difficulty> {
        Ok(match UnsignedByte::read_from(_buf)? {
            0 => Difficulty::Peaceful,
            1 => Difficulty::Easy,
            2 => Difficulty::Normal,
            3 => Difficulty::Hard,
            _ => {
                return Err(Error::from(InvalidValue {
                    expected: "0, 1, 2, 3".to_owned(),
                }))
            }
        })
    }
}

impl Writable for Difficulty {
    fn write_to<T: std::io::Write>(&self, _buf: &mut T) -> TetsuResult<()> {
        (match self {
            Difficulty::Peaceful => 0,
            Difficulty::Easy => 1,
            Difficulty::Normal => 2,
            Difficulty::Hard => 3,
        } as UnsignedByte)
            .write_to(_buf)
    }
}

impl Default for JoinGame {
    fn default() -> Self {
        Self {
            id: 0,
            is_hardcore: false,
            gamemode: Gamemode::Survival,
            worlds: None,
            dimension: None,
            dimension_registry: None,
            dimension_codec: None,
            world_name: None,
            difficulty: None,
            hashed_seed: None,
            max_players: 20,
            level_type: None,
            view_distance: None,
            reduced_debug: false,
            enable_respawn: None,
            is_debug: None,
            is_flat: None,
        }
    }
}
