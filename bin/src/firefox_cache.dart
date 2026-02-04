import 'dart:io';

import 'tools/dnf.dart';
import 'tools/yes_or_no.dart';

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
    await _scriptDst.parent.create(recursive: true);
  }
  await _scriptSrc.copy(_scriptDst.path);
  print('Copied ${_scriptSrc.path} to ${_scriptDst.path}');

  if (!_desktopDst.parent.existsSync()) {
    await _desktopDst.parent.create(recursive: true);
  }
  var desktopContent = await _desktopSrc.readAsString();
  desktopContent = desktopContent.replaceAll('Exec=~', 'Exec=$_home');
  await _desktopDst.writeAsString(desktopContent);
  print('Copied ${_desktopSrc.path} to ${_desktopDst.path}');
}

Future<void> uninstallFirefoxCacher() async {
  if (!Platform.isLinux) return;

  if (_scriptDst.existsSync()) {
    await _scriptDst.delete();
    print('Deleted ${_scriptDst.path}');
  }

  if (_desktopDst.existsSync()) {
    await _desktopDst.delete();
    print('Deleted ${_desktopDst.path}');
  }
}

Future<bool> _installVmtouch() async {
  if (!await Dnf.hasDnf) return false;
  if (await Dnf.installed('vmtouch')) return true;
  if (!await yesOrNo('Install vmtouch for faster precaching?')) return false;
  print('Installing vmtouch...');
  await Dnf.install(['vmtouch']);
  print('');
  return true;
}
