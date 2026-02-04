import 'src/tools/yes_or_no.dart';
import 'src/config.dart';
import 'src/firefox_cache.dart';
import 'src/firefox.dart';
import 'src/gtk.dart';
import 'src/install_apps.dart';
import 'src/install_codecs.dart';
import 'src/steam.dart';

Future<void> main(List<String> args) async {
  final noInteraction = args.contains('--no-interaction');
  alwaysYes = args.contains('-y') || args.contains('--yes');

  if (shouldInstallCodecs && !noInteraction) await installCodecs();
  if (shouldInstallApps && !noInteraction) await installApps();
  if (shouldThemeFirefox) await installFirefoxCss();
  if (shouldInstallFirefoxCacher) await installFirefoxCacher();
  if (shouldThemeWindowButtons) await installGtkCss();
  if (shouldThemeSteam) await installSteamTheme();
}
