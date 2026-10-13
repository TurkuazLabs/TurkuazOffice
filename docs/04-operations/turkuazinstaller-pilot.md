# 📄 Dosya Yolu: /docs/04-operations/turkuazinstaller-pilot.md
# 📌 Amac: Turkuaz Office icin TurkuazInstaller Velopack pilotunun guvenlik ve uyumluluk sinirlarini izlemek
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.6.0
# Aciklama: Test-only Velopack packaging, legacy NSIS korumasi ve Windows kabul kriterleri
# Bagimli Oldugu Katman: CI | Distribution | Documentation

# TurkuazInstaller — Turkuaz Office Windows pilot

## Scope

The first **real product** packaging pilot is Turkuaz Office Community,
not a general release channel. It leaves the existing Tauri/NSIS Windows
installer, Linux DEB/AppImage bundles, .tko association, and Writer/Sheet
Start Menu shortcuts untouched.

The experimental workflow `TurkuazInstaller Office Velopack Pilot` builds
the existing Tauri frontend assets, then the opt-in Velopack-aware Windows
Rust binary, and packages it using the **same pinned vpk 1.2.161 CLI**
used by the TurkuazInstaller real E2E test. It deliberately does NOT
build another redundant NSIS setup: the unchanged `community-preview`
workflow already produces and tests the Windows NSIS baseline.

It produces an unsigned, short-lived GitHub Actions artifact with two simulated package versions (the same desktop EXE and a distinct text marker):

- Experimental Velopack `*Setup.exe`, a v1 full `.nupkg`, and a v2 full `.nupkg`
- `SHA256SUMS.txt`
- `READ-ME-LAB-ONLY.txt`

The experimental package ID is `turkuazlabs.turkuazoffice.pilot`, intentionally
different from the existing Tauri product identifier and any future
production installer ID. This prevents a test package from being confused
with the installed production product. Never use this pilot ID in the real
production feed.

## Compatibility blockers — must fix before actual installation migration

1. **Lifecycle:** An opt-in Windows Cargo feature
   `installer-velopack-pilot` calls the official Rust Velopack
   `VelopackApp::build().set_auto_apply_on_startup(false).run()`
   **before** Tauri launches. Default NSIS and Linux binaries remain
   unchanged, and the pilot no longer bypasses `vpk` app verification.
   Signed-manifest full adoption, legacy interoperability and end-user GUI acceptance on Windows remain required.
2. **File associations:** `apps/desktop/src-tauri/tauri.conf.json5` lets
   Tauri/NSIS manage `.tko`. Do not replace this with registry-default hacks.
   TurkuazInstaller must use explicit signed Windows integration actions
   and supported user-controlled defaults.
3. **Shortcuts:** The NSIS hooks create Writer and Sheet shortcuts with
   `--module writer` and `--module sheet`. A new installer must preserve
   these features and uninstall them safely.
4. **Prerequisites:** WebView2 and supported Windows versions must be
   validated. A user machine is not equivalent to a GitHub CI runner.
5. **Trusted manifest:** A project-specific CMS/PKCS#7 signed manifest,
   `.p7s` sidecar, externally provisioned signer cert SHA-256 pin,
   artifact hash and sizes are mandatory. This pilot does NOT invent
   certificates or disable these requirements.
6. **Acceptance:** Use a disposable Windows test VM to exercise clean
   installation, upgrade, repair, rollback, uninstall, user data
   preservation, and incorrect-signature/hash failure cases.
7. **Coexistence/migration:** Do not silently overwrite an existing NSIS
   install or change app ownership until an explicit migration contract,
   discovery and side-by-side rollback tests are proven.

## CI and release policy

Run **Actions → TurkuazInstaller Office Velopack Pilot** on the PR or
manually on main once reviewed. Download the `LAB-ONLY` artifact and
verify SHA-256 checksums. It is retained only three days.

The existing `community-preview` workflow remains the normal Windows
NSIS + Linux packaging process. No tag, GitHub Release, signing bypass,
Pro entitlement, OIDC token, or production update feed is created.

When all real Windows acceptance gates pass, integrate project-specific
release artifacts with the **Community TurkuazInstaller**
`docs/PROJECT_INTEGRATION.md` standard on a separate reviewed PR.
Until then, do not remove NSIS or call this production-ready.

## Installer vs application update authority

The pilot exclusively adds `velopack = "=1.2.161"` behind a nondefault
Windows Cargo feature. The application handles Velopack lifecycle hooks,
but cannot independently apply pending updates on startup
(`set_auto_apply_on_startup(false)`), since TurkuazInstaller must retain
manifest signature, artifact SHA-256, staging and Pro policy control.

The pilot is still unsigned and must never be used as a customer
installer. See https://docs.velopack.io/getting-started/rust .

## Headless lifecycle hook regression

`tools/installer/Test-VelopackLifecycleHooks.ps1` executes all four native
Velopack lifecycle argument cases (`--veloapp-install`,
`--veloapp-obsolete`, `--veloapp-updated`, `--veloapp-uninstall`) against
the opt-in pilot binary. Each must exit successfully within 12 seconds
without launching the Tauri UI. A timeout kills the test process and fails
the workflow. This is NOT the same as the actual Setup.exe install/upgrade
acceptance, which still needs a disposable Windows VM with signed manifests.

## Real Windows host smoke test — installer engine, not production signature acceptance

The pilot also runs `tools/installer/Test-TurkuazOfficeVelopackInstall.ps1`
on the ephemeral GitHub-hosted Windows x64 runner, after packaging:

1. Strictly checks `SHA256SUMS.txt` against the newly built unsigned
   `Setup.exe` and `full.nupkg`.
2. Creates a unique test install path under GitHub `RUNNER_TEMP`,
   never touching the user's existing NSIS installation.
3. Executes **real** `Setup.exe --silent --installto <test-root>` with a
   bounded process timeout; checks for both `Update.exe` and the Office
   executable in the installed `current` directory.
4. Calls **real** `Update.exe --silent --rootDir <test-root> uninstall`,
   then asserts installed binaries are absent.
5. Always attempts best-effort official uninstall if a stage failed,
   removing only the disposable test folder. No production registry,
   file association or existing NSIS install is changed deliberately.

**Important**: This validates actual initial install and uninstall
mechanics, beyond only compiling or packaging. It does NOT validate
signed project manifests, an Authenticode-pinned publisher, protected
artifact distribution, product data retention, updates, repair, rollback,
NSIS side-by-side behavior or end-user GUI interactions. All those
remain required before a stable TurkuazInstaller release.

## Full package lifecycle CI (two lab-only package versions)

As an additional pre-release validation, the CI deliberately creates a
second, **artificial** `PATCH+1` full Velopack package containing the same
compiled native Office executable with a changed
`installer-pilot-version.txt` marker. This is NOT an actual new application
release and must not be published to customer update feeds.

The real Windows runner verifies each package hash, installs the first
`Setup.exe`, then uses the actual Velopack updater CLI to apply the second
full `.nupkg`. It asserts the on-disk marker changed. It repeats the
same-version full package for repair, then applies the first full package
for rollback and checks that the original version returned.

The test also creates a sentinel under the installation's sibling
`UserData` path and verifies the sentinel survives update, repair and
rollback. Finally it runs official uninstall and waits boundedly for
active binary removal (self-cleanup is asynchronous).

These are **CLI/process mechanics**, not TurkuazInstaller's high-level
signed CMS manifest, installer state journal, release entitlement gate,
or a production-ready Tauri upgrade. A separate reviewed signed-manifest
integration and normal Windows VM test remain required.

## Script parser and repair mutation checks

Before the resource-intensive Rust build, the pilot now runs
PowerShell AST parsing for all `tools/installer/*.ps1` scripts.
A parse error fails the workflow early. In the same-version repair stage
the smoke test deliberately corrupts a non-executable marker and
requires the real updater to restore its original contents. Updater
completion also requires a bounded check of the expected file bytes.

## User-reported localhost refusal — release asset acceptance

The initial user-installed pilot showed WebView2 `127.0.0.1`
`ERR_CONNECTION_REFUSED`, **even though CI Windows install/update/repair/
rollback/uninstall were all green**. Root cause: building the opt-in binary
using raw `cargo build --release` without enabling Tauri's
`custom-protocol` Cargo feature. With Tauri v2 this chooses the
development URL instead of the locally embedded `frontendDist`.

The `installer-velopack-pilot` Cargo feature now **depends on**
`tauri/custom-protocol` only for the opt-in pilot. Normal Tauri NSIS
and Linux release behavior is untouched.

The pilot has a hidden, opt-in, feature-gated
`--office-pilot-verify-embedded-frontend` process mode. It checks the
compiled Tauri context's actual embedded `index.html` asset and rejects
dev mode with exit code 31. CI executes it once on the built native binary
and again on the **installed** Windows executable. This regression
guard prevents a mere successful Setup.exe/process exit from masking the
broken interface again.

It is not yet a full automated screenshot-based GUI acceptance. A
freshly produced LAB ONLY setup still requires manual Windows WebView2
validation and screenshot feedback. The old failing pilot artifact must
never be retested or described as a working production UI build.

## Windows desktop field feedback: setup branding and interaction (2026-10-12)

A Windows 10 manual trial of the corrected LAB ONLY pilot successfully launched
the interactive Turkuaz Office Start Center, with no visible localhost refusal.
The user found the startup fast, but reported a generic installer EXE icon and
no installation questions.

The pilot pack script now reads the same `.ico` reference as the Tauri bundle
config and passes `--icon` to **both** Velopack pack invocations. It fails
closed when the declared ICO is absent or invalid. Windows CI inspects the
actual generated `Setup.exe` icon resource before exercising installation.

The existing Velopack `Setup.exe` deliberately remains a **one-click,
per-user** installer. Quick completion is not a failure and artificial waiting
must not be added. Velopack does not provide a classic install wizard in this
path; changing the prompt behavior requires a distinct opt-in MSI / reviewed
TurkuazInstaller WinUI flow, not disabling security or layering an unsafe
wrapper around `Setup.exe`.

### Next UX acceptance scope (not implemented in this patch)

- Use the existing TurkuazInstaller WinUI presentation/controller/service
  stack to show a branded welcome, installation scope/path review,
  shortcut and file-association preferences, progress, and completion.
- Keep `.tko` association and Writer/Sheet shortcuts explicit, with no
  silent reassignment of existing user defaults.
- Route actual installation through the signed-manifest trust,
  SHA-256 and package-engine workflow; never launch unverified payloads.
- Validate a visible `Setup.exe` brand icon in Windows Explorer and
  user-facing dialogs on a disposable Windows VM. This is separate from the
  CI resource test, which cannot prove Windows icon rendering or GUI layout.
- Preserve the existing NSIS distribution and prior user data while the
  LAB ONLY pilot stays draft.

## Real Office -> TurkuazInstaller signed CLI integration LAB

After its normal real Windows Tauri Velopack lifecycle, the pilot workflow
checks out **immutable** TurkuazInstaller PR #54 commit
05431511f8be83e6f6b88df009fb9fcf05f4b525 and builds its actual
Windows x64 CLI. This is not an invented installer shim: real installed
state, release provider, detached CMS verifier, SHA-256 artifact guard,
operation journal, lock and Velopack engine all run.

The additional Test-OfficeSignedTurkuazInstallerCli.ps1 is restricted to a
disposable GitHub-hosted Windows worker. It refuses existing pilot
state/journal/trust files and any preexisting target. It uses the genuinely
built Office Setup.exe and the two existing full nupkg test packages, after
SHA-256 validation. It creates a short-lived lab RSA certificate and
externally pinned trust store, then signs real YAML manifest bytes with
detached CMS. Private keys and trust files are never uploaded.

High-level signed CLI acceptance gates:

1. Manifest tampering is rejected before installation and state creation.
2. Signed V1 Office Setup.exe installation commits verified state/files.
3. Signed V2 manifest with wrong artifact digest is rejected while V1 and
   UserData remain unchanged.
4. Signed CLI update applies the genuine V2 full nupkg.
5. Same-version signed repair restores a deliberately damaged test marker.
6. Signed rollback to V1 preserves UserData bytes.
7. Official signed CLI uninstall removes active binaries and committed state.

Each native process has a finite timeout. This is beta-channel,
test-only package identity and ephemeral LAB trust; no production signing,
background update, per-machine elevation, registry default changes,
NSIS migration or customer install is authorized. V2 uses the same
compiled Office EXE as V1, with only a changed marker: do NOT call it
a genuine product-code update.

Successful new exact-head Windows CI is mandatory before claiming these
new gates pass. The signed product trust preparation is separate Office
PR #75. PR #71 remains Draft until production and actual Windows 10/11
interactive User Experience / shortcuts / .tko ownership checks are accepted.
