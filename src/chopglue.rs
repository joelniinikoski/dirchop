//! This module creates tar files and splits them

pub use std::fs::{File, create_dir, read_dir};
use std::io::{Read, BufWriter, Write};
pub use std::path::Path;
use anyhow::{Result, bail};

pub mod paths {
    pub static TEMP: &str = "dirchop-temparchive.tar";
    pub static TEMP_FIN: &str = "dirchop-tempfinished.tar";
    pub static CHUNK: &str = "dirchop-chunk";
    pub static DIRCHOP_TARGET: &str = "./dirchop-target";
}

pub fn chop(path: &str, megabytes: usize) -> Result<()> {
    // temporary archive that is used for splitting into chunks, then deleted.
    let tarfile = File::create(paths::TEMP)?;
    let mut builder = tar::Builder::new(tarfile);
    match std::fs::metadata(path)?.is_dir() {
        true => builder.append_dir_all(path, path)?,
        false => builder.append_path_with_name(path, path)?
    }
    builder.finish()?;

    // builder.finish() closes file; open temparchive again for splitting
    let mut splitfile = File::open(paths::TEMP)?;
    let chunksize = match megabytes.checked_mul(1000000) {
        Some(n) => n,
        None => bail!(format!("<{}>: number too large",crate::cli::options::MEGABYTES))
    };
    split(&mut splitfile, chunksize)?;
    std::fs::remove_file(paths::TEMP)?;
    Ok(())
}

fn split(file: &mut File, chunksize: usize) -> Result<()> {
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let chunks = buf.len()/chunksize;
    let remainder = buf.len()-chunksize*chunks;
    
    // create chunks from temparchive
    for chunk in 0..chunks+1 {
        let mut writer = BufWriter::new(
            File::create(format!("{}{}.tar",paths::CHUNK,chunk))?);
        let end = if !(chunk == chunks) {chunksize} else {remainder};
        match writer.write_all(&buf[chunk*chunksize..(chunk*chunksize+end)]) {
            Ok(_) => (),
            Err(_) => (),
        }

        writer.flush()?;
    }
    Ok(())
}

pub fn glue(d: bool) -> Result<()> {
    let mut chunk_amount = 0;
    let mut wholebuf = vec![];
    for result in read_dir("./")? {
        match result {
            Ok(entry) => if entry.file_name().len() >= 14 
            && &entry.file_name().to_str().unwrap()[0..13] == paths::CHUNK {
                // we don't process chunks here because we want to retain their order
                chunk_amount += 1;
            },
            Err(_) => (),
        }
    }
    // combine chunks into wholebuffer
    for chunk in 0..chunk_amount {
        let mut partbuf = vec![];
        let chunkstr = format!("{}{}.tar",paths::CHUNK,chunk);
        let mut chunkfile = File::open(&chunkstr)?;
        chunkfile.read_to_end(&mut partbuf)?;
        wholebuf.write_all(&partbuf)?;
        if d {
            std::fs::remove_file(&chunkstr)?;
        }
    }
    // write wholebuffer into finished tar file
    let mut writer = BufWriter::new(File::create(paths::TEMP_FIN)?);
    writer.write_all(&wholebuf)?;
    writer.flush()?;
    into_file()?;
    Ok(())
}

fn into_file() -> Result<()> {
    // create temporary archive object from finished tar file
    let tarfile =  File::open(paths::TEMP_FIN)?;
    let mut archive = tar::Archive::new(tarfile);
    if !Path::new(paths::DIRCHOP_TARGET).exists() {
        create_dir(paths::DIRCHOP_TARGET)?;
    }
    // unpack into target
    archive.unpack(paths::DIRCHOP_TARGET)?;
    std::fs::remove_file(paths::TEMP_FIN)?;
    Ok(())
}

pub fn check_temp_tar(path: &str, e: anyhow::Error) -> Result<()> {
    if Path::new(path).exists() {
        std::fs::remove_file(path)?;
    }
    Err(e)
}
