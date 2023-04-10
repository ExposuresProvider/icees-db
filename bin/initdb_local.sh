#!/usr/bin/env bash

# this script is a counterpart of initdb.sh while initdb.sh is a init script that is triggered when deploying to k8s,
# and the initdb_local.sh can be run locally outside k8s
set -e

set -o allexport
source .env
set +o allexport

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
export PYTHONPATH=.
python $DIR/initdb.py
python $DIR/convert.py
python $DIR/create_bins.py
