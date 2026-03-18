import 'dart:io';

import 'package:adil192_linux/src/tools/dnf.dart';
import 'package:adil192_linux/src/tools/device.dart';
import 'package:adil192_linux/src/tools/run.dart';
import 'package:adil192_linux/src/tools/yes_or_no.dart';

/// Follows https://rpmfusion.org/Howto/Multimedia
Future<void> installCodecs() async {
  if (!Platform.isLinux) return;

  if (!Dnf.hasDnf) {
    print('DNF is not available, skipping multimedia codecs installation.');
    return;
  }

  await Dnf.configureRpmFusion();

  await _switchToFullFfmpeg();

  await _installAdditionalCodecs();

  await _installHardwareAcceleration();
}

Future<void> _switchToFullFfmpeg() async {
  if (!Dnf.installed('ffmpeg-free')) return;

  if (!yesOrNo('Switch from ffmpeg-free to the full ffmpeg?')) return;

  print('Switching to the full ffmpeg package...');
  await Dnf.swap('ffmpeg-free', 'ffmpeg', allowErasing: true);
  await Dnf.install(['libavcodec-freeworld']);
}

Future<void> _installAdditionalCodecs() async {
  if (Dnf.installed('gstreamer1-plugins-ugly')) return;

  if (!yesOrNo('Install additional multimedia codecs?')) return;

  print('Installing additional multimedia codecs...');
  await Dnf.update([
    '@multimedia',
    '--setopt=install_weak_deps=False',
    '--exclude=PackageKit-gstreamer-plugin',
  ]);
}

Future<void> _installHardwareAcceleration() async {
  await _installMesaCopr();

  if (Device.hasIntelGpu()) await _installIntelDrivers();

  if (Device.hasNvidiaGpu()) await _installNvidiaDrivers();
}

Future<void> _installMesaCopr() async {
  if (!Platform.isLinux) return;
  if (!Dnf.hasDnf) return;
  final repoFile = File(
    '/etc/yum.repos.d/_copr:copr.fedorainfracloud.org:adil192:mesa-rc.repo',
  );
  if (repoFile.existsSync()) return;
  if (!yesOrNo('Install my repo for faster Mesa driver updates?')) return;
  print('Installing my repo for faster Mesa driver updates...');
  await run('sudo', ['dnf', 'copr', 'enable', 'adil192/mesa-rc']);
  print('Run `sudo dnf update` to update to the builds from my repo.');
}

Future<void> _installIntelDrivers() async {
  if (Dnf.installed('intel-media-driver')) return;
  if (!yesOrNo('Install Intel drivers?')) return;
  print('Installing Intel drivers...');
  await Dnf.install([
    'intel-media-driver',
    'libva-intel-driver',
    'mesa-libOpenCL',
    'intel-opencl',
  ]);
}

Future<void> _installNvidiaDrivers() async {
  if (Dnf.installed('libva-nvidia-driver')) return;
  if (!yesOrNo('Install (proprietary) Nvidia drivers?')) return;
  print('Installing Nvidia drivers...');
  await Dnf.install([
    'akmod-nvidia',
    'xorg-x11-drv-nvidia-cuda',
    'libva-nvidia-driver.{i686,x86_64}',
  ]);
}
