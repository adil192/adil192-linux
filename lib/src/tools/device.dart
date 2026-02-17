import 'dart:io';

import 'package:adil192_linux/src/tools/result_of_command.dart';

/// Information about the user's device
final class Device {
  static bool hasAmdCpu() => cpuInfo.contains('amd');
  static bool hasIntelCpu() => cpuInfo.contains('intel');

  static bool hasAmdGpu() => gpuInfo.toLowerCase().contains('amd');
  static bool hasIntelGpu() => gpuInfo.toLowerCase().contains('intel');
  static bool hasNvidiaGpu() => gpuInfo.toLowerCase().contains('nvidia');

  static final cpuInfo = File('/proc/cpuinfo').readAsStringSync().toLowerCase();

  // lspci | egrep -i "vga|display|3d"
  static final gpuInfo = resultOfCommandSync('lspci', ['-mm'])
      .split('\n')
      .where(
        (line) =>
            line.toLowerCase().contains('vga') ||
            line.toLowerCase().contains('display') ||
            line.toLowerCase().contains('3d'),
      )
      .join('\n');
}
