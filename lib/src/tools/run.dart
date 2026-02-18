import 'dart:convert';
import 'dart:io';

/// Runs the command and returns the output as a string.
///
/// Notes:
/// - Throws if the exit code is non-zero.
/// - Process inherits stdio: it prints to console and can take user input.
Future<String> run(String command, List<String> args) async {
  final process = await Process.start(command, args, mode: .inheritStdio);

  final exitCode = await process.exitCode;
  if (exitCode != 0) {
    throw StateError('$command command failed with exit code $exitCode');
  }

  return process.stdout.transform(utf8.decoder).join();
}

/// Runs the command and returns the output as a string.
///
/// Notes:
/// - Throws if the exit code is non-zero.
/// - Process does not inherit stdio:
///   Nothing is printed to console and no user input can be taken.
String runSilent(String command, List<String> args) {
  final result = Process.runSync(command, args);

  final exitCode = result.exitCode;
  if (exitCode != 0) {
    throw StateError('$command command failed with exit code $exitCode');
  }

  return result.stdout;
}
