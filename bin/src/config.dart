/// Installs various applications using dnf.
const shouldInstallApps = true;

/// Installs codecs for playing media.
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

/// Installs a cron job to pull the latest changes in this repo and apply them.
///
/// Note that this basically gives me the ability to run arbitrary code
/// on your machine, so unless you know me personally,
/// you should probably set this to false.
const shouldAutomaticallyUpdate = true;

/// The schedule for the cron job.
///
/// If [shouldAutomaticallyUpdate] is false, this is ignored.
///
/// By default, this runs at 18:48 every day.
const updateSchedule = '48 18 * * *';
