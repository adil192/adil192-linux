import 'package:adil192_linux/src/tools/run.dart';
import 'package:adil192_linux/src/tools/which.dart';
import 'package:adil192_linux/src/tools/yes_or_no.dart';

class Flatpak {
  static final hasFlatpak = Which.installed('flatpak');

  static var _allInstalled = runSilent('flatpak', ['list']);
  static bool installed(String name) {
    final installed = _allInstalled.contains(name);
    print('flatpak $name is ${installed ? 'installed' : 'not installed'}');
    return installed;
  }

  static Future<void> install(String name) async {
    await run('flatpak', ['install', name, ?yFlag]);
    _allInstalled += '\n$name';
  }
}
