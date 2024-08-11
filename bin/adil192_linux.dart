import 'src/config.dart';
import 'src/crontab.dart';
import 'src/firefox.dart';
import 'src/gtk.dart';
import 'src/install_apps.dart';
import 'src/install_codecs.dart';
import 'src/steam.dart';

Future<void> main(List<String> args) async {
  final noInteraction = args.contains('--no-interaction');

  if (shouldInstallCodecs && !noInteraction) await installCodecs();
  if (shouldInstallApps && !noInteraction) await installApps();
  if (shouldThemeFirefox) await installFirefoxCss();
  if (shouldThemeWindowButtons) await installGtkCss();
  if (shouldThemeSteam) await installSteamTheme();
  if (shouldAutomaticallyUpdate) await addCronJob();
}
