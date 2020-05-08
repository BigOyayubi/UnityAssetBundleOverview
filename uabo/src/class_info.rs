use std::io::{Read,Seek};
use log::{info};
use serde::{Serialize, Deserialize};
use crate::binary_reader::BinaryReader;
use crate::type_info::TypeInfo;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClassInfo {
    pub class_id: i32,
    pub stripped: Option<bool>,
    pub script_id: Option<i16>,
    pub hash: Option<String>,
    pub types: Option<Vec<TypeInfo>>,
}

impl ClassInfo {
    pub fn new<T: Read+Seek>(reader: &mut BinaryReader<T>, format: u32, has_type_tree: bool) -> ClassInfo{
        let class_id = reader.int32();
        let stripped = match format >= 16 {
            true => Some(reader.boolean()),
            false => None,
        };
        let script_id = match format >= 17 {
            true => Some(reader.int16()),
            false => None,
        };
        let hash = match format >= 13 {
            true => {
                let size = match format < 16 && class_id < 0 || format >= 16 && class_id == 114 {
                    true => 32,
                    false => 16,
                };
                let buf = reader.read(size);
                Some(buf.into_iter().map(|h| format!("{:02X}", h)).collect::<String>())
            },
            false => None,
        };
        info!("class_id {}, stripped {:?}, script_id {:?}", class_id, stripped, script_id);
        let type_tree = match has_type_tree {
            true => {
                Some(TypeInfo::load(reader, format))
            },
            false => None,
        };
        ClassInfo{
            class_id: class_id,
            stripped: stripped,
            script_id: script_id,
            hash: hash,
            types: type_tree,
        }
    }
}

