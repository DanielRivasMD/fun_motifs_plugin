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

pub fn bed_reader_input(
  input_bed_path: &str,
  motifs_overlapping_tracks_files: Vec<&str>,
  cells_assays: &str,
) -> anyResult<()> {
  // declare variables
  let headers = vec!["posrange", "chr", "motifstart", "motifend", "name", "score", "pval", "strand"];

  // loop over input files
  for motifs_overlapping_tracks_file in motifs_overlapping_tracks_files {
    let scored_motifs_chromatin_tracks_output_file = format!("{}{}", motifs_overlapping_tracks_file, "_scored.bed10");

    // create scored motif (chromatin-wise) files
    // score each motif-track_overlapping file
    if Path::new(&scored_motifs_chromatin_tracks_output_file).exists() {

      // collect lines
      let mut lines = byte_file_reader(input_bed_path)?;
      while let Some(line) = lines.next() {

        // read line
        let record_line = from_utf8(line?).context(FunMotifs::RegistryLine)?;

        // loop over all cell types
        let cells_assays = cells_assays.split_whitespace().collect::<Vec<&str>>();

        // TODO: sort cell assays
        for cell in cells_assays {
          // loop over all assay types that exist for this cell
          // TODO: sort cell assays
          // for assay in cells_assays[*cell] {
            // add letter so column name does not start with digit
            todo!("Do not allow digit as first character")


          // }
        }
      }




      // // format scaffold / chromosome name
      // let record_entry = record_line.split_whitespace().collect::<Vec<&str>>();

      // vcf_loci.push(VcfLocus::new(
      //   record_entry[0].to_string(),
      //   record_entry[1]
      //     .parse::<u32>()
      //     .context(FunMotifs::Parsing {
      //       f: format!("Position: {} @ line {}", record_entry[1], ct),
      //     })?,
      // ));

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
