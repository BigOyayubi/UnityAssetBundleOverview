
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ContainerInfo {
    name: String,
    preload_index: u32,
    preload_size: u32,
    file_id: u32,
    path_id: u32,
}