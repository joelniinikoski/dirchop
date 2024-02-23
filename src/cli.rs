pub use clap::{error::ErrorKind, Command, command, Arg, ArgMatches};

pub fn cli_command() -> Command {
    command!()
    .about("This application can split a file or directory into smaller tar-pieces and later combine pieces made by the program into the original file or directory.")
    .subcommand_required(true)
    .subcommand(
        Command::new("chop")
        .about("Create tar file of target and split it into files of specified size")
        .arg_required_else_help(true)
        .arg(
            Arg::new("TARGET")
            .help("File or directory to chop")
            .required(true)
        ).arg(
            Arg::new("MEGABYTES")
            .help("Megabytes per individual tar file")
            .required(true)
            .value_parser(clap::value_parser!(usize))
        )
    )
    .subcommand(
        Command::new("glue")
        .about("Glue created chunk-files into the original file")
        // .arg(
        //     Arg::new("DELETE_CHUNKS")
        //     .help("Option to delete tar chunks after glue")
        //     .short('d')
        // )
    )
}

// pub fn chop_matches(matches: &ArgMatches) -> (String, u64) {

// }