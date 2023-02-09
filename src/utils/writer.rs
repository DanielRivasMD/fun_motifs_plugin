////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::{
  Context,
  Result as anyResult,
};
use std::{
  fs::File,
  io::Write,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::utils::error::FunMotifs;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn csv_writer_create(output_file: &str) -> anyResult<File> {
  // create file
  let file = File::create(&output_file).context(FunMotifs::CreateFile {
    f: output_file.to_string(),
  })?;

  Ok(file)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn csv_writer_header(file: &mut File) -> anyResult<()> {

  // declare header
  let header = vec![
    "posrange",
    "chr",
    "motifstart",
    "motifend",
    "name",
    "score",
    "pval",
    "strand",
  ];

  // iterate on header
  for (ix, hd) in header.iter().enumerate() {
    // define output
    let to_write = match ix {
      i if i < header.len() - 1 => format!("{},", hd),
      i if i >= header.len() - 1 => format!("{}\n", hd),
      _ => String::from("Unexpected length"),
    };

    // write
    file
      .write_all(to_write.as_bytes())
      .context(FunMotifs::WriteFile {
        f: to_write
      })?;
  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
