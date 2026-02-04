import 'result_of_command.dart';
import 'which.dart';

abstract class Brew {
  static final hasBrew = Which.installed('brew');

  static final installedFormulae = resultOfCommandSync('brew', [
    'list',
    '--full-name',
    '-1',
  ]).split('\n');
  static bool installed(String formula) {
    final isInstalled = installedFormulae.contains(formula);
    print('brew $formula is ${isInstalled ? 'installed' : 'not installed'}');
    return isInstalled;
  }

  static Future<void> install(String formula) =>
      resultOfCommand('brew', ['install', formula]);
}
