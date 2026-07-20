#!/bin/bash

MODE="$1"
if [[ "$MODE" != dark && "$MODE" != "light" ]]; then
  echo "Argument must be light or dark, not $1."
  exit 1
fi

if [ ! -x "$(command -v crudini)" ]; then
  notify-send --transient --app-name=switch_gnome_theme.sh --icon=dark-mode-symbolic "Cannot switch theme" "Please install crudini"
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
  crudini \
    --set "$CONF" Appearance color_scheme_path "$KCOLORSCHEME" \
    --set "$CONF" Appearance custom_palette true \
    --set "$CONF" Appearance icon_theme "$KICONTHEME" \
    --set "$CONF" Appearance standard_dialogs xdgdesktopportal \
    --set "$CONF" Appearance style Darkly
done
