////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use clap::Parser;
use std::collections::HashMap;
use config::{
  Config,
  File,
  FileFormat,
};
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

// builder
pub fn config_builder(params: &Cli) -> HashMap<String, String> {

  let builder = Config::builder()
  .add_source(File::new(&params.config, FileFormat::Toml));

  let mut config_hm = HashMap::new();

  match builder.build() {
  Ok(config) => {
      // use your config
      config_hm = config.try_deserialize::<HashMap<String, String>>().unwrap();
      println!("{:?}", config_hm);
  },
  Err(_e) => {
      // something went wrong
  }
  }

  config_hm

}

////////////////////////////////////////////////////////////////////////////////////////////////////
