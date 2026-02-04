/// Installs various applications using dnf on Fedora or homebrew on macOS.
///
/// You'll be asked a series of yes/no questions to install each app.
const shouldInstallApps = true;

/// Installs codecs and drivers for playing media (Linux only).
///
/// You'll be asked a series of yes/no questions to install each package.
const shouldInstallCodecs = true;

/// Applies a gnome theme to firefox (Linux only)
/// and if [shouldThemeWindowButtons] is true,
/// themes Firefox's window buttons.
///
/// See https://github.com/rafaelmardojai/firefox-gnome-theme
const shouldThemeFirefox = true;

/// Precaches Firefox profiles and disk cache into RAM (Linux only).
/// This speeds up Firefox if you have enough RAM to spare.
/// If your RAM gets full, linux will automatically remove the precached data.
const shouldInstallFirefoxCacher = true;

/// Themes the window buttons (minimise, maximise, close)
/// in various applications to be larger and squarer (Linux only).
const shouldThemeWindowButtons = true;

/// Installs an Adwaita theme for Steam (Linux only)
///
/// See https://github.com/tkashkin/Adwaita-for-Steam
const shouldThemeSteam = true;
