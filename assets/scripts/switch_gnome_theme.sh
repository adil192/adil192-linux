#!/bin/bash
if [ "$1" == "dark" ]; then
  dconf write /org/gnome/shell/extensions/user-theme/name "'Yaru-dark'"
  dconf write /org/gnome/desktop/interface/accent-color "'teal'"
  dconf write /org/gnome/desktop/interface/cursor-theme "'Yaru'"
  dconf write /org/gnome/desktop/interface/gtk-theme "'Yaru-prussiangreen-dark'"
  dconf write /org/gnome/desktop/interface/icon-theme "'Yaru-prussiangreen-dark'"
elif [ "$1" == "light" ]; then
  dconf write /org/gnome/shell/extensions/user-theme/name "'Yaru'"
  dconf write /org/gnome/desktop/interface/accent-color "'teal'"
  dconf write /org/gnome/desktop/interface/cursor-theme "'Yaru'"
  dconf write /org/gnome/desktop/interface/gtk-theme "'Yaru-prussiangreen'"
  dconf write /org/gnome/desktop/interface/icon-theme "'Yaru-prussiangreen'"
else
  echo "Argument must be light or dark, not $1."
  exit 1
fi
