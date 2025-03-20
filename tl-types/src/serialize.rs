use std::time::{SystemTime, UNIX_EPOCH};

// todo: tests

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

fn serialize_dyn_len(len: usize, buf: &mut Vec<u8>) {
    if len < 255 {
        buf.push(len as u8);
    } else {
        buf.push(255);
        (len as i64).serialize(buf);
    }
}
