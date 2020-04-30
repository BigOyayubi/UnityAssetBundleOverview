use crate::Result;

pub fn decompress_chunk(src: &[u8], dst_size: i32, flags: u32) -> Result<Vec<u8>> {
    match flags & 0x3F {
        0   => Ok(src.to_vec()),
        1   => Ok(lzma::decompress(&src).unwrap()),
        2|3 => Ok(lz4::block::decompress(&src, Some(dst_size)).unwrap()),
        _   => Err(format!("invalid flag : {}", flags).into())
    }
}