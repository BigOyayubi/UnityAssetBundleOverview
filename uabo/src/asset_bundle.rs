use std::path::PathBuf;
use std::sync::Arc;
use std::fs::File;
use std::io::{BufReader, Read, Write, Seek, SeekFrom, Cursor};
use crate::decompress::decompress_chunk;
use crate::asset::Asset;
use crate::binary_reader::BinaryReader;
use crate::endian::Endian;
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
        let mut file = BinaryReader::new(BufReader::new(File::open(src).unwrap()), Endian::Big);

        let signiture = file.cstr();
        match &*signiture {
            "UnityFS" => AssetBundleImpl::read_asset_bundle(&mut file),
            _         => Err(format!("invalid signature").into())
        }
    }

    fn read_asset_bundle<T: Read + Seek>(file: &mut BinaryReader<T>) -> Result<AssetBundleImpl>{
        let header = AssetBundleHeader::read(file).unwrap();

        // read block infos
        let mut compressed_buf = vec![0u8; header.compressed_block_info_size as usize];
        match header.is_block_infos_at_end() {
            true => {
                let pos = file.as_mut_ref().seek(SeekFrom::Current(0))?;
                file.as_mut_ref().seek(SeekFrom::End(header.compressed_block_info_size as i64))?;
                file.as_mut_ref().read_exact(&mut compressed_buf)?;
                file.as_mut_ref().seek(SeekFrom::Start(pos))?;
            },
            false => {
                if header.file_version >= 7 {
                    let pos = file.as_mut_ref().seek(SeekFrom::Current(0))?;
                    file.as_mut_ref().seek(SeekFrom::Current(pos as i64 & 16))?;
                }
                //let pos = file.seek(SeekFrom::Current(0)).unwrap();
                //println!("file position   : {}",pos);
                //println!("compressed size : {}", compressed_block_info_size );
                //println!("decompressed size : {}", decompressed_block_info_size );
                file.as_mut_ref().read_exact(&mut compressed_buf)?;
                //println!("compressed block buf : {:X?}", buf);
            }
        }

        // decompress block infos
        let mut block_info_cursor = BinaryReader::new(
            Cursor::new(decompress_chunk(&compressed_buf, header.decompressed_block_info_size as i32, header.flags).unwrap()),
            Endian::Big
        );

        // read hash
        let hash: &mut[u8] = &mut [0u8; 16];
        block_info_cursor.as_mut_ref().read_exact(hash).unwrap();

        // read block info
        let block_count = block_info_cursor.int32();
        let mut block_infos: Vec<(i32, i32, u32)> = Vec::new();
        let mut total_block_decompress_size = 0;
        for _ in 0..block_count {
            let d_size = block_info_cursor.int32();
            let c_size = block_info_cursor.int32();
            let flags  = block_info_cursor.int16() as u32;
            //println!("d_size : {}, c_size : {}, flags : {}", d_size, c_size, flags);
            total_block_decompress_size += d_size;
            block_infos.push( (d_size, c_size, flags) );
        }

        // decompress asset data
        let mut raw_asset_cursor = Cursor::new(vec![0u8; total_block_decompress_size as usize]);
        for i in block_infos {
            let mut buf = vec![0u8; i.1 as usize];
            file.as_mut_ref().read_exact(&mut buf).unwrap();
            buf = decompress_chunk(buf.as_slice(), i.0, i.2).unwrap();
            std::io::copy(&mut buf.as_slice(), &mut raw_asset_cursor).unwrap();
        }
        let raw_asset_buf = raw_asset_cursor.into_inner();

        //println!("asset bundle header : {:?}", header);
        let asset_count = block_info_cursor.int32();
        let mut assets: Vec<Asset> = Vec::new();
        for _ in 0..asset_count {
            let offset = block_info_cursor.uint64() as usize;
            let size   = block_info_cursor.uint64() as usize;
            let status = block_info_cursor.uint32();
            let name   = block_info_cursor.cstr();
            let data   = &raw_asset_buf[offset .. offset + size];
            let asset = Asset::read(&name, status, &data).unwrap();
            assets.push(asset);
        }
        Ok(AssetBundleImpl{
            header: header,
            assets: assets,
        })
    }
}

impl AssetBundleHeader {
    pub fn read<T: Read + Seek> (file: &mut BinaryReader<T>) -> Result<AssetBundleHeader>{
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