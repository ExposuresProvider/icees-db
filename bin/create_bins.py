"""
This script combines all bin files per year into one single bins.json file with each year as a key for ICEES-API to use
"""
import os
import sys
import json
import regex


def combine_bins(bins_input_file_path):
    combined_dict = {}
    for filename in os.listdir(bins_input_file_path):
        # extract year from file name
        year = regex.search('^(.*)(\d{4})(.*)$', filename).groups()[1]
        with open(os.path.join(bins_input_file_path, filename)) as f:
           year_dict = json.load(f)
           combined_dict[year] = {
               "patient": year_dict
           }

    return combined_dict


if __name__ == "__main__":
    return_data = combine_bins(os.environ.get('BINS_DATA_PATH', './datad/patient_bins'))
    dump_data = json.dumps(return_data, indent=4).replace('Infinity', str(sys.maxsize))
    with open(os.path.join(os.environ.get('CONFIG_PATH', './config'), 'bins.json'), 'w') as outf:
        outf.write(dump_data)
