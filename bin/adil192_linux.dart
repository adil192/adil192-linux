import 'src/firefox.dart';
import 'src/gtk.dart';

Future<void> main() async {
  await installFirefoxCss();
  await installGtkCss();
}
