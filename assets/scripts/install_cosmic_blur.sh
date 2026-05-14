#!/bin/bash

set -e
mkdir -p ~/Documents/Sources/ && cd ~/Documents/Sources/

if [[ "${EUID}" -eq 0 ]]; then
  echo "Run this script as a regular user, not as root!"
  exit 1
fi

echo "This installs UNRELEASED WIP versions of critical system components."
echo "Frosted glass is really cool, but don't expect support if you run into issues!"
echo

echo "Please grant sudo access to install system components."
sudo echo "sudo granted!"
echo

echo "Downloading the source code. This will take a while..."
echo "You will need around 50GB of free disk space."
echo

clone() {
  PROJECT=$1
  BRANCH=$2
  [[ -d "${PROJECT}" ]] || git clone "https://github.com/pop-os/${PROJECT}.git" -b "${BRANCH}" --recurse-submodules
  git -C "${PROJECT}" fetch
  git -C "${PROJECT}" switch "${BRANCH}"
  git -C "${PROJECT}" pull
}
clone cosmic-app-library theme-v2
clone cosmic-applets theme-v2
clone cosmic-comp frosted-glass_noble
clone cosmic-edit theme-v2
clone cosmic-files theme-v2
clone cosmic-greeter theme-v2
clone cosmic-launcher theme-v2
clone cosmic-osd theme-v2
clone cosmic-panel theme-v2
clone cosmic-settings theme-v2
clone cosmic-store theme-v2
clone cosmic-term theme-v2
clone cosmic-workspaces-epoch theme-v2
clone libcosmic theme-v2
clone xdg-desktop-portal-cosmic theme-v2

# There isn't an official theme-v2 branch of cosmic-settings-daemon yet so use mine.
# Without this, gtk/qt apps will be stuck on whatever theme you had before installing theme-v2.
[[ -d "cosmic-settings-daemon" ]] || git clone "https://github.com/pop-os/cosmic-settings-daemon.git" --recurse-submodules
git -C "cosmic-settings-daemon" remote add adil192 "https://github.com/adil192/cosmic-settings-daemon.git" || true
git -C "cosmic-settings-daemon" fetch adil192
git -C "cosmic-settings-daemon" switch theme-v2-unofficial
git -C "cosmic-settings-daemon" pull

echo
echo "Starting the builds. This will take a while..."
echo "If the builds fail, ensure you have the necessary dependencies: https://github.com/pop-os/cosmic-epoch#setup-on-distributions-without-packaging-of-cosmic-components"
echo
sleep 3

# Suppress warnings since they flood the logs
export RUSTFLAGS=-Awarnings
install() {
  PROJECT=$1
  pushd "${PROJECT}"
  if [[ -f Makefile ]]; then
    make && sudo make install
  else
    just && sudo just install
  fi
  popd
}
install cosmic-app-library
install cosmic-applets
install cosmic-comp
install cosmic-edit
install cosmic-files
install cosmic-greeter
install cosmic-launcher
install cosmic-osd
install cosmic-panel
install cosmic-settings
install cosmic-settings-daemon
install cosmic-store
install cosmic-term
install cosmic-workspaces-epoch
install xdg-desktop-portal-cosmic

echo
echo "All done! Changes will take effect after a reboot/relogin."
echo
echo "If you wish to revert back to the stable versions, reinstall the official packages like this:"
echo "    sudo apt install --reinstall cosmic-app-library cosmic-applets cosmic-comp cosmic-edit cosmic-files cosmic-greeter cosmic-launcher cosmic-osd cosmic-panel cosmic-settings cosmic-store cosmic-term cosmic-workspaces xdg-desktop-portal-cosmic"
