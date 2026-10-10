# 📄 Dosya Yolu: /docs/04-operations/turkuazinstaller-pilot.md
# 📌 Amac: Turkuaz Office icin TurkuazInstaller Velopack pilotunun guvenlik ve uyumluluk sinirlarini izlemek
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.2
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

It produces an unsigned, short-lived GitHub Actions artifact with:

- Experimental Velopack `*Setup.exe` and `*-full.nupkg`
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
   Real install/update/uninstall acceptance on Windows is still required.
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
