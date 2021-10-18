use std::fs::File;
use bio::io::{Reader, Records};
use flate2::read::MultiGzDecoder;

#[test]
fn test_fastq() {
    let file = File::open("data/test.fastq").expect("Error");
    let fq_reader = Reader::from_file(file);
    let records = Records::new(fq_reader);

    let mut count = 0;
    for rec in records {
        assert!(!rec.unwrap().is_empty());
        count += 1;
    }
    assert_eq!(count, 2500);
}

#[test]
fn test_fastq_gz() {
    let file = File::open("data/test.fastq.gz").expect("Error");
    let gzfile = MultiGzDecoder::new(file);
    let fq_reader = Reader::from_file(gzfile);
    let records = Records::new(fq_reader);

    let mut count = 0;
    for rec in records {
        assert!(!rec.unwrap().is_empty());
        count += 1;
    }
    assert_eq!(count, 2500);
}
