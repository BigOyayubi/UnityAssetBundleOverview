use std::io::{Read,Seek};
use crate::binary_reader::BinaryReader;

#[derive(Clone, Debug)]
pub struct ObjectInfo {
    path_id: i64,
    offset: u64,
    size: u32,
    class_idx: Option<u32>,
    type_id: Option<i32>,
    class_id: Option<i16>,
    destroyed: Option<bool>,
    stripped: Option<bool>,
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
                println!("path_id: {}, offset: {}", path_id, offset);
                ObjectInfo{
                    path_id: path_id,
                    offset: offset,
                    size: size,
                    class_idx: Some(class_idx),
                    type_id: None,
                    class_id: None,
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
                    destroyed: Some(destroyed),
                    stripped: stripped,
                }
            },
        }
    }
}