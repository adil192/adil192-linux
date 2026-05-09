#!/bin/bash

set -e
mkdir -p ~/Documents/GitHub/ && cd ~/Documents/GitHub/

if [ "$EUID" -eq 0 ]; then
  echo "Run this script as a regular user, not as root!"
  exit 1
fi

echo "This installs UNRELEASED WIP versions of critical system components."
echo "Frosted glass is really cool, but don't expect support if you run into issues!"
echo

echo "Please grant sudo access to install system components."
sudo echo "sudo granted!"
echo

echo "Starting the builds. This will take a while..."
echo "If the builds fail, ensure you have the necessary dependencies: https://github.com/pop-os/cosmic-epoch#setup-on-distributions-without-packaging-of-cosmic-components"

install() {
  PROJECT=$1
  BRANCH=$2
  [ -d "$PROJECT" ] || git clone "https://github.com/pop-os/$PROJECT.git"
  pushd "$PROJECT"
  git fetch && git switch "$BRANCH" && git pull
  if [ -f Makefile ]; then
    make && sudo make install
  else
    just && sudo just install
  fi
  popd
}

# Suppress warnings since they flood the logs
export RUSTFLAGS=-Awarnings

install cosmic-app-library theme-v2
install cosmic-applets theme-v2
install cosmic-comp frosted-glass_noble
install cosmic-edit theme-v2
install cosmic-files theme-v2
install cosmic-greeter theme-v2
install cosmic-launcher theme-v2
install cosmic-osd theme-v2
install cosmic-panel theme-v2
install cosmic-settings theme-v2
install cosmic-store theme-v2
install cosmic-term theme-v2
install cosmic-workspaces-epoch theme-v2
install xdg-desktop-portal-cosmic theme-v2

echo
echo "All done! Changes will take effect after a reboot/relogin."
echo
echo "If you wish to revert back to the stable versions, reinstall the official packages like this:"
echo "    sudo apt install --reinstall cosmic-app-library cosmic-applets cosmic-comp cosmic-edit cosmic-files cosmic-greeter cosmic-launcher cosmic-osd cosmic-panel cosmic-settings cosmic-store cosmic-term cosmic-workspaces xdg-desktop-portal-cosmic"
