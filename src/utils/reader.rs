////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::{Context, Result as anyResult};
use bytelines::{ByteLines, ByteLinesReader};
use flate2::read::MultiGzDecoder;
use std::{
  fs::File,
  io::{BufReader, Read},
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::utils::error::FunMotifsError;

////////////////////////////////////////////////////////////////////////////////////////////////////

fn byte_file_reader(
  input_file: &str
) -> anyResult<ByteLines<BufReader<Box<dyn Read>>>> {
  let reader: BufReader<Box<dyn Read>> = BufReader::new(
    if input_file.ends_with(".gz") || input_file.ends_with(".bgz") {
      Box::new(MultiGzDecoder::new(File::open(input_file).context(
        FunMotifsError::ReadFile {
          f: input_file.to_string(),
        },
      )?))
    } else {
      Box::new(File::open(input_file).context(FunMotifsError::ReadFile {
        f: input_file.to_string(),
      })?)
    },
  );

  let lines = reader.byte_lines();
  Ok(lines)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: write reader
pub fn csv_reader_input(input_csv_path: &str) -> anyResult<()> {
  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn csv_reader_output(output_csv_path: &str) -> anyResult<()> {
  // declare variables
  let mut ct = 0;

  // collect lines
  let mut lines = byte_file_reader(output_csv_path)?;
  while let Some(line) = lines.next() {
    // line counter
    ct += 1;

    // read line
    let record_line = String::from_utf8_lossy(line?);

    match ct {
      ct if ct == 1 => {
        // headers
        let _headers = record_line.split(',').collect::<Vec<&str>>();
      }

      ct if ct > 1 => (),

      _ => (),
    }
  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
