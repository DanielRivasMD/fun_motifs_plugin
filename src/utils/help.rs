////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use clap::{Arg, Command};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn cli_fun_motifs_plugin() -> Command<'static> {
  Command::new("")
    .version("v0.3.0")
    .author("Daniel Rivas <danielrivasmd@gmail.com>")
    .arg(
      Arg::new("log")
        .short('l')
        .long("log")
        .possible_values(&[
          "i", "info", "d", "debug", "w", "warn", "e", "error",
        ])
        .default_value("info")
        .help("Set logging level"),
    )
    .arg(
      Arg::new("input")
        .long("input")
        .short('i')
        .takes_value(true)
        .required(true)
        .help("Input file"),
    )
    .arg(
      Arg::new("output")
        .long("output")
        .short('o')
        .takes_value(true)
        .required(true)
        .help("Output files"),
    )
}

////////////////////////////////////////////////////////////////////////////////////////////////////
