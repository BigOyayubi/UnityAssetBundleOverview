use std::sync::Arc;
use std::io::{Seek, SeekFrom};
use std::io::Cursor;
use std::io::{Read, Write};
use crate::binary_reader::BinaryReader;
use crate::Result;
use crate::endian::Endian;
use crate::type_info::TypeInfo;
use crate::object_info::ObjectInfo;

#[derive(Clone, Debug)]
pub struct Asset(Arc<AssetImpl>);

#[derive(Clone, Debug)]
struct AssetImpl {
    name: String,
    status: u32,
    type_infos: Vec<TypeInfo>,
}

impl Asset {
    pub fn read(name: &String, status: u32, data: &[u8]) -> Result<Asset>{
        let mut cursor = BinaryReader::new(Cursor::new(data), Endian::Big);
        let mut meta_size = cursor.uint32();
        let mut file_size = cursor.uint32() as u64;
        let format    = cursor.uint32();
        let mut offset    = cursor.uint32() as u64;
        let endian    = match format >= 9 {
            true => {
                let e = match cursor.uint8() != 0 {
                    true => Endian::Big, 
                    false => Endian::Little,
                };
                cursor.as_mut_ref().seek(SeekFrom::Current(3));
                e
            },
            false => {
                cursor.as_mut_ref().seek(SeekFrom::End(meta_size as i64))?;
                match cursor.uint8() != 0 {
                    true => Endian::Big,
                    false => Endian::Little,
                }
            }
        };

        if format >= 22 {
            meta_size = cursor.uint32();
            file_size = cursor.uint64();
            offset = cursor.uint64();
            cursor.as_mut_ref().seek(SeekFrom::Current(8))?;
        }

        println!("meta_size {}, file_size {}, format {}, offset {} endian {:?}", 
                 meta_size, 
                 file_size, 
                 format, 
                 offset, 
                 endian);

        cursor.set_endian(endian);

        let generator_version = match format >= 7 {
            true => cursor.cstr(),
            false => String::from("")
        };
        let target_platform = match format >= 8 {
            true => cursor.int32(),
            false => -1,
        };
        let has_type_trees = match format >= 13 {
            true => cursor.boolean(),
            false => true
        };
        let type_count = cursor.uint32();
        println!("gen_ver {}, plat {}, type_tree {}, type_count {}", generator_version, target_platform, has_type_trees, type_count);

        let mut type_infos: Vec<TypeInfo> = Vec::new();
        for _ in 0..type_count {
            let type_info = TypeInfo::new(&mut cursor, format, has_type_trees);
            type_infos.push(type_info);
        }

        let mut wide_path_id = false;
        if format >= 14 {
            wide_path_id = true;
        } else if format >= 7 {
            wide_path_id = cursor.int32() != 0;
        } else {
            wide_path_id = false;
        }
        println!("wide_path_id {}", wide_path_id);

        let object_count = cursor.uint32();
        println!("object_count : {}", object_count);
        let mut objects: Vec<ObjectInfo> = Vec::new();
        for _ in 0..object_count {
            let obj = ObjectInfo::new(&mut cursor, format, wide_path_id);
            //println!("{:?}", obj);
            objects.push(obj);
        }
        Ok(Asset(Arc::new(AssetImpl{
            name: name.to_string(),
            status: status,
            type_infos: type_infos,
        })))
    }
}