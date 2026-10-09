# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/06-adr/0021-suite-command-reuse-boundary.md
# 📌 Amac: Office modulleri arasinda tekrar etmeyen komut ve ortak UI altyapisi kararini tanimlar
# 📌 Modul - FileType: Architecture - Markdown
# Version: 0.1.0
# Aciklama: Ortak komut metadata, modul kabiliyeti, arama siniri, platform adaptorleri ve sifir olu kod prensiplerini tanimlar

Bagimli Oldugu Katman: Controller | Service | Repo | Tool | View | Language | Config

# ADR 0021 - Suite Command Reuse Boundary

Durum: Onerilen; mevcut runtime davranisini degistirmez.
Tarih: 2026-10-09
Baz: TurkuazLabs/TurkuazOffice main 46e5aa70f0c9fe5363e63118b53b3cc73801e2bc

## Baglam ve kanit

- `apps/desktop/src/config/keyboard.ts` Writer/Sheet typed action, `aria-keyshortcuts` ve menude gorunen kisa yol yazi tablolarini ayri ayri tutar. Degisikliklerin uyumsuz kalma riski vardir.
- `apps/desktop/src/services/keyboard-shortcut.service.ts` ve `sheet-keyboard-shortcut.service.ts` modul ozgulu shortcut cozumleyicileridir. Bunlarin komut davranislari farklidir.
- `apps/desktop/src/views/writer-shell.tsx` ve `sheet-shell.tsx` `window keydown` dinleyicilerini kendi icerisinde kurar. Ortak olay normalizasyonu adayidir; tum editor event yonetimini ortaklastirmak degildir.
- `apps/desktop/src/config/app-container.ts` iki modulun service/repository/tool nesnelerini tek kompozisyon noktasinda kurar; bu, kod tekrarini degil olculebilir startup maliyeti adayini gosterir.
- `apps/desktop/src/services/writer-session.service.ts` 1321 satir ve `sheet-session.service.ts` 978 satirdir. Sorumluluk yogunlasmasi riski vardir; satir sayisi tek basina olu kod kaniti degildir.
- `apps/desktop/src/views/sheet-functions-sidebar.tsx` yalnizca fonksiyon katalogu aramasi yapar. Writer metin arama veya Sheet hucre arama motoru degildir.
- `docs/05-roadmap/roadmap.md` M3 Web'i aktif, R2.y Desktop UI Refresh'i de aktif olarak tanimlar. Yeni Find/Replace gibi urun ozellikleri oncesinde roadmap acik olarak guncellenmelidir.

## Karar ilkeleri

1. Once var olan uyeleri bul: yeni service/tool/view eklemeden once `call site`, route, command id, kullanici aksiyonu ve mevcut test izi yazilir.
2. Ortaklik en az iki gercek runtime tuketicisi ile kanitlanir. Tahmini Slides, Draw, Cloud veya Mobile icin bos interface, switch ve provider eklenmez.
3. Ortak katman yalnizca keyboard event normalizasyonu, capability filtreleme, typed command ID, shortcut metadata ve ortak arayuz semantigini tasir. Writer ve Sheet belge mutasyonu kendi Core Service katmaninda kalir.
4. Menu, toolbar, kisayol, tooltip ve `aria-keyshortcuts` bir kaynaktan turetilen ayni command definition uzerinden beslenmelidir. Desteklenmeyen command ekranda calisir gibi sunulmaz.
5. Controller sadece request alir, Service'e iletir. Service izin, enablement, context ve command is kurallarini cozer. Repo session/canonical store'dur; Tool platform ve DOM/Tauri/WASM adaptoru; View yalnizca render/focus; Language etiket tanimidir.
6. UI common command kaydi ile Rust canonical command'i ayni sinif degildir. Rust Core typed operation tek canonical truth olarak kalir; TypeScript UI yalnizca tetikler.
7. Web, Desktop, Pro ve diger platformlar arasinda kopyalanmis domain servisleri olusmaz. Rust Core/WASM ve public capability/API kontratlari kullanilir. Desktop'a ozgu Tauri/DOM detayi Core'a tasinmaz.
8. Platform, editable-focus, IME, AltGr, print/recovery dialog ve native browser shortcut cakismalari once Service tarafinda cozulur. `preventDefault` sadece etkin ve destekli komuta uygulanir.
9. Var olan `Ctrl+B`, `Ctrl+S` gibi komutlar calismaya devam eder; yeni kayit modeli eski binding'lerle parity testini gecmeden eskisi silinmez.
10. Yalnizca gercek erisilemez dosya/export/bagimlilik kanitlandiginda silinir. Dummy API, kullanilmayan generic framework, bos future module adapter ve placeholder command yasaktir.

## Find / Replace ayirimi

- Ortak olan: `FindQuery`, `FindOptions` (case, whole-word gerekli ise), match count/selection/navigation, find bar focus/escape durumu ve komut tanimlari.
- Writer'e ozel olan: paragraph/run sinirlari, Unicode scalar offset, IME, selection, read-only ve styled insert/replace, tek undo zinciri. DOM text canonical arama kaynagi olmaz.
- Sheet'e ozel olan: sparse worksheet address, hucrede ham formuller vs gorunen degerler secimi, filtre/gizli hucre politikasi, range ve performans limitleri, hucre tipi ve yeniden hesaplama.
- `Ctrl+F`: once tamamlanan iki modul Service'i ve calisan arama View'u ile acilir. Sadece keyboard binding eklemek ozellik tamamlandi demek degildir.
- `Ctrl+H`: ancak gercek Replace mutation, file protection, typed errors ve undo testleri mevcut oldugunda acilir. Bul islevi tamamlanmadan replace registry eklenmez.
- Web tarayicisinda `Ctrl+F` zorla yakalanmaz; belgenin focus/runtime capability'sine gore acik karar gerekir.

## Minimum onayli teknik dilimler

### Dilim A - Olcum ve gozlem
- Desktop ve Web entrypoint map; Rust Tauri command map; test-only/dynamic entrypoint map.
- UI config shortcut action/hint/aria eslesme ve cakisma envanteri.
- TS `noUnusedLocals` / `noUnusedParameters` raporu; Knip (entrypoint dogrulamasi sonrasinda); Rust Cargo dependency lint ve cargo-machete aday raporu.
- Raporlar ilk asamada advisory; yanlis pozitif raporlar gerekceli kayda alinir. Dogrulanmadan toplu silme/otomatik fix yapilmaz.

### Dilim B - Var olani ortaklastirma
- Gercekten tekrar eden keyboard modifier/key normalize kodu Service/Tool sinirina cekilir.
- Typed command definition kaydindan menu/shortcut/aria/hint eslenir.
- Writer/Sheet service-specific command routing ve typed actions korunur.
- Iki modulden birinde bulunmayan komuta sahte handler ya da disabled shortcut eklenmez.

### Dilim C - Gercek urun ozelligi
- Roadmap guncellemesi sonrasi once Find Service + calisan UI + memory/performance regression.
- Sonra veriyi bozmayacak Replace Service + undo/readonly semantigi.
- Aktif M3 Web scope ve Pro public API siniri ihlal edilmez.

## Kabul kriterleri

- Her yeni shared export iki veya daha fazla gercek production call site ile gosterilir; degilse modul lokal kalir.
- Bir menu/toolbar/keyboard tetigi ayni command ID ile ayni enablement sonucuna gider.
- Inactive action `preventDefault` cagirmaz. IME ve AltGr false positive olusturmaz.
- Writer/Sheet uygulama bazli kendi Service testleri vardir. UI smoke (focus restore, Escape, read-only, menu action) vardir.
- Yeni runtime command, menu label ve shortcut eksiksiz testlenmeden ekranlara eklenmez.
- Knip ve cargo-machete advisory bulgulari production entrypoint ve feature-gated/FFI/macro kullanimlari ile dogrulanmadan silinmez.
- `cargo fmt`, `cargo check`, `cargo test`, frontend TypeScript/Vitest ve Windows/Linux CI korunur.

## Kaynaklar

- LibreOffice UNO command dispatch: https://dev.blog.documentfoundation.org/2022/02/23/adding-a-new-uno-command/
- W3C WAI-ARIA keyboard guidance: https://www.w3.org/WAI/ARIA/apg/practices/keyboard-interface/
- W3C aria-keyshortcuts: https://www.w3.org/TR/wai-aria-1.3/
- Tauri 2 Rust commands: https://v2.tauri.app/develop/calling-rust/
- Knip unused dependencies: https://knip.dev/typescript/unused-dependencies
- Knip reported issue validation: https://knip.dev/guides/handling-issues
- Cargo lints: https://doc.rust-lang.org/stable/cargo/reference/lints.html
- cargo-machete: https://github.com/bnjbvr/cargo-machete
