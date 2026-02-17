import 'package:args/args.dart';

final argParser = ArgParser()
  ..addFlag(
    'install-drivers',
    defaultsTo: true,
    help: 'Install drivers and codecs for playing media.',
  )
  ..addFlag(
    'install-apps',
    defaultsTo: true,
    help: 'Install my favourite applications.',
  )
  ..addFlag(
    'theme-firefox',
    defaultsTo: true,
    help: 'Theme Firefox to match my COSMIC theme.',
  )
  ..addFlag(
    'theme-thunderbird',
    defaultsTo: true,
    help: 'Theme Thunderbird to match my COSMIC theme.',
  )
  ..addFlag(
    'install-firefox-cacher',
    defaultsTo: true,
    help:
        'Precache Firefox data on boot.\n'
        'Useful to speed up the first launch of Firefox.',
  )
  ..addFlag(
    'no-interaction',
    abbr: 'n',
    negatable: false,
    help: 'Skip any steps needing confirmation.',
  )
  ..addFlag(
    'yes',
    abbr: 'y',
    negatable: false,
    help: 'Always answer yes to prompts.',
  )
  ..addFlag(
    'help',
    abbr: 'h',
    negatable: false,
    help: 'Show this help message',
  );
