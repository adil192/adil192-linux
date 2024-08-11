import 'dart:io';

abstract class Gsettings {
  static Future<void> set(String schema, String key, String value) async {
    final process =
        await Process.start('gsettings', ['set', schema, key, value]);

    stdout.addStream(process.stdout);
    stderr.addStream(process.stderr);

    final exitCode = await process.exitCode;
    if (exitCode != 0) {
      throw Exception('Failed to set gsetting: $schema $key $value');
    }

    print('Set gsetting $key to $value');
  }
}
