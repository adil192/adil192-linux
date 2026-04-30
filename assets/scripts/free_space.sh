#!/bin/bash

for dir in ~/Documents/{GitHub,Sources}/{*,*/*}/{.flatpak-builder,builddir,build,dist,node_modules,target}; do
  if [[ -d "$dir" ]]; then
    echo "Deleting $dir"
    rm -rf "$dir"
  fi
done
