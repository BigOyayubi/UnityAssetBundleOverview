macro_rules! read_primitive {
    ($reader:ident, $T:tt, $size:tt) => {
       {
           let mut buf: [u8;$size] = [0;$size];
           $reader.read(&mut buf).unwrap();
           $T::from_be_bytes(buf)
       } 
    };
}

pub trait ReadPrimitive {
    fn int16(&mut self) -> i16;
    fn int32(&mut self) -> i32;
    fn int64(&mut self) -> i64;
    fn uint16(&mut self) -> u16;
    fn uint32(&mut self) -> u32;
    fn uint64(&mut self) -> u64;
    fn cstr(&mut self) -> String;
}

impl<T: std::io::Read> ReadPrimitive for T {
    fn int16(&mut self) -> i16 { read_primitive!(self, i16, 2) }
    fn int32(&mut self) -> i32 { read_primitive!(self, i32, 4) }
    fn int64(&mut self) -> i64 { read_primitive!(self, i64, 8) }
    fn uint16(&mut self) -> u16 { read_primitive!(self, u16, 2) }
    fn uint32(&mut self) -> u32 { read_primitive!(self, u32, 4) }
    fn uint64(&mut self) -> u64 { read_primitive!(self, u64, 8) }
    fn cstr(&mut self) -> String {
        let mut buf = Vec::new();
        let mut tmp = [0u8; 1];
        loop {
            let readed = self.read(&mut tmp).unwrap();
            if readed == 0 { break; }
            if tmp[0] == 0 { break; }
            buf.push(tmp[0]);
        }
        String::from_utf8(buf).expect("can not read signature.")
    }
}