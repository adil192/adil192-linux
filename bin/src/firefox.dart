import 'dart:io';

Future<void> installFirefoxCss() async {
  final pwd = Platform.environment['PWD'];
  final target = File('$pwd/assets/firefox-css/customChrome.css');

  final customChromeCss = await findCustomChromeCss();

  if (customChromeCss.existsSync()) await customChromeCss.delete();
  await Link(customChromeCss.path).create(target.path);

  print('Linked ${customChromeCss.path} to ${target.path}');
}

/// Finds the customChrome.css file for Firefox.
Future<FileSystemEntity> findCustomChromeCss({
  bool downloadIfNotFound = true,
}) async {
  final home = Platform.environment['HOME'];
  final profilesDir = Directory('$home/.mozilla/firefox');

  Directory? installedProfileDir;
  for (final profileDir in profilesDir.listSync()) {
    if (profileDir is! Directory) continue;
    final readme =
        File('${profileDir.path}/chrome/firefox-gnome-theme/README.md');
    if (readme.existsSync()) {
      installedProfileDir = profileDir;
      break;
    }
  }

  if (installedProfileDir != null) {
    final customChromeCssPath =
        '${installedProfileDir.path}/chrome/firefox-gnome-theme/customChrome.css';
    final type =
        FileSystemEntity.typeSync(customChromeCssPath, followLinks: false);
    switch (type) {
      case FileSystemEntityType.file:
        return File(customChromeCssPath);
      case FileSystemEntityType.link:
      case FileSystemEntityType.notFound:
        return Link(customChromeCssPath);
      default:
        print('Warning: Unknown type $type for $customChromeCssPath');
    }
  }

  if (downloadIfNotFound) {
    print('Could not find customChrome.css, installing Firefox theme...');
    await _installFirefoxTheme();
    return findCustomChromeCss(downloadIfNotFound: false);
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
