import 'src/firefox.dart';
import 'src/gtk.dart';
import 'src/crontab.dart';
import 'src/config.dart';
import 'src/steam.dart';

Future<void> main() async {
  if (shouldThemeFirefox) await installFirefoxCss();
  if (shouldThemeWindowButtons) await installGtkCss();
  if (shouldThemeSteam) await installSteamTheme();
  if (shouldAutomaticallyUpdate) await addCronJob();
}
