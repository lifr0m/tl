use aws_lc_rs::digest;
use tl_parser::*;

pub(crate) trait Hash {
    fn hash(&self, cx: &mut digest::Context);
}

impl Hash for Schema {
    fn hash(&self, cx: &mut digest::Context) {
        self.types.iter().for_each(|def| def.hash(cx));
        self.functions.iter().for_each(|def| def.hash(cx));
    }
}

impl Hash for TypeDefinition {
    fn hash(&self, cx: &mut digest::Context) {
        cx.update(&self.id.to_le_bytes());
        cx.update(self.name.as_bytes());
        self.fields.iter().for_each(|f| f.hash(cx));
    }
}

impl Hash for FunctionDefinition {
    fn hash(&self, cx: &mut digest::Context) {
        cx.update(&self.id.to_le_bytes());
        cx.update(self.name.as_bytes());
        self.args.iter().for_each(|f| f.hash(cx));
        self.typ.hash(cx);
    }
}

impl Hash for Field {
    fn hash(&self, cx: &mut digest::Context) {
        cx.update(self.name.as_bytes());
        self.typ.hash(cx);
    }
}

impl Hash for Type {
    fn hash(&self, cx: &mut digest::Context) {
        match self {
            Self::Int32 => cx.update(&0_u8.to_le_bytes()),
            Self::Int64 => cx.update(&1_u8.to_le_bytes()),
            Self::Float => cx.update(&2_u8.to_le_bytes()),
            Self::Bool => cx.update(&3_u8.to_le_bytes()),
            Self::String => cx.update(&4_u8.to_le_bytes()),
            Self::Bytes => cx.update(&5_u8.to_le_bytes()),
            Self::Time => cx.update(&6_u8.to_le_bytes()),
            Self::Vector(inner) => {
                cx.update(&7_u8.to_le_bytes());
                inner.hash(cx);
            }
            Self::Option(inner) => {
                cx.update(&8_u8.to_le_bytes());
                inner.hash(cx);
            }
            Self::Defined(defined) => cx.update(defined.as_bytes()),
        }
    }
}
