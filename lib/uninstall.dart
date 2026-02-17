#!/usr/bin/env dart

import 'package:adil192_linux/src/tools/yes_or_no.dart';
import 'package:adil192_linux/src/firefox_cache.dart';
import 'package:adil192_linux/src/firefox.dart';
import 'package:adil192_linux/src/thunderbird.dart';

Future<void> main() async {
  if (yesOrNo('Uninstall custom firefox theme?')) {
    uninstallFirefoxCss();
  }
  print('');

  if (yesOrNo('Uninstall custom thunderbird theme?')) {
    uninstallThunderbirdCss();
  }
  print('');

  if (yesOrNo('Uninstall firefox cacher?')) {
    await uninstallFirefoxCacher();
  }
  print('');

  print('Uninstall complete');
}
