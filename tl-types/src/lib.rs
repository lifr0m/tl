#![feature(min_specialization)]

mod schema;
mod serialize;
pub mod deserialize;
mod identify;
mod function;
mod reader;

pub use deserialize::Deserialize;
pub use function::Function;
use identify::Identify;
use reader::Reader;
pub use schema::*;
pub use serialize::Serialize;
