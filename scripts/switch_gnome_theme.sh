#!/bin/bash
if [ "$1" == "dark" ]; then
  dconf write /org/gnome/shell/extensions/user-theme/name "'Yaru-dark'"
  dconf write /org/gnome/desktop/interface/accent-color "'teal'"
  dconf write /org/gnome/desktop/interface/cursor-theme "'Breeze_Light'"
  dconf write /org/gnome/desktop/interface/gtk-theme "'Yaru-prussiangreen-dark'"
  dconf write /org/gnome/desktop/interface/icon-theme "'Yaru-prussiangreen-dark'"
  KCOLORSCHEME="$HOME/.local/share/color-schemes/CosmicDark.colors"
  [ -f "$KCOLORSCHEME" ] || KCOLORSCHEME="/usr/share/color-schemes/BreezeDark.colors"
  cp "$KCOLORSCHEME" "$HOME/.config/kdeglobals"
  sed -i \
    -e "s|^color_scheme_path=.*|color_scheme_path=$KCOLORSCHEME|" \
    -e 's|^icon_theme=.*|icon_theme=breeze-dark|' \
    ~/.config/qt6ct/qt6ct.conf
  sed -i \
    -e "s|^color_scheme_path=.*|color_scheme_path=$KCOLORSCHEME|" \
    -e 's|^icon_theme=.*|icon_theme=breeze-dark|' \
    ~/.config/qt5ct/qt5ct.conf
elif [ "$1" == "light" ]; then
  dconf write /org/gnome/shell/extensions/user-theme/name "'Yaru'"
  dconf write /org/gnome/desktop/interface/accent-color "'teal'"
  dconf write /org/gnome/desktop/interface/cursor-theme "'Breeze_cursors'"
  dconf write /org/gnome/desktop/interface/gtk-theme "'Yaru-prussiangreen'"
  dconf write /org/gnome/desktop/interface/icon-theme "'Yaru-prussiangreen'"
  KCOLORSCHEME="$HOME/.local/share/color-schemes/CosmicLight.colors"
  [ -f "$KCOLORSCHEME" ] || KCOLORSCHEME="/usr/share/color-schemes/BreezeLight.colors"
  cp "$KCOLORSCHEME" "$HOME/.config/kdeglobals"
  sed -i \
    -e "s|^color_scheme_path=.*|color_scheme_path=$KCOLORSCHEME|" \
    -e 's|^icon_theme=.*|icon_theme=breeze|' \
    ~/.config/qt6ct/qt6ct.conf
  sed -i \
    -e "s|^color_scheme_path=.*|color_scheme_path=$KCOLORSCHEME|" \
    -e 's|^icon_theme=.*|icon_theme=breeze|' \
    ~/.config/qt5ct/qt5ct.conf
else
  echo "Argument must be light or dark, not $1."
  exit 1
fi
