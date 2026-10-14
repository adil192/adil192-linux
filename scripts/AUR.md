```bash

# Graphics drivers: https://github.com/CachyOS/CachyOS-PKGBUILDS.git
cd ~/aur/CachyOS-PKGBUILDS/mesa/mesa && git pull --rebase && time makepkg -si


# GNOME compositor: https://gitlab.archlinux.org/archlinux/packaging/packages/mutter.git
cd ~/aur/mutter                      && git pull          && time makepkg -si

# GNOME shell: https://gitlab.archlinux.org/archlinux/packaging/packages/gnome-shell.git
cd ~/aur/gnome-shell                 && git pull          && time makepkg -si

# GTK4: https://gitlab.archlinux.org/archlinux/packaging/packages/gtk4.git
cd ~/aur/gtk4                        && git pull          && time makepkg -si


# Qt theming: https://aur.archlinux.org/qt6ct-kde.git
cd ~/aur/qt6ct-kde                   &&                      time makepkg -si

# PS3 emulator: https://aur.archlinux.org/rpcs3-git.git
cd ~/aur/rpcs3-git                   &&                      time makepkg -si

```
