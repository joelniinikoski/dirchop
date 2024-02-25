//! Program entry point. 

use anyhow::{Result, bail};

mod cli;
mod chopglue;

use cli::*;
use chopglue::*;

fn main() -> Result<()> {
    let command = cli_command();
    let matches = command.get_matches();

    match matches.subcommand() {
        Some(("chop", sub_matches)) => {
            // panic should never happen as usize is parsed before argmatching
            // and .required(true) in cli
            let megabytes = sub_matches.get_one::<usize>(options::MEGABYTES).unwrap().to_owned();
            if megabytes == 0 {
                bail!(format!("<{}>: must be greater than zero", options::MEGABYTES))
            }
            // panic should never happen on unwrap: TARGET exists
            // before argmatching because .required(true) in cli
            if let Err(e) = 
                chop(sub_matches.get_one::<String>(options::TARGET).unwrap().as_str(), 
                megabytes) {
                check_temp_tar(paths::TEMP, e)?
            }
        },
        Some(("glue", sub_matches)) => {
            if let Err(e) = glue(sub_matches.get_flag(options::CLEAN_CHUNKS)) {
                check_temp_tar(paths::TEMP_FIN, e)?
            }
        }
        _ => (),
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    const TESTFILE: &str = "integrationtestfile.txt";
    use crate::*;
    use std::io::Read;
    #[test]
    fn test_chop_then_glue() {
        let path_to_result = format!("{}/{}", paths::DIRCHOP_TARGET, TESTFILE);
        {
            let mut startbuf = vec![];
            // read file into startbuf, for comparison later
            File::open(TESTFILE).unwrap().read_to_end(&mut startbuf).unwrap();
            // using 10GB max chunks as integrationtestfile.txt is very likely never bigger
            if let Err(e) = chopglue::chop(TESTFILE, 10000) {
                check_temp_tar(chopglue::paths::TEMP, e).unwrap()
            }
            //verify that a chunk has been created
            let chunkpath = format!("{}0.tar",chopglue::paths::CHUNK);
            assert!(Path::new(&chunkpath).exists());

            if let Err(e) = chopglue::glue(true) {
                check_temp_tar(chopglue::paths::TEMP, e).unwrap()
            }

            let mut endbuf = vec![];
            File::open(&path_to_result).unwrap().read_to_end(&mut endbuf).unwrap();
            assert_eq!(startbuf, endbuf);
        }
        std::fs::remove_file(&path_to_result).unwrap();
        std::fs::remove_dir(chopglue::paths::DIRCHOP_TARGET).unwrap();
    }
}