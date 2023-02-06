////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Result as anyResult;
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

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn overlay(motif: String, output_dir: String, chromatin_dir: String, chromatin_files: Vec<String>) -> anyResult<()> {

  // read motif file

  // iterate on chromatin files
  for f in chromatin_files {

    // check motif file is in chromatin dir

    // define overlap file

    // create files

    // sort files by columns

    // do some bedtool operation

    // reformat temporary file

    // based reformat on column 6 (zero-indexed)

  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
