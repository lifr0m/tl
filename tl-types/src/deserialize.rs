use std::io::{self, Cursor, Read};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use thiserror::Error;

// todo: tests

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("io error: {0}")]
    Io(#[from] io::Error),

    #[error("string decode error: {0}")]
    StringDecode(#[from] std::string::FromUtf8Error),
}

pub(crate) trait Deserialize
where
    Self: Sized,
{
    fn deserialize(cur: &mut Cursor<Vec<u8>>) -> Result<Self, Error>;
}

impl Deserialize for u16 {
    fn deserialize(cur: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let mut buf = [0; 2];
        cur.read_exact(&mut buf)?;
        Ok(Self::from_le_bytes(buf))
    }
}

impl Deserialize for i32 {
    fn deserialize(cur: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let mut buf = [0; 4];
        cur.read_exact(&mut buf)?;
        Ok(Self::from_le_bytes(buf))
    }
}

impl Deserialize for i64 {
    fn deserialize(cur: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let mut buf = [0; 8];
        cur.read_exact(&mut buf)?;
        Ok(Self::from_le_bytes(buf))
    }
}

impl Deserialize for f64 {
    fn deserialize(cur: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let mut buf = [0; 8];
        cur.read_exact(&mut buf)?;
        Ok(Self::from_le_bytes(buf))
    }
}

impl Deserialize for bool {
    fn deserialize(cur: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let mut buf = [0; 1];
        cur.read_exact(&mut buf)?;
        Ok(buf[0] != 0)
    }
}

impl Deserialize for String {
    fn deserialize(cur: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let buf = Vec::<u8>::deserialize(cur)?;
        Ok(Self::from_utf8(buf)?)
    }
}

impl Deserialize for Vec<u8> {
    fn deserialize(cur: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let len = deserialize_dyn_len(cur)?;
        let mut buf = vec![0; len];
        cur.read_exact(&mut buf)?;
        Ok(buf)
    }
}

impl Deserialize for SystemTime {
    fn deserialize(cur: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let millis = i64::deserialize(cur)?;
        Ok(UNIX_EPOCH + Duration::from_millis(millis as u64))
    }
}

impl<T: Deserialize> Deserialize for Vec<T> {
    fn deserialize(cur: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let len = deserialize_dyn_len(cur)?;
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(T::deserialize(cur)?);
        }
        Ok(vec)
    }
}

impl<T: Deserialize> Deserialize for Option<T> {
    fn deserialize(cur: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        if bool::deserialize(cur)? {
            Ok(Some(T::deserialize(cur)?))
        } else {
            Ok(None)
        }
    }
}

fn deserialize_dyn_len(cur: &mut Cursor<Vec<u8>>) -> Result<usize, Error> {
    let mut buf = [0; 1];
    cur.read_exact(&mut buf)?;
    if buf[0] < 255 {
        Ok(buf[0] as usize)
    } else {
        Ok(i64::deserialize(cur)? as usize)
    }
}
