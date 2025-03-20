mod schema;
mod serialize;
mod deserialize;

pub use schema::*;

trait Identify {
    const ID: u16;
}

trait Function {
    type Return: serialize::Serialize + deserialize::Deserialize;
}
