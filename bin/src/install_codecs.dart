import 'tools/dnf.dart';
import 'tools/model.dart';
import 'tools/result_of_command.dart';
import 'tools/yes_or_no.dart';

/// Follows https://rpmfusion.org/Howto/Multimedia
Future<void> installCodecs() async {
  await _configureRpmFusion();

  await _switchToFullFfmpeg();

  await _installAdditionalCodecs();

  await _installHardwareAcceleration();
}

Future<void> _configureRpmFusion() async {
  final repoList = await Dnf.repoList();
  final hasFree = repoList.contains('rpmfusion-free');
  final hasNonFree = repoList.contains('rpmfusion-nonfree');
  final hasOpenh264 = repoList.contains('fedora-cisco-openh264');

  if (hasFree && hasNonFree && hasOpenh264) return;

  if (!await yesOrNo('Do you want to install RPM Fusion repositories?')) return;

  final fedoraVersion = await resultOfCommand('rpm', ['-E', '%fedora']);
  print('Installing RPM Fusion repositories...');
  await Dnf.install([
    'https://mirrors.rpmfusion.org/free/fedora/rpmfusion-free-release-$fedoraVersion.noarch.rpm',
    'https://mirrors.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$fedoraVersion.noarch.rpm',
  ]);
  print('Enabling the fedora-cisco-openh264 repository...');
  await Dnf.enableCiscoOpenh264();
  print('Updating Appstream metadata...');
  await Dnf.update(['@core']);
}

Future<void> _switchToFullFfmpeg() async {
  if (!await Dnf.installed('ffmpeg-free')) return;

  if (!await yesOrNo('Switch from ffmpeg-free to the full ffmpeg?')) return;

  print('Switching to the full ffmpeg package...');
  await Dnf.swap('ffmpeg-free', 'ffmpeg', allowErasing: true);
}

Future<void> _installAdditionalCodecs() async {
  if (await Dnf.installed('gstreamer1-plugins-ugly')) return;

  if (!await yesOrNo('Install additional multimedia codecs?')) return;

  print('Installing additional multimedia codecs...');
  await Dnf.update([
    '@multimedia',
    '--setopt=install_weak_deps=False',
    '--exclude=PackageKit-gstreamer-plugin',
  ]);
}

Future<void> _installHardwareAcceleration() async {
  if (await Model.hasAmdGpu()) await _installAmdDrivers();

  if (await Model.hasIntelGpu()) await _installIntelDrivers();

  if (await Model.hasNvidiaGpu()) await _installNvidiaDrivers();
}

Future<void> _installAmdDrivers() async {
  if (await Dnf.installed('mesa-va-drivers-freeworld')) return;
  if (!await yesOrNo('Install AMD drivers?')) return;
  print('Installing AMD drivers...');

  await Dnf.swap('mesa-va-drivers.i686', 'mesa-va-drivers-freeworld.i686');
  await Dnf.swap(
      'mesa-vdpau-drivers.i686', 'mesa-vdpau-drivers-freeworld.i686');
  await Dnf.swap('mesa-va-drivers', 'mesa-va-drivers-freeworld');
  await Dnf.swap('mesa-vdpau-drivers', 'mesa-vdpau-drivers-freeworld');
}

Future<void> _installIntelDrivers() async {
  if (await Dnf.installed('intel-media-driver')) return;
  if (!await yesOrNo('Install Intel drivers?')) return;
  print('Installing Intel drivers...');
  await Dnf.install(['intel-media-driver', 'libva-intel-driver']);
}

Future<void> _installNvidiaDrivers() async {
  if (await Dnf.installed('libva-nvidia-driver.x86_64')) return;
  if (!await yesOrNo('Install (proprietary) Nvidia drivers?')) return;
  print('Installing Nvidia drivers...');
  await Dnf.install([
    'akmod-nvidia',
    'xorg-x11-drv-nvidia-cuda',
    'libva-nvidia-driver.{i686,x86_64}',
  ]);
}
