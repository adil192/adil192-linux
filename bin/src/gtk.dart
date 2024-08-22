import 'dart:io';

import 'tools/gsettings.dart';

final _pwd = Platform.environment['PWD'];
final _home = Platform.environment['HOME'];
final _gtk3Css = File('$_home/.config/gtk-3.0/gtk.css');
final _gtk4Css = File('$_home/.config/gtk-4.0/gtk.css');

Future<void> installGtkCss() async {
  final customCssContent =
      await File('$_pwd/assets/gtk/gtk.css').readAsString();

  await _injectCss(_gtk3Css, customCssContent);
  await _injectCss(_gtk4Css, customCssContent);

  await _setGsettings();
}

Future<void> uninstallGtkCss() async {
  const emptyCss = '';

  await _injectCss(_gtk3Css, emptyCss);
  await _injectCss(_gtk4Css, emptyCss);
}

/// Injects the custom CSS into the given file.
///
/// If older custom CSS is found, the content between
/// /* START adil192-linux gtk.css */
/// and
/// /* END adil192-linux gtk.css */
/// is replaced with the new custom CSS.
Future<void> _injectCss(File file, String customCssContent) async {
  final content = file.existsSync() ? await file.readAsString() : '';

  const start = '/* START adil192-linux gtk.css */';
  const end = '/* END adil192-linux gtk.css */';

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

Future<void> _setGsettings() async {
  await Gsettings.set('org.gnome.desktop.wm.preferences', 'button-layout',
      'appmenu:minimize,maximize,close');
}
