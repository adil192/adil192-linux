import 'dart:io';

import '_utils.dart';

Future<void> addCronJob() async {
  final cronContents = await Process.run('crontab', ['-l'])
      .then((result) => result.stdout as String);

  if (cronContents.contains('adil192-linux')) {
    print('Cron job already exists');
    return;
  }

  final pwd = Platform.environment['PWD'];
  final newCronJob = "48 18 * * * root '$pwd/cron.sh'";
  final newCronContents = '$cronContents\n$newCronJob\n';

  final tempFile = File('/tmp/crontab-adil192-linux');
  await tempFile.writeAsString(newCronContents);

  await resultOfCommand('crontab', [tempFile.path]);
}
