#!/usr/bin/env bash

set -e

set -o allexport
source .env
set +o allexport

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
export PYTHONPATH=.
python $DIR/initdb.py
python $DIR/convert.py
