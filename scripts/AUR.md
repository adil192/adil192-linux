```bash

alias makepkgplus="time game-performance makepkg -si"

# Graphics drivers: https://github.com/CachyOS/CachyOS-PKGBUILDS.git
cd ~/aur/CachyOS-PKGBUILDS/mesa/mesa && git pull --rebase && makepkgplus


# GNOME compositor: https://gitlab.archlinux.org/archlinux/packaging/packages/mutter.git
# cd ~/aur/mutter                      && git pull          && makepkgplus
cd ~/aur/CachyOS-PKGBUILDS/mutter    && git pull --rebase && makepkgplus

# GNOME shell: https://gitlab.archlinux.org/archlinux/packaging/packages/gnome-shell.git
cd ~/aur/gnome-shell                 && git pull          && makepkgplus

# GTK4: https://gitlab.archlinux.org/archlinux/packaging/packages/gtk4.git
cd ~/aur/gtk4                        && git pull          && makepkgplus


# Qt theming: https://aur.archlinux.org/qt6ct-kde.git
cd ~/aur/qt6ct-kde                   &&                      makepkgplus

# PS3 emulator: https://aur.archlinux.org/rpcs3-git.git
cd ~/aur/rpcs3-git                   &&                      makepkgplus

```
