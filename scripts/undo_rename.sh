#!/bin/bash

# This script recursively renames all files starting from the parent directory
# by replacing the extension with _extension.txt
# For example: file.jpg becomes file_jpg.txt

# Function to process files
rename_files() {
  # Loop through all files in the current directory
  for file in *; do
    # If it's a directory, recursively process it
    if [ -d "$file" ]; then
      (cd "$file" && rename_files)
    
    # If it's a regular file with an extension
    elif [ -f "$file" ] && [[ "$file" == *.* ]]; then
      # Extract the base name and extension
      base_name="${file%.*}"
      extension="${file##*.}"
      
      # Create the new filename
      new_name="${base_name}_${extension}.txt"
      
      # Rename the file and print the change
      echo "Renaming: $file -> $new_name"
      mv "$file" "$new_name"
    fi
  done
}

# Start the renaming process from the parent directory
echo "Starting file extension renaming process from parent directory..."
(cd .. && rename_files)
echo "Renaming process completed."