#![feature(min_specialization)]

mod serialize;
pub mod deserialize;
mod call;
mod read;

pub use call::Call;
pub use deserialize::Deserialize;
use read::Read;
pub use serialize::Serialize;
