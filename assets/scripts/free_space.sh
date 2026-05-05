#!/bin/bash

for dir in ~/Documents/{GitHub,Sources}/{*,*/*}/{.flatpak-builder,builddir,build,dist,node_modules,target}; do
  if [[ -d "$dir" ]]; then
    read -p "Delete $dir? (y/N) "
    if [[ $REPLY =~ ^[Yy] ]]; then
      rm -rf "$dir"
    fi
  fi
done
