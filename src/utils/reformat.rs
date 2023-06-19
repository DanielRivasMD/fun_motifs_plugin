////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::{
  Context,
  Result as anyResult,
};
use bytelines::{
  ByteLines,
  ByteLinesReader,
};
use flate2::read::MultiGzDecoder;
use std::{
  fs::File,
  io::{
    BufReader,
    Read,
  },
  path::Path,
  str::from_utf8,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::utils::error::FunMotifs;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn byte_file_reader(
  input_file: &str
) -> anyResult<ByteLines<BufReader<Box<dyn Read>>>> {
  let reader: BufReader<Box<dyn Read>> = BufReader::new(
    if input_file.ends_with(".gz") || input_file.ends_with(".bgz") {
      Box::new(MultiGzDecoder::new(File::open(input_file).context(
        FunMotifs::ReadFile {
          f: input_file.to_string(),
        },
      )?))
    } else {
      Box::new(File::open(input_file).context(FunMotifs::ReadFile {
        f: input_file.to_string(),
      })?)
    },
  );

  let lines = reader.byte_lines();
  Ok(lines)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn reformat(
  input: &str,
  output: &str,
) -> anyResult<()> {

  let mut lines = byte_file_reader(input)?;
  while let Some(line) = lines.next() {

    // read line
    let record_line = from_utf8(line?).context(FunMotifs::RegistryLine)?;

    // loop over all cell types
    let sline = record_line.split('\t').collect::<Vec<&str>>();

    // check fields
    if sline.len() > 6 {

      if sline[6] != "." && sline[6] != "\n" {

        let my_list = sline[6].split(',').collect::<Vec<&str>>();

      }
    }



  }



  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn bed_reader_output(output_bed_path: &str) -> anyResult<()> {
  // declare variables
  let mut ct = 0;

  // collect lines
  let mut lines = byte_file_reader(output_bed_path)?;
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
