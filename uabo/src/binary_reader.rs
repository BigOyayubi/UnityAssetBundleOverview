use std::io::{Read,Seek,SeekFrom};
use crate::endian::Endian;

macro_rules! read_primitive {
    ($reader:expr, $T:tt, $size:tt, $endian:expr) => {
       {
           let mut buf: [u8;$size] = [0;$size];
           $reader.read(&mut buf).unwrap();
           match $endian {
               Endian::Big => $T::from_be_bytes(buf),
               Endian::Little => $T::from_le_bytes(buf),
           }
       } 
    };
}

pub struct BinaryReader<T>{
    io: T,
    endian: Endian,
}

impl<T: Read+Seek> BinaryReader<T>{
    pub fn new(io: T, endian: Endian) -> BinaryReader<T> {
        BinaryReader{
            io: io,
            endian: endian
        }
    }
    pub fn as_mut_ref(&mut self) -> &mut T{ &mut self.io }
    pub fn set_endian(&mut self, endian: Endian) {
        self.endian = endian;
    }
    pub fn int16(&mut self) -> i16 {
        read_primitive!(self.as_mut_ref(), i16, 2, self.endian)
    }
    pub fn int32(&mut self) -> i32{
        read_primitive!(self.as_mut_ref(), i32, 4, self.endian)
    }
    pub fn int64(&mut self) -> i64 {
        read_primitive!(self.as_mut_ref(), i64, 8, self.endian)
    }
    pub fn uint8(&mut self) -> u8 {
        read_primitive!(self.as_mut_ref(), u8, 1, self.endian)
    }
    pub fn uint16(&mut self) -> u16 {
        read_primitive!( self.as_mut_ref(), u16, 2, self.endian)
    }
    pub fn uint32(&mut self) -> u32 {
        read_primitive!( self.as_mut_ref(), u32, 4, self.endian)
    }
    pub fn uint64(&mut self) -> u64 {
        read_primitive!( self.as_mut_ref(), u64, 8, self.endian)
    }
    pub fn boolean(&mut self) -> bool {
        self.uint8() != 0
    }
    pub fn cstr(&mut self) -> String {
        let mut buf = Vec::new();
        let mut tmp = [0u8; 1];
        loop {
            let readed = self.as_mut_ref().read(&mut tmp).unwrap();
            if readed == 0 { break; }
            if tmp[0] == 0 { break; }
            buf.push(tmp[0]);
        }
        String::from_utf8(buf).expect("can not read signature.")
    }

    pub fn pos(&mut self) -> u64 {
        self.as_mut_ref().seek(SeekFrom::Current(0)).unwrap()
    }

    pub fn indexed_cstr(&mut self, idx: u64 ) -> String {
        let pos = self.pos();
        self.as_mut_ref().seek(SeekFrom::Start(idx)).unwrap();
        let s = self.cstr();
        self.as_mut_ref().seek(SeekFrom::Start(pos)).unwrap();
        s
    }

    pub fn skip<I: num_traits::PrimInt>(&mut self, val: I) {
        match val.to_i64() {
            Some(offset) => self.as_mut_ref().seek(SeekFrom::Current(offset)).unwrap(),
            None => { 0 },
        };
    }
    pub fn align(&mut self, val: u64) {
        let pos = self.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
        let offset = pos % val;
        if offset > 0 {
            self.as_mut_ref().seek(SeekFrom::Current((val - offset) as i64)).unwrap();
        }
    }
    pub fn read(&mut self, val: usize) -> Vec<u8> {
        let mut buf = vec![0u8; val];
        self.as_mut_ref().read_exact(&mut buf).unwrap();
        buf
    }
}
