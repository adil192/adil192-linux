import 'package:adil192_linux/src/tools/run.dart';
import 'package:adil192_linux/src/tools/yes_or_no.dart';
import 'package:adil192_linux/src/tools/which.dart';

class Dnf {
  static final hasDnf = Which.installed('dnf');

  static Future<void> install(List<String> packages) async {
    await run('sudo', ['dnf', 'install', ?yFlag, ...packages]);
    _installedPackages.addAll(packages);
  }

  static Future<void> update(List<String> packages) =>
      run('sudo', ['dnf', 'update', ?yFlag, ...packages]);

  static String repoList() => runSilent('dnf', ['repolist']);

  static Future<void> swap(
    String from,
    String to, {
    bool allowErasing = false,
  }) => run('sudo', [
    'dnf',
    'swap',
    from,
    to,
    if (allowErasing) '--allowerasing',
    ?yFlag,
  ]);

  static final _installedPackages = runSilent('dnf', [
    'list',
    '--installed',
  ]).split('\n').toSet();
  static bool installed(String package) {
    // could be package.x86_64, package.noarch, etc.
    final installed = _installedPackages.any(
      (installedPackage) => installedPackage.startsWith('$package.'),
    );

    print('$package is ${installed ? 'installed' : 'not installed'}');

    return installed;
  }

  static Future<void> configureRpmFusion() async {
    final repoList = Dnf.repoList();
    final hasFree = repoList.contains('rpmfusion-free');
    final hasNonFree = repoList.contains('rpmfusion-nonfree');
    if (hasFree && hasNonFree) return;

    if (!yesOrNo('Enable RPM Fusion repositories?')) return;

    final fedoraVersion = runSilent('rpm', ['-E', '%fedora']).trim();
    print('Installing RPM Fusion repositories...');
    await install([
      'https://mirrors.rpmfusion.org/free/fedora/rpmfusion-free-release-$fedoraVersion.noarch.rpm',
      'https://mirrors.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$fedoraVersion.noarch.rpm',
    ]);
    print('Updating Appstream metadata...');
    await update(['@core']);
  }

  static Future<void> enableCopr(String repo) =>
      run('sudo', ['dnf', 'copr', 'enable', ?yFlag, repo]);
}
