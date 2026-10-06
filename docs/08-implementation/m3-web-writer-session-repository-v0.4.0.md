# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/08-implementation/m3-web-writer-session-repository-v0.4.0.md
# 📌 Amac: M3 Web Writer TKO session Repository ve lifecycle Service kararlarini belgeler
# 📌 Modul - FileType: Documentation - Markdown
# Version: 0.4.0
# Aciklama: Rich TKO byte payloadinin kayipsiz session state olarak korunmasini ve import/export Service composition sinirini tanimlar

Bagimli Oldugu Katman: Controller -> Service -> Repo -> Tool

# M3 Web Writer Session Repository v0.4.0

## Karar

Writer Web oturumu, TKO importundan gelen rich byte payloadini basit Core text kaydina donusturmez.

Aktif session kaydi su verileri birlikte tutar:

- fileName
- izole TKO byte kopyasi
- Rust-backed inspect summary

Bu Repository session-only ve in-memory'dir. IndexedDB canonical Core document store ile karistirilmaz.

## Neden ayri Repository

Mevcut Web canonical Core kaydi id/title/text/schema/revision alanlarina sahiptir. Writer TKO ise section, table, image ve asset payloadlari tasiyabilir.

Rich Writer TKO'yu yalniz plain text kaydina indirgemek round-trip veri kaybina yol acar. Bu nedenle M3 Writer session hatti kaynak TKO bytes'i korur.

## Lifecycle

WebWriterSessionService:

1. importNativeDocument ile TKO'yu codec hattinda dogrular.
2. Basarili sonucu session Repository'ye clone ederek yazar.
3. Picker cancel durumunda mevcut session'i degistirmez.
4. Export isteginde aktif session byte'larini WebImportExportService canonical re-encode hattina verir.
5. Aktif session yoksa fail-closed hata verir.
6. Close yalniz session Repository'yi temizler.

## Kapsam disi

- Rich Writer editing
- TKO -> editable full Writer DTO projection
- IndexedDB rich Writer persistence
- Kullaniciya acik Open/Download View aksiyonlari
- Offline cache

View aksiyonlari ayri bir sonraki M3 diliminde acilacaktir.
