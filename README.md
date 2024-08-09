# adil192-linux

This is a collection of scripts that apply my personal preferences to a Linux system.

## Requirements

These scripts are written in Dart,
so you will need to have the Dart SDK installed on your system.
You can either
[install Dart](https://dart.dev/get-dart) directly
or
[install Flutter](https://flutter.dev/docs/get-started/install)
which comes bundled with Dart.

## Installation

1. Clone this repository.

2. Run `dart run` in the root directory of the repository.

Note that by default,
a cron job is set up to update the scripts
and apply the changes every day.
While I'm not going to do anything malicious with this repository,
you should still disable this unless you trust me,
since I could theoretically push a malicious change at any time.
See the [Configuration](#configuration) section for more details.

## Configuration

You can configure which of the changes you want to apply
by editing the [`bin/src/config.dart`](bin/src/config.dart) file.

## Uninstallation

Run `dart run bin/uninstall.dart` in the root directory of the repository
and follow the yes/no prompts.
