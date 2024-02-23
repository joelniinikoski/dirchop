//! Program entry point. 

mod cli;
mod chopglue;

use chopglue::*;
use cli::*;

fn main() -> Result<(), std::io::Error>{
    let command = cli_command();
    let matches = command.get_matches();

    match matches.subcommand() {
        Some(("chop", sub_matches)) => {
            let megabytes = match sub_matches.get_one::<usize>("MEGABYTES") {
                Some(t) => t.to_owned(),
                _ => 0,
            };
            match sub_matches.get_one::<String>("TARGET") {
                Some(t) => into_tar(t.as_str(), megabytes),
                _ => (),
            }
        },
        Some(("glue", sub_matches)) => {
            glue(sub_matches.get_flag("DELETE_CHUNKS"));
            into_file();
        }
        _ => (),
    };
    Ok(())
}

#[cfg(test)]
mod tests;