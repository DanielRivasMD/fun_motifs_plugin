import sys, os
from multiprocessing.pool import Pool
from pybedtools import BedTool, set_tempdir, cleanup
import glob
from itertools import starmap
from itertools import product
from collections import Counter


def score_motifs_main(motifs_overlapping_tracks_files, force_overwrite, cells_assays_dict, run_in_parallel_param,
                      number_processes_to_run_in_parallel, normal_expression_per_tissue_origin_per_TF,
                      matching_tissue_to_cell, motifTFName_TFNames_matches_dict, cell_tfs, tf_cells,
                      assay_cells_datatypes):
    # create a list to save file names
    scored_motifs_overlapping_tracks_files = []

    # loop over input files
    for motifs_overlapping_tracks_file in motifs_overlapping_tracks_files:
        # get name of new file based on name of input file
        scored_motifs_chromatin_tracks_output_file = '.'.join(
            motifs_overlapping_tracks_file.split('.')[0:-1]) + '_scored.bed10'

        # create or overwrite scored motif (chromatin-wise) files
        # score each motif-track_overlapping file
        if force_overwrite or not os.path.exists(scored_motifs_chromatin_tracks_output_file):
            # TODO: control change below
            # hard coded variable (column numbers in input file)
            index_track_names = 6
            index_motif_name = 3

            # open file
            with open(scored_motifs_chromatin_tracks_output_file, 'w') as scored_motifs_writefile:
                # create standard header line (more columns will be added based on data, but this part always exists)
                header_line = ['posrange', 'chr', 'motifstart', 'motifend', 'name', 'score', 'pval', 'strand']
                # loop over all cell types
                for cell in sorted(cells_assays_dict.keys()):
                    # loop over all assay types that exist for this cell
                    for assay in sorted(cells_assays_dict[cell].keys()):
                        # add letter so column name does not start with digit
                        if cell[0].isdigit():
                            cell = 'a' + cell

                        # create name for columns based on existing cell and assay
                        # (example: Adipose____Subcutaneous___TFExpr)
                        cell_name = '_'.join(((cell + "___" + assay).replace('(', '').replace(')', '')
                                              .replace('-', '__').replace('.', '').replace("'", "")).split())
                        # append this name to header
                        header_line.append('"' + cell_name + '"')
                # write header into file
                scored_motifs_writefile.write('\t'.join(header_line) + '\n')

            # score motifs, either in parallel processing or not
            if run_in_parallel_param:
                print("Run score_motifs per cell in parallel")
                # splits the input file into several files for parallel processing (as far as I understand)
                os.system("""split -l 200000 {} {}""".format(motifs_overlapping_tracks_file,
                                                             motifs_overlapping_tracks_file + '_tmp'))
                # lists all files that end with _tmp
                motifs_overlapping_tracks_file_splitted = glob.glob(motifs_overlapping_tracks_file + '_tmp*')
                # parallel processing init
                p = Pool(int(number_processes_to_run_in_parallel))
                # call function score_motifs_per cell in parallel
                # product: creates list of all possible combinations of parameters
                p.starmap(score_motifs_per_cell, product(motifs_overlapping_tracks_file_splitted,
                                                         [normal_expression_per_tissue_origin_per_TF],
                                                         [matching_tissue_to_cell],
                                                         [motifTFName_TFNames_matches_dict],
                                                         [cells_assays_dict],
                                                         [cell_tfs],
                                                         [tf_cells],
                                                         [assay_cells_datatypes],
                                                         [index_track_names],
                                                         [index_motif_name]))
                p.close()
                p.join()

                # open file with header
                with open(scored_motifs_chromatin_tracks_output_file, 'a') as scored_motifs_writefile:
                    # loop over created scored motif files
                    for f in motifs_overlapping_tracks_file_splitted:
                        # open created file
                        with open(f + '_scored', 'r') as f_score_ifile:
                            # read lines
                            l = f_score_ifile.readline()
                            while l:
                                # write lines to file (that contained only header before)
                                scored_motifs_writefile.write(l)
                                l = f_score_ifile.readline()
                        # close files and remove temporary files
                        f_score_ifile.close()
                        os.remove(f)
                        os.remove(f + '_scored')
                scored_motifs_writefile.close()

            else:
                # if not run in parallel: call this function and write into outpur file
                print("Do not run score_motifs per cell in parallel")
                scored_file_tmp = score_motifs_per_cell(motifs_overlapping_tracks_file,
                                                        normal_expression_per_tissue_origin_per_TF,
                                                        matching_tissue_to_cell,
                                                        motifTFName_TFNames_matches_dict,
                                                        cells_assays_dict,
                                                        cell_tfs,
                                                        tf_cells,
                                                        assay_cells_datatypes,
                                                        index_track_names,
                                                        index_motif_name)
                # write scores into scored file with header
                with open(scored_file_tmp, 'r') as infile, \
                        open(scored_motifs_chromatin_tracks_output_file, 'a') as outfile:
                    outfile.write(infile.read())

        # append created scored output file to output file list created at very beginning
        scored_motifs_overlapping_tracks_files.append(scored_motifs_chromatin_tracks_output_file)

        return scored_motifs_overlapping_tracks_files


"""
The following function is called in the previous function
"""


def score_motifs_per_cell(motifs_overlapping_tracks_file,
                          normal_expression_per_tissue_origin_per_TF,
                          matching_tissue_to_cell,
                          motifTFName_TFNames_matches_dict,
                          cells_assays_dict,
                          cell_tfs,
                          tf_cells,
                          assay_cells_datatypes,
                          index_track_names,
                          index_motif_name):
    # from Husen:
    """
    Input: a list of motifs overlapping cell tracks in bed7 format
           normal gene expression dictionary: keys are cell#TF and values are expression levels (float)

    Return: list of scored motifs files
    """
    # create output file name
    scored_motifs_chromatin_tracks_output_file = motifs_overlapping_tracks_file + '_scored'
    # TODO: check necessity of force_overwrite below
    if not os.path.exists(scored_motifs_chromatin_tracks_output_file):
        # TODO: remove hardcoded variable
        sep = '\t'
        # open input file as 'read' and output file as 'write'
        with open(motifs_overlapping_tracks_file, 'r') as motifs_overlapping_tracks_readfile, open(
                scored_motifs_chromatin_tracks_output_file, 'w') as scored_motifs_writefile:
            # read line by line
            line = motifs_overlapping_tracks_readfile.readline()
            while line:
                # split line according to separator (bed files usually are tab separated)
                split_line = line.strip().split(sep)
                # TODO: check if statement
                # check that line has at least as many columns as are necessary by the specified column indices
                if len(split_line) >= max(index_motif_name, index_track_names) + 1:
                    # call this function (also in this file)
                    reset_cells_assays_dict = reset_cells_assays_matrix(
                        split_line[index_motif_name].split('_')[0].upper(),
                        cells_assays_dict,
                        cell_tfs,
                        tf_cells,
                        motifTFName_TFNames_matches_dict,
                        assay_cells_datatypes)

                    # call this function (also in this file)
                    scored_motif_per_cell_per_assay = get_motif_score(split_line,
                                                                      normal_expression_per_tissue_origin_per_TF,
                                                                      matching_tissue_to_cell,
                                                                      motifTFName_TFNames_matches_dict,
                                                                      reset_cells_assays_dict,
                                                                      index_track_names,
                                                                      index_motif_name)

                    # call this function (also in this file)
                    field_values = process_scored_motif_per_cell_per_assay(split_line[0:index_track_names],
                                                                           scored_motif_per_cell_per_assay,
                                                                           cells_assays_dict)

                    # write output into output file
                    scored_motifs_writefile.write('\t'.join(field_values) + '\n')
                    # read next line
                    line = motifs_overlapping_tracks_readfile.readline()

    # return create output file
    return scored_motifs_chromatin_tracks_output_file


"""
The following three functions are called in the previous function
"""


def reset_cells_assays_matrix(tf_name_from_motif_name,
                              cells_assays_dict,
                              cell_tfs,
                              tf_cells,
                              motifTFName_TFNames_matches_dict,
                              assay_cells_datatypes):
    """

    As far as I understand, this function initializes the values for the scored output file to prevent empty entries
    in this matrix like format. For now it is done in a dictionary
    """
    # loop over cell names
    for representative_cell in cells_assays_dict:
        # loop over assays that exist for this cell type
        for assay in cells_assays_dict[representative_cell]:
            # my guess: initializing the output file, so that there are no empty entries
            # initialize with 0.0 or NO if not TFBinding or TFExpr
            if "TFBinding" not in assay and assay != "TFExpr":
                try:
                    if assay_cells_datatypes[assay] == "real":
                        cells_assays_dict[representative_cell][assay] = 0.0
                    else:
                        cells_assays_dict[representative_cell][assay] = "NO"
                except ValueError:
                    cells_assays_dict[representative_cell][assay] = "NO"
            # initialize TFExpr with NaN
            elif "TFExpr" in assay:
                cells_assays_dict[representative_cell][assay] = "NaN"

            # checking whether this TF is available in the current cell
            # set value to 0.0 if data exists
            elif "TFBinding" in assay:
                tf_exists = 0
                if tf_name_from_motif_name in tf_cells:
                    if representative_cell in tf_cells[tf_name_from_motif_name]:
                        cells_assays_dict[representative_cell][assay] = 0.0
                        tf_exists = 1
                else:
                    # check dict with alternative tf names
                    for alt_tf_name in motifTFName_TFNames_matches_dict[tf_name_from_motif_name]:
                        if alt_tf_name in tf_cells:
                            if representative_cell in tf_cells[alt_tf_name]:
                                cells_assays_dict[representative_cell][assay] = 0.0
                                tf_exists = 1
                                break
                # if TFBinding data does not exist: initialize with NaN
                if tf_exists == 0:
                    cells_assays_dict[representative_cell][assay] = 'NaN'

                # check if other tf (than the expected one) were shown to bind to the motif
                # initialize with 0.0 if yes, NaN else
                if len(cell_tfs[representative_cell]) - tf_exists > 0.0:
                    cells_assays_dict[representative_cell]["NumOtherTFBinding"] = 0.0
                    cells_assays_dict[representative_cell]["OtherTFBinding"] = []
                else:
                    cells_assays_dict[representative_cell]["NumOtherTFBinding"] = 'NaN'
                    cells_assays_dict[representative_cell]["OtherTFBinding"] = []
    return cells_assays_dict


def get_motif_score(split_line,
                    normal_expression_per_tissue_origin_per_TF,
                    matching_tissue_to_cell,
                    motifTFName_TFNames_matches_dict,
                    cells_assays_dict,
                    index_track_names,
                    index_motif_name):
    """
    Gets score for a given motif per cell line. (?)
    """
    # gets TF name that binds to motif, I guess
    tf_name_from_motif = split_line[index_motif_name].split('_')[0].upper()

    # Get expression value for the current TF in all tissues
    # loop over cells
    for representative_cell in cells_assays_dict:
        # try-except in case we try to access non existing columns
        try:
            # check if TFExpr assay exists for cell
            if 'TFExpr' in cells_assays_dict[representative_cell]:
                # check if tf name (binding to motif) is in expression data (saved as dictionary)
                if tf_name_from_motif in normal_expression_per_tissue_origin_per_TF[representative_cell]:
                    # check that value for expression data exists and is not NaN
                    if normal_expression_per_tissue_origin_per_TF[representative_cell][tf_name_from_motif] != 'NaN':
                        cells_assays_dict[representative_cell]['TFExpr'] = float(
                            normal_expression_per_tissue_origin_per_TF[representative_cell][tf_name_from_motif])
        except ValueError:
            pass

    # loop over existing tracks (example: K562#RepliDomain#ERD,Sknsh#RepliDomain#ERD,Mcf7#RepliDomain#ERD, ...)
    for trackname in split_line[index_track_names].split(','):
        # split information of track
        ts = trackname.split('#')
        # check for matching cell names
        try:
            # check if cell name of track (ts[0]) is known to belong to a tissue
            # example: {'MCF-7': 'Breast'}, should not be case-sensitive however
            matching_tissues_cell = matching_tissue_to_cell[ts[0]]
        except KeyError:
            # skip tracks of cells that have no matching in the rep_cell dict file
            continue
        # if not is already: make list of matching tissues cell
        if type(matching_tissues_cell) is not list:
            matching_tissues_cell = [matching_tissues_cell]

        # check if assays exists for the cell type, otherwise skip to next track
        # TODO: this is an logical error: it is made to a list before, the dictionary keys are strings --> skips all
        if not matching_tissues_cell in list(cells_assays_dict.keys()):
            continue

        # for this for loop it must be a list
        # loop over matching tissues (don't know why more than 1 might be expected)
        for matching_tissue_cell in matching_tissues_cell:
            # next step depends on length of track (which probably depends on kind of assay)
            # for me it seems like this can be done more logical
            # (more like looping over list, and checking entries on the go)

            # write track information into output dictionary (that will create output file)
            if len(ts) == 2:
                cells_assays_dict[matching_tissue_cell][ts[1]] = 1

            elif len(ts) == 3 and ts[1] != "TFBinding":
                if cells_assays_dict[matching_tissue_cell][ts[1]] == 0.0 or cells_assays_dict[matching_tissue_cell][
                    ts[1]] == 'NO':
                    try:
                        cells_assays_dict[matching_tissue_cell][ts[1]] = float(ts[2])
                    except ValueError:
                        cells_assays_dict[matching_tissue_cell][ts[1]] = ts[2]

            elif ts[1] == "TFBinding" and (len(ts) == 3 or len(ts) == 4):
                # a sample motif name is: ZBTB18_MA0698.1 (name_id) only the first is the factor name
                if ts[2].upper() == tf_name_from_motif or ts[2].upper() in motifTFName_TFNames_matches_dict[
                    tf_name_from_motif]:
                    binding_value = 1.0
                    if len(ts) == 4:
                        binding_value = float(ts[3])
                    cells_assays_dict[matching_tissue_cell][ts[1]] = binding_value
                else:
                    # I think in this case the else statement would be enough (I don't see any difference in the effect)
                    if cells_assays_dict[matching_tissue_cell]['NumOtherTFBinding'] == 0.0:
                        cells_assays_dict[matching_tissue_cell]['NumOtherTFBinding'] = 1.0
                        cells_assays_dict[matching_tissue_cell]['OtherTFBinding'] = [ts[2]]
                    else:
                        cells_assays_dict[matching_tissue_cell]['NumOtherTFBinding'] += 1.0
                        cells_assays_dict[matching_tissue_cell]['OtherTFBinding'].append(ts[2])
    # return the dictionary that contains new information now
    return cells_assays_dict


def process_scored_motif_per_cell_per_assay(motif_info,
                                            scored_motif_per_cell_per_assay,
                                            cells_assays_dict):
    """
    Adds values from the dict to a list and imputes values for NaNs from the other tissues when possible
    """
    # get information from motif in a list
    # save position range of motif
    field_values = ['[{},{})'.format(motif_info[1], str(int(motif_info[2]) + 1))]
    # save chromosome number
    field_values.append(motif_info[0].replace("X", '23').replace('Y', '24').replace('M', '25').replace('chr', ''))
    # save start and end position of motif
    field_values.append(str(int(motif_info[1])))
    field_values.append(str(int(motif_info[2])))
    # save motif name
    field_values.append(motif_info[3])

    # save motif p value, and score (from FIMO)
    if 'P' in motif_info[4]:
        field_values.append(motif_info[4].split('P')[0].strip('S'))
        if 'P' in motif_info[4]:
            field_values.append(motif_info[4].split('P')[1])
        field_values.append(motif_info[5])
    else:
        # score
        field_values.append(motif_info[4])
        # p-value
        field_values.append(motif_info[5])
        # strand
        field_values.append(motif_info[6])

    # new dictionary
    processed_cells_assays_dict = {}
    # loop over cells
    for cell in sorted(cells_assays_dict.keys()):
        # add cell as key to dictionary
        processed_cells_assays_dict[cell] = {}
        # loop over assays that exist for cells
        for assay in sorted(cells_assays_dict[cell].keys()):

            # extract the information saved in the previous function and combine with motif information
            if assay == "OtherTFBinding":
                value = ';'.join(set(scored_motif_per_cell_per_assay[cell][assay]))
            else:
                if scored_motif_per_cell_per_assay[cell][assay] != "NaN":
                    try:
                        value = float(scored_motif_per_cell_per_assay[cell][assay])
                    except ValueError:
                        value = scored_motif_per_cell_per_assay[cell][assay]
                else:
                    value = 'NaN'
            processed_cells_assays_dict[cell][assay] = value
            field_values.append(str(value))
    return field_values
