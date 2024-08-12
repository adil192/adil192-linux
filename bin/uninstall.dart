import 'src/tools/yes_or_no.dart';
import 'src/auto_update.dart';
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

  if (await yesOrNo('Disable auto update?')) {
    await disableAutoUpdate();
  }
  print('');

  print('Uninstall complete');
}
