use std::path::PathBuf;
use std::sync::Arc;
use std::fs::File;
use std::io::{BufReader, Read, Write, Seek, SeekFrom, Cursor};
use log::{info, trace, warn};
use serde::{Serialize, Deserialize};

use crate::decompress::decompress_chunk;
use crate::asset::Asset;
use crate::binary_reader::BinaryReader;
use crate::endian::Endian;
use crate::Result;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AssetBundle{
    signiture: String,                  //Unityアセットバンドル定型文
    file_version: u32,                  //ファイルフォーマットバージョン
    lower_player_version: String,       //対応するUnityバージョン下限
    upper_player_version: String,       //ビルドしたUnityバージョン
    total_file_size: i64,               //アセットバンドルサイズ
    compressed_block_info_size: u32,    //圧縮後管理情報サイズ
    decompressed_block_info_size: u32,  //解凍後管理情報サイズ
    flags: u32,                         //フラグ群
    assets: Vec<Asset>,                 //各アセット情報
}

impl AssetBundle {
    /// 指定パスよりアセットバンドル情報を抽出します
    pub fn load(src: &PathBuf) -> Result<AssetBundle> {
        let mut file = BinaryReader::new(BufReader::new(File::open(src).unwrap()), Endian::Big);

        let signiture = file.cstr();
        match &*signiture {
            "UnityFS" => AssetBundle::read_asset_bundle(&mut file),
            _         => Err(format!("invalid signature").into())
        }
    }

    /// アセットバンドル情報抽出
    fn read_asset_bundle<T: Read + Seek>(file: &mut BinaryReader<T>) -> Result<AssetBundle>{
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
        let compressed_block_info_size = file.uint32();

        //4byte big-endian
        let decompressed_block_info_size = file.uint32();

        //4byte big-endian
        let flags = file.uint32();

        //read block infos
        let mut compressed_buf = vec![0u8; compressed_block_info_size as usize];
        match flags & 0x80 != 0 {
            true => {
                let pos = file.as_mut_ref().seek(SeekFrom::Current(0))?;
                file.as_mut_ref().seek(SeekFrom::End(compressed_block_info_size as i64))?;
                file.as_mut_ref().read_exact(&mut compressed_buf)?;
                file.as_mut_ref().seek(SeekFrom::Start(pos))?;
            },
            false => {
                if file_version >= 7 {
                    let pos = file.as_mut_ref().seek(SeekFrom::Current(0))?;
                    file.as_mut_ref().seek(SeekFrom::Current(pos as i64 & 16))?;
                }
                file.as_mut_ref().read_exact(&mut compressed_buf)?;
            }
        }

        // decompress block infos
        let mut block_info_cursor = BinaryReader::new(
            Cursor::new(decompress_chunk(&compressed_buf, decompressed_block_info_size as i32, flags).unwrap()),
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
            //info!("d_size : {}, c_size : {}, flags : {}", d_size, c_size, flags);
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

        // 各アセット情報抽出
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
        Ok(AssetBundle{
            signiture: String::from("UnityFS"),
            file_version: file_version,
            lower_player_version: lower_player_version,
            upper_player_version: upper_player_version,
            total_file_size: total_file_size,
            compressed_block_info_size: compressed_block_info_size,
            decompressed_block_info_size: decompressed_block_info_size,
            flags: flags,
            assets: assets,
        })
    }
}
