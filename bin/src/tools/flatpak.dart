import 'result_of_command.dart';
import 'which.dart';

class Flatpak {
  static bool? _hasFlatpak;
  static Future<bool> get hasFlatpak async =>
      _hasFlatpak ??= await Which.installed('flatpak');

  static Future<void> install(String name) =>
      resultOfCommand('flatpak', ['install', name, '-y']);

  static Future<bool> installed(String name) async {
    final allInstalled = await resultOfCommand('flatpak', [
      'list',
    ], silent: true);
    final installed = allInstalled.contains(name);
    print('flatpak $name is ${installed ? 'installed' : 'not installed'}');
    return installed;
  }
}
