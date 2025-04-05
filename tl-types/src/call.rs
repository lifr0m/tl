use crate::{Deserialize, Serialize};

pub trait Call {
    type Return: Serialize + Deserialize;
    type Result = Result<Self::Return, crate::Error>;
}
