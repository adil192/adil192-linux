import 'dart:async';
import 'dart:io';

import 'tools/dnf.dart';
import 'tools/flatpak.dart';
import 'tools/result_of_command.dart';
import 'tools/which.dart';
import 'tools/yes_or_no.dart';

Future<void> installApps() async {
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
  await _installQtBreezeTheme();
  await _installVlc();
  await _installUpscaledVlc();
}

Future<void> _installSteam() => _installDnfApp('steam', 'Steam');

Future<void> _installDiscord() => _installDnfApp('discord', 'Discord');

Future<void> _installVSCode() async {
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
}

Future<void> _installAndroidStudio() async {
  final home = Platform.environment['HOME'] ?? '~';
  final applicationsDir = Directory('$home/Applications');
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
}

Future<void> _installAndroidEmulatorIntegration() async {
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
  await File('assets/emulator_integration/com.adilhanney.pixel8.desktop')
      .copy(desktopFile.path);
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
  if (await Which.installed('zed')) return;
  if (!await yesOrNo('Install Zed?')) return;
  print('Installing Zed...');
  await resultOfCommand(
      'wget', ['https://zed.dev/install.sh', '-O', '/tmp/zed-install.sh']);
  await resultOfCommand('bash', ['/tmp/zed-install.sh']);
}

Future<void> _installSpotify() =>
    _installFlatpakApp('com.spotify.Client', 'Spotify');

Future<void> _installGitHubDesktop() async {
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
}

Future<void> _installRicochlime() =>
    _installFlatpakApp('com.adilhanney.ricochlime', 'Ricochlime');

Future<void> _installSaber() =>
    _installFlatpakApp('com.adilhanney.saber', 'Saber');

Future<void> _installPrismLauncher() =>
    _installFlatpakApp('org.prismlauncher.PrismLauncher', 'Prism Launcher');

Future<void> _installGnomeTweaks() =>
    _installDnfApp('gnome-tweaks', 'Gnome Tweaks');

Future<void> _installQtBreezeTheme() async {
  if (!await Dnf.hasDnf) return;
  if (await Dnf.installed('plasma-breeze')) return;
  if (!await yesOrNo('Install Qt Breeze Theme?')) return;
  print('Installing Qt Breeze Theme...');
  await Dnf.install(['plasma-breeze', 'qt5ct', 'qt6ct']);
}

Future<void> _installVlc() => _installDnfApp('vlc', 'VLC');

Future<void> _installUpscaledVlc() async {
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

Future<void> _installFlatpakApp(String id, String name) async {
  if (await Flatpak.installed(id)) return;
  if (!await yesOrNo('Install $name?')) return;
  print('Installing $name...');
  await Flatpak.install(id);
}

Future<void> _installDnfApp(String package, String name) async {
  if (!await Dnf.hasDnf) return;
  if (await Dnf.installed(package)) return;
  if (!await yesOrNo('Install $name?')) return;
  print('Installing $name...');
  await Dnf.install([package]);
}
