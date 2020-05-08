use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LocalObjectEntry {
    file_id: i32,
    local_id: i64,
}

impl LocalObjectEntry {
    pub fn new(file_id: i32, local_id: i64) -> LocalObjectEntry {
        LocalObjectEntry
        {
            file_id: file_id,
            local_id: local_id
        }
    }
}
