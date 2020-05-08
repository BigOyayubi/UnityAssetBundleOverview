use std::io::{Seek, SeekFrom};
use std::io::Cursor;
use log::{info};
use serde::{Serialize, Deserialize};

use crate::binary_reader::BinaryReader;
use crate::Result;
use crate::endian::Endian;
use crate::class_info::ClassInfo;
use crate::object_info::ObjectInfo;
use crate::reference::Reference;
use crate::local_object_entry::LocalObjectEntry;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Asset {
    name: String,               // アセット名
    meta_size: u32,             // 
    file_size: u64,             // 
    format: u32,                // フォーマットバージョン
    endian: Endian,             // アセットバイナリエンディアン
    generator_version: String,  // アセット生成バージョン
    target_platform: i32,       // 対象プラットフォーム
    has_type_trees: bool,       // タイプ情報有無
    with_path_id: bool,         // 
    comment: String,            // コメント
    status: u32,                // 
    add_ids: Vec<LocalObjectEntry>,
    references: Vec<Reference>,
    classes: Vec<ClassInfo>,
    objects: Vec<ObjectInfo>,
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
                cursor.as_mut_ref().seek(SeekFrom::Current(3))?;
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
        let meta_size = meta_size;
        let file_size = file_size;
        let offset = offset;

        info!("meta_size {}, file_size {}, format {}, offset {} endian {:?}", 
                 meta_size, 
                 file_size, 
                 format, 
                 offset, 
                 endian);

        cursor.set_endian(endian.clone());

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
        info!("gen_ver {}, plat {}, type_tree {}, type_count {}", generator_version, target_platform, has_type_trees, type_count);

        let mut classes: Vec<ClassInfo> = Vec::new();
        for _ in 0..type_count {
            classes.push(ClassInfo::new(&mut cursor, format, has_type_trees));
        }

        let wide_path_id = format >= 14 || format >= 7 && cursor.int32() != 0;
        info!("wide_path_id {}", wide_path_id);

        let object_count = cursor.uint32();
        info!("object_count : {}", object_count);
        let mut objects: Vec<ObjectInfo> = Vec::new();
        for _ in 0..object_count {
            let obj = ObjectInfo::new(&mut cursor, format, wide_path_id);
            //info!("{:?}", obj);
            objects.push(obj);
        }
        let mut add_ids: Vec<LocalObjectEntry> = Vec::new();
        if format >= 11 {
            let add_id_count = cursor.uint32();
            info!("add_id_count {}", add_id_count);
            for _ in 0..add_id_count {
                if format >= 14 {
                    cursor.align(4);
                }
                let file_id = cursor.int32();
                let local_id = match wide_path_id {
                    true => cursor.int64(),
                    false => cursor.int32() as i64,
                };
                add_ids.push( LocalObjectEntry::new(file_id, local_id));
            }
        }
        let reference_count = cursor.uint32();
        info!("reference_count {}", reference_count);
        let mut references: Vec<Reference> = Vec::new();
        for _ in 0..reference_count {
            let path = match format >= 6 {
                true => cursor.cstr(),
                false => String::from(""),
            };
            let guid = match format >= 5 {
                true => Some(cursor.read(16)),
                false => None,
            };
            let type_ = match format >= 5 {
                true => Some(cursor.int32()),
                false => None,
            };
            let file_path = cursor.cstr();
            references.push(
                Reference::new(
                    path,
                    guid,
                    type_,
                    file_path,
                )
            );
        }

        let comment = cursor.cstr();
        info!("comment {}", comment);

        for o in &mut objects {
            cursor.as_mut_ref().seek(SeekFrom::Start(offset + o.offset))?;
            let b = cursor.read(o.size as usize);
            o.hash = Some(blake3::hash(&b).as_bytes().into_iter().map(|h| format!("{:02X}", h)).collect::<String>());
        }

        Ok(Asset{
            name: name.to_string(),
            meta_size: meta_size,
            file_size: file_size,
            format: format,
            endian: endian,
            generator_version: generator_version.to_string(),
            target_platform: target_platform,
            has_type_trees: has_type_trees,
            with_path_id: wide_path_id,
            comment: comment.to_string(),
            status: status,
            classes: classes,
            objects: objects,
            add_ids: add_ids,
            references: references,
        })
    }
}