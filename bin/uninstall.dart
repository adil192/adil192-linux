import 'src/tools/yes_or_no.dart';
import 'src/firefox_cache.dart';
import 'src/firefox.dart';

Future<void> main() async {
  if (yesOrNo('Uninstall custom firefox theme?')) {
    uninstallFirefoxWindowButtons();
  }
  print('');

  if (yesOrNo('Uninstall firefox cacher?')) {
    await uninstallFirefoxCacher();
  }
  print('');

  print('Uninstall complete');
}
