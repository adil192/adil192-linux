#!/bin/bash

if [ ! -f Cargo.toml ]; then
  echo "Error: Run this script from the project root!"
  exit 1
fi

sass \
  assets/gtk/gtk3.scss:my_home/.config/gtk-3.0/gtk.css \
  assets/gtk/gtk4.scss:my_home/.config/gtk-4.0/gtk.css
