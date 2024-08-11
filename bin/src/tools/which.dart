import 'result_of_command.dart';

class Which {
  static Future<bool> installed(String name) async {
    String? location;
    try {
      location = await resultOfCommand('which', [name],
          runInShell: true, silent: true);
      location = location.trim();
    } on StateError {
      // ignore
    }

    print('which $name: $location');
    return location?.isNotEmpty ?? false;
  }
}
