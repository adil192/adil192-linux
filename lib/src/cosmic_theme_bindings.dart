import 'package:adil192_linux/src/cosmic_theme_bindings.g.dart';
export 'package:adil192_linux/src/cosmic_theme_bindings.g.dart' hide get_theme;

CosmicThemeFfi getLightCosmicTheme() => get_theme(false);
CosmicThemeFfi getDarkCosmicTheme() => get_theme(true);

extension RgbaConverter on Rgb {
  String toHex() {
    return '#'
        '${r.toRadixString(16).padLeft(2, '0')}'
        '${g.toRadixString(16).padLeft(2, '0')}'
        '${b.toRadixString(16).padLeft(2, '0')}';
  }
}
