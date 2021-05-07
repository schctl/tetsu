//! This module defines serializable types over the network.
//! The type name indicates the type that is sent/to be sent.
//! It's methods return/write the equivalent type.

use std::convert::From;
use std::io::{self, prelude::*};
use std::marker::PhantomData;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};

pub use nbt::Blob as NbtBlob;
pub use uuid::Uuid;

pub trait Serializable {
    fn read_from<T: io::Read>(buf: &mut T) -> Self;
    fn write_to<T: io::Write>(&self, buf: &mut T);
}

// -----------------------------------
// All type implementations
// https://wiki.vg/Protocol#Data_types
// -----------------------------------

// ---- Bool ---------------

pub type Bool = bool;

impl Serializable for Bool {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> Self {
        buf.read_u8().unwrap() == 0x01
    }

    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) {
        buf.write_u8(if *self { 0x01 } else { 0x00 }).unwrap();
    }
}

// ---- Byte ---------------

pub type Byte = i8;

impl Serializable for Byte {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> Self {
        buf.read_i8().unwrap()
    }

    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) {
        buf.write_i8(*self).unwrap();
    }
}

// ---- Unsigned Byte ------

pub type UnsignedByte = u8;

impl Serializable for UnsignedByte {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> Self {
        buf.read_u8().unwrap()
    }

    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) {
        buf.write_u8(*self).unwrap();
    }
}

// ---- Short --------------

pub type Short = i16;

impl Serializable for Short {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> Self {
        buf.read_i16::<BigEndian>().unwrap()
    }

    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) {
        buf.write_i16::<BigEndian>(*self).unwrap();
    }
}

// ---- Unsigned Short -----

pub type UnsignedShort = u16;

impl Serializable for UnsignedShort {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> Self {
        buf.read_u16::<BigEndian>().unwrap()
    }

    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) {
        buf.write_u16::<BigEndian>(*self).unwrap();
    }
}

// ---- Int ----------------

pub type Int = i32;

impl Serializable for Int {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> Self {
        buf.read_i32::<BigEndian>().unwrap()
    }

    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) {
        buf.write_i32::<BigEndian>(*self).unwrap();
    }
}

// ---- Unsigned Int -------

// This type isn't actually used
// but is implemented anyway.

pub type UnsignedInt = u32;

impl Serializable for UnsignedInt {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> Self {
        buf.read_u32::<BigEndian>().unwrap()
    }

    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) {
        buf.write_u32::<BigEndian>(*self).unwrap();
    }
}

// ---- Long ---------------

// This type isn't actually used
// but is implemented anyway.

pub type Long = i64;

impl Serializable for Long {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> Self {
        buf.read_i64::<BigEndian>().unwrap()
    }

    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) {
        buf.write_i64::<BigEndian>(*self).unwrap();
    }
}

// ---- Unsigned Long ------

// This type isn't actually used
// but is implemented anyway.

pub type UnsignedLong = u64;

impl Serializable for UnsignedLong {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> Self {
        buf.read_u64::<BigEndian>().unwrap()
    }

    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) {
        buf.write_u64::<BigEndian>(*self).unwrap();
    }
}

// ---- Float --------------

pub type Float = f32;

impl Serializable for Float {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> Self {
        buf.read_f32::<BigEndian>().unwrap()
    }

    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) {
        buf.write_f32::<BigEndian>(*self).unwrap();
    }
}

// ---- Double -------------

pub type Double = f64;

impl Serializable for Double {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> Self {
        buf.read_f64::<BigEndian>().unwrap()
    }

    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) {
        buf.write_f64::<BigEndian>(*self).unwrap();
    }
}

// ---- String -------------

impl Serializable for String {
    fn read_from<T: io::Read>(buf: &mut T) -> Self {
        let len = VarInt::read_from(buf).0;
        let mut bytes = Vec::<u8>::new();
        buf.take(len as u64).read_to_end(&mut bytes).unwrap();
        Self::from_utf8(bytes).unwrap()
    }

    fn write_to<T: io::Write>(&self, buf: &mut T) {
        let bytes = self.as_bytes();
        let x = VarInt(bytes.len() as i32);
        x.write_to(buf);
        buf.write_all(bytes).unwrap();
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

impl Serializable for Chat {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> Self {
        let val = String::read_from(buf);
        serde_json::from_str(&val[..]).unwrap()
    }

    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) {
        let val = serde_json::to_string(&self).unwrap();
        val.write_to(buf)
    }
}

// ---- Identifier ---------

// Same as String

// ---- VarInt -------------

#[derive(Debug, PartialEq, Eq)]
pub struct VarInt(pub i32);

impl Serializable for VarInt {
    fn read_from<T: io::Read>(buf: &mut T) -> Self {
        let mut res: u32 = 0;
        let mut byte;

        for byte_index in 0..6 {
            byte = buf.read_u8().unwrap() as u32;

            res |= (byte & 0x7F) << (byte_index * 7);

            if (byte & 0x80) == 0 {
                break;
            }
        }

        Self(res as i32)
    }

    fn write_to<T: io::Write>(&self, buf: &mut T) {
        let mut val = self.0 as u32;

        loop {
            let byte = val & 0b01111111;

            val >>= 7;

            if val == 0 {
                buf.write_u8(byte as u8).unwrap();
                return;
            }

            buf.write_u8((byte | 0b10000000) as u8).unwrap();
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

impl Serializable for Uuid {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> Self {
        Self::from_u128(buf.read_u128::<BigEndian>().unwrap())
    }

    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) {
        buf.write_u128::<BigEndian>(self.as_u128()).unwrap();
    }
}

// ---- Byte Arrays --------

#[derive(Debug, PartialEq, Eq)]
pub struct ByteArrayVarInt(pub usize, pub Vec<u8>);

impl Serializable for ByteArrayVarInt {
    fn read_from<T: io::Read>(buf: &mut T) -> Self {
        let len = VarInt::read_from(buf).0 as usize;
        let mut data = Vec::with_capacity(len);
        buf.take(len as u64).read_to_end(&mut data).unwrap();
        Self(len, data)
    }

    fn write_to<T: io::Write>(&self, buf: &mut T) {
        let len = VarInt(self.1.len() as i32);
        len.write_to(buf);
        buf.write_all(&self.1[..]).unwrap();
    }
}

// ---- Arrays -------------

#[derive(Debug, PartialEq, Eq)]
pub struct GenericArray<L: Into<usize> + From<usize> + Serializable, C: Serializable>(
    pub usize,
    pub Vec<C>,
    PhantomData<L>,
);

impl<L: Into<usize> + From<usize> + Serializable, C: Serializable> Serializable
    for GenericArray<L, C>
{
    fn read_from<T: io::Read>(buf: &mut T) -> Self {
        let len = L::read_from(buf).into();
        let mut data = Vec::with_capacity(len);
        for _ in 0..len {
            data.push(C::read_from(buf));
        }
        Self(len, data, PhantomData)
    }

    fn write_to<T: io::Write>(&self, buf: &mut T) {
        let len: L = self.1.len().into();
        len.write_to(buf);
        for i in &self.1 {
            i.write_to(buf);
        }
    }
}

impl<L: Into<usize> + From<usize> + Serializable, C: Serializable> From<Vec<C>>
    for GenericArray<L, C>
{
    fn from(item: Vec<C>) -> Self {
        Self(item.len(), item, PhantomData)
    }
}

// ---- Vec ----------------

impl Serializable for Vec<UnsignedByte> {
    #[inline]
    fn read_from<R: io::Read>(buf: &mut R) -> Self {
        let mut v = Vec::new();
        buf.read_to_end(&mut v).unwrap();
        v
    }

    #[inline]
    fn write_to<W: io::Write>(&self, buf: &mut W) {
        buf.write_all(&self[..]).unwrap()
    }
}

// ---- Named Binary Tags --

impl Serializable for NbtBlob {
    #[inline]
    fn read_from<T: io::Read>(buf: &mut T) -> Self {
        Self::from_reader(buf).unwrap()
    }

    #[inline]
    fn write_to<T: io::Write>(&self, buf: &mut T) {
        self.to_writer(buf).unwrap()
    }
}
