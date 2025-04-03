use std::io::{self, Cursor, Read};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] io::Error),

    #[error("invalid string: {0}")]
    InvalidString(#[from] std::string::FromUtf8Error),

    #[error("unexpected definition id: {0}")]
    UnexpectedDefinitionId(u32),
}

pub trait Deserialize
where
    Self: Sized,
{
    fn deserialize(cur: &mut Cursor<Vec<u8>>) -> Result<Self, Error>;
}

impl Deserialize for i32 {
    fn deserialize(cur: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let mut buf = [0; 4];
        cur.read_exact(&mut buf)?;
        Ok(Self::from_le_bytes(buf))
    }
}

impl Deserialize for u32 {
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
        (0..len).map(|_| T::deserialize(cur)).collect()
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn primitives() {
        assert_eq!(de::<i32>(vec![0x4e, 0x19, 0x8f, 0x1c]), 479140174_i32);
        assert_eq!(de::<i64>(vec![0x4d, 0xbe, 0x90, 0x9, 0xa2, 0xc6, 0x35, 0x1]), 87194167051075149_i64);
        assert_eq!(de::<f64>(vec![0xbc, 0x90, 0x0e, 0x0f, 0x61, 0x3a, 0x81, 0x40]), 551.297392_f64);
        assert_eq!(de::<bool>(vec![0x1]), true);
        assert_eq!(de::<bool>(vec![0x0]), false);
        assert_eq!(de::<String>(vec![0x5, b'h', b'e', b'l', b'l', b'o']), "hello".to_string());
        assert_eq!(
            de::<Vec<u8>>([vec![0xFF, 0xe8, 0x3, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0], vec![0xdd; 997], vec![b'j', b'o', b'y']].concat()),
            [vec![0xdd; 997], vec![b'j', b'o', b'y']].concat()
        );
        assert_eq!(de::<Option<i32>>(vec![0x1, 0x28, 0x0, 0x0, 0x0]), Some(0x28));
        assert_eq!(de::<Option<i32>>(vec![0x0]), None::<i32>);
    }

    #[test]
    fn dyn_len() {
        assert_eq!(de_dyn_len(vec![0x50]), 0x50);
        assert_eq!(de_dyn_len(vec![0xFF, 0x97, 0x43, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0]), 0x24397);
    }

    fn de<T: Deserialize>(buf: Vec<u8>) -> T {
        let mut cur = Cursor::new(buf);
        T::deserialize(&mut cur).unwrap()
    }

    fn de_dyn_len(buf: Vec<u8>) -> usize {
        let mut cur = Cursor::new(buf);
        deserialize_dyn_len(&mut cur).unwrap()
    }
}
