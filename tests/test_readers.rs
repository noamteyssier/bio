use bio::io::{Reader, Records};

#[test]
fn test_fastq_reader() {
    let reader = Reader::from_file("data/test.fastq");
    let records = Records::new(reader);

    let mut count = 0;
    for rec in records {
        assert!(!rec.unwrap().is_empty());
        count += 1;
    }
    assert_eq!(count, 2500);
}

#[test]
fn test_fastq_gz_reader() {
    let reader = Reader::from_gzip_file("data/test.fastq.gz");
    let records = Records::new(reader);

    let mut count = 0;
    for rec in records {
        assert!(!rec.unwrap().is_empty());
        count += 1;
    }
    assert_eq!(count, 2500);
}

#[test]
fn test_fastq_records() {
    let records = Records::from_file("data/test.fastq");

    let mut count = 0;
    for rec in records {
        assert!(!rec.unwrap().is_empty());
        count += 1;
    }
    assert_eq!(count, 2500);
}

#[test]
fn test_fastq_gz_records() {
    let records = Records::from_gzip("data/test.fastq.gz");

    let mut count = 0;
    for rec in records {
        assert!(!rec.unwrap().is_empty());
        count += 1;
    }
    assert_eq!(count, 2500);
}
