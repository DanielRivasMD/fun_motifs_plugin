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
use std::path::Path;
use std::path::PathBuf;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::utils::reader::byte_file_reader;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn overlay(motif_sites_input_file: &str, motifs_overlapping_tracks_output_dir: &str, chromatin_tracks_dir_path: &str, chromatin_tracks_files: Vec<String>) -> anyResult<String> {

  let motifs_overlapping_tracks_file = String::new();

  // read motif file
  let mut f = byte_file_reader(&motif_sites_input_file)?;

  // check motif file is in chromatin dir

  // iterate on chromatin files
  for chr_n_file in chromatin_tracks_files {

    // define overlap file
    let motifs_overlapping_tracks_file = format!("{}/{}_overlapping_tracks.bed7", motifs_overlapping_tracks_output_dir, motif_sites_input_file);
    let motifs_overlapping_tracks_file_tmp = format!("{}_tmp", motifs_overlapping_tracks_file);

    // create files
    if !Path::new(&motifs_overlapping_tracks_file).exists() {

      let motif_sites_input_file_sorted = format!("{}_sorted", motif_sites_input_file);
      let chromatin_tracks_input_file = format!("{}/{}", chromatin_tracks_dir_path, chr_n_file);
      let chromatin_tracks_input_file_sorted = format!("{}_sorted", chromatin_tracks_input_file);

      println!("Intersecting: {} & {}", motif_sites_input_file, chromatin_tracks_input_file)
      // intersecting: /proj/snic2020-16-187/nobackup/funMotifs_analysis/funMotifs_datafiles/Motifs_26_03/motifs_per_chr//chrX.bed and /proj/snic2020-16-187/nobackup/funMotifs_analysis/Weight_training_logit_12_05/results/processed_data/chromatin_marks_all_cells_onlynarrowpeaks/chrX.bed

    // sort files by columns
    // TODO: sort like `sort -k1,1 -k2,2 -k3,3`

    // do some bedtool operation
    // TODO: BedTool stuff

    // reformat temporary file
    // TODO: reformat function

    // based reformat on column 6 (zero-indexed)

    }

  }

  Ok(motifs_overlapping_tracks_file)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
