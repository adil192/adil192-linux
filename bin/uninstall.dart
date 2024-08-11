import 'dart:io';

import 'src/crontab.dart';
import 'src/firefox.dart';
import 'src/gtk.dart';

Future<void> main() async {
  print(
      'If you want to uninstall the gnome firefox theme, uninstall it manually by following the instructions at:');
  print('https://github.com/rafaelmardojai/firefox-gnome-theme#uninstalling');
  print('');

  if (await yesOrNo('Uninstall custom firefox window buttons?')) {
    await uninstallFirefoxWindowButtons();
  }
  print('');

  if (await yesOrNo('Uninstall custom gtk window buttons?')) {
    await uninstallGtkCss();
  }
  print('');

  if (await yesOrNo('Uninstall cron job?')) {
    await removeCronJob();
  }
  print('');

  print('Uninstall complete');
}

Future<bool> yesOrNo(
  String question, [
  String hint = 'Y/n',
  String defaultResponse = 'y',
]) async {
  stdout.write('$question ($hint): ');

  String response;
  do {
    response = stdin.readLineSync()?.toLowerCase() ?? '';
    if (response.isEmpty) response = defaultResponse;
  } while (response != 'y' && response != 'n');

  return response == 'y';
}
