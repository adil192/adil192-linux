import 'dart:io';

import 'result_of_command.dart';

final class Model {
  static Future<bool> hasAmdCpu() async {
    cpuInfo ??= await _getCpuInfo();
    return cpuInfo!.contains('AMD');
  }

  static Future<bool> hasIntelCpu() async {
    cpuInfo ??= await _getCpuInfo();
    return cpuInfo!.contains('Intel');
  }

  static Future<bool> hasAmdGpu() async {
    gpuInfo ??= await _getGpuInfo();
    return gpuInfo!.contains('AMD');
  }

  static Future<bool> hasIntelGpu() async {
    gpuInfo ??= await _getGpuInfo();
    return gpuInfo!.contains('Intel');
  }

  static Future<bool> hasNvidiaGpu() async {
    gpuInfo ??= await _getGpuInfo();
    return gpuInfo!.contains('NVIDIA');
  }

  static String? cpuInfo;
  static Future<String> _getCpuInfo() => File('/proc/cpuinfo').readAsString();

  static String? gpuInfo;
  static Future<String> _getGpuInfo() =>
      resultOfCommand('glxinfo', ['-B'], silent: true);
}
