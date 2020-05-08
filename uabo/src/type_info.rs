use std::io::{Read, Seek, Cursor};
use log::{info};
use serde::{Serialize, Deserialize};
use crate::endian::Endian;
use crate::constants;
use crate::binary_reader::BinaryReader;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TypeInfo{
    version: u16,
    level: u8,
    is_array: bool,
    type_id: u32,
    type_str: String,
    name_id: u32,
    name_str: String,
    size: i32,
    index: u32,
    flags: u32,
    v18meta: Option<u64>,
    children: Vec<TypeInfo>,
}

impl TypeInfo {
    pub fn load<T: Read+Seek>(reader: &mut BinaryReader<T>, format: u32) -> Vec<TypeInfo> {
        let node_count = reader.uint32();
        let buf_size = reader.uint32();
        let mut nodes: Vec<TypeInfo> = Vec::new();
        for _ in 0..node_count {
            let node = TypeInfo::new(reader);
            nodes.push(node);
        }
        let buf = reader.read(buf_size as usize);
        let mut reader = BinaryReader::new(Cursor::new(buf), Endian::Big);

        for mut node in &mut nodes {
            node.type_str = constants::get_string_or_default(node.type_id, &mut reader);
            node.name_str = constants::get_string_or_default(node.name_id, &mut reader);
        }
        if format >= 21 {
            reader.skip(4);
        }
        let mut reverse: Vec<TypeInfo> = Vec::new();
        while !nodes.is_empty() {
            let n = nodes.pop().unwrap();
            if n.level > 0 {
                let idx = n.level - 1;
                nodes[ idx as usize ].children.push(n);
            } else {
                reverse.push(n);
            }
        };

        reverse
    }
    fn new<T: Read + Seek>(reader: &mut BinaryReader<T>) -> TypeInfo {
        let ver = reader.uint16();
        let lv  = reader.uint8();
        let is_arr = reader.boolean();
        let ty = reader.uint32();
        let name= reader.uint32();
        let size = reader.int32();
        let index = reader.uint32();
        let flags = reader.uint32();
        let v18meta = match ver >= 18 {
            true => Some(reader.uint64()),
            false => None,
        };
        info!("version : {}, level : {}, type_id : {}, name_id : {}", ver, lv, ty, name);
        TypeInfo{
            version: ver,
            level: lv,
            is_array: is_arr,
            type_id: ty,
            type_str: String::from(""),
            name_id: name,
            name_str: String::from(""),
            size: size,
            index: index,
            flags: flags,
            v18meta: v18meta,
            children: Vec::new(),
        }
    }
}
