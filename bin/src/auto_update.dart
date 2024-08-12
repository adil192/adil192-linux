import 'dart:io';

import 'tools/result_of_command.dart';
import 'config.dart';

const _serviceName = 'adil192-linux.service';
const _timerName = 'adil192-linux.timer';

final _serviceFile = File('/etc/systemd/user/$_serviceName');
final _timerFile = File('/etc/systemd/user/$_timerName');

final _pwd = Platform.environment['PWD'] ?? Directory.current.path;
final _serviceTemplateFile = File('$_pwd/assets/auto_update/$_serviceName');
final _timerTemplateFile = File('$_pwd/assets/auto_update/$_timerName');

Future<void> enableAutoUpdate() async {
  final serviceFileExists = _serviceFile.existsSync();
  final timerFileExists = _timerFile.existsSync();

  if (!serviceFileExists || true) {
    var content = await _serviceTemplateFile.readAsString();
    content = content
        .replaceAll('\$WORKINGDIRECTORY', _pwd)
        .replaceAll('\$USER', Platform.environment['USER'] ?? 'ahann')
        .replaceAll('\$PATH', Platform.environment['PATH']!);
    final tempFile = File('/tmp/adil192-linux.service');
    await tempFile.writeAsString(content);
    print('Adding ${_serviceFile.path}');
    await resultOfCommand('sudo', ['mv', tempFile.path, _serviceFile.path]);
  } else {
    print('$_serviceName already exists');
  }

  if (!timerFileExists) {
    final content = await _timerTemplateFile.readAsString();
    final tempFile = File('/tmp/adil192-linux.timer');
    await tempFile.writeAsString(content);
    print('Adding ${_timerFile.path}');
    await resultOfCommand('sudo', ['mv', tempFile.path, _timerFile.path]);
  } else {
    print('$_timerName already exists');
  }

  print('Activating $_timerName');
  await resultOfCommand('systemctl', ['--user', 'daemon-reload']);
  await resultOfCommand('systemctl', ['--user', 'enable', _timerFile.path]);
  await resultOfCommand('systemctl', ['--user', 'start', _timerName]);
}

Future<void> disableAutoUpdate() async {
  if (_serviceFile.existsSync()) {
    print('Deactivating $_serviceName');
    await resultOfCommand('systemctl', ['--user', 'stop', _serviceName]);
    await resultOfCommand('systemctl', ['--user', 'disable', _serviceName]);
    await resultOfCommand('sudo', ['rm', _serviceFile.path]);
  } else {
    print('$_serviceName does not exist');
  }

  if (_timerFile.existsSync()) {
    print('Deactivating $_timerName');
    await resultOfCommand('systemctl', ['--user', 'stop', _timerName]);
    await resultOfCommand('systemctl', ['--user', 'disable', _timerName]);
    await resultOfCommand('sudo', ['rm', _timerFile.path]);
  } else {
    print('$_timerName does not exist');
  }
}
