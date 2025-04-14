use crate::Read;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("read error: {0}")]
    Read(#[from] crate::read::Error),

    #[error("invalid string: {0}")]
    InvalidString(#[from] std::string::FromUtf8Error),

    #[error("unexpected definition id: {0}")]
    UnexpectedDefinitionId(u32),
}

pub trait Deserialize
where
    Self: Sized,
{
    fn deserialize(src: &mut &[u8]) -> Result<Self, Error>;

    fn from_bytes(buf: &[u8]) -> Result<Self, Error> {
        let mut src = buf;
        Self::deserialize(&mut src)
    }
}

impl Deserialize for u8 {
    fn deserialize(src: &mut &[u8]) -> Result<Self, Error> {
        Ok(Self::from_le_bytes(src.read_to()?))
    }
}

impl Deserialize for i32 {
    fn deserialize(src: &mut &[u8]) -> Result<Self, Error> {
        Ok(Self::from_le_bytes(src.read_to()?))
    }
}

impl Deserialize for u32 {
    fn deserialize(src: &mut &[u8]) -> Result<Self, Error> {
        Ok(Self::from_le_bytes(src.read_to()?))
    }
}

impl Deserialize for i64 {
    fn deserialize(src: &mut &[u8]) -> Result<Self, Error> {
        Ok(Self::from_le_bytes(src.read_to()?))
    }
}

impl Deserialize for f64 {
    fn deserialize(src: &mut &[u8]) -> Result<Self, Error> {
        Ok(Self::from_le_bytes(src.read_to()?))
    }
}

impl Deserialize for bool {
    fn deserialize(src: &mut &[u8]) -> Result<Self, Error> {
        let byte = u8::deserialize(src)?;
        Ok(byte == 1)
    }
}

impl Deserialize for String {
    fn deserialize(src: &mut &[u8]) -> Result<Self, Error> {
        let buf = Vec::<u8>::deserialize(src)?;
        Ok(Self::from_utf8(buf)?)
    }
}

impl Deserialize for Vec<u8> {
    fn deserialize(src: &mut &[u8]) -> Result<Self, Error> {
        let len = deserialize_dyn_len(src)?;
        Ok(src.read_to_vec(len)?)
    }
}

impl Deserialize for SystemTime {
    fn deserialize(src: &mut &[u8]) -> Result<Self, Error> {
        let millis = i64::deserialize(src)?;
        Ok(UNIX_EPOCH + Duration::from_millis(millis as u64))
    }
}

impl<T: Deserialize> Deserialize for Vec<T> {
    default fn deserialize(src: &mut &[u8]) -> Result<Self, Error> {
        let len = deserialize_dyn_len(src)?;
        (0..len).map(|_| T::deserialize(src)).collect()
    }
}

impl<T: Deserialize> Deserialize for Option<T> {
    fn deserialize(src: &mut &[u8]) -> Result<Self, Error> {
        if bool::deserialize(src)? {
            Ok(Some(T::deserialize(src)?))
        } else {
            Ok(None)
        }
    }
}

impl<T: Deserialize, E: Deserialize> Deserialize for Result<T, E> {
    fn deserialize(src: &mut &[u8]) -> Result<Self, Error> {
        if bool::deserialize(src)? {
            match T::deserialize(src) {
                Ok(value) => Ok(Ok(value)),
                Err(error) => Err(error),
            }
        } else {
            match E::deserialize(src) {
                Ok(error) => Ok(Err(error)),
                Err(error) => Err(error),
            }
        }
    }
}

fn deserialize_dyn_len(src: &mut &[u8]) -> Result<usize, Error> {
    let byte = u8::deserialize(src)?;
    if byte < 255 {
        Ok(byte as usize)
    } else {
        Ok(i64::deserialize(src)? as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn primitives() -> Result<(), Error> {
        assert_eq!(u8::from_bytes(&[0x2a])?, 42_u8);
        assert_eq!(i32::from_bytes(&[0x4e, 0x19, 0x8f, 0x1c])?, 479140174_i32);
        assert_eq!(i64::from_bytes(&[0x4d, 0xbe, 0x90, 0x9, 0xa2, 0xc6, 0x35, 0x1])?, 87194167051075149_i64);
        assert_eq!(f64::from_bytes(&[0xbc, 0x90, 0x0e, 0x0f, 0x61, 0x3a, 0x81, 0x40])?, 551.297392_f64);
        assert_eq!(bool::from_bytes(&[0x1])?, true);
        assert_eq!(bool::from_bytes(&[0x0])?, false);
        assert_eq!(String::from_bytes(&[0x5, b'h', b'e', b'l', b'l', b'o'])?, String::from("hello"));
        assert_eq!(
            Vec::<u8>::from_bytes(&[vec![0xff, 0xe8, 0x3, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0], vec![0xdd; 997], vec![b'j', b'o', b'y']].concat())?,
            [vec![0xdd; 997], vec![b'j', b'o', b'y']].concat()
        );
        assert_eq!(Option::<i32>::from_bytes(&[0x1, 0x28, 0x0, 0x0, 0x0])?, Some(0x28));
        assert_eq!(Option::<i32>::from_bytes(&[0x0])?, None::<i32>);
        Ok(())
    }

    #[test]
    fn dyn_len() -> Result<(), Error> {
        assert_eq!(dyn_len_from_bytes(vec![0x50])?, 0x50);
        assert_eq!(dyn_len_from_bytes(vec![0xff, 0x97, 0x43, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0])?, 0x24397);
        Ok(())
    }

    fn dyn_len_from_bytes(buf: Vec<u8>) -> Result<usize, Error> {
        let mut src = &buf as &[u8];
        deserialize_dyn_len(&mut src)
    }
}
