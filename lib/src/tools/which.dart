import 'package:adil192_linux/src/tools/result_of_command.dart';

class Which {
  static bool installed(String name) {
    String? location;
    try {
      location = resultOfCommandSync('which', [name], runInShell: true).trim();
    } on StateError {
      // ignore
    }

    print('which $name: $location');
    return location?.isNotEmpty ?? false;
  }
}
