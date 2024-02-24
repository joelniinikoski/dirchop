//! Program entry point. 

mod cli;
mod chopglue;

use chopglue::*;
use cli::*;
use std::io::{Error, ErrorKind};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let command = cli_command();
    let matches = command.get_matches();

    match matches.subcommand() {
        Some(("chop", sub_matches)) => {
            // panic should never happen as usize is parsed before argmatching
            let megabytes = sub_matches.get_one::<usize>("MEGABYTES").unwrap().to_owned();
            if megabytes == 0 {
                return Err(Box::new(Error::new(ErrorKind::InvalidInput, "MEGABYTES must be larger than 0")));
            }
            // panic should never happen on unwrap: TARGET exists
            // before argmatching because .required(true) in cli
            into_tar(sub_matches.get_one::<String>("TARGET").unwrap().as_str(), megabytes)?;
        },
        Some(("glue", sub_matches)) => {
            glue(sub_matches.get_flag("DELETE_CHUNKS"))?;
            into_file()?;
        }
        _ => (),
    };
    Ok(())
}

#[cfg(test)]
mod tests;