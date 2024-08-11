import 'dart:convert';
import 'dart:io';

Future<String> resultOfCommand(
  String command,
  List<String> args, {
  String? workingDirectory,
  Map<String, String>? environment,
  bool includeParentEnvironment = true,
  bool runInShell = false,
  ProcessStartMode mode = ProcessStartMode.normal,
  bool silent = false,
}) async {
  final process = await Process.start(
    command,
    args,
    workingDirectory: workingDirectory,
    environment: environment,
    includeParentEnvironment: includeParentEnvironment,
    runInShell: runInShell,
    mode: mode,
  );
  final output = [];
  process.stdout.listen((data) {
    if (!silent) stdout.add(data);
    output.add(utf8.decode(data));
  });
  stderr.addStream(process.stderr);

  final exitCode = await process.exitCode;
  if (exitCode != 0) {
    throw StateError('$command command failed with exit code $exitCode');
  }

  return output.join();
}
