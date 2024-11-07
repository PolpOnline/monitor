#!/bin/bash

# Get the directory where the script is located
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# Check if jq is installed
if ! command -v jq &> /dev/null; then
  echo "jq is not installed. Please install jq and try again."
  exit 1
fi

# Loop through all .json files in the script's directory
for file in "$SCRIPT_DIR"/*.json; do
  # Check if there are any JSON files in the directory
  if [ ! -e "$file" ]; then
    echo "No JSON files found in the directory."
    exit 1
  fi

  # Flatten the JSON, insert % before { in strings, add _version key at the beginning, and reformat
  jq -c --argjson version 1 '
      . as $original |
      { "_version": $version } +
      ($original | walk(
          if type == "object" then with_entries(select(.value | type != "object"))
          elif type == "string" then sub("\\{"; "%{")
          else .
          end
      ))
  ' "$file" | jq '.' > "${file}.tmp" && mv "${file}.tmp" "$file"

  echo "Flattened, modified variables, added _version key at the beginning, and reformatted $file"
done
