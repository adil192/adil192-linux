#!/bin/bash

# Remove temporary development files
for dir in ~/Documents/{GitHub,Sources}/{*,*/*}/{.flatpak-builder,builddir,build,dist,node_modules,target}; do
  if [[ -d "$dir" ]]; then
    read -p "Delete $dir? (y/N) "
    if [[ $REPLY =~ ^[Yy] ]]; then
      rm -rf "$dir"
    fi
  fi
done

# Prune unused Dart packages
if command -v dart; then
  dart pub cache gc
fi
