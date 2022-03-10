import sys
import yaml
import os


mappings_file_path = os.environ["INPUT_MAPPING_FILE_WITH_PATH"]
identifiers_file_path = os.environ["INPUT_IDENTIFIERS_FILE_WITH_PATH"]
new_mappings_file_path = os.environ["OUTPUT_MAPPING_FILE_WITH_PATH"]
new_value_sets_file_path = os.environ["OUTPUT_VALUE_SETS_FILE_WITH_PATH"]

with open(mappings_file_path) as mappings_file:
    old_mappings = yaml.safe_load(mappings_file)

with open(identifiers_file_path) as identifiers_file:
    old_identifiers = yaml.safe_load(identifiers_file)

# merge tables

mappings = {}

value_sets = {}

for table, table_mappings in old_mappings.items():
    for column, column_mapping in table_mappings.items():
        if column in mappings:
            pass
        else:
            categories = column_mapping["categories"]
            if len(categories) == 0:
                print("no biolinkType, default to NameThing")
                categories = ["biolink:NamedThing"]
            ty = column_mapping["type"]
            # DUMMY:000000 is added when identifier for the variable column is null or empty since otherwise,
            # the created mapping file with a null identifier will trigger an exception in ICEES API that does not
            # allow an empty identifier for any variable
            idt = old_identifiers[table].get(column)
            if not idt:
                idt = 'DUMMY:000000'
            mappings[column] = {
                "categories": categories,
                "identifiers": idt,
                "type": ty
            }
            maximum = column_mapping.get("maximum")
            minimum = column_mapping.get("minimum")
            enum = column_mapping.get("enum")
            if maximum is not None and minimum is not None:
                value_sets[column] = list(range(minimum, maximum+1))
            elif enum is not None:
                value_sets[column] = [item.strip() if isinstance(item, str) else item for item in enum]

with open(new_mappings_file_path, "w") as new_mappings_file:
    yaml.dump(mappings, new_mappings_file)

with open(new_value_sets_file_path, "w") as new_value_sets_file:
    yaml.dump(value_sets, new_value_sets_file)

