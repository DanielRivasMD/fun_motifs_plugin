////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use colored::*;
use thiserror::Error;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Error)]
pub enum FunMotifs {
  #[error("\n{}: {f:?}\n", "Fail to create file".red())]
  CreateFile { f: String },

  #[error("\n{}: {f:?}\n", "Fail to read file".red())]
  ReadFile { f: String },

  #[error("\n{}: {f:?}\n", "Fail to read external script".red())]
  ReadExternalScript { f: String },

  #[error("\n{}: {f:?}\n", "Fail to write file".red())]
  WriteFile { f: String },

  #[error("\n{}: {f:?}\n", "Fail to parse".red())]
  Parsing { f: String },

  #[error("\n{}: {f:?}\n", "Fail to parse flag".red())]
  ParseFlag { f: String },

  #[error("\n{}\n", "Fail to read lines".red())]
  RegistryLine,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Error)]
pub enum ConfigError {
  #[error("\n{}: {f:?}\n", "Fail to read configuration".red())]
  ConfigHashMap { f: String },

  #[error("\n{}\n{}{}\n\n", "Configuration file was not set:".red(), "Set configuration file with option ", "'-c --config'".cyan(), )]
  EmptyConfigOption,

  #[error("\n{}\n", "Configuration file not found".red())]
  NoConfigFile,

  #[error("\n{}\n{}{}\n", "Directory was not set properly in configuration file".red(), "Example: directory = ", "'/home/favorite_directory/'".cyan())]
  BadDirectoryVar,

  #[error("\n{}\n{}{}\n", "Output directory was not set properly in configuration file".red(), "Example: directory = ", "'/home/write_output_here/'".cyan())]
  BadOutput,

  #[error("\n{}\n{}{}\n", "Error directory was not set properly in configuration file".red(), "Example: directory = ", "'/home/write_error_here/'".cyan())]
  BadError,

  #[error("Error TODO")]
  TODO,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
