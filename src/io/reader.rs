use std::{fs::File, io::{self, BufReader}};
use flate2::read::MultiGzDecoder;
use super::{FastqRecord, FastqRead};

pub type BufGz = io::BufReader<MultiGzDecoder<File>>;
pub type Buf = io::BufReader<File>;

pub struct Reader <B> {
    buffer: B,
    line: String
}
impl <R> Reader <io::BufReader<R>> 
where
    R: io::Read
{
    pub fn from_reader(reader: R) -> Self {
        Self {
            buffer: BufReader::new(reader),
            line: String::new()
        }
    }
}
impl Reader<Buf> {
    pub fn from_file(filename: &str) -> Self {
        let file = File::open(filename).expect("");
        Reader::from_reader(file)
    }
}
impl Reader<BufGz> {
    pub fn from_gzip_file(filename: &str) -> Self {
        let file = File::open(filename).expect("");
        let gfile = MultiGzDecoder::new(file);
        Reader::from_reader(gfile)
    }
}

impl <B> FastqRead for Reader<B>
where
    B: io::BufRead
{
    fn read(&mut self, record: &mut FastqRecord) -> Result<(), io::Error> {
        record.clear();
        
        for i in 0..4 {
            self.line.clear();
            if self.buffer.read_line(&mut self.line)? > 0 {
                match i {
                    0 => {
                        assert!(self.line.starts_with('@'));
                        record.assign_name(self.line.trim_matches('@').trim_end());
                    },
                    1 => {
                        record.assign_seq(self.line.trim_end());
                    }
                    3 => {
                        record.assign_qual(self.line.trim_end());
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }
}
