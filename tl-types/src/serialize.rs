use std::time::{SystemTime, UNIX_EPOCH};

pub trait Serialize {
    fn serialize(&self, dst: &mut Vec<u8>);

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.serialize(&mut buf);
        buf
    }
}

impl Serialize for u8 {
    fn serialize(&self, dst: &mut Vec<u8>) {
        dst.extend(self.to_le_bytes());
    }
}

impl Serialize for i32 {
    fn serialize(&self, dst: &mut Vec<u8>) {
        dst.extend(self.to_le_bytes());
    }
}

impl Serialize for u32 {
    fn serialize(&self, dst: &mut Vec<u8>) {
        dst.extend(self.to_le_bytes());
    }
}

impl Serialize for i64 {
    fn serialize(&self, dst: &mut Vec<u8>) {
        dst.extend(self.to_le_bytes());
    }
}

impl Serialize for f64 {
    fn serialize(&self, dst: &mut Vec<u8>) {
        dst.extend(self.to_le_bytes());
    }
}

impl Serialize for bool {
    fn serialize(&self, dst: &mut Vec<u8>) {
        dst.push(if *self { 1 } else { 0 });
    }
}

impl Serialize for str {
    fn serialize(&self, dst: &mut Vec<u8>) {
        self.as_bytes().serialize(dst);
    }
}

impl Serialize for String {
    fn serialize(&self, dst: &mut Vec<u8>) {
        self.as_str().serialize(dst);
    }
}

impl Serialize for [u8] {
    fn serialize(&self, dst: &mut Vec<u8>) {
        serialize_dyn_len(self.len(), dst);
        dst.extend(self);
    }
}

impl Serialize for Vec<u8> {
    fn serialize(&self, dst: &mut Vec<u8>) {
        self.as_slice().serialize(dst);
    }
}

impl Serialize for SystemTime {
    fn serialize(&self, dst: &mut Vec<u8>) {
        (self.duration_since(UNIX_EPOCH).unwrap().as_millis() as i64).serialize(dst);
    }
}

impl<T: Serialize> Serialize for [T] {
    default fn serialize(&self, dst: &mut Vec<u8>) {
        serialize_dyn_len(self.len(), dst);
        self.iter().for_each(|e| e.serialize(dst));
    }
}

impl<T: Serialize> Serialize for Vec<T> {
    default fn serialize(&self, dst: &mut Vec<u8>) {
        self.as_slice().serialize(dst);
    }
}

impl<T: Serialize> Serialize for Option<T> {
    fn serialize(&self, dst: &mut Vec<u8>) {
        if let Some(value) = self {
            true.serialize(dst);
            value.serialize(dst);
        } else {
            false.serialize(dst);
        }
    }
}

impl<T: Serialize, E: Serialize> Serialize for Result<T, E> {
    fn serialize(&self, dst: &mut Vec<u8>) {
        match self {
            Ok(value) => {
                true.serialize(dst);
                value.serialize(dst);
            }
            Err(error) => {
                false.serialize(dst);
                error.serialize(dst);
            }
        };
    }
}

fn serialize_dyn_len(len: usize, dst: &mut Vec<u8>) {
    if len < 255 {
        dst.push(len as u8);
    } else {
        dst.push(255);
        (len as i64).serialize(dst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitives() {
        assert_eq!(42_u8.to_bytes(), vec![0x2a]);
        assert_eq!(479140174_i32.to_bytes(), vec![0x4e, 0x19, 0x8f, 0x1c]);
        assert_eq!(87194167051075149_i64.to_bytes(), vec![0x4d, 0xbe, 0x90, 0x9, 0xa2, 0xc6, 0x35, 0x1]);
        assert_eq!(551.297392_f64.to_bytes(), vec![0xbc, 0x90, 0x0e, 0x0f, 0x61, 0x3a, 0x81, 0x40]);
        assert_eq!(true.to_bytes(), vec![0x1]);
        assert_eq!(false.to_bytes(), vec![0x0]);
        assert_eq!("hello".to_bytes(), vec![0x5, b'h', b'e', b'l', b'l', b'o']);
        assert_eq!(
            [vec![0xdd; 997], vec![b'j', b'o', b'y']].concat().to_bytes(),
            [vec![0xff, 0xe8, 0x3, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0], vec![0xdd; 997], vec![b'j', b'o', b'y']].concat()
        );
        assert_eq!(Some(0x28).to_bytes(), vec![0x1, 0x28, 0x0, 0x0, 0x0]);
        assert_eq!(None::<i32>.to_bytes(), vec![0x0]);
    }

    #[test]
    fn dyn_len() {
        assert_eq!(dyn_len_to_bytes(0x50), vec![0x50]);
        assert_eq!(dyn_len_to_bytes(0x24397), vec![0xff, 0x97, 0x43, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0]);
    }

    fn dyn_len_to_bytes(len: usize) -> Vec<u8> {
        let mut buf = Vec::new();
        serialize_dyn_len(len, &mut buf);
        buf
    }
}
