import 'dart:io';

Future<bool> yesOrNo(
  String question, [
  String hint = 'Y/n',
  String defaultResponse = 'y',
]) async {
  stdout.write('$question ($hint): ');

  String response;
  do {
    response = stdin.readLineSync()?.toLowerCase() ?? '';
    if (response.isEmpty) response = defaultResponse;
  } while (response != 'y' && response != 'n');

  return response == 'y';
}
