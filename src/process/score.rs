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

pub fn score(overlap: String, expr_tissue: String) -> anyResult<()> {

  // declare file

  // check whether output file exists

  // read input file

  // motif score function
  motif_score()?;

  // process scored motif function
  process_scored_motif_per_cell_per_assay()?;

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn motif_score() -> anyResult<()> {

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn process_scored_motif_per_cell_per_assay() -> anyResult<()> {

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
