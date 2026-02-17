import 'dart:async';
import 'dart:io';

import 'package:adil192_linux/src/tools/brew.dart';
import 'package:adil192_linux/src/tools/dnf.dart';
import 'package:adil192_linux/src/tools/flatpak.dart';
import 'package:adil192_linux/src/tools/result_of_command.dart';
import 'package:adil192_linux/src/tools/which.dart';
import 'package:adil192_linux/src/tools/yes_or_no.dart';

Future<void> installApps() async {
  await _installFirefox();
  await _installSteam();
  await _installEquibop();
  await _installVSCode();
  await _installAndroidStudio();
  await _installAndroidEmulatorIntegration();
  await _installZed();
  await _installSpotify();
  await _installGitHubDesktop();
  await _installRicochlime();
  await _installSaber();
  await _installPrismLauncher();
  await _installQtBreezeTheme();
  await _installVlc();
  await _installUpscaledVlc();
  await _installApplite();
  await _installChromium();
  await _installWine();
}

Future<void> _installFirefox() =>
    _installApp(name: 'Firefox', dnf: 'firefox', brew: 'firefox');

Future<void> _installSteam() =>
    _installApp(name: 'Steam', dnf: 'steam', brew: 'steam');

Future<void> _installEquibop() => _installApp(
  name: 'Equibop (Discord client)',
  flatpak: 'org.equicord.equibop',
  brew: 'equibop',
);

Future<void> _installVSCode() async {
  if (Platform.isLinux) {
    if (!Dnf.hasDnf) return;
    if (Dnf.installed('code')) return;
    if (!yesOrNo('Install Visual Studio Code?')) return;
    print('Installing Visual Studio Code...');

    // Download rpm https://code.visualstudio.com/sha/download?build=stable&os=linux-rpm-x64
    await Dnf.install([
      'https://code.visualstudio.com/sha/download?build=stable&os=linux-rpm-x64',
    ]);
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
      print(
        'Jetbrains Toolbox already installed. '
        'Please manually install Android Studio through the GUI.',
      );
      return;
    }

    if (!yesOrNo('Install Android Studio via Jetbrains Toolbox?')) return;
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
  final desktopFile = File(
    '$home/.local/share/applications/com.adilhanney.pixel8.desktop',
  );
  final iconFile = File(
    '$home/.local/share/icons/hicolor/256x256/apps/com.adilhanney.pixel8.png',
  );

  if (desktopFile.existsSync() && iconFile.existsSync()) {
    print('Android Emulator integration already installed.');
    return;
  }
  if (!yesOrNo('Install Android Emulator integration?')) return;

  print('Installing Android Emulator integration...');
  desktopFile.createSync(recursive: true);
  await File(
    'assets/emulator_integration/com.adilhanney.pixel8.desktop',
  ).copy(desktopFile.path);
  iconFile.createSync(recursive: true);
  await File(
    'assets/emulator_integration/com.adilhanney.pixel8.png',
  ).copy(iconFile.path);

  if (home != '/home/ahann') {
    print('Patching .desktop file to use $home instead of /home/ahann');
    final desktopContents = await desktopFile.readAsString();
    await desktopFile.writeAsString(
      desktopContents.replaceAll('/home/ahann', home),
    );
  }
}

Future<void> _installZed() async {
  if (Platform.isLinux) {
    if (Which.installed('zed')) return;
    if (!yesOrNo('Install Zed?')) return;
    print('Installing Zed...');
    await resultOfCommand('wget', [
      'https://zed.dev/install.sh',
      '-O',
      '/tmp/zed-install.sh',
    ]);
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
    if (Which.installed('github-desktop-plus') ||
        Which.installed('github-desktop')) {
      return;
    }
    if (!yesOrNo('Install GitHub Desktop Plus?')) return;
    print('Installing GitHub Desktop Plus...');

    await resultOfCommand('sudo', [
      'rpm',
      '--import',
      'https://gpg.polrivero.com/public.key',
    ]);
    await resultOfCommand('sudo', [
      'sh',
      '-c',
      'echo -e "[github-desktop-plus]\nname=GitHub Desktop Plus\nbaseurl=https://rpm.github-desktop.polrivero.com/\nenabled=1\ngpgcheck=1\nrepo_gpgcheck=1\ngpgkey=https://gpg.polrivero.com/public.key" > /etc/yum.repos.d/github-desktop-plus.repo',
    ]);
    await Dnf.install(['github-desktop-plus']);
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

Future<void> _installQtBreezeTheme() async {
  if (!Platform.isLinux) return;
  if (!Dnf.hasDnf) return;
  if (Dnf.installed('plasma-breeze')) return;
  if (!yesOrNo('Install Qt Breeze Theme?')) return;
  print('Installing Qt Breeze Theme...');
  await Dnf.install(['plasma-breeze', 'breeze-icon-theme', 'qt5ct', 'qt6ct']);
}

Future<void> _installVlc() => _installApp(name: 'VLC', dnf: 'vlc', brew: 'vlc');

Future<void> _installUpscaledVlc() async {
  if (!Platform.isLinux) return;

  if (Which.installed('upscaled_vlc.sh')) return;

  const gitRepo = 'https://github.com/adil192/upscaled_vlc';
  if (!yesOrNo('Install Upscaled VLC ($gitRepo)?')) return;

  if (Dnf.hasDnf) {
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

Future<void> _installApplite() =>
    _installApp(name: 'Applite (homebrew frontend)', brew: 'applite');

Future<void> _installChromium() async {
  if (Platform.isMacOS) {
    _installBrewApp('google-chrome', 'Chrome');
    return;
  }

  final installed = await _installFlatpakApp(
    'org.chromium.Chromium',
    'Chromium',
  );

  // Set CHROME_EXECUTABLE so Flutter can find the flatpak
  if (!installed) return;
  if (Platform.environment['CHROME_EXECUTABLE']?.isNotEmpty ?? false) return;
  final home = Platform.environment['HOME'] ?? '~';
  await resultOfCommand('sed', [
    '-i',
    '\$aexport CHROME_EXECUTABLE="$home/.local/share/flatpak/app/org.chromium.Chromium/x86_64/stable/active/export/bin/org.chromium.Chromium"',
    '$home/.bashrc',
  ]);
}

Future<void> _installWine() async {
  if (!Platform.isLinux) return;
  if (!Dnf.hasDnf) return;
  if (Dnf.installed('wine') ||
      Dnf.installed('winehq-stable') ||
      Dnf.installed('winehq-devel') ||
      Dnf.installed('winehq-staging')) {
    return;
  }
  if (!yesOrNo('Install Wine?')) return;
  print('Installing Wine...');
  await Dnf.install(['wine']);
}

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
  if (!Flatpak.hasFlatpak) return false;
  if (Flatpak.installed(id)) return true;
  if (!yesOrNo('Install $name?')) return false;
  print('Installing $name...');
  await Flatpak.install(id);
  print('');
  return true;
}

Future<bool> _installDnfApp(String package, String name) async {
  if (!Dnf.hasDnf) return false;
  if (Dnf.installed(package)) return true;
  if (!yesOrNo('Install $name?')) return false;
  print('Installing $name...');
  await Dnf.install([package]);
  print('');
  return true;
}

Future<bool> _installBrewApp(String package, String name) async {
  if (!Brew.hasBrew) return false;
  if (Brew.installed(package)) return true;
  if (!yesOrNo('Install $name?')) return false;
  print('Installing $name...');
  await Brew.install(package);
  print('');
  return true;
}
