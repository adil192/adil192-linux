import 'dart:io';

final _pwd = Platform.environment['PWD'];
final _home = Platform.environment['HOME'];

Future<void> installGtkCss() async {
  final gtk3Css = File('$_home/.config/gtk-3.0/gtk.css');
  final gtk4Css = File('$_home/.config/gtk-4.0/gtk.css');

  final customCssContent = await File('$_pwd/gtk/gtk.css').readAsString();

  await injectCss(gtk3Css, customCssContent);
  await injectCss(gtk4Css, customCssContent);
}

/// Injects the custom CSS into the given file.
///
/// If older custom CSS is found, the content between
/// /* START adil192-linux gtk.css */
/// and
/// /* END adil192-linux gtk.css */
/// is replaced with the new custom CSS.
Future<void> injectCss(File file, String customCssContent) async {
  final content = await file.readAsString();
  final start = '/* START adil192-linux gtk.css */';
  final end = '/* END adil192-linux gtk.css */';

  final startIndex = content.indexOf(start);
  final endIndex = content.indexOf(end);

  if (startIndex != -1 && endIndex != -1) {
    final newContent = content.replaceRange(
      startIndex + start.length,
      endIndex,
      '\n$customCssContent\n',
    );
    await file.writeAsString(newContent);
    print('Injected custom CSS into ${file.path}');
  } else {
    await file.writeAsString(
      '$content\n\n$start\n$customCssContent\n$end\n',
    );
    print('Injected custom CSS into ${file.path}');
  }
}
