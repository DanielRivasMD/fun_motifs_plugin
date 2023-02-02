////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use clap::Parser;
use std::path::PathBuf;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::utils::log_flag::LogFlag;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  /// Set looging level
  #[arg(short, long, value_enum, default_value_t = LogFlag::Info)]
  pub log: LogFlag,

  /// Sets a custom config file
  #[arg(short, long, value_name = "FILE")]
  pub config: String,

  // /// Input file
  // #[arg(short, long, value_name = "INPUT")]
  // pub input: String,

  // /// Output file
  // #[arg(short, long, value_name = "OUTPUT")]
  // pub output: String,
}

// TODO: Activate iteration only when debug is active

////////////////////////////////////////////////////////////////////////////////////////////////////
