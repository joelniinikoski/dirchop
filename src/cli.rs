pub use clap::{ArgAction, Command, command, Arg};

//Custom error handling not required; clap handles commands
pub fn cli_command() -> Command {
    command!()
    .about("This application can split a file or directory into smaller tar-pieces and later combine pieces made by the program into the original file or directory.")
    .subcommand_required(true)
    .subcommand(
        Command::new("chop")
        .about("Create tar file of target and split it into chunks of specified size")
        .arg_required_else_help(true)
        .arg(
            Arg::new("TARGET")
            .help("File or directory to chop")
            .required(true)
        ).arg(
            Arg::new("KILOBYTES")
            .help("Kilobytes per individual tar file")
            .required(true)
            .value_parser(clap::value_parser!(usize))
        )
    )
    .subcommand(
        Command::new("glue")
        .about("Glue chunk-files into their source")
        .arg(
            Arg::new("CLEAN_CHUNKS")
            .help("Delete chunks after succesful glue")
            .short('c')
            .long("clean")
            .action(
                ArgAction::SetTrue
            )
        )
    )
}