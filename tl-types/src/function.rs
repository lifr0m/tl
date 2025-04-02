use crate::{Deserialize, Serialize};

pub(crate) trait Function {
    type Return: Serialize + Deserialize;
}
