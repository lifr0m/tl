use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("eof")]
    Eof,
}

pub(crate) struct Reader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    pub(crate) fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    pub(crate) fn read(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        if buf.len() > self.buf.len() - self.pos {
            return Err(Error::Eof);
        }

        buf.clone_from_slice(&self.buf[self.pos..][..buf.len()]);
        self.pos += buf.len();

        Ok(())
    }
    
    pub(crate) fn read_to<const N: usize>(&mut self) -> Result<[u8; N], Error> {
        let mut buf = [0; N];
        self.read(&mut buf)?;
        Ok(buf)
    }
    
    pub(crate) fn read_to_vec(&mut self, len: usize) -> Result<Vec<u8>, Error> {
        let mut buf = vec![0; len];
        self.read(&mut buf)?;
        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let data = [1, 2, 3, 4];
        let mut reader = Reader::new(&data);

        let mut buf = [0; 2];
        reader.read(&mut buf).unwrap();
        assert_eq!(buf, [1, 2]);

        let mut buf = [0; 3];
        reader.read(&mut buf).unwrap_err();

        let mut buf = [0; 2];
        reader.read(&mut buf).unwrap();
        assert_eq!(buf, [3, 4]);

        let mut buf = [0; 1];
        reader.read(&mut buf).unwrap_err();
        
        reader.pos = 0;
        
        assert_eq!(reader.read_to().unwrap(), [1, 2, 3]);
        assert_eq!(reader.read_to().unwrap(), [4]);
        assert_eq!(reader.read_to().unwrap(), []);
        reader.read_to::<1>().unwrap_err();

        reader.pos = 0;

        assert_eq!(reader.read_to_vec(3).unwrap(), [1, 2, 3]);
        assert_eq!(reader.read_to_vec(1).unwrap(), [4]);
        assert_eq!(reader.read_to_vec(0).unwrap(), []);
        reader.read_to_vec(1).unwrap_err();
    }
}
