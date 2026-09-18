# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/06-adr/0013-desktop-ipc-thin-bridge.md
# 📌 Amac: Tauri IPC ile Writer domain arasindaki thin bridge mimari kararini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Frontend'in canonical belgeyi mutate etmemesi ve Controller-Service-Tool sinirinin korunmasi kararini belgeler

Bagimli Oldugu Katman: Documentation

# ADR 0013 - Desktop IPC Thin Bridge

## Durum

Accepted.

## Karar

Tauri frontend canonical Writer struct'larina dogrudan erisemez. Frontend yalnizca serializable read-only DTO alir ve typed IPC request gonderir. Rust Desktop Controller request alir, Desktop Service canonical Writer commandlarini uretir ve Writer Domain Controller/Service katmanini cagirir.

## Sonuc

- UI ile Rust domain arasinda implementation struct sizintisi yoktur.
- Web ve Mobile istemcileri ileride ayni semantic request kontratini farkli Tool adaptorleriyle uygulayabilir.
- Tauri command adlari frontend Config katmaninda merkezi tutulur.
- Hata ayrintilari yerine stabil error code DTO doner.
