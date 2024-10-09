import 'dart:io';

bool alwaysYes = false;

Future<bool> yesOrNo(
  String question, [
  String hint = 'Y/n',
  String defaultResponse = 'y',
]) async {
  stdout.write('$question ($hint): ');

  var response = alwaysYes ? defaultResponse : '';
  while (response != 'y' && response != 'n') {
    response = stdin.readLineSync()?.toLowerCase() ?? '';
    if (response.isEmpty) response = defaultResponse;
  }

  return response == 'y';
}
