import 'result_of_command.dart';
import 'which.dart';

class Flatpak {
  static final hasFlatpak = Which.installed('flatpak');

  static var _allInstalled = resultOfCommandSync('flatpak', ['list']);
  static bool installed(String name) {
    final installed = _allInstalled.contains(name);
    print('flatpak $name is ${installed ? 'installed' : 'not installed'}');
    return installed;
  }

  static Future<void> install(String name) async {
    await resultOfCommand('flatpak', ['install', name, '-y']);
    _allInstalled += '\n$name';
  }
}
