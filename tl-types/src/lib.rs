#![feature(min_specialization)]
#![feature(associated_type_defaults)]

mod schema;
mod serialize;
pub mod deserialize;
mod call;
mod reader;

pub use call::Call;
pub use deserialize::Deserialize;
pub use reader::Reader;
pub use schema::*;
pub use serialize::Serialize;
