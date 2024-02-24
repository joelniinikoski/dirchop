//! This module creates tar files and splits them

pub use std::fs::{File, create_dir, read_dir};
use std::error::Error;
use std::io::{Read, BufWriter, Write};
pub use std::path::Path;

pub fn into_tar(path: &str, kilobytes: usize) -> Result<(), Box<dyn Error>> {
    // temporary archive that is used for splitting into chunks, then deleted.
    let temp_path = "dirchop_temparchive.tar";
    let tarfile = File::create(temp_path)?;
    let mut builder = tar::Builder::new(tarfile);
    match std::fs::metadata(path)?.is_dir() {
        true => builder.append_dir_all(path, path)?,
        false => builder.append_path_with_name(path, path)?,
    }
    builder.finish()?;

    // builder.finish() closes file; open temparchive again for splitting
    let mut splitfile = File::open(temp_path)?;
    let chunksize = kilobytes * 1000;
    split(&mut splitfile, chunksize)?;
    std::fs::remove_file(temp_path)?;
    Ok(())
}

fn split(file: &mut File, chunksize: usize) -> Result<(), Box<dyn Error>> {
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let chunks = buf.len()/chunksize;
    let remainder = buf.len()-chunksize*chunks;
    
    // create chunks from temparchive
    for chunk in 0..chunks+1 {
        let mut writer = BufWriter::new(File::create(format!("dirchop_chunk{}.tar",chunk))?);
        let end = if !(chunk == chunks) {chunksize} else {remainder};
        match writer.write_all(&buf[chunk*chunksize..(chunk*chunksize+end)]) {
            Ok(_) => (),
            Err(_) => (),
        }

        writer.flush()?;
    }
    Ok(())
}

pub fn glue(d: bool) -> Result<(), Box<dyn Error>> {
    let mut chunk_amount = 0;
    let mut wholebuf = vec![];
    for result in read_dir("./")? {
        match result {
            Ok(entry) => if entry.file_name().len() >= 14 && &entry.file_name().to_str().unwrap()[0..13] == "dirchop_chunk" {
                // we don't process chunks here because we want to retain their order
                chunk_amount += 1;
            },
            Err(_) => (),
        }
    }
    // combine chunks into wholebuffer
    for chunk in 0..chunk_amount {
        let mut partbuf = vec![];
        let chunkstr = format!("dirchop_chunk{}.tar",chunk);
        let mut chunkfile = File::open(&chunkstr)?;
        chunkfile.read_to_end(&mut partbuf)?;
        wholebuf.write_all(&partbuf)?;
        if d {
            std::fs::remove_file(&chunkstr)?;
        }
    }
    // write wholebuffer into finished tar file
    let mut writer = BufWriter::new(File::create("dirchop_tempfinished.tar")?);
    writer.write_all(&wholebuf)?;
    into_file()?;
    Ok(())
}

pub fn into_file() -> Result<(), Box<dyn Error>> {
    // create temporary archive object from finished tar file
    let tempfinishedpath = "dirchop_tempfinished.tar";
    let tarfile =  File::open(&tempfinishedpath)?;
    let mut archive = tar::Archive::new(tarfile);
    if !Path::new("./dirchop_target").exists() {
        create_dir("./dirchop_target")?;
    }
    // unpack into target
    archive.unpack("./dirchop_target")?;
    std::fs::remove_file(&tempfinishedpath)?;
    Ok(())
}

pub fn check_temp_tar(path: &str, e: Box<dyn Error>) -> Result<(), Box<dyn Error>> {
    if Path::new(path).exists() {
        std::fs::remove_file(path)?;
    }
    Err(e)
}
