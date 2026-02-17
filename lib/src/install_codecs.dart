import 'dart:io';

import 'package:adil192_linux/src/tools/dnf.dart';
import 'package:adil192_linux/src/tools/device.dart';
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
  await _installMesaDrivers();

  if (Device.hasIntelGpu()) await _installIntelDrivers();

  if (Device.hasNvidiaGpu()) await _installNvidiaDrivers();
}

Future<void> _installMesaDrivers() async {
  if (Dnf.installed('mesa-va-drivers-freeworld') &&
      Dnf.installed('mesa-vdpau-drivers-freeworld')) {
    return;
  }
  if (!yesOrNo('Install mesa drivers?')) return;
  print('Installing mesa drivers...');

  await Dnf.swap('mesa-va-drivers.i686', 'mesa-va-drivers-freeworld.i686');
  await Dnf.swap(
    'mesa-vdpau-drivers.i686',
    'mesa-vdpau-drivers-freeworld.i686',
  );
  await Dnf.swap('mesa-va-drivers', 'mesa-va-drivers-freeworld');
  await Dnf.swap('mesa-vdpau-drivers', 'mesa-vdpau-drivers-freeworld');
}

Future<void> _installIntelDrivers() async {
  if (Dnf.installed('intel-media-driver')) return;
  if (!yesOrNo('Install Intel drivers?')) return;
  print('Installing Intel drivers...');
  await Dnf.install(['intel-media-driver', 'libva-intel-driver']);
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
