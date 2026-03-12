import 'dart:io';

import 'package:ffigen/ffigen.dart';

void main() {
  final packageRoot = Platform.script.resolve('../');
  FfiGenerator(
    headers: Headers(entryPoints: [packageRoot.resolve('rust/bindings.h')]),
    output: Output(
      dartFile: packageRoot.resolve('lib/src/cosmic_theme_bindings.g.dart'),
    ),
    functions: Functions.includeSet({'get_theme'}),
  ).generate();
}
