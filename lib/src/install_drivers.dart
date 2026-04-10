import 'dart:io';

import 'package:adil192_linux/src/tools/dnf.dart';
import 'package:adil192_linux/src/tools/device.dart';
import 'package:adil192_linux/src/tools/run.dart';
import 'package:adil192_linux/src/tools/yes_or_no.dart';

/// Follows https://rpmfusion.org/Howto/Multimedia
Future<void> installDrivers() async {
  if (!Platform.isLinux) return;

  if (!Dnf.hasDnf) {
    print('DNF is not available, skipping drivers installation.');
    return;
  }

  await Dnf.configureRpmFusion();

  await _switchToFullFfmpeg();

  await _installAdditionalCodecs();

  await _installMesaCopr();

  if (Device.hasIntelGpu()) await _installIntelGpuDrivers();
  if (Device.hasIntelCpu()) {
    await _installIntelWebcamDrivers();
    await _installIntelBatteryOptimizer();
  }

  if (Device.hasNvidiaGpu()) await _installNvidiaGpuDrivers();

  await _installBroadcomFingerprintDrivers();
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

Future<void> _installIntelGpuDrivers() async {
  if (Dnf.installed('intel-media-driver')) return;
  if (!yesOrNo('Install Intel GPU drivers?')) return;
  print('Installing Intel GPU drivers...');
  await Dnf.install([
    'intel-media-driver',
    'libva-intel-driver',
    'mesa-libOpenCL',
    'intel-opencl',
  ]);
}

Future<void> _installIntelWebcamDrivers() async {
  if (Dnf.installed('ipu6-camera-hal')) return;
  if (!yesOrNo('Install Intel webcam drivers?')) return;
  print('Installing Intel webcam drivers...');
  await Dnf.install([
    'intel-media-driver',
    'intel-vision',
    'akmod-intel-ipu6',
    'ipu6-camera-bins',
    'ipu6-camera-hal',
    'gstreamer1-plugins-icamerasrc',
    'akmod-v4l2loopback',
    'v4l2-relayd',
    'libcamera',
    'libcamera-gstreamer',
    'libcamera-v4l2',
  ]);
  print('Your webcam should work after a reboot :)');
}

Future<void> _installIntelBatteryOptimizer() async {
  if (Dnf.installed('intel-lpmd')) return;
  if (!yesOrNo('Install Intel\'s battery optimizer?')) return;
  print('Installing Intel\'s battery optimizer...');
  await Dnf.install(['intel-lpmd']);
  await run('sudo', ['systemctl', 'enable', '--now', 'intel_lpmd']);
  await run('sudo', ['intel_lpmd_control', 'AUTO']);
}

Future<void> _installNvidiaGpuDrivers() async {
  if (Dnf.installed('libva-nvidia-driver')) return;
  if (!yesOrNo('Install (proprietary) Nvidia drivers?')) return;
  print('Installing Nvidia drivers...');
  await Dnf.install([
    'akmod-nvidia',
    'xorg-x11-drv-nvidia-cuda',
    'libva-nvidia-driver.{i686,x86_64}',
  ]);
}

Future<void> _installBroadcomFingerprintDrivers() async {
  final lsusb = runSilent('lsusb', []);
  final hasOlderBroadcom = lsusb.contains('0a5c:584');
  final hasNewerBroadcom = lsusb.contains('0a5c:586');
  if (!hasOlderBroadcom && !hasNewerBroadcom) return;
  final driver = hasOlderBroadcom
      ? 'libfprint-2-tod1-broadcom'
      : 'libfprint-2-tod1-broadcom-cv3plus';

  if (Dnf.installed(driver)) return;
  if (!yesOrNo('Install Broadcom fingerprint drivers?')) return;
  print('Installing Broadcom fingerprint drivers...');

  await run('sudo', ['dnf', 'copr', 'enable', 'grahamwhiteuk/libfprint-tod']);
  await Dnf.swap('libfprint', 'libfprint-tod');
  await Dnf.install([driver]);
}
