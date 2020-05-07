use std::io::{Read,Seek};
use log::{info, trace, warn};
use serde::{Serialize, Deserialize};
use crate::binary_reader::BinaryReader;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ObjectInfo {
    pub path_id: i64,                   // path ID
    pub offset: u64,                    // data offset
    pub size: u32,                      // data size
    pub class_idx: Option<u32>,         // class table index
    pub type_id: Option<i32>,           // type id
    pub class_id: Option<i16>,          // class id
    pub hash: Option<String>,           // blake3 hash value of data
    pub destroyed: Option<bool>,        // destroyed or not
    pub stripped: Option<bool>,         // stripped or not
}

impl ObjectInfo {
    pub fn new<T: Read + Seek>(reader: &mut BinaryReader<T>, format: u32, wide_path_id: bool) -> ObjectInfo {
        if format >= 14 {
            reader.align(4);
        }
        match format >= 16 {
            true => {
                let path_id = match wide_path_id {
                    true => reader.int64(),
                    false => reader.int32() as i64,
                };
                let offset = match format >= 22 {
                    true => reader.uint64(),
                    false => reader.uint32() as u64,
                };
                let size = reader.uint32();
                let class_idx = reader.uint32();
                let stripped = match format == 16 {
                    true => Some(reader.boolean()),
                    false => None,
                };
                info!("path_id: {}, offset: {}", path_id, offset);
                ObjectInfo{
                    path_id: path_id,
                    offset: offset,
                    size: size,
                    class_idx: Some(class_idx),
                    type_id: None,
                    class_id: None,
                    hash: None,
                    destroyed: None,
                    stripped: stripped,
                }
            },
            false => {
                let path_id = match wide_path_id {
                    true => reader.int64(),
                    false => reader.int32() as i64,
                };
                let offset = reader.uint32();
                let size = reader.uint32();
                let type_id = reader.int32();
                let class_id = reader.int16();
                let destroyed = reader.int16() == 1;
                let stripped = match format == 15 {
                    true => Some(reader.boolean()),
                    false => None,
                };
                ObjectInfo{
                    path_id: path_id,
                    offset: offset as u64,
                    size: size,
                    class_idx: None,
                    type_id: Some(type_id),
                    class_id: Some(class_id),
                    hash: None,
                    destroyed: Some(destroyed),
                    stripped: stripped,
                }
            },
        }
   }
}