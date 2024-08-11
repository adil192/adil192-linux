import 'dart:io';

import '_utils.dart';

Future<void> installSteamTheme() async {
  final home = Platform.environment['HOME'];
  final repoDir = Directory('$home/Documents/Sources/Adwaita-for-Steam/');
  if (!repoDir.existsSync()) {
    await resultOfCommand('git', [
      'clone',
      'https://github.com/tkashkin/Adwaita-for-Steam',
      repoDir.path,
    ]);
  }

  await resultOfCommand(
    '${repoDir.path}/install.py',
    const [],
    workingDirectory: repoDir.path,
  );
}
