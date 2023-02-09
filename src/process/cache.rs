////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Result as anyResult;
use std::fs::read_dir;
use std::fs::create_dir_all;
use std::path::Path;
use std::process::exit;

////////////////////////////////////////////////////////////////////////////////////////////////////

// // modules
// use crate::modules::fasta_read::{
//   fasta_cache_read,
//   fasta_cache_write,
//   fasta_file_read,
// };

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn cache_controller(
  motif_sites_dir: &str,
  all_chromatin_makrs_all_cells_input_dir: &str,
  motif_overlapping_tracks_output_dir: &str,
) -> anyResult<(Vec<String>, Vec<String>)> {

  // declare motif files
  let motif_files: Vec<String>;

  // check if input motif_sites_dir is directory & get files
  if Path::new(&motif_sites_dir).exists() {
    if Path::new(&motif_sites_dir).is_dir() && Path::new(&motif_sites_dir).is_file() {
      motif_files = vec![motif_sites_dir.to_string()];
    } else {

      // collect files to vector
      motif_files = read_dir(&motif_sites_dir)?
        .filter_map(|perhaps_dir_entry| {
          let dir_entry = perhaps_dir_entry.ok()?;
          let path_buf = dir_entry.path();
          let file_name = path_buf.file_name()?;
          let string = file_name.to_str()?;
          Some(string.to_string())
        })
        .collect::<Vec<String>>();

    }
  } else {
    exit(1);
  }

  // get list of paths to all motif files
  let mut motif_files_full_path = vec![];
  for s in motif_files {
    motif_files_full_path.push(format!("{}{}", motif_sites_dir, s));
  }

  // get list of all files of combined tracks
  let chromatin_tracks_files = read_dir(all_chromatin_makrs_all_cells_input_dir)?
    .filter_map(|perhaps_dir_entry| {
      let dir_entry = perhaps_dir_entry.ok()?;
      let path_buf = dir_entry.path();
      let file_name = path_buf.file_name()?;
      let string = file_name.to_str()?;
      Some(string.to_string())
    })
    .collect::<Vec<String>>();

  // create output directory if not existing
  if !Path::new(&motif_overlapping_tracks_output_dir).exists() {
    create_dir_all(&motif_overlapping_tracks_output_dir);
  }

  Ok((motif_files_full_path, chromatin_tracks_files))
}

////////////////////////////////////////////////////////////////////////////////////////////////////
