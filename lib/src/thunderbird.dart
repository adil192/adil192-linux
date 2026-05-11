import 'dart:io';

void installThunderbirdCss() {
  final profileDir = _findThunderbirdProfileDir();
  final pwd = Platform.environment['PWD'] ?? '.';

  for (final cssFileName in ['userChrome.css', 'userContent.css']) {
    /* our firefox-css is also applicable for thunderbird */
    final src = File('$pwd/assets/firefox-css/$cssFileName');
    final dest = _findChromeCssFile(cssFileName, profileDir);

    if (dest.existsSync()) dest.deleteSync();
    dest.parent.createSync(recursive: true);
    Link(dest.path).createSync(src.path);
    print('Linked ${dest.path} to ${src.path}');
  }
}

void uninstallThunderbirdCss() {
  final profileDir = _findThunderbirdProfileDir();
  for (final cssFileName in ['userChrome.css', 'userContent.css']) {
    final dest = _findChromeCssFile(cssFileName, profileDir);
    if (!dest.existsSync()) continue;
    dest.deleteSync();
    print('Deleted ${dest.path}');
  }
}

/// Finds the userChrome.css or userContent.css file for Thunderbird.
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

Directory _findThunderbirdProfileDir() {
  final home = Platform.environment['HOME'];

  final systemProfilesDir = Directory('$home/.thunderbird');
  final flatpakProfilesDir = Directory(
    '$home/.var/app/org.mozilla.Thunderbird/.thunderbird',
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
    throw StateError('Could not find Thunderbird profile: $defaultProfileId');
  }

  return defaultProfileDir;
}
