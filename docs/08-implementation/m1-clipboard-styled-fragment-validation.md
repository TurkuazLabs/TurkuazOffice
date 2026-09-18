# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m1-clipboard-styled-fragment-validation.md
# 📌 Amac: M1 Clipboard styled-fragment mutation diliminin validation sonucunu kaydeder
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Static contract, domain test kapsami ve ortam sinirlarini belgeler

Bagimli Oldugu Katman: Documentation

# Clipboard Styled Fragment Validation

## Sonuc

Static proje verifier basarili.

`tools/verify-project.sh` yeni typed fragment command, frontend IPC key, Service/Controller baglantisi ve domain regression testlerini zorunlu contract olarak kontrol eder.

## Eklenen regression kapsami

- Styled fragment replace komsu run stillerini korur.
- Iki farkli fragment stili canonical run olarak korunur.
- Styled paste tek undo ile onceki belge snapshot'ina doner.
- Tum paragraf empty-fragment cut sonrasinda tek editable bos run tasir.

## Ortam siniri

Bu calisma ortaminda `cargo` ve `rustc` bulunmadigi icin compiler-backed Rust testleri burada calistirilamadi. Bu nedenle Rust compile/test sonucu basarili olarak iddia edilmez.

Frontend dependency install bu ortamda tamamlanmadigi icin TypeScript production build de compiler-backed kabul kriteri olarak kapatilamadi.

CI veya local development makinesinde asagidaki kontroller zorunludur:

- `cargo fmt --all -- --check`
- `cargo test --workspace`
- Desktop dependency install sonrasi frontend build/typecheck

Static verifier gecmesi compiler-backed kontrollerin yerine gecmez.
