# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/04-operations/release-distribution.md
# 📌 Amac: Windows Linux ve gelecek platformlar icin build, signing, updater ve paket dagitim sinirlarini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Release artefactlarinin izlenebilir ve dogrulanabilir olmasini saglayan operasyon politikasidir

Bagimli Oldugu Katman: Documentation

# Release ve Distribution

## Desktop hedefleri

M1 boyunca:

- Windows installer.
- Windows portable evaluation build opsiyonel.
- Linux AppImage.
- Linux deb.

RPM/Flatpak/Snap sonraki distribution ihtiyacina gore acilir.

## Signing

Public stable release oncesi platform signing zorunludur. Unsigned development build stable release etiketi alamaz.

## Updater

Auto-updater default olarak signed manifest ve integrity check kullanmadan etkinlestirilmez. Update channel en az stable ve preview olarak ayrilabilir.

## Reproducibility

CI build metadata; git commit, platform, toolchain ve dependency lock bilgisini artefact metadata'ya yazar.

## Rollback

Schema migration forward-only olsa bile application update rollback durumunda future-schema belge riski vardir. Stable release migration policy bu riski compatibility test ile kapatmadan destructive schema migration yapmaz.


## Native TKO file association

M1 Desktop Writer native .tko association'i bundle-time installer metadata ile kurulur.

- Windows: npm run tauri:build:windows -> NSIS installer.
- Linux: npm run tauri:build:linux -> DEB + AppImage.
- MIME: application/x-turkuaz-office.
- DEB association metadata tasir.
- AppImage portable build association kaydini garanti etmez.

Association ile cold-start edilen process, path'i process argument olarak alir. StartupArgumentsTool ve StartupFileService yalniz existing canonical .tko path'i kabul eder.

Calisan uygulamaya ikinci file-open isteginin tek instance'a aktarimi bu M1 minimumunun disindadir.
