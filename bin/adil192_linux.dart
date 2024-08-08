import 'src/firefox.dart';
import 'src/gtk.dart';
import 'src/crontab.dart';

Future<void> main() async {
  await installFirefoxCss();
  await installGtkCss();
  await addCronJob();
}
