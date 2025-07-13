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
///
/// Note that this tweak will not be installed if you have 8GB or less of RAM,
/// even if this variable is set to true.
const shouldInstallFirefoxCacher = true;

/// Themes the window buttons (minimise, maximise, close)
/// in various applications to be larger and squarer (Linux only).
const shouldThemeWindowButtons = true;

/// Installs an Adwaita theme for Steam (Linux only)
///
/// See https://github.com/tkashkin/Adwaita-for-Steam
const shouldThemeSteam = true;

/// Installs a systemd timer
/// to pull the latest changes in this repo and apply them
/// once a day (Linux only).
///
/// Note that this basically gives me the ability to run arbitrary code
/// on your machine, so unless you know me personally,
/// you should probably set this to false.
const shouldAutomaticallyUpdate = true;
