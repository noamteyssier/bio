mod fastq;
mod record;
mod reader;

pub use reader::Reader;
pub use fastq::{FastqRead, FastqRecord};
pub use record::Records;
