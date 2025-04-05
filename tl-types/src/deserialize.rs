use crate::Reader;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unexpected eof")]
    UnexpectedEof(#[from] crate::reader::Error),

    #[error("invalid string: {0}")]
    InvalidString(#[from] std::string::FromUtf8Error),

    #[error("unexpected definition id: {0}")]
    UnexpectedDefinitionId(u32),
}

pub trait Deserialize
where
    Self: Sized,
{
    fn deserialize(reader: &mut Reader) -> Result<Self, Error>;
}

impl Deserialize for u8 {
    fn deserialize(reader: &mut Reader) -> Result<Self, Error> {
        Ok(Self::from_le_bytes(reader.read_to()?))
    }
}

impl Deserialize for i32 {
    fn deserialize(reader: &mut Reader) -> Result<Self, Error> {
        Ok(Self::from_le_bytes(reader.read_to()?))
    }
}

impl Deserialize for u32 {
    fn deserialize(reader: &mut Reader) -> Result<Self, Error> {
        Ok(Self::from_le_bytes(reader.read_to()?))
    }
}

impl Deserialize for i64 {
    fn deserialize(reader: &mut Reader) -> Result<Self, Error> {
        Ok(Self::from_le_bytes(reader.read_to()?))
    }
}

impl Deserialize for f64 {
    fn deserialize(reader: &mut Reader) -> Result<Self, Error> {
        Ok(Self::from_le_bytes(reader.read_to()?))
    }
}

impl Deserialize for bool {
    fn deserialize(reader: &mut Reader) -> Result<Self, Error> {
        let byte = u8::deserialize(reader)?;
        Ok(byte == 1)
    }
}

impl Deserialize for String {
    fn deserialize(reader: &mut Reader) -> Result<Self, Error> {
        let buf = Vec::<u8>::deserialize(reader)?;
        Ok(Self::from_utf8(buf)?)
    }
}

impl Deserialize for Vec<u8> {
    fn deserialize(reader: &mut Reader) -> Result<Self, Error> {
        let len = deserialize_dyn_len(reader)?;
        Ok(reader.read_to_vec(len)?)
    }
}

impl Deserialize for SystemTime {
    fn deserialize(reader: &mut Reader) -> Result<Self, Error> {
        let millis = i64::deserialize(reader)?;
        Ok(UNIX_EPOCH + Duration::from_millis(millis as u64))
    }
}

impl<T: Deserialize> Deserialize for Vec<T> {
    default fn deserialize(reader: &mut Reader) -> Result<Self, Error> {
        let len = deserialize_dyn_len(reader)?;
        (0..len).map(|_| T::deserialize(reader)).collect()
    }
}

impl<T: Deserialize> Deserialize for Option<T> {
    fn deserialize(reader: &mut Reader) -> Result<Self, Error> {
        if bool::deserialize(reader)? {
            Ok(Some(T::deserialize(reader)?))
        } else {
            Ok(None)
        }
    }
}

fn deserialize_dyn_len(reader: &mut Reader) -> Result<usize, Error> {
    let byte = u8::deserialize(reader)?;
    if byte < 255 {
        Ok(byte as usize)
    } else {
        Ok(i64::deserialize(reader)? as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn primitives() {
        assert_eq!(de::<u8>(vec![0x2a]), 42_u8);
        assert_eq!(de::<i32>(vec![0x4e, 0x19, 0x8f, 0x1c]), 479140174_i32);
        assert_eq!(de::<i64>(vec![0x4d, 0xbe, 0x90, 0x9, 0xa2, 0xc6, 0x35, 0x1]), 87194167051075149_i64);
        assert_eq!(de::<f64>(vec![0xbc, 0x90, 0x0e, 0x0f, 0x61, 0x3a, 0x81, 0x40]), 551.297392_f64);
        assert_eq!(de::<bool>(vec![0x1]), true);
        assert_eq!(de::<bool>(vec![0x0]), false);
        assert_eq!(de::<String>(vec![0x5, b'h', b'e', b'l', b'l', b'o']), "hello".to_string());
        assert_eq!(
            de::<Vec<u8>>([vec![0xff, 0xe8, 0x3, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0], vec![0xdd; 997], vec![b'j', b'o', b'y']].concat()),
            [vec![0xdd; 997], vec![b'j', b'o', b'y']].concat()
        );
        assert_eq!(de::<Option<i32>>(vec![0x1, 0x28, 0x0, 0x0, 0x0]), Some(0x28));
        assert_eq!(de::<Option<i32>>(vec![0x0]), None::<i32>);
    }

    #[test]
    fn dyn_len() {
        assert_eq!(de_dyn_len(vec![0x50]), 0x50);
        assert_eq!(de_dyn_len(vec![0xff, 0x97, 0x43, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0]), 0x24397);
    }

    fn de<T: Deserialize>(buf: Vec<u8>) -> T {
        let mut reader = Reader::new(&buf);
        T::deserialize(&mut reader).unwrap()
    }

    fn de_dyn_len(buf: Vec<u8>) -> usize {
        let mut reader = Reader::new(&buf);
        deserialize_dyn_len(&mut reader).unwrap()
    }
}
