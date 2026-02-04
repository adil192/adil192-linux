/// Installs various applications using dnf on Fedora or homebrew on macOS.
///
/// You'll be asked a series of yes/no questions to install each app.
const shouldInstallApps = true;

/// Installs codecs and drivers for playing media (Linux only).
///
/// You'll be asked a series of yes/no questions to install each package.
const shouldInstallCodecs = true;

/// Themes Firefox's window buttons to match those of COSMIC.
const shouldThemeFirefox = true;

/// Precaches Firefox profiles and disk cache into RAM (Linux only).
/// This speeds up Firefox if you have enough RAM to spare.
/// If your RAM gets full, linux will automatically remove the precached data.
const shouldInstallFirefoxCacher = true;

/// Installs an Adwaita theme for Steam (Linux only)
///
/// See https://github.com/tkashkin/Adwaita-for-Steam
const shouldThemeSteam = true;
