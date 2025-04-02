use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) trait Serialize {
    fn serialize(&self, buf: &mut Vec<u8>);
}

impl Serialize for u16 {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.extend(self.to_le_bytes());
    }
}

impl Serialize for i32 {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.extend(self.to_le_bytes());
    }
}

impl Serialize for i64 {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.extend(self.to_le_bytes());
    }
}

impl Serialize for f64 {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.extend(self.to_le_bytes());
    }
}

impl Serialize for bool {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.push(if *self { 1 } else { 0 });
    }
}

impl Serialize for str {
    fn serialize(&self, buf: &mut Vec<u8>) {
        self.as_bytes().serialize(buf);
    }
}

impl Serialize for String {
    fn serialize(&self, buf: &mut Vec<u8>) {
        self.as_str().serialize(buf);
    }
}

impl Serialize for [u8] {
    fn serialize(&self, buf: &mut Vec<u8>) {
        serialize_dyn_len(self.len(), buf);
        buf.extend(self);
    }
}

impl Serialize for Vec<u8> {
    fn serialize(&self, buf: &mut Vec<u8>) {
        self.as_slice().serialize(buf);
    }
}

impl Serialize for SystemTime {
    fn serialize(&self, buf: &mut Vec<u8>) {
        (self.duration_since(UNIX_EPOCH).unwrap().as_millis() as i64).serialize(buf);
    }
}

impl<T: Serialize> Serialize for [T] {
    fn serialize(&self, buf: &mut Vec<u8>) {
        serialize_dyn_len(self.len(), buf);
        self.iter().for_each(|e| e.serialize(buf));
    }
}

impl<T: Serialize> Serialize for Vec<T> {
    fn serialize(&self, buf: &mut Vec<u8>) {
        self.as_slice().serialize(buf);
    }
}

impl<T: Serialize> Serialize for Option<T> {
    fn serialize(&self, buf: &mut Vec<u8>) {
        if let Some(value) = self {
            true.serialize(buf);
            value.serialize(buf);
        } else {
            false.serialize(buf);
        }
    }
}

impl Serialize for crate::Id {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.extend(self.0);
    }
}

fn serialize_dyn_len(len: usize, buf: &mut Vec<u8>) {
    if len < 255 {
        buf.push(len as u8);
    } else {
        buf.push(255);
        (len as i64).serialize(buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitives() {
        assert_eq!(ser(43913_u16), vec![0x89, 0xab]);
        assert_eq!(ser(479140174_i32), vec![0x4e, 0x19, 0x8f, 0x1c]);
        assert_eq!(ser(87194167051075149_i64), vec![0x4d, 0xbe, 0x90, 0x9, 0xa2, 0xc6, 0x35, 0x1]);
        assert_eq!(ser(551.297392_f64), vec![0xbc, 0x90, 0x0e, 0x0f, 0x61, 0x3a, 0x81, 0x40]);
        assert_eq!(ser(true), vec![0x1]);
        assert_eq!(ser(false), vec![0x0]);
        assert_eq!(ser("hello".to_string()), vec![0x5, b'h', b'e', b'l', b'l', b'o']);
        assert_eq!(
            ser([vec![0xdd; 997], vec![b'j', b'o', b'y']].concat()),
            [vec![0xFF, 0xe8, 0x3, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0], vec![0xdd; 997], vec![b'j', b'o', b'y']].concat()
        );
        assert_eq!(ser(Some(0x28)), vec![0x1, 0x28, 0x0, 0x0, 0x0]);
        assert_eq!(ser(None::<i32>), vec![0x0]);
        assert_eq!(ser(crate::Id([0x1, 0x2, 0x3, 0x4])), vec![0x1, 0x2, 0x3, 0x4]);
    }

    #[test]
    fn dyn_len() {
        assert_eq!(ser_dyn_len(0x50), vec![0x50]);
        assert_eq!(ser_dyn_len(0x24397), vec![0xFF, 0x97, 0x43, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0]);
    }

    fn ser<T: Serialize>(value: T) -> Vec<u8> {
        let mut buf = Vec::<u8>::new();
        value.serialize(&mut buf);
        buf
    }

    fn ser_dyn_len(len: usize) -> Vec<u8> {
        let mut buf = Vec::<u8>::new();
        serialize_dyn_len(len, &mut buf);
        buf
    }
}
