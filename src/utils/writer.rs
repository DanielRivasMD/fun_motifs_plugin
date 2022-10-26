////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::{Context, Result as anyResult};
use std::{fs::File, io::Write};

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::utils::error::FunMotifsError;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn csv_writer_create(output_file: &str) -> anyResult<File> {
  // create file
  let file_name = output_file;
  let file = File::create(&file_name).context(FunMotifsError::CreateFile {
    f: file_name.to_string(),
  })?;

  Ok(file)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: write writer
pub fn csv_writer_header(file: &mut File) -> anyResult<()> {
  // declare header
  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
