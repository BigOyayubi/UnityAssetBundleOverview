use std::sync::Arc;
use std::io::SeekFrom;
use std::io::Cursor;
use std::io::{Read, Write};
use crate::decompress::decompress_chunk;
use crate::read_ext::ReadPrimitive;
use crate::Result;

#[derive(Clone, Debug)]
pub struct Asset(Arc<AssetImpl>);

#[derive(Clone, Debug)]
struct AssetImpl {
    name: String,
    data: Vec<u8>,
    status: u32,
}

impl Asset {
    pub fn read<T: std::io::Read + std::io::Seek>(file: &mut T, version: u32, is_block_info_at_end: bool, compressed_block_info_size: u32, decompressed_block_info_size: u32, flags: u32) -> Result<Vec<Asset>>{
        // read block infos
        let mut buf = vec![0u8; compressed_block_info_size as usize];
        match is_block_info_at_end {
            true => {
                let pos = file.seek(SeekFrom::Current(0))?;
                file.seek(SeekFrom::End(compressed_block_info_size as i64))?;
                file.read_exact(&mut buf)?;
                file.seek(SeekFrom::Start(pos))?;
            },
            false => {
                if version >= 7 {
                    let pos = file.seek(SeekFrom::Current(0))?;
                    file.seek(SeekFrom::Current(pos as i64 & 16))?;
                }
                let pos = file.seek(SeekFrom::Current(0)).unwrap();
                //println!("file position   : {}",pos);
                //println!("compressed size : {}", compressed_block_info_size );
                //println!("decompressed size : {}", decompressed_block_info_size );
                file.read_exact(&mut buf)?;
                //println!("compressed block buf : {:X?}", buf);
            }
        }
        // decompress block infos
        let mut block_info_cursor = Cursor::new(decompress_chunk(&buf, decompressed_block_info_size as i32, flags).unwrap());

        // read hash
        let hash: &mut[u8] = &mut [0u8; 16];
        block_info_cursor.read_exact(hash).unwrap();

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
            file.read_exact(&mut buf).unwrap();
            buf = decompress_chunk(buf.as_slice(), i.0, i.2).unwrap();
            std::io::copy(&mut buf.as_slice(), &mut raw_asset_cursor).unwrap();
        }

        // read assets
        let asset_count = block_info_cursor.uint32();
        let mut assets: Vec<Asset> = Vec::new();
        for _ in 0..asset_count {
            let offset = block_info_cursor.uint64() as usize;
            let size   = block_info_cursor.uint64() as usize;
            let status = block_info_cursor.uint32();
            let name   = block_info_cursor.cstr();
            //TODO split buf from raw_asset_cursor
            //println!("offset : {}, size : {}, status : {}, name : {}", offset, size, status, name);
            let asset = Asset(Arc::new(AssetImpl{
                name: name,
                data: buf.to_vec(), //TODO
                status: status,
            }));
            assets.push(asset);
        }
        Ok(assets)
    }
}