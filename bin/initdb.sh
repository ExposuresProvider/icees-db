#!/usr/bin/env bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
export PYTHONPATH=.
python $DIR/initdb.py
python $DIR/convert.py
python $DIR/create_bins.py
