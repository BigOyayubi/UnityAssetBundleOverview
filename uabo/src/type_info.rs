use std::io::{Read,Seek,SeekFrom,Cursor};
use std::collections::HashMap;
use crate::endian::Endian;
use crate::binary_reader::BinaryReader;
use crate::constants;

#[derive(Clone, Debug)]
pub struct TypeInfo {
    class_id: i32,
    stripped: Option<bool>,
    script_id: Option<i16>,
    hash: Option<Vec<u8>>,
    type_tree: Option<TypeTree>
}

#[derive(Clone, Debug)]
pub struct TypeTree {
    nodes: Vec<TypeTreeNode>
}

#[derive(Clone, Debug)]
pub struct TypeTreeNode{
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
    children: Vec<usize>, //index to TypeTreeNodeArray
}

impl TypeInfo {
    pub fn new<T: Read+Seek>(reader: &mut BinaryReader<T>, format: u32, has_type_tree: bool) -> TypeInfo{
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
                let mut buf = vec![0u8; size];
                reader.as_mut_ref().read_exact(&mut buf);
                Some(buf)
            },
            false => None,
        };
        println!("class_id {}, stripped {:?}, script_id {:?}", class_id, stripped, script_id);
        let type_tree = match has_type_tree {
            true => {
                Some(TypeTree::new(reader, format))
            },
            false => None,
        };
        TypeInfo{
            class_id: class_id,
            stripped: stripped,
            script_id: script_id,
            hash: hash,
            type_tree: type_tree,
        }
    }
}

impl TypeTree {
    pub fn new<T: Read+Seek>(reader: &mut BinaryReader<T>, format: u32) -> TypeTree {
        let node_count = reader.uint32();
        let buf_size = reader.uint32();
        let mut nodes: Vec<TypeTreeNode> = Vec::new();
        for _ in 0..node_count {
            let mut node = TypeTreeNode::new(reader);
            if node.level > 0 {
                let idx = nodes.as_slice().len();
                nodes.as_mut_slice()[(node.level-1) as usize].children.push(idx);
            }
            nodes.push(node);
        }
        let mut buf = vec![0u8; buf_size as usize];
        reader.as_mut_ref().read_exact(&mut buf);
        let mut reader = BinaryReader::new(Cursor::new(buf), Endian::Big);

        for mut node in &mut nodes {
            node.type_str = constants::get_string_or_default(node.type_id, &mut reader);
            node.name_str = constants::get_string_or_default(node.name_id, &mut reader);
        }
        if format >= 21 {
            reader.as_mut_ref().seek(SeekFrom::Current(4));
        }
        TypeTree {
            nodes: nodes,
        }
    }
}

impl TypeTreeNode {
    pub fn new<T: Read + Seek>(reader: &mut BinaryReader<T>) -> TypeTreeNode {
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
        println!("version : {}, level : {}, type_id : {}, name_id : {}", ver, lv, ty, name);
        TypeTreeNode{
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
