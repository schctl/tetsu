//! This module defines serializable types over the network.
//! The type name indicates the type that is sent/to be sent.
//! It's methods return/write the equivalent type.

use crate::errors;

use std::convert::From;
use std::io::{self, prelude::*};
use std::marker::PhantomData;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};

pub use nbt::Blob as NbtBlob;
pub use uuid::Uuid;

pub trait Readable: Sized {
    fn read_from<T: io::Read>(buf: &mut T) -> Result<Self, errors::Error>;
}

pub trait Writable: Sized {
    fn write_to<T: io::Write>(&self, buf: &mut T) -> Result<(), errors::Error>;
}

// -----------------------------------
// All type implementations
// https://wiki.vg/Protocol#Data_types
// -----------------------------------

// ---- Bool ---------------

pub type Bool = bool;

impl Readable for Bool {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> Result<Self, errors::Error> {
        Ok(buf.read_u8()? == 0x01)
    }
}

impl Writable for Bool {
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> Result<(), errors::Error> {
        Ok(buf.write_u8(if *self { 0x01 } else { 0x00 })?)
    }
}

// ---- Byte ---------------

pub type Byte = i8;

impl Readable for Byte {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> Result<Self, errors::Error> {
        Ok(buf.read_i8()?)
    }
}

impl Writable for Byte {
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> Result<(), errors::Error> {
        Ok(buf.write_i8(*self)?)
    }
}

// ---- Unsigned Byte ------

pub type UnsignedByte = u8;

impl Readable for UnsignedByte {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> Result<Self, errors::Error> {
        Ok(buf.read_u8()?)
    }
}

impl Writable for UnsignedByte {
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> Result<(), errors::Error> {
        Ok(buf.write_u8(*self)?)
    }
}

// ---- Short --------------

pub type Short = i16;

impl Readable for Short {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> Result<Self, errors::Error> {
        Ok(buf.read_i16::<BigEndian>()?)
    }
}

impl Writable for Short {
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> Result<(), errors::Error> {
        Ok(buf.write_i16::<BigEndian>(*self)?)
    }
}

// ---- Unsigned Short -----

pub type UnsignedShort = u16;

impl Readable for UnsignedShort {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> Result<Self, errors::Error> {
        Ok(buf.read_u16::<BigEndian>()?)
    }
}

impl Writable for UnsignedShort {
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> Result<(), errors::Error> {
        Ok(buf.write_u16::<BigEndian>(*self)?)
    }
}

// ---- Int ----------------

pub type Int = i32;

impl Readable for Int {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> Result<Self, errors::Error> {
        Ok(buf.read_i32::<BigEndian>()?)
    }
}

impl Writable for Int {
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> Result<(), errors::Error> {
        Ok(buf.write_i32::<BigEndian>(*self)?)
    }
}

// ---- Unsigned Int -------

// This type isn't actually used
// but is implemented anyway.

pub type UnsignedInt = u32;

impl Readable for UnsignedInt {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> Result<Self, errors::Error> {
        Ok(buf.read_u32::<BigEndian>()?)
    }
}

impl Writable for UnsignedInt {
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> Result<(), errors::Error> {
        Ok(buf.write_u32::<BigEndian>(*self)?)
    }
}

// ---- Long ---------------

// This type isn't actually used
// but is implemented anyway.

pub type Long = i64;

impl Readable for Long {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> Result<Self, errors::Error> {
        Ok(buf.read_i64::<BigEndian>()?)
    }
}

impl Writable for Long {
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> Result<(), errors::Error> {
        Ok(buf.write_i64::<BigEndian>(*self)?)
    }
}

// ---- Unsigned Long ------

// This type isn't actually used
// but is implemented anyway.

pub type UnsignedLong = u64;

impl Readable for UnsignedLong {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> Result<Self, errors::Error> {
        Ok(buf.read_u64::<BigEndian>()?)
    }
}

impl Writable for UnsignedLong {
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> Result<(), errors::Error> {
        Ok(buf.write_u64::<BigEndian>(*self)?)
    }
}

// ---- Float --------------

pub type Float = f32;

impl Readable for Float {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> Result<Self, errors::Error> {
        Ok(buf.read_f32::<BigEndian>()?)
    }
}

impl Writable for Float {
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> Result<(), errors::Error> {
        Ok(buf.write_f32::<BigEndian>(*self)?)
    }
}

// ---- Double -------------

pub type Double = f64;

impl Readable for Double {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> Result<Self, errors::Error> {
        Ok(buf.read_f64::<BigEndian>()?)
    }
}

impl Writable for Double {
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> Result<(), errors::Error> {
        Ok(buf.write_f64::<BigEndian>(*self)?)
    }
}

// ---- String -------------

impl Readable for String {
    fn read_from<T: io::Read>(buf: &mut T) -> Result<Self, errors::Error> {
        let len = VarInt::read_from(buf)?.0;
        let mut bytes = Vec::<u8>::new();
        buf.take(len as u64).read_to_end(&mut bytes)?;
        Ok(Self::from_utf8(bytes)?)
    }
}

impl Writable for String {
    fn write_to<T: io::Write>(&self, buf: &mut T) -> Result<(), errors::Error> {
        let bytes = self.as_bytes();
        let x = VarInt(bytes.len() as i32);
        x.write_to(buf)?;
        Ok(buf.write_all(bytes)?)
    }
}

// ---- Chat ---------------

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Action {
    action: String,
    value: String,
}

/// Information that defines contents/style of a chat message.
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    pub text: Option<String>,
    pub translate: Option<String>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underlined: Option<bool>,
    pub strikethrough: Option<bool>,
    pub obfuscated: Option<bool>,
    pub color: Option<String>,
    pub click_event: Option<Action>,
    pub hover_event: Option<Action>,
    pub extra: Option<Vec<Self>>,
}

impl Readable for Chat {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> Result<Self, errors::Error> {
        let val = String::read_from(buf)?;
        Ok(serde_json::from_str(&val[..])?)
    }
}

impl Writable for Chat {
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> Result<(), errors::Error> {
        let val = serde_json::to_string(&self).unwrap();
        val.write_to(buf)
    }
}

// ---- Identifier ---------

// Same as String

// ---- VarInt -------------

#[derive(Debug, PartialEq, Eq)]
pub struct VarInt(pub i32);

impl Readable for VarInt {
    fn read_from<T: io::Read>(buf: &mut T) -> Result<Self, errors::Error> {
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
    fn write_to<T: io::Write>(&self, buf: &mut T) -> Result<(), errors::Error> {
        let mut val = self.0 as u32;

        loop {
            let byte = val & 0b01111111;

            val >>= 7;

            if val == 0 {
                buf.write_u8(byte as u8)?;
                return Ok(());
            }

            buf.write_u8((byte | 0b10000000) as u8)?;
        }
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

// ---- UUID ---------------

impl Readable for Uuid {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> Result<Self, errors::Error> {
        Ok(Self::from_u128(buf.read_u128::<BigEndian>()?))
    }
}

impl Writable for Uuid {
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> Result<(), errors::Error> {
        Ok(buf.write_u128::<BigEndian>(self.as_u128())?)
    }
}

// ---- Byte Arrays --------

#[derive(Debug, PartialEq, Eq)]
pub struct ByteArrayVarInt(pub usize, pub Vec<u8>);

impl Readable for ByteArrayVarInt {
    fn read_from<T: io::Read>(buf: &mut T) -> Result<Self, errors::Error> {
        let len = VarInt::read_from(buf)?.0 as usize;
        let mut data = Vec::with_capacity(len);
        buf.take(len as u64).read_to_end(&mut data)?;
        Ok(Self(len, data))
    }
}

impl Writable for ByteArrayVarInt {
    fn write_to<T: io::Write>(&self, buf: &mut T) -> Result<(), errors::Error> {
        let len = VarInt(self.1.len() as i32);
        len.write_to(buf)?;
        Ok(buf.write_all(&self.1[..])?)
    }
}

// ---- Arrays -------------

#[derive(Debug, PartialEq, Eq)]
pub struct GenericArray<L: Into<usize> + From<usize> + Readable + Writable, C: Readable + Writable>(
    pub usize,
    pub Vec<C>,
    PhantomData<L>,
);

impl<L: Into<usize> + From<usize> + Readable + Writable, C: Readable + Writable> Readable
    for GenericArray<L, C>
{
    fn read_from<T: io::Read>(buf: &mut T) -> Result<Self, errors::Error> {
        let len = L::read_from(buf)?.into();
        let mut data = Vec::with_capacity(len);
        for _ in 0..len {
            data.push(C::read_from(buf)?);
        }
        Ok(Self(len, data, PhantomData))
    }
}

impl<L: Into<usize> + From<usize> + Readable + Writable, C: Readable + Writable> Writable
    for GenericArray<L, C>
{
    fn write_to<T: io::Write>(&self, buf: &mut T) -> Result<(), errors::Error> {
        let len: L = self.1.len().into();
        len.write_to(buf)?;
        for i in &self.1 {
            i.write_to(buf)?;
        }
        Ok(())
    }
}

impl<L: Into<usize> + From<usize> + Readable + Writable, C: Readable + Writable> From<Vec<C>>
    for GenericArray<L, C>
{
    fn from(item: Vec<C>) -> Self {
        Self(item.len(), item, PhantomData)
    }
}

// ---- Vec ----------------

impl Readable for Vec<UnsignedByte> {
    #[inline]
    fn read_from<R: io::Read>(buf: &mut R) -> Result<Self, errors::Error> {
        let mut v = Vec::new();
        buf.read_to_end(&mut v)?;
        Ok(v)
    }
}

impl Writable for Vec<UnsignedByte> {
    #[inline]
    fn write_to<W: io::Write>(&self, buf: &mut W) -> Result<(), errors::Error> {
        Ok(buf.write_all(&self[..])?)
    }
}

// ---- Named Binary Tags --

impl Readable for NbtBlob {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> Result<Self, errors::Error> {
        Ok(Self::from_reader(buf)?)
    }
}

impl Writable for NbtBlob {
    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) -> Result<(), errors::Error> {
        Ok(self.to_writer(buf)?)
    }
}
