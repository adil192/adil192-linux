import 'dart:io';
import 'dart:math';

import '../_utils.dart';

class Dnf {
  static Future<void> install(List<String> packages) =>
      resultOfCommand('sudo', ['dnf', 'install', ...packages]);

  static Future<void> update(List<String> packages) =>
      resultOfCommand('sudo', ['dnf', 'update', ...packages]);

  static Future<String> repoList() =>
      resultOfCommand('dnf', ['repolist'], silent: true);

  static Future<void> enableCiscoOpenh264() => resultOfCommand(
      'sudo', ['dnf', 'config-manager', '--enable', 'fedora-cisco-openh264']);

  static Future<void> swap(
    String from,
    String to, {
    bool allowErasing = false,
  }) =>
      resultOfCommand('sudo',
          ['dnf', 'swap', from, to, if (allowErasing) '--allowerasing']);

  static List<String>? installedPackages;
  static Future<bool> installed(String package) async {
    installedPackages ??=
        (await resultOfCommand('dnf', ['list', 'installed'], silent: true))
            .split('\n');

    // could be package.x86_64, package.noarch, etc.
    final installed = installedPackages!
        .any((installedPackage) => installedPackage.startsWith('$package.'));

    print('$package is ${installed ? 'installed' : 'not installed'}');

    return installed;
  }
}
