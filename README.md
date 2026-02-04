# adil192-linux

This is a collection of scripts that apply my personal preferences to a Linux system
(plus limited support on macOS).

## Requirements

These scripts are written in Dart,
so if you don't have Dart (or Flutter) installed,
run `./bootstrap/install_flutter.sh` to install Flutter+Dart
or install Dart/Flutter manually.

On macOS, please install [Homebrew](https://brew.sh/).

## Installation

```bash
# Cd into a directory where you want to clone the repository
mkdir -p ~/Documents/GitHub/
cd ~/Documents/GitHub/

# Clone the repository
git clone https://github.com/adil192/adil192-linux.git
cd adil192-linux

# Consider altering the config before running the scripts
gnome-text-editor bin/src/config.dart

# Run the scripts
dart pub get
dart run
```

## Uninstallation

Run `dart run bin/uninstall.dart` in the root directory of the repository
and follow the yes/no prompts.
