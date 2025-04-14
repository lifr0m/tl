use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("eof")]
    Eof,
}

pub(crate) trait Read {
    fn read(&mut self, dst: &mut [u8]) -> Result<(), Error>;

    fn read_to<const N: usize>(&mut self) -> Result<[u8; N], Error> {
        let mut buf = [0; N];
        self.read(&mut buf)?;
        Ok(buf)
    }

    fn read_to_vec(&mut self, len: usize) -> Result<Vec<u8>, Error> {
        let mut buf = vec![0; len];
        self.read(&mut buf)?;
        Ok(buf)
    }
}

impl Read for &[u8] {
    fn read(&mut self, dst: &mut [u8]) -> Result<(), Error> {
        if dst.len() > self.len() {
            return Err(Error::Eof);
        }

        dst.clone_from_slice(&self[..dst.len()]);
        *self = &self[dst.len()..];

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let data = [1_u8, 2, 3, 4];

        let mut src = &data as &[u8];

        let mut buf = [0; 2];
        src.read(&mut buf).unwrap();
        assert_eq!(buf, [1, 2]);

        let mut buf = [0; 3];
        src.read(&mut buf).unwrap_err();

        let mut buf = [0; 2];
        src.read(&mut buf).unwrap();
        assert_eq!(buf, [3, 4]);

        let mut buf = [0; 1];
        src.read(&mut buf).unwrap_err();

        let mut src = &data as &[u8];

        assert_eq!(src.read_to().unwrap(), [1, 2, 3]);
        assert_eq!(src.read_to().unwrap(), [4]);
        assert_eq!(src.read_to().unwrap(), []);
        src.read_to::<1>().unwrap_err();

        let mut src = &data as &[u8];

        assert_eq!(src.read_to_vec(3).unwrap(), [1, 2, 3]);
        assert_eq!(src.read_to_vec(1).unwrap(), [4]);
        assert_eq!(src.read_to_vec(0).unwrap(), []);
        src.read_to_vec(1).unwrap_err();
    }
}
