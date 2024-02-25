use clap::crate_version;
pub use clap::{ArgAction, Command, command, Arg};

pub mod commands {
    pub static CHOP: &str = "chop";
    pub static GLUE: &str = "glue";
}

pub mod options {
    pub static TARGET: &str = "target";
    pub static MEGABYTES: &str = "megabytes";
    pub static CLEAN_CHUNKS: &str = "clean-chunks";
}

//Custom error handling not required; clap handles commands
pub fn cli_command() -> Command {
    command!()
    .version(crate_version!())
    .about("This application splits a file or directory 
    into smaller tar-pieces, and later combines them 
    into the original file or directory.")
    .subcommand_required(true)
    .subcommand(
        Command::new(commands::CHOP)
        .about("Create tar file of target and split 
        it into chunks of specified size")
        .arg_required_else_help(true)
        .arg(
            Arg::new(options::TARGET)
            .help("File or directory to chop")
            .required(true)
        ).arg(
            Arg::new(options::MEGABYTES)
            .help("Megabytes per individual tar file")
            .required(true)
            .value_parser(clap::value_parser!(usize))
        )
    )
    .subcommand(
        Command::new(commands::GLUE)
        .about("Glue chunk-files into their source")
        .arg(
            Arg::new(options::CLEAN_CHUNKS)
            .help("Delete chunks after succesful glue")
            .short('c')
            .long("clean")
            .action(
                ArgAction::SetTrue
            )
        )
    )
}