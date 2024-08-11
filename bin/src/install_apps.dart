import 'tools/dnf.dart';
import 'tools/flatpak.dart';
import 'tools/result_of_command.dart';
import 'tools/which.dart';
import 'tools/yes_or_no.dart';

Future<void> installApps() async {
  await _installSteam();
  await _installDiscord();
  await _installVSCode();
  await _installZed();
  await _installSpotify();
  await _installGitHubDesktop();
}

Future<void> _installSteam() => _installDnfApp('steam', 'Steam');

Future<void> _installDiscord() => _installDnfApp('discord', 'Discord');

Future<void> _installVSCode() async {
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

Future<void> _installFlatpakApp(String id, String name) async {
  if (await Flatpak.installed(id)) return;
  if (!await yesOrNo('Install $name?')) return;
  print('Installing $name...');
  await Flatpak.install(id);
}

Future<void> _installDnfApp(String package, String name) async {
  if (await Dnf.installed(package)) return;
  if (!await yesOrNo('Install $name?')) return;
  print('Installing $name...');
  await Dnf.install([package]);
}
