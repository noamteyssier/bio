use std::io::Error;
use super::{FastqRead, Reader, FastqRecord, Buf, BufGz};


pub struct Records <R>{
    reader: R
}
impl <R: FastqRead> Records <R> {
    pub fn new(reader: R) -> Records<R> {
        Self{ reader }
    }
}
impl Records<Reader<BufGz>> {
    pub fn from_gzip(filename: &str) -> Self {
        let reader = Reader::from_gzip_file(filename);
        Self{ reader }
    }
}
impl Records<Reader<Buf>> {
    pub fn from_file(filename: &str) -> Self {
        let reader = Reader::from_file(filename);
        Self{ reader }
    }
}
impl <B> Iterator for Records <B>
where
    B: FastqRead,
{
    type Item = Result<FastqRecord, Error>;

    fn next(&mut self) -> Option<Result<FastqRecord, Error>> {
        let mut record = FastqRecord::new();
        match self.reader.read(&mut record) {
            Ok(_) if record.is_empty() => None,
            Ok(_) => Some(Ok(record)),
            Err(err) => Some(Err(err))
        }
    }
}
