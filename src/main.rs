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
            // and .required(true) in cli
            let kilobytes = sub_matches.get_one::<usize>(options::MEGABYTES).unwrap().to_owned();
            if kilobytes == 0 {
                return Err(Box::new(Error::new(ErrorKind::InvalidInput, "Megabytes must be larger than 0")));
            }
            // panic should never happen on unwrap: TARGET exists
            // before argmatching because .required(true) in cli
            if let Err(e) = 
                into_tar(sub_matches.get_one::<String>(options::TARGET).unwrap().as_str(), kilobytes) {
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
mod tests;