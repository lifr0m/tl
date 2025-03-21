mod schema;
mod serialize;
mod deserialize;

use deserialize::Deserialize;
pub use schema::*;
use serialize::Serialize;

trait Identify {
    const ID: u16;
}

trait Function {
    type Return: Serialize + Deserialize;
}
