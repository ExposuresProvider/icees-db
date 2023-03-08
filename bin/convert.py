import yaml
import os


feature_file_path = os.environ["INPUT_FEATURE_FILE_WITH_PATH"]
new_value_sets_file_path = os.environ["OUTPUT_VALUE_SETS_FILE_WITH_PATH"]

with open(feature_file_path) as mappings_file:
    features = yaml.safe_load(mappings_file)


value_sets = {}

for table, table_mappings in features.items():
    for column, column_mapping in table_mappings.items():
        if column in value_sets:
            pass
        else:
            maximum = column_mapping.get("maximum")
            minimum = column_mapping.get("minimum")
            enum = column_mapping.get("enum")
            if maximum is not None and minimum is not None:
                value_sets[column] = list(range(minimum, maximum+1))
            elif enum is not None:
                value_sets[column] = [item.strip() if isinstance(item, str) else item for item in enum]

with open(new_value_sets_file_path, "w") as new_value_sets_file:
    yaml.dump(value_sets, new_value_sets_file)
