import 'src/tools/yes_or_no.dart';
import 'src/args.dart';
import 'src/firefox_cache.dart';
import 'src/firefox.dart';
import 'src/install_apps.dart';
import 'src/install_codecs.dart';

Future<void> main(List<String> args) async {
  final parsedArgs = argParser.parse(args);
  if (parsedArgs.flag('help')) {
    print(argParser.usage);
    return;
  }

  final noInteraction = parsedArgs.flag('no-interaction');
  alwaysYes = parsedArgs.flag('yes');

  if (!noInteraction && parsedArgs.flag('install-drivers')) {
    await installCodecs();
  }
  if (!noInteraction && parsedArgs.flag('install-apps')) {
    await installApps();
  }
  if (parsedArgs.flag('theme-firefox')) {
    await installFirefoxCss();
  }
  if (parsedArgs.flag('install-firefox-cacher')) {
    await installFirefoxCacher();
  }
}
