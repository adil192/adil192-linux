import 'dart:convert';
import 'dart:io';

Future<String> resultOfCommand(
  String command, [
  List<String> args = const [],
]) async {
  final process = await Process.start(command, args);
  stderr.addStream(process.stderr);

  final exitCode = await process.exitCode;
  if (exitCode != 0) {
    throw StateError('$command command failed with exit code $exitCode');
  }

  return await process.stdout.transform(utf8.decoder).join();
}
