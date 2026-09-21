# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m1-clipboard-styled-fragment-validation.md
# 📌 Amac: M1 Clipboard styled-fragment mutation diliminin validation sonucunu kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Static contract, domain test kapsami ve ortam sinirlarini belgeler

Bagimli Oldugu Katman: Documentation

# Clipboard Styled Fragment Validation

## Sonuc

Static proje verifier runtime clipboard dosyalarini ve event baglantilarini da zorunlu contract olarak kontrol eder. Compiler-backed sonuc GitHub Actions workspace CI ile ayrica dogrulanir.

`tools/verify-project.sh` yeni typed fragment command, frontend IPC key, Service/Controller baglantisi ve domain regression testlerini zorunlu contract olarak kontrol eder.

## Frontend unit test kapsami

Vitest Service-level regression testleri internal MIME paste priority, sanitized HTML fallback, plain-text fallback, canonical copy payload ve cut-before-empty-fragment mutation siralamasini kapsar.

## Runtime acceptance kapsami

- Copy ayni secimi internal MIME, HTML ve plain text olarak yazar.
- Paste representation onceligi internal MIME, sanitized HTML, plain text ve image-only payload sirasindadir.
- Internal payload schema/version/type/font limitleri dogrulanir.
- HTML parser sonucunda executable/resource taglari atilir; yalniz basic text style semantigi canonical run'a map edilir.
- Cut bos fragment mutation ile tek undo adiminda calisir.
- Paste/cut sonrasi caret eklenen fragment sonuna tasinir.
- PNG/JPEG/WebP image-only paste active paragraph sonrasina canonical ImageBlock + WriterAsset olarak eklenir.
- Image binary asset TKO package round-trip ile korunur ve reopen sonrasinda lazy writer_get_asset IPC ile render edilir.
- Image insert undo/redo snapshot ile asset registry ve block'u birlikte geri alir/geri getirir.
- Gecersiz/unsupported paste payload'inda browser default contenteditable paste calismaz.

## Eklenen regression kapsami

- Styled fragment replace komsu run stillerini korur.
- Iki farkli fragment stili canonical run olarak korunur.
- Styled paste tek undo ile onceki belge snapshot'ina doner.
- Tum paragraf empty-fragment cut sonrasinda tek editable bos run tasir.

## CI runner durumu

GitHub Actions run'lari workflow parse ediliyor olsa da hosted job'lar `runner_id: 0` ve `steps: []` ile step baslamadan failure oluyor. Bu nedenle son commitler icin compiler-backed sonucu kod failure'i olarak yorumlamiyoruz. Onceki runner calisan run'da frontend build basariliydi; eski Rust failure'lari Rust 1.85 dependency uyumsuzlugu ve rustfmt farklariydi, bunlar toolchain 1.98.1 ve rustfmt normalization ile duzeltildi.

## Ortam siniri

Bu calisma ortaminda `cargo` ve `rustc` bulunmadigi icin compiler-backed Rust testleri burada calistirilamadi. Bu nedenle Rust compile/test sonucu basarili olarak iddia edilmez.

Frontend dependency install bu ortamda tamamlanmadigi icin TypeScript production build de compiler-backed kabul kriteri olarak kapatilamadi.

CI veya local development makinesinde asagidaki kontroller zorunludur:

- `cargo fmt --all -- --check`
- `cargo test --workspace`
- Desktop dependency install sonrasi frontend build/typecheck

Static verifier gecmesi compiler-backed kontrollerin yerine gecmez.
