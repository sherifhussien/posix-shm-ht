use clap::{Arg, Command, crate_version};


pub fn parse_args() -> bool {
    let matches = Command::new("client")
        .version(crate_version!())
        .about("to run either normal or test mode")
        .arg(
            Arg::new("test")
                .short('t')
                .long("test")
                .action(clap::ArgAction::SetTrue)
                .help("run test scripts")
        )
        .get_matches(); // parse env::args_os

        matches.get_flag("test")
}