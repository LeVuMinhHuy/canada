#!/bin/bash
#
function move_files() {
    local files=$(ls src/problem | grep -v mod.rs)
    for file in $files; do
        mv src/problem/$file src/solution/$file
    done

    cat src/problem/mod.rs >> src/solution/mod.rs

    echo -n "" > src/problem/mod.rs
    
    if [ -n "$files" ]; then 
      echo "https://github.com/LeVuMinhHuy/canada/blob/main/src/solution/$files"
    else
      echo "No files found"
    fi
}

move_files
