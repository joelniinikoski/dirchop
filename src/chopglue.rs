//! This module creates tar files and splits them

pub use std::fs::{File, create_dir, read_dir};
use std::io::{Read, BufWriter, Write};
pub use std::path::Path;

pub fn into_tar(path: &str, chunksize: usize) {
    let temp_path = "dirchop_temparchive.tar";
    let tarfile = File::create(temp_path).unwrap();
    let mut builder = tar::Builder::new(tarfile);
    builder.append_path_with_name(path, path).unwrap();
    builder.finish().unwrap();

    let mut splitfile = File::open(temp_path).unwrap();
    let chunksize = chunksize * 1000;
    split(&mut splitfile, chunksize);
    std::fs::remove_file(temp_path).expect("Error deleting temporary file.");
}

fn split(file: &mut File, chunksize: usize) {
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    let chunks = buf.len()/chunksize;
    let remainder = buf.len()-chunksize*chunks;
    
    for chunk in 0..chunks+1 {
        let mut writer = BufWriter::new(File::create(format!("dirchop_chunk{}.tar",chunk)).unwrap());
        let end = if !(chunk == chunks) {chunksize} else {remainder};
        match writer.write_all(&buf[chunk*chunksize..(chunk*chunksize+end)]) {
            Ok(_) => (),
            Err(_) => (),
        }

        writer.flush().unwrap();
    }
    
}

pub fn glue(d: bool) {
    let mut chunk_amount = 0;
    let mut wholebuf = vec![];
    for result in read_dir("./").expect("Error in readdir") {
        match result {
            Ok(entry) => if entry.file_name().len() >= 14 && &entry.file_name().to_str().unwrap()[0..13] == "dirchop_chunk" {
                chunk_amount += 1;
            },
            Err(_) => (),
        }
    }
    for chunk in 0..chunk_amount {
        let mut partbuf = vec![];
        let chunkstr = format!("dirchop_chunk{}.tar",chunk);
        let mut chunkfile = File::open(&chunkstr).unwrap();
        chunkfile.read_to_end(&mut partbuf).expect("Partbuf write chunk");
        wholebuf.write_all(&partbuf).expect("wholebuf write chunk");
        if d {
            std::fs::remove_file(&chunkstr).expect("Error deleting chunk");
        }
    }
    let mut writer = BufWriter::new(File::create("dirchop_tempfinished.tar").unwrap());
    writer.write_all(&wholebuf).expect("Write whole file");
}

pub fn into_file() {
    let tempfinishedpath = "dirchop_tempfinished.tar";
    let tarfile =  File::open(&tempfinishedpath).unwrap();
    let mut archive = tar::Archive::new(tarfile);
    if !Path::new("./dirchop_target").exists() {
        create_dir("./dirchop_target").unwrap();
    }
    archive.unpack("./dirchop_target").unwrap();
    std::fs::remove_file(&tempfinishedpath).unwrap();
}
