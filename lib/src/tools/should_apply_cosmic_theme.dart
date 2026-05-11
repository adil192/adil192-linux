import 'dart:io';

/// Returns true when we're currently running in the COSMIC DE
/// and the COSMIC 'apply_theme_global' setting is true.
final shouldApplyCosmicTheme = _getShouldApplyCosmicTheme();
bool _getShouldApplyCosmicTheme() {
  if (Platform.environment['XDG_SESSION_DESKTOP']?.toUpperCase() != 'COSMIC') {
    return false;
  }

  final home = Platform.environment['HOME'] ?? '~';
  final file = File(
    '$home/.config/cosmic/com.system76.CosmicTk/v1/apply_theme_global',
  );
  if (!file.existsSync()) return false;
  final content = file.readAsStringSync().trim();
  return content == 'true';
}
