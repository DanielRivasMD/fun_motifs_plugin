////////////////////////////////////////////////////////////////////////////////////////////////////

// fun_motifs_plugin wrapper
use fun_motifs_plugin::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Result as anyResult;
use std::collections::HashMap;
use std::fmt::format;
use config::{
  Config,
  File,
  FileFormat,
};
use clap::Parser;
use std::path::Path;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::process::cache::cache_controller;
use crate::process::overlay::overlay;
use crate::process::score::score;
use crate::utils::help::{Cli, config_builder};
use crate::utils::writer::{csv_writer_create, csv_writer_header};

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::utils::error::ConfigError;

////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() -> anyResult<()> {

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // collect command line arguments
  let params = Cli::parse();

  println!("{:?}", params);

  let config_hm = config_builder(&params);

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // declare dummy variables
  let motif_sites_dir = String::new();
  let all_chromatin_makrs_all_cells_combined_dir_path = String::new();
  let motifs_overlapping_tracks_output_dir = String::new();

  let normal_expression_per_tissue_origin_per_TF = String::new();
  let matching_tissue_to_cell = String::new();
  let assay_cells_datatypes = String::new();
  let motifTFName_TFNames_matches_dict = String::new();
  let cells_assays_dict = String::new();
  let cell_tfs = String::new();
  let tf_cells = String::new();

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // verify & create paths
  let (motif_files_full_path, chromatin_tracks_files) = cache_controller(&motif_sites_dir, &all_chromatin_makrs_all_cells_combined_dir_path, &motifs_overlapping_tracks_output_dir)?;

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // overlay_resources_score_motifs
  // compute overlay resource score motif to find overlapping structures
  let mut motifs_overlapping_tracks_files = vec![];
  for i in motif_files_full_path {
    if Path::new(&i).exists() {
      let motifs_overlapping_tracks_file = overlay(&i, &motifs_overlapping_tracks_output_dir, &all_chromatin_makrs_all_cells_combined_dir_path, chromatin_tracks_files.clone())?;
      motifs_overlapping_tracks_files.push(motifs_overlapping_tracks_file);
    } else {
      println!("Motif file {} cannot be found and will be ignored", &i);
    }
  }

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // score_motifs_per_cell
  let mut scored_motifs_overlapping_tracks_files = vec![];
  for motifs_overlapping_tracks_file in motifs_overlapping_tracks_files {
    // TODO: format file name
    let scored_motifs_chromatin_tracks_output_file = format!("{}_scored.bed10", motifs_overlapping_tracks_file);

    // create or overwrite scored motif (chromatin-wise) files
    if !Path::new(&scored_motifs_chromatin_tracks_output_file).exists() {

      let index_track_names = 6;
      let index_motif_name = 3;

      // prepare to write
      let mut output_file = csv_writer_create(&scored_motifs_chromatin_tracks_output_file)?;
      csv_writer_header(&mut output_file);

      score(&motifs_overlapping_tracks_file, &normal_expression_per_tissue_origin_per_TF, &matching_tissue_to_cell, &motifTFName_TFNames_matches_dict, &cells_assays_dict, &cell_tfs, &tf_cells, &assay_cells_datatypes, index_track_names, index_motif_name)?;

      // TODO: define overlapping tracks
      let scored_motifs_overlapping_tracks_file = String::new();
      scored_motifs_overlapping_tracks_files.push(scored_motifs_overlapping_tracks_file);
    }

  }


  //

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////
// reset_cells_assays_matrix
////////////////////////////////////////////////////////////////////////////////////////////////////
// NO DOCUMENTATION

////////////////////////////////////////////////////////////////////////////////////////////////////
// get_motif_score
////////////////////////////////////////////////////////////////////////////////////////////////////
// Calculates a score for a given motif per cell line.

////////////////////////////////////////////////////////////////////////////////////////////////////
// process_scored_motif_per_cell_per_assay
////////////////////////////////////////////////////////////////////////////////////////////////////
// Adds values from the dict to a list and imputate values for NaNs from the other tissues when possible.

////////////////////////////////////////////////////////////////////////////////////////////////////
// score_motifs_per_cell
////////////////////////////////////////////////////////////////////////////////////////////////////
// Input: a list of motifs overlapping cell tracks in bed7 format
//        normal gene expression dictionary: keys are cell#TF and values are expression levels (float)
// Return: list of scored motifs files

////////////////////////////////////////////////////////////////////////////////////////////////////
// overlay_resources_score_motifs
////////////////////////////////////////////////////////////////////////////////////////////////////
// intersect motifs with chromatin tracks, sort and group the tracks per motif
// Input: motif instances file (motif pos, name_id, scorePval, strand)
//        chromatin data collection file in bed4 format; track pos, track cell#assaytype#value or cell#TFname in case of chip-seq
// Return a file in bed7 format (motif info (6cols), overlapping_tracks.

////////////////////////////////////////////////////////////////////////////////////////////////////
// run_overlay_resources_score_motifs
////////////////////////////////////////////////////////////////////////////////////////////////////
// pairs matching chromosomes in motif_sites_input_dir and all_chromatin_makrs_all_cells_input_dir and calls
// overlay_resources_score_motifs
// Input: moitf instances input dir (one file per chr) chromatin data collection dir
// (one file per chr, bed4 format; track pos, track cell#assaytype#value or cell#TFname in case of chip-seq)
// Return: a list of motif_overlapping_track files Precondition: files in motif_sites_input_dir and
// chromatin_tracks_input_dir should have the same names
// Recommended: name files in both dirs as chrNumber, chrX or chrY (where number is between 1-22)

////////////////////////////////////////////////////////////////////////////////////////////////////
