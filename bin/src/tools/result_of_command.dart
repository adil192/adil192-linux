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
    stdout.add(data);
    output.add(utf8.decode(data));
  });
  stderr.addStream(process.stderr);

  final exitCode = await process.exitCode;
  if (exitCode != 0) {
    throw StateError('$command command failed with exit code $exitCode');
  }

  return output.join();
}

/// Runs the command synchronously and returns the output as a string.
/// This does not print to stdout or stderr unlike [resultOfCommand].
String resultOfCommandSync(
  String command,
  List<String> args, {
  String? workingDirectory,
  Map<String, String>? environment,
  bool includeParentEnvironment = true,
  bool runInShell = false,
}) {
  final result = Process.runSync(
    command,
    args,
    workingDirectory: workingDirectory,
    environment: environment,
    includeParentEnvironment: includeParentEnvironment,
    runInShell: runInShell,
  );

  final exitCode = result.exitCode;
  if (exitCode != 0) {
    throw StateError('$command command failed with exit code $exitCode');
  }

  return result.stdout;
}
