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

Future<void> installFirefoxCss() async {
  if (!Platform.isLinux) return;

  final pwd = Platform.environment['PWD'];
  final target = File('$pwd/assets/firefox-css/userChrome.css');

  final profileDir = await _findFirefoxProfileDir();
  final userChromeCss = await _findUserChromeCss(profileDir);

  if (userChromeCss.existsSync()) await userChromeCss.delete();
  await userChromeCss.parent.create(recursive: true);
  await Link(userChromeCss.path).create(target.path);
  print('Linked ${userChromeCss.path} to ${target.path}');

  await _alterUserJs(profileDir);
}

Future<void> uninstallFirefoxWindowButtons() async {
  if (!Platform.isLinux) return;

  final profileDir = await _findFirefoxProfileDir();
  final userChromeCss = await _findUserChromeCss(profileDir);
  if (userChromeCss.existsSync()) await userChromeCss.delete();
  print('Deleted ${userChromeCss.path}');
}

/// Alters Firefox's user.js
/// (i.e. changes settings in about:config)
Future<void> _alterUserJs(Directory profileDir) async {
  final userJs = File('${profileDir.path}/user.js');
  if (!userJs.existsSync()) throw StateError('Could not find user.js');

  final lines = await userJs.readAsLines();
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
    await userJs.writeAsString(lines.join('\n'));
    print('Altered $settingsAltered settings in user.js');
  } else {
    print('No settings altered in user.js');
  }
}

/// Finds the userChrome.css file for Firefox.
Future<FileSystemEntity> _findUserChromeCss(Directory profileDir) async {
  final userChromeCssPath = '${profileDir.path}/chrome/userChrome.css';
  final type = FileSystemEntity.typeSync(userChromeCssPath, followLinks: false);
  return switch (type) {
    .file => File(userChromeCssPath),
    .link || .notFound => Link(userChromeCssPath),
    _ => throw StateError('Warning: Unknown type $type for $userChromeCssPath'),
  };
}

Future<Directory> _findFirefoxProfileDir() async {
  final home = Platform.environment['HOME'];

  final systemProfilesDir = Directory('$home/.mozilla/firefox');
  final flatpakProfilesDir = Directory(
    '$home/.var/app/org.mozilla.firefox/.mozilla/firefox',
  );
  final profilesDir = systemProfilesDir.existsSync()
      ? systemProfilesDir
      : flatpakProfilesDir;

  final installsIni = File('${profilesDir.path}/installs.ini');
  final installsIniContent = await installsIni.readAsLines();
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
