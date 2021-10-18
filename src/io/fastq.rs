use std::io::{self, BufRead, Error};
use std::io::BufReader;

use super::FastqRecord;

pub trait FastqRead {
    fn read(&mut self, record: &mut FastqRecord) -> Result<(), Error>;
}

pub struct Reader <B> {
    reader: B,
    line_buffer: String
}
impl <R: io::Read> Reader <io::BufReader<R>> {
    pub fn from_file(reader: R) -> Self {
        Self {
            reader: BufReader::new(reader),
            line_buffer: String::new()
        }
    }
}
impl<R: io::Read> FastqRead for Reader<io::BufReader<R>> {
    fn read(&mut self, record: &mut FastqRecord) -> Result<(), Error> {
        record.clear();
        
        for i in 0..4 {
            self.line_buffer.clear();
            if self.reader.read_line(&mut self.line_buffer)? > 0 {
                match i {
                    0 => {
                        assert!(self.line_buffer.starts_with('@'));
                        record.assign_name(self.line_buffer.trim_matches('@').trim_end());
                    },
                    1 => {
                        record.assign_seq(self.line_buffer.trim_end());
                    }
                    3 => {
                        record.assign_qual(self.line_buffer.trim_end());
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }
}
