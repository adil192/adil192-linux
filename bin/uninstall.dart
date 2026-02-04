import 'src/tools/yes_or_no.dart';
import 'src/firefox_cache.dart';
import 'src/firefox.dart';

Future<void> main() async {
  if (await yesOrNo('Uninstall custom firefox window buttons?')) {
    await uninstallFirefoxWindowButtons();
  }
  print('');

  if (await yesOrNo('Uninstall firefox cacher?')) {
    await uninstallFirefoxCacher();
  }
  print('');

  print('Uninstall complete');
}
