import 'dart:io';

import 'package:adil192_linux/src/tools/dnf.dart';
import 'package:adil192_linux/src/tools/yes_or_no.dart';

final _home = Platform.environment['HOME'];
final _pwd = Platform.environment['PWD'];
final _scriptSrc = File('$_pwd/assets/firefox_cache/cache_firefox.sh');
final _desktopSrc = File(
  '$_pwd/assets/firefox_cache/com.adilhanney.cache_firefox.desktop',
);
final _scriptDst = File('$_home/.local/bin/cache_firefox.sh');
final _desktopDst = File(
  '$_home/.config/autostart/com.adilhanney.cache_firefox.desktop',
);

Future<void> installFirefoxCacher() async {
  if (!Platform.isLinux) return;

  await _installVmtouch();

  if (!_scriptDst.parent.existsSync()) {
    _scriptDst.parent.createSync(recursive: true);
  }
  _scriptSrc.copySync(_scriptDst.path);
  print('Copied ${_scriptSrc.path} to ${_scriptDst.path}');

  if (!_desktopDst.parent.existsSync()) {
    _desktopDst.parent.createSync(recursive: true);
  }
  final desktopContent = _desktopSrc.readAsStringSync().replaceAll(
    'Exec=~',
    'Exec=$_home',
  );
  _desktopDst.writeAsStringSync(desktopContent);
  print('Copied ${_desktopSrc.path} to ${_desktopDst.path}');
}

Future<void> uninstallFirefoxCacher() async {
  if (!Platform.isLinux) return;

  if (_scriptDst.existsSync()) {
    _scriptDst.deleteSync();
    print('Deleted ${_scriptDst.path}');
  }

  if (_desktopDst.existsSync()) {
    _desktopDst.deleteSync();
    print('Deleted ${_desktopDst.path}');
  }
}

Future<bool> _installVmtouch() async {
  if (!Dnf.hasDnf) return false;
  if (Dnf.installed('vmtouch')) return true;
  if (!yesOrNo('Install vmtouch for faster precaching?')) return false;
  print('Installing vmtouch...');
  await Dnf.install(['vmtouch']);
  print('');
  return true;
}
