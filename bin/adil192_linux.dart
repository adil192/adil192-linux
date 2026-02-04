import 'package:args/args.dart';

import 'src/tools/yes_or_no.dart';
import 'src/config.dart';
import 'src/firefox_cache.dart';
import 'src/firefox.dart';
import 'src/install_apps.dart';
import 'src/install_codecs.dart';

Future<void> main(List<String> args) async {
  final parsedArgs = _argParser.parse(args);
  if (parsedArgs.flag('help')) {
    print(_argParser.usage);
    return;
  }

  final noInteraction = parsedArgs.flag('no-interaction');
  alwaysYes = parsedArgs.flag('yes');

  if (shouldInstallCodecs && !noInteraction) await installCodecs();
  if (shouldInstallApps && !noInteraction) await installApps();
  if (shouldThemeFirefox) await installFirefoxCss();
  if (shouldInstallFirefoxCacher) await installFirefoxCacher();
}

final _argParser = ArgParser()
  ..addFlag(
    'no-interaction',
    abbr: 'n',
    negatable: false,
    help: 'Skip any steps needing confirmation',
  )
  ..addFlag(
    'yes',
    abbr: 'y',
    negatable: false,
    help: 'Always answer yes to prompts',
  )
  ..addFlag(
    'help',
    abbr: 'h',
    negatable: false,
    help: 'Show this help message',
  );
