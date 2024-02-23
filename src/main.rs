//! Program entry point. 

mod cli;
mod chopglue;

use chopglue::*;
use cli::*;

fn main() -> Result<(), std::io::Error>{
    let mut command = cli_command();
    let matches = command.get_matches();

    match matches.subcommand() {
        Some(("chop", sub_matches)) => {
            let megabytes = match sub_matches.get_one::<usize>("MEGABYTES") {
                Some(t) => t.to_owned(),
                _ => 0,
            };
            match sub_matches.get_one::<String>("TARGET") {
                Some(t) => into_tar(t.as_str()),
                _ => (),
            }
        },
        Some(("glue", sub_matches)) => {
            // let delchunks = match sub_matches.get_one::<String>("DELETE_CHUNKS") {
            //     Some(t) => true,
            //     _ => false,
            // };
            glue();
            into_file();
        }
        _ => (),
    };
    Ok(())
}

#[cfg(test)]
mod tests;