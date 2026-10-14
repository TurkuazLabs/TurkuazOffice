# 📄 Dosya Yolu: /docs/04-operations/turkuazinstaller-pilot.md
# 📌 Amac: Turkuaz Office icin TurkuazInstaller Velopack pilotunun guvenlik ve uyumluluk sinirlarini izlemek
# 📌 Modul - FileType: Docs - Markdown
# Version: 1.4.0
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
Rust binaries, and packages them using the **same pinned vpk 1.2.161 CLI**
used by the TurkuazInstaller real E2E test. It deliberately does NOT
build another redundant NSIS setup: the unchanged `community-preview`
workflow already produces and tests the Windows NSIS baseline.

The updated pilot produces an unsigned, short-lived GitHub Actions artifact with two **separately compiled** Windows executables (a different LAB-only Cargo variant per package), not the same EXE under two marker versions. Both binaries still share the same Office source and business logic; this is not a historical source-version upgrade:

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
11c75eacc745b0acf141b7fd5ccaa7cac351df2b and builds its actual
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

## Complete installed Office payload audit, not just a version marker

The next signed Office workflow phase compiles the pinned Installer
`TurkuazInstaller.Inventory.LabProbe` in the same isolated checkout.
After the real V2 Office CLI update, it resolves the signed V2 full
package manifest through the independent external CMS certificate pin,
confirms the true nupkg size and SHA-256, then enumerates every
`lib/app` file with the shared Installer builder (including Velopack
relocated updater/launcher binaries). Raw deterministic inventory JSON
is signed with the same temporary LAB CMS certificate.

The **real** `TurkuazInstaller.Cli.exe audit` command then checks the
V2 disk state against this signed archive-derived inventory. It must:
- accept a clean V2 report without any unaccounted `current/` files;
- return nonzero with an explicit marker hash mismatch after controlled
  corruption, with no automatic repair;
- accept clean V2 again **only after** the user-invoked explicit repair;
- build and sign a separate V1 full nupkg inventory, then accept the
  actual V1 rollback disk state.

The audit retains its journal and state-change checks. The inventory
probe never creates production private keys or installer trust anchors.
This still does not prove snapshot atomicity, full installation-root
ownership, actual second Office source version, UAC/NSIS migration,
end-user UX, or publicly trusted code signing. The new exact-head CI
must succeed before any of these claims are considered verified.

## Additional fail-closed signed inventory acceptance

After the first complete real Office signed V2 inventory passed on
[Windows CI #38367693830](https://github.com/TurkuazLabs/TurkuazOffice/actions/runs/38367693830)
(all five signed expected payload files matched, damaged version marker
was detected and restored by explicit repair, and V1 rollback audit
passed), a new adversarial acceptance phase was added.

While the real V2 Office installation is active, the disposable hosted
Windows test must now:

1. Copy the signed V2 inventory and valid detached signature, append
   harmless JSON whitespace **after** signing and require operation
   failure exit code 4, never silently validating the modified bytes.
2. Replace only the lab's independently provisioned external
   trust-store SHA-256 pin with an incorrect pin. Require operation
   failure exit 4, then restore the original trust policy in a
   structured finally block and require a clean audit again.
3. Add a benign unexpected file inside the actual installed
   current/ directory. Require exit 4, an explicit unattributed path
   and **no mismatch of previously signed files**. Assert that audit
   has **not deleted or modified** the unexpected file. Explicitly
   clean only this GUID-isolated test fixture, then require a
   clean signed audit again.

The existing corrupted expected-file, repaired, rollback and uninstall
checks run afterward. These are test-only adversarial fixtures, **not**
automatic repair permission, production signing, recovery journal
unlock or a replacement for Windows 10/11 UAC and NSIS migration
acceptance. The latest exact-commit CI must be green before claiming
the new negative gates passed.

## Recovery hold and cross-version baseline rejection on real Office

The full inventory pilot passed on [Windows CI #38377433913](https://github.com/TurkuazLabs/TurkuazOffice/actions/runs/38377433913), including
(1) a post-signature JSON tamper rejected with exit 4;
(2) a deliberately incorrect independently pinned certificate rejected with exit 4;
(3) an unexpected actual installed `current/` file reported without deletion
and followed by a clean audit; and (4) marker corruption/explicit repair/V1
rollback/inventory clean/uninstall.

Two further **non-production negative acceptance** tests now run on the
already-installed genuine Office V2 binary:

- Derive and ephemeral-CMS-sign the true V1 full-package file inventory.
  Present it to the valid signed **V2** manifest. Even a valid signature
  from the same pinned signer cannot authorize use of the wrong release
  identity or archive digest; require operation failure exit 4, no changed
  installed state or UserData.
- Create a distinct **test-owned** applying-phase (5) update (1) journal
  under the real Installer per-user journal directory. The read-only CLI
  audit must reject the otherwise clean V2 installation with exit 4
  and preserve the exact journal bytes. Test cleanup then removes only
  its own created checkpoint and requires clean V2 audit again.

No runtime journal deletion, recovery ownership, automatic repair or
unsafe restart policy is introduced. This validates existing fail-closed
`audit` behavior across the actual Office/Installer boundary. The new
exact-head Windows CI is required before these two additional claims can
be called verified. Existing Tauri NSIS installer, public-trust signing,
real different-version Office binaries, file-association migration and
interactive Windows 10/11 UAC acceptance remain separate open gates.

## Missing detached signatures and cross-product rollback negative gates

The exact-head [Office Windows CI #38386114359](https://github.com/TurkuazLabs/TurkuazOffice/actions/runs/38386114359)
verified all four Office workflows green, including:
- V1 valid CMS inventory rejected against the V2 signed release, exit 4.
- An Applying-stage update journal causes the otherwise clean signed
  Office audit to fail closed, exit 4, while preserving the journal's
  exact SHA-256; explicit test-owned cleanup restores a clean audit.
- Full V1→V2→repair→V1 rollback→uninstall mechanics and UserData checks.

The next real-Office LAB gate adds independent **missing signature**
checks to reject a manifest lacking its detached `.p7s` at both clean
installation and V1→V2 update. It also rejects a copied genuine V2
inventory JSON when its detached signature is missing. Expected exit
code is an operation failure (4), not an invalid command syntax (2),
and Office state and preserved data are rechecked after each rejection.

A separate LAB-only package ID receives a **second independently
provisioned trust-store entry** pointing to the same short-lived CMS
signer. A valid, foreign-package V1 rollback manifest is presented
against the installed Office V2 product. The CLI must still refuse
rollback based on package identity (exit 4), leave V2 installed and
pass its original clean signed inventory audit afterward. Same signer
trust does not authorize crossing product boundaries.

The new CI has not yet been verified. Do not use this shared test
certificate, lab-only identities or external trust entries for
production releases; normal Office NSIS remains authoritative.

## Two separately compiled Windows executables (new LAB acceptance)

The opt-in Windows CI now invokes Cargo **twice** using the same pinned
toolchain and source tree:
- `installer-velopack-pilot` creates `.pilot-vpk/binaries/office-v1.exe`.
- `installer-velopack-pilot-v2` implies the first feature and creates
  `.pilot-vpk/binaries/office-v2.exe`. Only the opt-in LAB executable
  attestation branch is different; Writer/Sheet runtime features remain
  unchanged.

Both compiled EXEs must embed the release frontend, accept their own
headless `--office-pilot-verify-compiled-v1/v2` argument, refuse the
opposite identity with exit 32 and have **different SHA-256 hashes**.
The Velopack packager consumes V1 compiled bytes for Setup + V1 full
nupkg and V2 compiled bytes for V2 full nupkg; it refuses identical
inputs. The direct Velopack lifecycle and real pinned
TurkuazInstaller signed CLI lifecycle check the **installed Windows EXE
hash against its own compiled input** after every install,
update, repair and rollback, not merely a marker file or state JSON.

The release artifact continues to be LAB-only and unsigned; the
production NSIS and DEB/AppImage releases are unchanged. These two
build variants demonstrate actual EXE replacement and byte-level
rollback through the real installer engines. They **do not** constitute
two separately developed/authentic Office source revisions, public
trusted signatures, a fully validated GUI launch, or production
patch compatibility. The new exact-head Windows CI must pass before
the two-executable improvement is called verified.

## LAB source and executable provenance

The verified two-executable Windows run
[#38399146552](https://github.com/TurkuazLabs/TurkuazOffice/actions/runs/38399146552)
passed both native Velopack and independently CMS-trusted Installer CLI
workflows, installed binary SHA-256 checks, signed full-file inventory,
UserData persistence, repair, rollback, and uninstall.

A follow-up acceptance gate now writes **LAB_BUILD_PROVENANCE.json**
alongside Setup, both full nupkg packages, README and SHA256SUMS.
Its fields identify the exact Git checkout commit, both distinct
feature-gated EXE SHA-256s and package hashes/sizes, the two LAB
versions, and Git-*tracked* Cargo/npm lockfile status. The provenance
explicitly asserts `lab_only=true`,
`production_release_approved=false` and
`independently_developed_office_versions=false`.
The Windows package smoke checks its contents against the checkout,
the actual compiled EXEs and both installed package source artifacts.
It is itself covered by SHA256SUMS.

`Cargo.lock` and `apps/desktop/package-lock.json` are currently
absent **from Git source tracking**. Cargo/npm can generate local
untracked locks during builds: the recorded reproducibility flags use
`git ls-files`, not simple file existence, to prevent misleading
provenance. Until tracked/reviewed locks are added and all release
builds enforce `--locked`/`npm ci` (see
[Office release gate #72](https://github.com/TurkuazLabs/TurkuazOffice/issues/72)),
this evidence is diagnostic LAB-only and cannot prove a reproducible
production release. The current-head CI result must be verified before
this new provenance validation is accepted.

## Writer/Sheet and .tko migration contract regression (read only)

The real signed Installer LAB + provenance run [#38407193439](https://github.com/TurkuazLabs/TurkuazOffice/actions/runs/38407193439)
passed all four Office workflows and confirmed the
`LAB_BUILD_PROVENANCE.json` source/EXE/package digests.

Before authorizing any migration from NSIS, preserve the **actual existing**
Windows integration contract:

- NSIS `windows/installer-hooks.nsh` installs **two** Turkuaz Office
  Start Menu entries targeting the Office EXE with
  `--module writer` and `--module sheet`, and removes both on uninstall
- Tauri `tauri.conf.json5` declares the existing NSIS hook,
  `Turkuaz Office` Start Menu folder and `.tko` file association with
  `application/x-turkuaz-office`
- The native Office launcher parses `--module writer/sheet` and
  `--module=writer/sheet`, with home as safe default

The Windows LAB pilot now runs
`tools/installer/Test-OfficeWindowsIntegrationContract.ps1` on
both separately compiled versions. It rejects a source contract drift
and invokes a real headless `--office-pilot-verify-module-launch` probe
on each executable. Both the native Velopack and the signed
TurkuazInstaller CLI test also run this headless contract check
against the *actually installed EXE* after install, update, repair
and rollback.

**Important:** these are non-mutating source and native command-line
regression checks. They do **not** install shortcuts, set `.tko`
registry associations/defaults, claim app ownership, create COM objects,
or validate real clickable Start Menu entries in a Windows 10/11 VM.
NSIS remains authoritative. A separately reviewed Windows integration
plan, clean/NSIS-coexistence tests, uninstallation ownership and
interactive acceptance are still required. The latest exact-head CI
must be green before the new read-only acceptance can be called passed.

## Portable manual Windows WinUI integration kit (draft, not production)

A user confirmed the **unsigned** Windows 10 x64 TurkuazInstaller
Bootstrapper opens and the updated UI safely disables actions until
package ID and manifest source are entered. That does **not** prove
installation through the shared WinUI runtime.

The previous Office signed-CMS lifecycle tests created two-hour ephemeral
certificates and signed YAML manifests using **GitHub runner-local**
absolute file URIs. Exporting those signed manifests would produce
unusable paths on another machine. The new
`tools/installer/Initialize-OfficeManualLab.ps1` addresses this by
creating portable *package assets*, **not portable private keys**:

1. The existing hosted Windows CI builds and verifies separate V1/V2
   Office LAB EXEs, real Velopack Setup/full-nupkg artifacts, provenance
   and full signed Installer CLI lifecycle.
2. It adds the helper and
   `tools/installer/README-OFFICE-MANUAL-WINUI-LAB.txt` to the
   **short-lived GitHub Actions artifact**, then regenerates
   `SHA256SUMS.txt` covering all included files.
3. In `-Mode VerifyOnly`, the helper checks every checksum and all
   three package hashes/sizes against `LAB_BUILD_PROVENANCE.json`.
   It does not create state, target, certificates or trust files.
4. In **explicit** `-Mode Prepare` on a disposable Windows 10/11 x64
   VM with PowerShell 7+, it first refuses to overwrite existing LAB
   install target, state, journal, trust policy or preparation folder.
   It creates a *local* temporary RSA CMS signer and uses the actual
   downloaded Office package paths in three signed manifests:
   `office-install-v1.yml`, `office-update-v2.yml`,
   `office-rollback-v1.yml`, each with a detached `.p7s`.
   The tool verifies each CMS signature before writing it and writes a
   single external manifest trust record with the public certificate
   SHA-256 pin under the normal Installer per-user config.
   The private key is disposed, **never written or uploaded**.
5. Launch the **separately downloaded** unsigned WinUI Bootstrapper.
   Enter the printed fixed package ID, **Beta** channel and V1
   manifest path; use signed manifest default target (Quick / PerUser)
   and later the printed V2/rollback manifest paths. The preparer
   refuses existing real Office installs and never attempts to adopt
   a raw NSIS or Velopack Office installation.
6. Complete installation/update/repair/rollback/uninstall in the same
   isolated VM within the local test certificate's **72-hour** life.
   Never edit signed files or the external trust policy after signing.
   Only remove test-generated trust after complete uninstall, no
   outstanding state/journal, and confirmation it belongs to this lab.

**Important:** the test certificate is deliberately self-signed and
locally trusted for **only the LAB package ID**; public-trust
Authenticode, real separate source-version upgrade, signed production
feeds, Windows 10/11 GUI/UAC/NSIS migration remain open. An existing
external trust policy is **not modified**. GitHub artifact checksums
detect accidental corruption but do not establish public producer
authenticity. The latest exact-head CI must pass before offering the
new downloadable kit.
