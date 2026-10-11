# 📄 Dosya Yolu: /docs/04-operations/office-signed-manifest-preflight.md
# 📌 Amac: Gercek Office Windows kurulum artifacts icin SHA256 ve detached CMS imzali manifest hazirligini guvenli yapmak
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.1.0
# Aciklama: Gercek signer gereksinimi, file/manifest pinning ve yayin yasaği olan lab testini aciklar
# Bagimli Oldugu Katman: CI | Distribution | Signing

# Turkuaz Office — signed installer manifest preflight

This independently reviewed feature adds a **fail-closed manifest preparation
tool**, not a release channel, certificate authority, or production signing
credential. It is separate from the unsigned Velopack package feasibility
PR #71 and the Community installer issue #74.

Run New-OfficeSignedManifest.ps1 only on Windows after the real Velopack
Setup.exe has been Authenticode signed with a trusted publisher credential.
The script refuses unsigned/invalid Windows signatures. Inputs must include
an externally approved artifact certificate SHA-256 and publisher subject,
plus a manifest signer certificate thumbprint, separately approved subject
and SHA-256 pin. The manifest signer private key is read from the Windows
CurrentUser Personal certificate store and is never exported.

The script emits installer-manifest.yml and its exact-byte detached PKCS#7
signature installer-manifest.yml.p7s together. The manifest records actual
artifact SHA-256, size, HTTPS URL, publisher identity, install target,
Windows architecture/build prerequisites, UserData preservation and rollback
policy. It pins the artifact signing certificate inside the signed manifest.
Signer identity for the **manifest** must instead be provisioned independently
through TurkuazInstaller's external manifest-trust.yml store. This script
never writes or downloads that trust anchor.

Example (values below are placeholders, not credentials):

    pwsh ./tools/installer/New-OfficeSignedManifest.ps1 \
      -SetupPath "C:\signed\TurkuazOffice-Setup.exe" \
      -ArtifactUri "https://downloads.example.invalid/TurkuazOffice-Setup.exe" \
      -ArtifactPublisherSubject "CN=Approved Publisher" \
      -ExpectedArtifactCertificateSha256 "<approved-artifact-cert-sha256>" \
      -ManifestSignerThumbprint "<certstore-sha1-thumbprint>" \
      -ManifestSignerSubject "CN=Approved Manifest Signer" \
      -ExpectedManifestCertificateSha256 "<external-manifest-cert-sha256>" \
      -OutputDirectory "C:\release-candidate\manifest"

The script requires the checked-out apps/desktop/package.json version to
match the emitted manifest. Output collisions fail rather than overwriting.
A detached signature does NOT authorize production publication. The external
trust-store provisioning, full real Office Core CLI E2E and NSIS coexistence
acceptance still must pass.

## CI safety

The preflight workflow uses an ephemeral self-signed LAB signer and a
Windows-authenticated fixture EXE only as a **metadata/CMS signing fixture**.
It does not generate a usable Office installer or store the private key.
It verifies that changed manifest bytes break detached CMS verification
and that a wrong signer certificate SHA-256 is rejected. It deletes the
fixture and lab signer after the test. No release, upload, stable update feed,
registry/file association change or end-user installation is authorized.

Production certification and full installer flow remain blocked on Office
issue #74. Run the ordinary Office pilot separately for real app binary
install/update/repair/rollback/uninstall.
