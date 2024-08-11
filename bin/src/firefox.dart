import 'dart:convert';
import 'dart:io';

import 'config.dart';

Future<void> installFirefoxCss() async {
  final pwd = Platform.environment['PWD'];
  final target = File('$pwd/assets/firefox-css/customChrome.css');

  final profileDir = await _findFirefoxProfileDir();
  final customChromeCss = await _findCustomChromeCss(profileDir);

  if (shouldThemeWindowButtons) {
    if (customChromeCss.existsSync()) await customChromeCss.delete();
    await Link(customChromeCss.path).create(target.path);
    print('Linked ${customChromeCss.path} to ${target.path}');
  }

  await _alterUserJs(profileDir);
}

Future<void> uninstallFirefoxWindowButtons() async {
  final profileDir = await _findFirefoxProfileDir();
  final customChromeCss = await _findCustomChromeCss(profileDir);
  if (customChromeCss.existsSync()) await customChromeCss.delete();
  print('Deleted ${customChromeCss.path}');
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

  const newSettings = {
    'widget.gtk.rounded-bottom-corners.enabled': true,
    'gnomeTheme.bookmarksToolbarUnderTabs': true,
    'gnomeTheme.normalWidthTabs': true,
  };
  for (final entry in newSettings.entries) {
    setSetting(entry.key, entry.value);
  }

  if (settingsAltered > 0) {
    await userJs.writeAsString(lines.join('\n'));
    print('Altered $settingsAltered settings in user.js');
  } else {
    print('No settings altered in user.js');
  }
}

/// Finds the customChrome.css file for Firefox.
Future<FileSystemEntity> _findCustomChromeCss(Directory profileDir) async {
  final customChromeCssPath =
      '${profileDir.path}/chrome/firefox-gnome-theme/customChrome.css';
  final type =
      FileSystemEntity.typeSync(customChromeCssPath, followLinks: false);
  switch (type) {
    case FileSystemEntityType.file:
      return File(customChromeCssPath);
    case FileSystemEntityType.link:
    case FileSystemEntityType.notFound:
      return Link(customChromeCssPath);
    default:
      throw StateError('Warning: Unknown type $type for $customChromeCssPath');
  }
}

Future<Directory> _findFirefoxProfileDir({
  bool downloadIfNotFound = true,
}) async {
  final home = Platform.environment['HOME'];
  final profilesDir = Directory('$home/.mozilla/firefox');

  for (final profileDir in profilesDir.listSync()) {
    if (profileDir is! Directory) continue;
    final readme =
        File('${profileDir.path}/chrome/firefox-gnome-theme/README.md');
    if (readme.existsSync()) return profileDir;
  }

  if (downloadIfNotFound) {
    print('Could not find customChrome.css, installing Firefox theme...');
    await _installFirefoxTheme();
    return _findFirefoxProfileDir(downloadIfNotFound: false);
  } else {
    throw StateError('Could not find customChrome.css');
  }
}

Future<void> _installFirefoxTheme() async {
  final curl = await Process.start('curl', [
    '-s',
    '-o-',
    'https://raw.githubusercontent.com/rafaelmardojai/firefox-gnome-theme/master/scripts/install-by-curl.sh',
  ]);
  final bash = await Process.start('bash', []);
  curl.stdout.pipe(bash.stdin);
  stdout.addStream(bash.stdout);
  stderr.addStream(bash.stderr);
  await bash.exitCode;
}
