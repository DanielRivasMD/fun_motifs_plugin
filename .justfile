####################################################################################################

_default:
  @just --list

####################################################################################################

# print justfile
@show:
  bat .justfile --language make

####################################################################################################

# edit justfile
@edit:
  micro .justfile

####################################################################################################

# aliases

####################################################################################################

# check
@check:
  cargo watch --clear --why --exec check

####################################################################################################

# test list
@test-list:
  cargo watch --clear --shell 'cargo nextest list'

####################################################################################################

# test run
@test-run:
  cargo watch --clear --shell 'cargo nextest run'

####################################################################################################

# build
@debug:
  cargo build

####################################################################################################

# build release
@build:
  cargo build --release

####################################################################################################

# format rustfmt
@fmt:
  rustfmt +nightly src/*rs

####################################################################################################

####################################################################################################
# cluster deployment
####################################################################################################

# Hermes is an Olympian deity in ancient Greek religion and mythology.
# Hermes is considered the herald of the gods.
# He is also considered the protector of human heralds, travellers, thieves, merchants, and orators. He is able to move quickly and freely between the worlds of the mortal and the divine, aided by his winged sandals.
# Hermes plays the role of the psychopomp or "soul guide"—a conductor of souls into the afterlife.

# In myth, Hermes functions as the emissary and messenger of the gods, and is often presented as the son of Zeus and Maia, the Pleiad.
# Hermes is regarded as "the divine trickster," about which the Homeric Hymn to Hermes offers the most well-known account.

# His attributes and symbols include the herma, the rooster, the tortoise, satchel or pouch, talaria (winged sandals), and winged helmet or simple petasos, as well as the palm tree, goat, the number four, several kinds of fish, and incense.
# However, his main symbol is the caduceus, a winged staff intertwined with two snakes copulating and carvings of the other gods.
# His attributes had previously influenced the earlier Etruscan god Turms, a name borrowed from the Greek "herma".

# In Roman mythology and religion many of Hermes' characteristics belong to Mercury, a name derived from the Latin merx, meaning "merchandise," and the origin of the words "merchant" and "commerce."

####################################################################################################

# deliver repository to remote cluster
Hermes-source:
  #!/bin/bash
  set -euo pipefail

  # declarations
  source ".just.sh"

  echo 'Deploying source to Ulam...'
  rsync -azvhP "${projLocal}/src/" "${ulamID}:${projRemote}/src"
  rsync -azvhP "${projLocal}/python/" "${ulamID}:${projRemote}/python"
  rsync -azvhP "${projLocal}/.just.sh" "${ulamID}:${projRemote}/"
  rsync -azvhP "${projLocal}/.justfile" "${ulamID}:${projRemote}/"
  rsync -azvhP "${projLocal}/.gitignore" "${ulamID}:${projRemote}/"
  rsync -azvhP "${projLocal}/Cargo.toml" "${ulamID}:${projRemote}/"
  rsync -azvhP "${projLocal}/rustfmt.toml" "${ulamID}:${projRemote}/"
  echo 'Deploying source to Uppmax...'
  rsync -azvhP --delete "${projLocal}/src" "${uppmaxID}:${projRemote}/"
  rsync -azvhP --delete "${projLocal}/.just.sh" "${uppmaxID}:${projRemote}/"
  rsync -azvhP --delete "${projLocal}/.justfile" "${uppmaxID}:${projRemote}/"
  rsync -azvhP --delete "${projLocal}/.gitignore" "${uppmaxID}:${projRemote}/"
  rsync -azvhP --delete "${projLocal}/Cargo.toml" "${uppmaxID}:${projRemote}/"

####################################################################################################

# deliver sample data to remote cluster
Hermes-data:
  #!/bin/bash
  set -euo pipefail

  # declarations
  source ".just.sh"

  echo 'Deploying source to Ulam...'
  rsync -azvhP "${projLocal}/data/" "${ulamID}:${projRemote}/data"

####################################################################################################
