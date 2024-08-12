/// Installs various applications using dnf.
///
/// You'll be asked a series of yes/no questions to install each app.
const shouldInstallApps = true;

/// Installs codecs and drivers for playing media.
///
/// You'll be asked a series of yes/no questions to install each package.
const shouldInstallCodecs = true;

/// Applies a gnome theme to firefox
/// (https://github.com/rafaelmardojai/firefox-gnome-theme)
/// and if [shouldThemeWindowButtons] is true,
/// themes Firefox's window buttons.
const shouldThemeFirefox = true;

/// Themes the window buttons (minimise, maximise, close)
/// in various applications to be larger and squarer.
const shouldThemeWindowButtons = true;

/// Installs an Adwaita theme for Steam
/// (https://github.com/tkashkin/Adwaita-for-Steam).
const shouldThemeSteam = true;

/// Installs a systemd timer
/// to pull the latest changes in this repo and apply them
/// once a day.
///
/// Note that this basically gives me the ability to run arbitrary code
/// on your machine, so unless you know me personally,
/// you should probably set this to false.
const shouldAutomaticallyUpdate = true;
