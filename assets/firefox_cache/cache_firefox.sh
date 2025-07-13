#!/bin/bash

# Precaches a directory using vmtouch, or cat if vmtouch is not available.
# Note that vmtouch is recommended as it is much faster.
function precache() {
    local dir="$1"
    if [ ! -d "$dir" ]; then
        echo "Directory $dir does not exist. Skipping precache."
        return
    fi

    if command -v vmtouch &> /dev/null; then
        echo "Using vmtouch to precache $dir..."
        vmtouch -tf "$dir"
    else
        echo "vmtouch not found: Using cat to precache $dir..."
        find "$dir" -type f -exec cat {} > /dev/null \;
    fi
    echo
}

precache ~/.mozilla/firefox/
precache ~/.cache/mozilla/firefox/
