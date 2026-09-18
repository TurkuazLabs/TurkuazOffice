# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/06-adr/0012-solidjs-desktop-ui.md
# 📌 Amac: Turkuaz Office Desktop ve ortak web UI baseline framework kararini kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: M1 prototip sonucunda SolidJS + TypeScript + Vite seciminin gerekce, sinir ve geri donus kosullarini belgeler

Bagimli Oldugu Katman: Documentation

# ADR 0012 - SolidJS Desktop UI Baseline

## Durum

Accepted for M1 Desktop Writer shell.

## Karar

Desktop frontend baseline `SolidJS 1.9.x + TypeScript 7.x + Vite 8.x` olarak secilmistir. Tauri shell frontend-framework bagimsiz kalir; canonical Writer mantigi Rust Service katmaninda kalmaya devam eder.

## Neden

- Fine-grained reactivity Writer ve Sheet gibi cok sayida kucuk UI guncellemesi olan yuzeylere uygundur.
- Virtual DOM zorunlulugu olmadan gercek DOM uzerinde calisir.
- TypeScript ve Vite entegrasyonu basittir.
- Desktop ve ileride Web istemcisi ayni component standardini paylasabilir.
- Framework UI state icindir; Document Model veya format parser sorumlulugu almaz.

## Sinirlar

- Rich text editing motoru SolidJS component state'ine baglanmayacak.
- Canonical document state Rust Core/Writer tarafinda kalacak.
- Tauri `invoke` isimleri Tool katmaninda merkezi tutulacak.
- Accessibility ve keyboard davranisi framework shortcut'i ile gecistirilmeyecek.
- Mobile UI icin ayni framework kullanimi zorunlu karar degildir; Core kontrati platformdan bagimsizdir.

## Yeniden degerlendirme kosulu

SolidJS'in accessibility, IME, WebView veya buyuk Sheet virtualizasyonu gereksinimlerinde somut engel olusturdugu benchmark/test ile kanitlanirsa ADR yenilenebilir. Framework degisikligi Document Model veya Rust Service rewrite'i gerektirmemelidir.
