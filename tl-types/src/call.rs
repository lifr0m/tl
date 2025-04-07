use crate::{Deserialize, Serialize};

pub trait Call {
    type Return: Serialize + Deserialize;
}
