#!/usr/bin/env dart

import 'package:adil192_linux/src/tools/run.dart';
import 'package:adil192_linux/src/tools/which.dart';
import 'package:adil192_linux/src/tools/yes_or_no.dart';
import 'package:adil192_linux/src/args.dart';
import 'package:adil192_linux/src/cosmic_theme_bindings.dart'
    deferred as cosmic_theme_bindings;
import 'package:adil192_linux/src/firefox_cache.dart';
import 'package:adil192_linux/src/firefox.dart';
import 'package:adil192_linux/src/install_apps.dart';
import 'package:adil192_linux/src/install_codecs.dart';
import 'package:adil192_linux/src/thunderbird.dart';

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
  if (parsedArgs.flag('theme-firefox') ||
      parsedArgs.flag('theme-thunderbird')) {
    if (Which.installed('rustc')) {
      await run('./bootstrap/run_ffigen.sh', []);
      await cosmic_theme_bindings.loadLibrary();
      cosmic_theme_bindings.generateCosmicTokensCss();
    }
  }
  if (parsedArgs.flag('theme-firefox')) {
    installFirefoxCss();
  }
  if (parsedArgs.flag('theme-thunderbird')) {
    installThunderbirdCss();
  }
  if (parsedArgs.flag('install-firefox-cacher')) {
    await installFirefoxCacher();
  }
}
