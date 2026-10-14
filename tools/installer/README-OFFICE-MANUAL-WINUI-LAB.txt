# 📄 Dosya Yolu: /tools/installer/README-OFFICE-MANUAL-WINUI-LAB.txt
# 📌 Amac: Gercek TurkuazInstaller WinUI ile Office LAB kurulumunu guvenli ve tasinabilir test eder
# 📌 Modul - Documentation Text
# Version: 1.0.0
# Aciklama: Test varliklarini dogrula, yerel ephemeral signer olustur, GUI islemleri elle test et
# Bagimli Oldugu Katman: Tool | Config | CI

TURKUAZ OFFICE x TURKUAZINSTALLER / MANUAL GUI LAB
===================================================

WARNING: TEST ONLY. Not a signed production installer or an official
update feed. Use an isolated disposable Windows 10/11 x64 VM. Do not
run on a customer PC, overwrite an existing Office installation,
disable Windows Defender/AV or force UAC elevation.

This artifact contains:
- Original LAB-only V1 Office Setup.exe, V1 and V2 full .nupkg files
- SHA256SUMS.txt and LAB_BUILD_PROVENANCE.json
- Initialize-OfficeManualLab.ps1 (script only, never a private key)
- This manual and the existing LAB README

IMPORTANT: No signed local file URI can be valid before the archive is
extracted to YOUR PC. To avoid packaging CI-only machine paths,
the included script generates a new temporary RSA key locally,
verifies each signed manifest immediately, and writes a detached .p7s.
The private signing key is NEVER exported, written to disk or uploaded.

Prerequisites:
- PowerShell 7+ ("pwsh"), Windows 10/11 x64, a throwaway test VM
- Existing TurkuazInstaller GUI unsigned test kit extracted separately
- Both GitHub Actions artifacts from their corresponding successful runs
- KEEP the original downloaded Office package files in place until
  uninstall; the signed artifact file:/// paths depend on this folder
- No previously installed LAB product, journal, trust or target folders
  at the fixed, test-only paths. The script refuses conflicts.

Steps in PowerShell 7, from the extracted Office artifact folder:
  pwsh -NoProfile -File .\Initialize-OfficeManualLab.ps1 -Mode VerifyOnly
  pwsh -NoProfile -File .\Initialize-OfficeManualLab.ps1 -Mode Prepare

The first command checks all LAB checksums, package hashes and
provenance and changes nothing. Prepare runs only after explicitly
requested. It checks for existing Installer trust/state/journal and
does NOT replace them. It creates a new LocalAppData test root,
three signed manifests and the separate pinned test trust file.

Start the separately downloaded:
  TurkuazInstaller.Bootstrapper.exe

Fields on the WinUI screen:
  Paket Kimligi: turkuazlabs.turkuazoffice.pilot
  Kanal: Beta
  Manifest: copy the INSTALL_MANIFEST path printed by Prepare
  Geri alma manifest: leave empty for the initial install
  Kurulum Dizini: leave blank (signed manifest supplies test target)

Use "Kur" > "Hizli Kurulum" > "Yalnizca benim icin" only in the VM.
Confirm "Kurulu Uygulamalar" lists the LAB package after refresh.

For UPDATE, replace manifest field with printed UPDATE_MANIFEST,
then use "Guncelle". For REPAIR, keep UPDATE_MANIFEST and use "Onar".
For ROLLBACK, keep UPDATE_MANIFEST and supply printed
ROLLBACK_MANIFEST as the previous-version source, then use "Geri Al".
For UNINSTALL, use "Kaldir". Observe the installed marker and package
version. The two compiled EXEs differ but share Office application
source code: they do NOT represent two production code releases.

If a step fails, stop; report the exact UI message and a screenshot.
Do not edit a signed .yml, .p7s, or signed package, attempt an unsafe
retry after interrupted mutation, or bypass signer verification.

Local outputs:
  %LOCALAPPDATA%\TurkuazOfficeInstallerManualLab\office-install-v1.yml(.p7s)
  %LOCALAPPDATA%\TurkuazOfficeInstallerManualLab\office-update-v2.yml(.p7s)
  %LOCALAPPDATA%\TurkuazOfficeInstallerManualLab\office-rollback-v1.yml(.p7s)
  %LOCALAPPDATA%\TurkuazInstaller\config\manifest-trust.yml
  %LOCALAPPDATA%\Programs\TurkuazOfficeInstallerLab\

The short-lived test certificate expires 72 hours after Prepare. Finish
the GUI lifecycle and uninstall within this period. After COMPLETE
UNINSTALL and verification that the journal and state are absent, the
test-only trust file may be manually removed, but ONLY if this script
created it for this isolated VM; never remove an existing trust file.
Never delete UserData or any production installation folder.

The SHA256SUMS checksum file detects local corruption, not independent
producer authenticity. A temporary lab certificate is NOT public-trust
Authenticode and does not make an unsigned Office Setup production
signed. This kit is for manual research; PRs remain Draft.
