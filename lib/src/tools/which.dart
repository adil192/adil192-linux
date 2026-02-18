import 'package:adil192_linux/src/tools/run.dart';

class Which {
  static bool installed(String name) {
    String? location;
    try {
      location = runSilent('which', [name]).trim();
    } on StateError {
      // ignore
    }

    print('which $name: $location');
    return location?.isNotEmpty ?? false;
  }
}
