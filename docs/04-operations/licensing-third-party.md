# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/04-operations/licensing-third-party.md
# 📌 Amac: Turkuaz Office lisans karari ve ucuncu taraf dependency lisans surecini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Public release oncesi yanlis lisans verme riskini onleyen gecici politika ve dependency notice surecidir

Bagimli Oldugu Katman: Documentation

# Licensing ve Third-Party

## Kaynak kod lisansi

Foundation v0.1.1 asamasinda TurkuazLabs tarafindan nihai open-source lisans secimi explicit olarak onaylanmamistir. Bu nedenle repository root `LICENSE.md` gecici olarak all-rights-reserved durumunu bildirir. Public open-source release oncesi MIT, Apache-2.0 veya secilecek baska lisans ADR ile sabitlenir.

## Dependency kabul kriteri

Yeni dependency eklenirken en az su bilgiler review edilir:

- Proje URL.
- License identifier.
- Runtime/build-only durumu.
- Copyleft etkisi.
- Native binary dagitim etkisi.
- Security maintenance durumu.

## Third-party notice

Dagitilan binary icindeki notice gerektiren dependency'ler `THIRD_PARTY_NOTICES.md` altinda tutulur. Liste CI ile otomatik uretilmeye uygun tasarlanir.
