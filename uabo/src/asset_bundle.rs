use std::path::PathBuf;
use std::sync::Arc;
use std::fs::File;
use std::io::{BufReader, Read, Write, Seek, SeekFrom, Cursor};
use lz4::block::decompress;
use crate::asset::Asset;
use crate::read_ext::ReadPrimitive;
use crate::Result;

#[derive(Clone, Debug)]
pub struct AssetBundle(Arc<AssetBundleImpl>);

impl AssetBundle {
    pub fn load(src: PathBuf) -> Result<AssetBundle> {
        let i = AssetBundleImpl::load(&src);
        match i {
            Ok(o) => Ok(AssetBundle(Arc::new(o))),
            Err(o) => Err(format!("can not load {:?}", src.to_string_lossy()).into())
        }
    }
    pub fn header(&self) -> &AssetBundleHeader {
        &self.0.header
    }
    pub fn Assets(&self) -> &Vec<Asset> {
        &self.0.assets
    }
}

#[derive(Clone, Debug)]
pub struct AssetBundleHeader{
    signiture: String,
    file_version: u32,
    lower_player_version: String,
    upper_player_version: String,
    total_file_size: i64,
    compressed_block_info_size: u32,
    decompressed_block_info_size: u32,
    flags: u32,
}

#[derive(Clone, Debug)]
struct AssetBundleImpl{
    header: AssetBundleHeader,
    assets: Vec<Asset>,
}

impl AssetBundleImpl {
    pub fn load(src: &PathBuf) -> Result<AssetBundleImpl> {
        let mut file = BufReader::new(File::open(src).unwrap());

        let signiture = file.cstr();
        match &*signiture {
            "UnityFS" => AssetBundleImpl::read_asset_bundle(&mut file),
            _         => Err(format!("invalid signature").into())
        }
    }

    fn read_asset_bundle<T: std::io::Read+std::io::Seek>(file: &mut T) -> Result<AssetBundleImpl>{
        let header = AssetBundleHeader::read(file).unwrap();
        println!("asset bundle header : {:?}", header);
        let assets = Asset::read(file,
                                 header.file_version,
                                 header.is_block_infos_at_end(),
                                 header.compressed_block_info_size,
                                 header.decompressed_block_info_size,
                                 header.flags).unwrap(); 
        Ok(AssetBundleImpl{
            header: header,
            assets: assets,
        })
    }
}

impl AssetBundleHeader {
    pub fn read<T: std::io::Read + std::io::Seek> (file: &mut T) -> Result<AssetBundleHeader>{
        //file version
        //4byte big-endian 
        let file_version = file.uint32();

        //min player revision
        //0-terminated; 
        let lower_player_version = file.cstr();

        //max player version
        //0-terminated; 
        let upper_player_version = file.cstr();

        //total file size
        //8byte big-endian
        let total_file_size = file.int64();

        //4byte big-endian
        let compressed_block_size_info = file.uint32();

        //4byte big-endian
        let decompressed_block_size_info = file.uint32();

        //4byte big-endian
        let flags = file.uint32();

        Ok(AssetBundleHeader{
            signiture: "UnityFS".to_owned(),
            file_version: file_version,
            lower_player_version: lower_player_version,
            upper_player_version: upper_player_version,
            total_file_size: total_file_size,
            compressed_block_info_size: compressed_block_size_info,
            decompressed_block_info_size: decompressed_block_size_info,
            flags: flags,
        })
    }

    pub fn is_block_infos_at_end(&self) -> bool {
        &self.flags & 0x80 != 0
    }
    pub fn is_block_info_contains_dir_info(&self) -> bool {
        &self.flags & 0x40 != 0
    }
}