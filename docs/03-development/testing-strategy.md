# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/03-development/testing-strategy.md
# 📌 Amac: Core editor format ve platform test piramidini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Core editor format ve platform test piramidini tanimlar

Bagimli Oldugu Katman: Documentation

# Test Stratejisi

## Seviye 1: Unit

Service ve pure document operation testleri. Hizli ve dis dunya bagimsiz.

## Seviye 2: Contract

Repository, Tool ve format adapter kontratlarinin tum implementasyonlarda ayni davranisi gostermesi.

## Seviye 3: Fixture

DOCX/XLSX/PPTX gibi gercek fixture dosyalari ile import/export ve round-trip.

## Seviye 4: Integration

Desktop bridge -> Controller -> Service -> Repository akisinin birlikte testi.

## Seviye 5: UI/E2E

Kritik kullanici akislarinin Windows ve Linux'ta otomatik kontrolu.

## Veri kaybi senaryolari

Crash, disk full, permission denied, corrupted file ve interrupted save zorunlu test kategorisidir.
