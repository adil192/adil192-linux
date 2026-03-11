# adil192-linux

This is a collection of scripts that apply my personal preferences to a Linux system.
It installs apps, drivers and themes.

It is generally intended to be used with the [Fedora COSMIC Spin](https://fedoraproject.org/spins/cosmic/)
but it will work for other Fedora/RHEL distributions.

It also has partial support for macOS with [Homebrew](https://brew.sh/) since I'm forced to use it for iOS development.

## Installation

```bash
# Cd into a directory where you want to clone the repository
mkdir -p ~/Documents/GitHub/
cd ~/Documents/GitHub/

# Clone the repository
git clone https://github.com/adil192/adil192-linux.git
cd adil192-linux

# Install Flutter if you don't have it yet
./bootstrap/install_flutter.sh

# Run the scripts
dart pub get
./lib/install.dart
```

You can view available options by running `./lib/install.dart --help`.

## Uninstallation

Run `./lib/uninstall.dart` in the root directory of the repository
and follow the yes/no prompts.
