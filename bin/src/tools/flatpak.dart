import 'result_of_command.dart';

class Flatpak {
  static Future<void> install(String name) =>
      resultOfCommand('flatpak', ['install', name, '-y']);

  static Future<bool> installed(String name) async {
    final allInstalled =
        await resultOfCommand('flatpak', ['list'], silent: true);
    final installed = allInstalled.contains(name);
    print('flatpak $name is ${installed ? 'installed' : 'not installed'}');
    return installed;
  }
}
