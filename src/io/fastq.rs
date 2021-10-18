use std::io::Error;

pub trait FastqRead {
    fn read(&mut self, record: &mut FastqRecord) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct FastqRecord {
    name: String,
    seq: String,
    qual: String
}
impl FastqRecord {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            seq: String::new(),
            qual: String::new()
        }
    }
    pub fn assign_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    pub fn assign_seq(&mut self, seq: &str) {
        self.seq = seq.to_string();
    }
    pub fn assign_qual(&mut self, qual: &str) {
        self.qual = qual.to_string();
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn seq(&self) -> &str {
        &self.seq
    }
    pub fn qual(&self) -> &str {
        &self.qual
    }
    pub fn clear(&mut self) {
        self.name.clear();
        self.seq.clear();
        self.qual.clear();
    }
    pub fn is_empty(&self) -> bool {
        self.name.is_empty() & 
            self.seq.is_empty() &
            self.qual.is_empty()
    }
}
