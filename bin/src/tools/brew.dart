import 'result_of_command.dart';
import 'which.dart';

abstract class Brew {
  static bool? _hasBrew;
  static Future<bool> get hasBrew async =>
      _hasBrew ??= await Which.installed('brew');

  static List<String>? installedFormulae;
  static Future<bool> installed(String formula) async {
    installedFormulae ??= await resultOfCommand('brew', [
      'list',
      '--full-name',
      '-1',
    ], silent: true).then((String output) => output.split('\n'));
    final isInstalled = installedFormulae!.contains(formula);
    print('brew $formula is ${isInstalled ? 'installed' : 'not installed'}');
    return isInstalled;
  }

  static Future<void> install(String formula) =>
      resultOfCommand('brew', ['install', formula]);
}
