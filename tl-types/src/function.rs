use crate::{Deserialize, Serialize};

pub trait Function {
    type Return: Serialize + Deserialize;
}
