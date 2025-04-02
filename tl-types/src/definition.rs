use crate::{Deserialize, Identify, Serialize};

pub(crate) trait Definition: Identify + Serialize + Deserialize {}
