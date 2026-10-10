# TurkuazInstaller — Turkuaz Office Windows pilot

## Scope

The first **real product** packaging pilot is Turkuaz Office Community,
not a general release channel. It leaves the existing Tauri/NSIS Windows
installer, Linux DEB/AppImage bundles, .tko association, and Writer/Sheet
Start Menu shortcuts untouched.

The experimental workflow `TurkuazInstaller Office Velopack Pilot` builds the
existing Windows Tauri executable and also packages it using the **same
pinned vpk 1.2.161 CLI** used by the TurkuazInstaller real E2E test.

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

1. **Lifecycle:** Tauri currently has no proven Velopack startup/update
   hooks. `--skipVeloAppCheck true` is ONLY used to test the packaging tool.
   A successful CI package does not mean a correct installer lifecycle.
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
