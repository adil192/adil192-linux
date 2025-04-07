import 'dart:async';
import 'dart:io';

import 'tools/brew.dart';
import 'tools/dnf.dart';
import 'tools/flatpak.dart';
import 'tools/result_of_command.dart';
import 'tools/which.dart';
import 'tools/yes_or_no.dart';

Future<void> installApps() async {
  await _installFirefox();
  await _installSteam();
  await _installDiscord();
  await _installVSCode();
  await _installAndroidStudio();
  await _installAndroidEmulatorIntegration();
  await _installZed();
  await _installSpotify();
  await _installGitHubDesktop();
  await _installRicochlime();
  await _installSaber();
  await _installPrismLauncher();
  await _installGnomeTweaks();
  if (await _installGnomeExtensionManager()) {
    await _installPopShell();
    await _installDashToPanel();
    await _installAppindicatorSupport();
    await _disableFedoraBgLogo();
  }
  await _installQtBreezeTheme();
  await _installVlc();
  await _installUpscaledVlc();
  await _installAltTab();
  await _installApplite();
  await _installChrome();
}

Future<void> _installFirefox() => _installApp(
      name: 'Firefox',
      dnf: 'firefox',
      brew: 'firefox',
    );

Future<void> _installSteam() => _installApp(
      name: 'Steam',
      dnf: 'steam',
      brew: 'steam',
    );

Future<void> _installDiscord() => _installApp(
      name: 'Discord',
      dnf: 'discord',
      brew: 'discord',
    );

Future<void> _installVSCode() async {
  if (Platform.isLinux) {
    if (!await Dnf.hasDnf) return;
    if (await Dnf.installed('code')) return;
    if (!await yesOrNo('Install Visual Studio Code?')) return;
    print('Installing Visual Studio Code...');

    // Download rpm https://code.visualstudio.com/sha/download?build=stable&os=linux-rpm-x64
    await resultOfCommand('wget', [
      'https://code.visualstudio.com/sha/download?build=stable&os=linux-rpm-x64',
      '-O',
      '/tmp/code.rpm',
    ]);
    await Dnf.install(['/tmp/code.rpm']);
  } else if (Platform.isMacOS) {
    await _installBrewApp('visual-studio-code', 'Visual Studio Code');
  }
}

Future<void> _installAndroidStudio() async {
  if (Platform.isLinux) {
    final home = Platform.environment['HOME'] ?? '~';
    final applicationsDir = Directory('$home/Applications')..createSync();
    final toolboxExe = File('$home/Applications/jetbrains-toolbox');

    if (toolboxExe.existsSync()) {
      print('Jetbrains Toolbox already installed. '
          'Please manually install Android Studio through the GUI.');
      return;
    }

    if (!await yesOrNo('Install Android Studio via Jetbrains Toolbox?')) return;
    print('Installing Jetbrains Toolbox...');

    final tarFile = File('/tmp/jetbrains-toolbox.tar.gz');
    final archiveName = 'jetbrains-toolbox-2.4.2.32922';
    await resultOfCommand('wget', [
      'https://download.jetbrains.com/toolbox/$archiveName.tar.gz',
      '-O',
      tarFile.path,
    ]);
    await resultOfCommand('tar', [
      '-xf',
      tarFile.path,
      '-C',
      applicationsDir.path,
      '--strip-components=1',
      '$archiveName/jetbrains-toolbox',
    ]);
    tarFile.delete(recursive: true);
    unawaited(Process.start(toolboxExe.path, const []));
  } else if (Platform.isMacOS) {
    await _installBrewApp('android-studio', 'Android Studio');
  }
}

Future<void> _installAndroidEmulatorIntegration() async {
  if (!Platform.isLinux) return;

  final home = Platform.environment['HOME'] ?? '~';
  final desktopFile =
      File('$home/.local/share/applications/com.adilhanney.pixel8.desktop');
  final iconFile = File(
      '$home/.local/share/icons/hicolor/256x256/apps/com.adilhanney.pixel8.png');

  if (desktopFile.existsSync() && iconFile.existsSync()) {
    print('Android Emulator integration already installed.');
    return;
  }
  if (!await yesOrNo('Install Android Emulator integration?')) return;

  print('Installing Android Emulator integration...');
  desktopFile.createSync(recursive: true);
  await File('assets/emulator_integration/com.adilhanney.pixel8.desktop')
      .copy(desktopFile.path);
  iconFile.createSync(recursive: true);
  await File('assets/emulator_integration/com.adilhanney.pixel8.png')
      .copy(iconFile.path);

  final user = Platform.environment['USER'] ?? 'ahann';
  if (user != 'ahann') {
    print('Patching .desktop file to use /home/$user instead of /home/ahann');
    final desktopContents = await desktopFile.readAsString();
    await desktopFile.writeAsString(
        desktopContents.replaceAll('/home/ahann/', '/home/$user/'));
  }
}

Future<void> _installZed() async {
  if (Platform.isLinux) {
    if (await Which.installed('zed')) return;
    if (!await yesOrNo('Install Zed?')) return;
    print('Installing Zed...');
    await resultOfCommand(
        'wget', ['https://zed.dev/install.sh', '-O', '/tmp/zed-install.sh']);
    await resultOfCommand('bash', ['/tmp/zed-install.sh']);
  } else if (Platform.isMacOS) {
    await _installBrewApp('zed', 'Zed');
  }
}

Future<void> _installSpotify() => _installApp(
      name: 'Spotify',
      flatpak: 'com.spotify.Client',
      brew: 'spotify',
    );

Future<void> _installGitHubDesktop() async {
  if (Platform.isLinux) {
    if (await Which.installed('github-desktop')) return;
    if (!await yesOrNo('Install GitHub Desktop?')) return;
    print('Installing GitHub Desktop...');

    await resultOfCommand('sudo',
        ['rpm', '--import', 'https://mirror.mwt.me/shiftkey-desktop/gpgkey']);
    await resultOfCommand('sudo', [
      'sh',
      '-c',
      'echo -e "[mwt-packages]\nname=GitHub Desktop\nbaseurl=https://mirror.mwt.me/shiftkey-desktop/rpm\nenabled=1\ngpgcheck=1\nrepo_gpgcheck=1\ngpgkey=https://mirror.mwt.me/shiftkey-desktop/gpgkey" > /etc/yum.repos.d/mwt-packages.repo',
    ]);
    await Dnf.install(['github-desktop']);
  } else if (Platform.isMacOS) {
    await _installBrewApp('github', 'GitHub Desktop');
  }
}

Future<void> _installRicochlime() =>
    _installFlatpakApp('com.adilhanney.ricochlime', 'Ricochlime');

Future<void> _installSaber() =>
    _installFlatpakApp('com.adilhanney.saber', 'Saber');

Future<void> _installPrismLauncher() =>
    _installFlatpakApp('org.prismlauncher.PrismLauncher', 'Prism Launcher');

Future<void> _installGnomeTweaks() =>
    _installDnfApp('gnome-tweaks', 'Gnome Tweaks');

Future<bool> _installGnomeExtensionManager() => _installFlatpakApp(
    'com.mattjakeman.ExtensionManager', 'Gnome Extension Manager');

Future<void> _installPopShell() async {
  if (await _installDnfApp('gnome-shell-extension-pop-shell', 'Pop Shell')) {
    await resultOfCommand(
        'gnome-extensions', ['enable', 'pop-shell@system76.com']);
  }
}

Future<void> _installDashToPanel() async {
  if (await _installDnfApp(
      'gnome-shell-extension-dash-to-panel', 'Dash to Panel')) {
    await resultOfCommand(
        'gnome-extensions', ['enable', 'dash-to-panel@jderose9.github.com']);
  }
}

Future<void> _installAppindicatorSupport() async {
  if (await _installDnfApp('gnome-shell-extension-appindicator',
      'AppIndicator/KStatusNotifierItem support for GNOME Shell')) {
    await resultOfCommand('gnome-extensions',
        ['enable', 'appindicatorsupport@rgcjonas.gmail.com']);
  }
}

Future<void> _disableFedoraBgLogo() async {
  if (!Platform.isLinux) return;
  if (!await yesOrNo('Disable Fedora\'s background logo?')) return;
  print('Disabling Fedora\'s background logo...');
  await resultOfCommand(
      'gnome-extensions', ['disable', 'background-logo@fedorahosted.org']);
}

Future<void> _installQtBreezeTheme() async {
  if (!Platform.isLinux) return;
  if (!await Dnf.hasDnf) return;
  if (await Dnf.installed('plasma-breeze')) return;
  if (!await yesOrNo('Install Qt Breeze Theme?')) return;
  print('Installing Qt Breeze Theme...');
  await Dnf.install(['plasma-breeze', 'qt5ct', 'qt6ct']);
}

Future<void> _installVlc() => _installApp(
      name: 'VLC',
      dnf: 'vlc',
      brew: 'vlc',
    );

Future<void> _installUpscaledVlc() async {
  if (!Platform.isLinux) return;

  if (await Which.installed('upscaled_vlc.sh')) return;

  const gitRepo = 'https://github.com/adil192/upscaled_vlc';
  if (!await yesOrNo('Install Upscaled VLC ($gitRepo)?')) return;

  if (await Dnf.hasDnf) {
    print('Installing Upscaled VLC\'s dependencies...');
    await Dnf.configureRpmFusion();
    await Dnf.install(['ffmpeg', 'xdpyinfo', 'vlc', 'gamescope']);
  } else {
    print('Please install the following dependencies manually:');
    print('\tffmpeg xdpyinfo vlc gamescope');
  }

  print('Installing Upscaled VLC...');

  const installScriptUrl =
      'https://raw.githubusercontent.com/adil192/upscaled_vlc/main/install.sh';
  const installScriptPath = '/tmp/install_upscaled_vlc.sh';
  await resultOfCommand('wget', [installScriptUrl, '-O', installScriptPath]);
  await resultOfCommand('bash', [installScriptPath]);
  await resultOfCommand('rm', [installScriptPath]);
}

Future<void> _installAltTab() => _installApp(
      name: 'AltTab',
      brew: 'alt-tab',
    );
Future<void> _installApplite() => _installApp(
      name: 'Applite (homebrew frontend)',
      brew: 'applite',
    );
Future<void> _installChrome() => _installApp(
      name: 'Chrome',
      brew: 'google-chrome',
    );

Future<bool> _installApp({
  required String name,
  String? dnf,
  String? flatpak,
  String? brew,
}) async {
  if (Platform.isLinux && dnf != null) {
    if (await _installDnfApp(dnf, name)) return true;
  }
  if (Platform.isLinux && flatpak != null) {
    if (await _installFlatpakApp(flatpak, name)) return true;
  }
  if (Platform.isMacOS && brew != null) {
    if (await _installBrewApp(brew, name)) return true;
  }
  return false;
}

Future<bool> _installFlatpakApp(String id, String name) async {
  if (!await Flatpak.hasFlatpak) return false;
  if (await Flatpak.installed(id)) return true;
  if (!await yesOrNo('Install $name?')) return false;
  print('Installing $name...');
  await Flatpak.install(id);
  return true;
}

Future<bool> _installDnfApp(String package, String name) async {
  if (!await Dnf.hasDnf) return false;
  if (await Dnf.installed(package)) return true;
  if (!await yesOrNo('Install $name?')) return false;
  print('Installing $name...');
  await Dnf.install([package]);
  return true;
}

Future<bool> _installBrewApp(String package, String name) async {
  if (!await Brew.hasBrew) return false;
  if (await Brew.installed(package)) return true;
  if (!await yesOrNo('Install $name?')) return false;
  print('Installing $name...');
  await Brew.install(package);
  return true;
}
