#!/bin/bash
   
# write a bash function that move files (except mod.rs) from src/problem to src/solution and return a list of that files name
# store those files name into a global list variable

function move_files() {
    local files=$(ls src/problem | grep -v mod.rs)
    for file in $files; do
        mv src/problem/$file src/solution/$file
    done

    
    if [ -n "$files" ]; then 
      echo "https://github.com/LeVuMinhHuy/canada/blob/main/src/solution/$files"
    else
      echo "No files found"
    fi
}

move_files
