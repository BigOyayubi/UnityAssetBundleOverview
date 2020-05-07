use serde::{Serialize, Deserialize};

/// bit endian
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Endian {
    Big,
    Little,
}
