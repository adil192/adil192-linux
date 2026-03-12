import 'dart:convert';
import 'dart:io';

import 'package:adil192_linux/src/cosmic_theme_bindings.dart';

const _customSettings = {
  'widget.gtk.rounded-bottom-corners.enabled': true,
  'gnomeTheme.bookmarksToolbarUnderTabs': true,
  'gnomeTheme.normalWidthTabs': true,
  // Replaces the Fedora start page
  'browser.startup.homepage': 'about:newtab',
  // Enables userChrome.css
  'toolkit.legacyUserProfileCustomizations.stylesheets': true,
};

void installFirefoxCss() {
  if (!Platform.isLinux) return;

  final profileDir = _findFirefoxProfileDir();
  final pwd = Platform.environment['PWD'] ?? '.';

  generateCosmicTokensCss();
  for (final cssFileName in ['userChrome.css', 'userContent.css']) {
    final src = File('$pwd/assets/firefox-css/$cssFileName');
    final dest = _findChromeCssFile(cssFileName, profileDir);

    if (dest.existsSync()) dest.deleteSync();
    dest.parent.createSync(recursive: true);
    Link(dest.path).createSync(src.path);
    print('Linked ${dest.path} to ${src.path}');
  }

  _alterUserJs(profileDir);
}

void uninstallFirefoxCss() {
  if (!Platform.isLinux) return;

  final profileDir = _findFirefoxProfileDir();
  for (final cssFileName in ['userChrome.css', 'userContent.css']) {
    final dest = _findChromeCssFile(cssFileName, profileDir);
    if (!dest.existsSync()) continue;
    dest.deleteSync();
    print('Deleted ${dest.path}');
  }
}

/// Alters Firefox's user.js
/// (i.e. changes settings in about:config)
void _alterUserJs(Directory profileDir) {
  final userJs = File('${profileDir.path}/user.js');
  if (!userJs.existsSync()) throw StateError('Could not find user.js');

  final lines = userJs.readAsLinesSync();
  int settingsAltered = 0;
  void setSetting(String key, dynamic value) {
    final newLine = 'user_pref("$key", ${jsonEncode(value)});';
    for (int l = 0; l < lines.length; ++l) {
      final line = lines[l];
      if (line.startsWith('user_pref("$key",')) {
        if (line != newLine) settingsAltered++;
        lines[l] = newLine;
        return;
      }
    }
    settingsAltered++;
    lines.add(newLine);
  }

  for (final entry in _customSettings.entries) {
    setSetting(entry.key, entry.value);
  }

  if (settingsAltered > 0) {
    userJs.writeAsStringSync(lines.join('\n'));
    print('Altered $settingsAltered settings in user.js');
  } else {
    print('No settings altered in user.js');
  }
}

/// Finds the userChrome.css or userContent.css file for Firefox.
FileSystemEntity _findChromeCssFile(String cssFileName, Directory profileDir) {
  assert(cssFileName.endsWith('.css'));
  final cssFilePath = '${profileDir.path}/chrome/$cssFileName';
  final type = FileSystemEntity.typeSync(cssFilePath, followLinks: false);
  return switch (type) {
    .file => File(cssFilePath),
    .link || .notFound => Link(cssFilePath),
    _ => throw StateError('Warning: Unknown type $type for $cssFilePath'),
  };
}

Directory _findFirefoxProfileDir() {
  final home = Platform.environment['HOME'];

  final systemProfilesDir = Directory('$home/.mozilla/firefox');
  final flatpakProfilesDir = Directory(
    '$home/.var/app/org.mozilla.firefox/.mozilla/firefox',
  );
  final profilesDir = systemProfilesDir.existsSync()
      ? systemProfilesDir
      : flatpakProfilesDir;

  final installsIni = File('${profilesDir.path}/installs.ini');
  final installsIniContent = installsIni.readAsLinesSync();
  // Find the line with `Default=8972389472934.default-release`
  final defaultProfileId = installsIniContent
      .firstWhere((line) => line.startsWith('Default='))
      .substring('Default='.length);

  final defaultProfileDir = Directory('${profilesDir.path}/$defaultProfileId');
  if (!defaultProfileDir.existsSync()) {
    throw StateError('Could not find Firefox profile: $defaultProfileId');
  }

  return defaultProfileDir;
}
