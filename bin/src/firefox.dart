import 'dart:convert';
import 'dart:io';

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

  final pwd = Platform.environment['PWD'];
  final target = File('$pwd/assets/firefox-css/userChrome.css');

  final profileDir = _findFirefoxProfileDir();
  final userChromeCss = _findUserChromeCss(profileDir);

  if (userChromeCss.existsSync()) userChromeCss.deleteSync();
  userChromeCss.parent.createSync(recursive: true);
  Link(userChromeCss.path).createSync(target.path);
  print('Linked ${userChromeCss.path} to ${target.path}');

  _alterUserJs(profileDir);
}

void uninstallFirefoxWindowButtons() {
  if (!Platform.isLinux) return;

  final profileDir = _findFirefoxProfileDir();
  final userChromeCss = _findUserChromeCss(profileDir);
  if (userChromeCss.existsSync()) userChromeCss.deleteSync();
  print('Deleted ${userChromeCss.path}');
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

/// Finds the userChrome.css file for Firefox.
FileSystemEntity _findUserChromeCss(Directory profileDir) {
  final userChromeCssPath = '${profileDir.path}/chrome/userChrome.css';
  final type = FileSystemEntity.typeSync(userChromeCssPath, followLinks: false);
  return switch (type) {
    .file => File(userChromeCssPath),
    .link || .notFound => Link(userChromeCssPath),
    _ => throw StateError('Warning: Unknown type $type for $userChromeCssPath'),
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
