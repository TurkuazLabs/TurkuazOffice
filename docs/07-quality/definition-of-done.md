# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/definition-of-done.md
# 📌 Amac: Bir Turkuaz Office degisikliginin tamamlanmis sayilma kriterlerini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Kod, docs, security, accessibility, schema ve performance kontrollerini tek kalite bariyerinde toplar

Bagimli Oldugu Katman: Documentation

# Definition of Done

Bir degisiklik ancak asagidaki kosullari sagliyorsa tamamlanmis kabul edilir:

1. Layer ihlali yok.
2. Controller is kurali icermiyor.
3. Yeni davranis testli.
4. Public contract etkisi dokumante.
5. Magic string eklenmemis.
6. Inline config eklenmemis.
7. Hata mesaji kullanici verisi sizdirmiyor.
8. Format etkisi varsa fixture/round-trip plani var.
9. Schema etkisi varsa migration ve compatibility etkisi dokumante.
10. Guvenilmeyen input etkisi varsa resource/security limiti degerlendirilmis.
11. UI etkisi varsa keyboard ve accessibility davranisi kontrol edilmis.
12. Performance-sensitive degisiklik ilgili budget'i bozmadigi gosterilmis.
13. Localization etkisi varsa literal kullanici metni eklenmemis.
14. Roadmap disina cikilmamis.
15. Dosya header standardi korunmus.
