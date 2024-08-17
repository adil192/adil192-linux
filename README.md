# adil192-linux

This is a collection of scripts that apply my personal preferences to a Linux system.

## Requirements

These scripts are written in Dart,
so if you don't have Dart (or Flutter) installed,
run `./bootstrap/install_flutter.sh` to install Flutter+Dart
or install Dart/Flutter manually.

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

Note that by default,
a timer is set up to update the scripts
and apply the changes every day.
While I'm not going to do anything malicious with this repository,
you should still disable this unless you trust me,
since I could theoretically push a malicious change at any time.
Set `shouldAutomaticallyUpdate` to `false` in
[`bin/src/config.dart`](bin/src/config.dart) to disable this.

## Uninstallation

Run `dart run bin/uninstall.dart` in the root directory of the repository
and follow the yes/no prompts.
