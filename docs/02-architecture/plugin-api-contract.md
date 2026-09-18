# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/02-architecture/plugin-api-contract.md
# 📌 Amac: Plugin API ve ABI versionlama, capability ve permission kurallarini sabitler
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Harici pluginlerin Core sinirlarini bozmadan kontrollu calismasi icin contract tanimlar

Bagimli Oldugu Katman: Documentation

# Plugin API Contract

## Version

Her plugin manifest `api_version` ve plugin version tasir. Core desteklemedigi major API version'i yuklemez.

## Capability

Plugin sadece manifestte istedigi ve kullanici/policy tarafindan verilen capability'lere erisebilir. Ornek capability siniflari:

- document.read
- document.write
- command.register
- panel.register
- import.register
- export.register
- network.request
- filesystem.scoped

## Permission

Network ve filesystem varsayilan olarak kapali kabul edilir. Capability olmadan Tool adapterina ulasilamaz.

## ABI

Native ABI M1'de public edilmez. Ilk external plugin yolu process/WASM benzeri sandboxlanabilir model degerlendirmesinden sonra acilir.

## Imza

Public marketplace veya auto-install olmadan once package integrity ve signature policy zorunludur.

## Compatibility

Plugin API major degisikligi Office Platform major version degisikligi gerektirmek zorunda degildir; ancak plugin API kendi semver lifecycle'ini tasir.
