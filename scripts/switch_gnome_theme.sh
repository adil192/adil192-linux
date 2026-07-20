#!/bin/bash

MODE="$1"
if [[ "$MODE" != dark && "$MODE" != "light" ]]; then
  echo "Argument must be light or dark, not $1."
  exit 1
fi

light-dark() {
  [ "$MODE" == "light" ] && echo "$1" || echo "$2"
}

gsettings set org.gnome.shell.extensions.user-theme name "$(light-dark Yaru Yaru-dark)"
gsettings set org.gnome.desktop.interface accent-color teal
gsettings set org.gnome.desktop.interface cursor-theme "$(light-dark Breeze_cursors Breeze_Light)"
gsettings set org.gnome.desktop.interface gtk-theme "$(light-dark Yaru-prussiangreen Yaru-prussiangreen-dark)"
gsettings set org.gnome.desktop.interface icon-theme "$(light-dark Yaru-prussiangreen Yaru-prussiangreen-dark)"

KCOLORSCHEME="/usr/share/color-schemes/$(light-dark BreezeLight.colors BreezeDark.colors)"
KICONTHEME=$(light-dark breeze breeze-dark)
cp "$KCOLORSCHEME" ~/.config/kdeglobals
for CONF in ~/.config/qt5ct/qt5ct.conf ~/.config/qt6ct/qt6ct.conf; do
  sed -i \
    -e "s|^color_scheme_path=.*|color_scheme_path=$KCOLORSCHEME|" \
    -e "s|^icon_theme=.*|icon_theme=$KICONTHEME|" \
    "$CONF"
done
