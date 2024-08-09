import 'dart:io';

import '_utils.dart';
import 'config.dart';

Future<void> addCronJob() async {
  final cronContents = await _readCrontab();

  if (cronContents.contains('adil192-linux')) {
    print('Cron job already exists');
    return;
  }

  final pwd = Platform.environment['PWD'];
  final newCronJob = "$updateSchedule root '$pwd/cron.sh'";
  final newCronContents = '$cronContents\n$newCronJob\n';

  await _writeCrontab(newCronContents);
}

Future<void> removeCronJob() async {
  final oldCronContents = await _readCrontab();
  final oldCronLines = oldCronContents.split('\n');

  final newCronLines =
      oldCronLines.where((line) => !line.contains('adil192-linux')).toList();
  final newCronContents = newCronLines.join('\n');

  final linesRemoved = oldCronLines.length - newCronLines.length;
  if (linesRemoved == 0) {
    print('Cron job not found');
    return;
  } else if (linesRemoved > 1) {
    print('Found multiple lines to remove, aborting as a safety measure');
    return;
  }

  await _writeCrontab(newCronContents);
}

Future<String> _readCrontab() =>
    Process.run('crontab', ['-l']).then((result) => result.stdout as String);

Future<void> _writeCrontab(String contents) async {
  final tempFile = File('/tmp/crontab-adil192-linux');
  await tempFile.writeAsString(contents);

  await resultOfCommand('crontab', [tempFile.path]);
}
