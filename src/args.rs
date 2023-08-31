use clap::{Arg, ArgAction, Command};
use std::env;

pub struct Args {
    pub infile: String,
    pub outfile: String,
    pub silent: bool,
}

impl Args {
    pub fn parse() -> Self {
        let matches = Command::new("Pipe Viewer")
            .version("1.0.0")
            .arg(
                Arg::new("infile")
                    .short('i')
                    .long("infile")
                    .default_value("")
                    .help("Read from file instead of stdin"),
            )
            .arg(
                Arg::new("outfile")
                    .short('o')
                    .long("outfile")
                    .default_value("")
                    .help("Write to file instead of stdout"),
            )
            .arg(
                Arg::new("silent")
                    .short('s')
                    .action(ArgAction::SetTrue)
                    .long("silent"),
            )
            .get_matches();

        let infile = matches
            .get_one::<String>("infile")
            .expect("invalid input file")
            .to_string();

        let outfile = matches
            .get_one::<String>("outfile")
            .expect("invalid output file")
            .to_string();

        let silent = if matches.get_flag("silent") {
            true
        } else {
            !env::var("PV_SILENT").unwrap_or_default().is_empty()
        };

        Self {
            infile,
            outfile,
            silent,
        }
    }
}
