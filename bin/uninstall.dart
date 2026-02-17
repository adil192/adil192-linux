import 'src/tools/yes_or_no.dart';
import 'src/firefox_cache.dart';
import 'src/firefox.dart';
import 'src/thunderbird.dart';

Future<void> main() async {
  if (yesOrNo('Uninstall custom firefox theme?')) {
    uninstallFirefoxCss();
  }
  print('');

  if (yesOrNo('Uninstall custom thunderbird theme?')) {
    uninstallThunderbirdCss();
  }
  print('');

  if (yesOrNo('Uninstall firefox cacher?')) {
    await uninstallFirefoxCacher();
  }
  print('');

  print('Uninstall complete');
}
