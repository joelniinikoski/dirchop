//! This module creates tar files and splits them

pub use std::fs::{File, create_dir, read_dir, DirEntry};
use std::io::{Read, BufWriter, Write};
pub use std::path::Path;

pub fn into_tar(path: &str, chunksize: usize) {
    let tarfile = File::create("test.tar").unwrap();
    let mut builder = tar::Builder::new(tarfile);
    builder.append_path_with_name(path, path).unwrap();
    builder.finish().unwrap();

    let mut splitfile = File::open("test.tar").unwrap();
    split(&mut splitfile);
}

fn split(file: &mut File, chunksize: usize) {
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    let chunks = buf.len()/chunksize;
    let remainder = buf.len()-chunksize*chunks;
    
    for chunk in 0..chunks+1 {
        let mut writer = BufWriter::new(File::create(format!("chunk{}.tar",chunk)).unwrap());
        let end = if !(chunk == chunks) {chunksize} else {remainder};
        match writer.write_all(&buf[chunk*chunksize..(chunk*chunksize+end)]) {
            Ok(_) => (),
            Err(_) => (),
        }

        writer.flush().unwrap();
    }
    
}

pub fn glue() {
    let mut chunk_amount = 0;
    let mut wholebuf = vec![];
    for result in read_dir("./").expect("Readdir") {
        match result {
            Ok(entry) => if entry.file_name().len() >= 6 && &entry.file_name().to_str().unwrap()[0..5] == "chunk" {
                chunk_amount += 1;
            },
            Err(_) => (),
        }
    }
    for chunk in 0..chunk_amount {
        let mut partbuf = vec![];
        let mut chunkfile = File::open(format!("chunk{}.tar",chunk)).unwrap();
        chunkfile.read_to_end(&mut partbuf).expect("Partbuf write chunk");
        wholebuf.write_all(&partbuf).expect("wholebuf write chunk");
    }
    let mut writer = BufWriter::new(File::create("whole.tar").unwrap());
    writer.write_all(&wholebuf).expect("Write whole file");
}

pub fn into_file() {
    let tarfile =  File::open("whole.tar").unwrap();
    let mut archive = tar::Archive::new(tarfile);
    if !Path::new("./ready").exists() {
        create_dir("./ready").unwrap();
    }
    archive.unpack("./ready").unwrap();
}
