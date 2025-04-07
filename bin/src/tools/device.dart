import 'dart:io';

import 'result_of_command.dart';

/// Information about the user's device
final class Device {
  static Future<bool> hasAmdCpu() async {
    cpuInfo ??= await _getCpuInfo();
    return cpuInfo!.toLowerCase().contains('amd');
  }

  static Future<bool> hasIntelCpu() async {
    cpuInfo ??= await _getCpuInfo();
    return cpuInfo!.toLowerCase().contains('intel');
  }

  static Future<bool> hasAmdGpu() async {
    gpuInfo ??= await _getGpuInfo();
    return gpuInfo!.toLowerCase().contains('amd');
  }

  static Future<bool> hasIntelGpu() async {
    gpuInfo ??= await _getGpuInfo();
    return gpuInfo!.toLowerCase().contains('intel');
  }

  static Future<bool> hasNvidiaGpu() async {
    gpuInfo ??= await _getGpuInfo();
    return gpuInfo!.toLowerCase().contains('nvidia');
  }

  static String? cpuInfo;
  static Future<String> _getCpuInfo() => File('/proc/cpuinfo').readAsString();

  static String? gpuInfo;
  static Future<String> _getGpuInfo() async {
    // lspci | egrep -i "vga|display|3d"
    final lspci = await resultOfCommand('lspci', ['-mm'], silent: true);
    return lspci
        .split('\n')
        .where((line) =>
            line.toLowerCase().contains('vga') ||
            line.toLowerCase().contains('display') ||
            line.toLowerCase().contains('3d'))
        .join('\n');
  }
}
