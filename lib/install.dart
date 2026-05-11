#!/usr/bin/env dart

import 'dart:io';

import 'package:adil192_linux/src/tools/run.dart';
import 'package:adil192_linux/src/tools/should_apply_cosmic_theme.dart';
import 'package:adil192_linux/src/tools/which.dart';
import 'package:adil192_linux/src/tools/yes_or_no.dart';
import 'package:adil192_linux/src/args.dart';
import 'package:adil192_linux/src/cosmic_theme_bindings.dart'
    deferred as cosmic_theme_bindings;
import 'package:adil192_linux/src/firefox_cache.dart';
import 'package:adil192_linux/src/firefox.dart';
import 'package:adil192_linux/src/install_apps.dart';
import 'package:adil192_linux/src/install_drivers.dart';
import 'package:adil192_linux/src/thunderbird.dart';

Future<void> main(List<String> args) async {
  assert(Platform.isLinux);

  final parsedArgs = argParser.parse(args);
  if (parsedArgs.flag('help')) {
    print(argParser.usage);
    return;
  }

  final noInteraction = parsedArgs.flag('no-interaction');
  alwaysYes = parsedArgs.flag('yes');

  if (!noInteraction && parsedArgs.flag('install-drivers')) {
    await installDrivers();
  }
  if (!noInteraction && parsedArgs.flag('install-apps')) {
    await installApps();
  }
  if (parsedArgs.flag('install-firefox-cacher')) {
    await installFirefoxCacher();
  }
  if (shouldApplyCosmicTheme) {
    if (parsedArgs.flag('theme-firefox') ||
        parsedArgs.flag('theme-thunderbird')) {
      if (Which.installed('rustc')) {
        // Generate customized CSS if we have rust
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
  } else {
    if (parsedArgs.flag('theme-firefox') ||
        parsedArgs.flag('theme-thunderbird')) {
      print(
        'It doesn\'t look like you\'re using COSMIC, so we\'ll skip theme generation',
      );
    }
  }
}
