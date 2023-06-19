
motif_sites_dir = "/proj/snic2020-16-187/nobackup/funMotifs_analysis/funMotifs_datafiles/Motifs_26_03/motifs_per_chr/"
all_chromatin_makrs_all_cells_combined_dir_path = "/proj/snic2020-16-187/nobackup/funMotifs_analysis/Weight_training_logit_12_05/results/processed_data/chromatin_marks_all_cells_onlynarrowpeaks"
motifs_overlapping_tracks_output_dir = "/proj/snic2020-16-187/nobackup/funMotifs_analysis/Weight_training_logit_12_05/results/output_regulatorymotifs/overlapping_scored_motifs_onlynarrowpeaks"

motifs_overlapping_tracks_file = "/proj/snic2020-16-187/nobackup/funMotifs_analysis/Weight_training_logit_12_05/results/output_regulatorymotifs/overlapping_scored_motifs_onlynarrowpeaks/chrX_overlapping_tracks.bed10"

scored_motifs_overlapping_tracks_files = ""

# posrange        chr     motifstart      motifend        name    score   pval    strand  "Adipose____Subcutaneous___TFExpr"      "Adipose____Visceral_Omentum___TFExpr"  "Adrenal_Gland___TFExpr"        "Artery____Aorta___TFExpr"      "Artery____Coronary___TFExpr"   "Artery____Tibial___TFExpr"     "Bladder___TFExpr"      "Brain____Amygdala___TFExpr"    "Brain____Anterior_cingulate_cortex_BA24___TFExpr"      "Brain____Caudate_basal_ganglia___TFExpr"       "Brain____Cerebellar_Hemisphere___TFExpr"       "Brain____Cerebellum___TFExpr"  "Brain____Cortex___TFExpr"      "Brain____Frontal_Cortex_BA9___TFExpr"  "Brain____Hippocampus___TFExpr" "Brain____Hypothalamus___TFExpr"        "Brain____Nucleus_accumbens_basal_ganglia___TFExpr"     "Brain____Putamen_basal_ganglia___TFExpr"       "Brain____Spinal_cord_cervical_c__1___TFExpr"   "Brain____Substantia_nigra___TFExpr"    "Breast____Mammary_Tissue___TFExpr"     "Cells____EBV__transformed_lymphocytes___TFExpr"        "Cells____Transformed_fibroblasts___TFExpr"     "Cervix____Ectocervix___TFExpr" "Cervix____Endocervix___TFExpr" "Colon____Sigmoid___TFExpr"     "Colon____Transverse___TFExpr"  "Esophagus____Gastroesophageal_Junction___TFExpr"       "Esophagus____Mucosa___TFExpr"  "Esophagus____Muscularis___TFExpr"      "Fallopian_Tube___TFExpr"       "Heart____Atrial_Appendage___TFExpr"    "Heart____Left_Ventricle___TFExpr"      "Kidney____Cortex___TFExpr"     "Liver___TFExpr"        "Lung___TFExpr" "Minor_Salivary_Gland___TFExpr" "Muscle____Skeletal___TFExpr"   "Nerve____Tibial___TFExpr"      "Ovary___TFExpr"        "Pancreas___TFExpr"     "Pituitary___TFExpr"    "Prostate___TFExpr"     "Skin____Not_Sun_Exposed_Suprapubic___TFExpr"   "Skin____Sun_Exposed_Lower_leg___TFExpr"        "Small_Intestine____Terminal_Ileum___TFExpr"    "Spleen___TFExpr"       "Stomach___TFExpr"      "Testis___TFExpr"       "Thyroid___TFExpr"      "Uterus___TFExpr"       "Vagina___TFExpr"       "Whole_Blood___TFExpr"
# [101190393,101190413)   10      101190393       101190412       PPARG_MA0066.1  11.7222 1.04e-05        -       44.83   43.89   1.324   1.47    4.408   1.376   21.33   0.6424  0.9508  0.5116  1.087   0.9473  0.8896  0.9516  0.7427  0.5669  0.7582  0.4065  0.6792  0.4962  29.74   0.1399  6.298   4.778   2.179   2.139   14.97   1.515   2.138   1.312   2.994   2.757   2.814   4.647   1.902   7.193   2.161   0.9506  5.029   6.271   0.5698  0.1678  2.166   1.947   2.485   5.08    5.248   5.827   3.027   6.965   3.45    3.458   0.9281

# normal_expression_per_tissue_origin_per_TF: output in section 1 {"Adipose - Subcutaneous": {"LHX9": 0.00354, "GSX2": 0.0, ... } i.e., {tissue: {cell type: value, ...}, ...}

# matching_tissue_to_cell: output in section 1 {'MCF-7': 'Breast', 'T47D': 'Breast', 'HeLa-S3': 'Cervix', 'ME-180': 'Cervix', 'GM12878': 'Blood', 'Colon - Sigmoid': 'Colon'}

# motifTFName_TFNames_matches_dict: output section 1 {MotifName1: [MotifName1a, Motifname1b], ... }

# cells_assays_dict: output section 1 {HepG2: [DNase-seq,TFBinding,ReplicationDomains], ...}

# cell_tfs: output section 1 {HepG2: [CTCF, FOXA1,...], ...}

# tf_cells: output section 1 {CTCF: [HepG2, K562,...], ...}

# assay_cells_datatypes: output section 1 {'OtherTFBinding': 'text', ...}

# index_track_names = 6 (hard-coded above, column in bed file)

# index_motif_name = 3 (hard-coded above, column in bed file)

# number_processes_to_run_in_parallel: in conf file (6)

# run_in_parallel_param: in conf file (yes)


# Rest is explained in "Rust_input_variables"
# run_in_parallel_param,
# params['number_processes_to_run_in_parallel'],
# normal_expression_per_tissue_origin_per_TF,
# matching_tissue_to_cell,
# motifTFName_TFNames_matches_dict,
# cells_assays_dict, cell_tfs, tf_cells,
# assay_cells_datatypes)
