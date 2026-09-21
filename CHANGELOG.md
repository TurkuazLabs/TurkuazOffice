# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/CHANGELOG.md
# 📌 Amac: Turkuaz Office surum degisikliklerini kullanici ve gelistirici seviyesinde izler
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Foundation, hardening ve Writer Domain degisiklik kaydini tutar

Bagimli Oldugu Katman: Documentation

# Changelog

## v0.2.0 - M1 Writer Development

- Session-only Print Preview canonical WriterPage read-modeli ile eklendi.
- PrintTool system print dialog adapteri canonical twip page size degerini physical @page rule'a aktarir.
- Ctrl+P preview, preview icinde Ctrl+P print ve Escape close keyboard akisi eklendi.
- Print preview belge revision, dirty baseline ve canonical document modelini mutate etmez.
- Printer, page range, copies, orientation, paper size, margin ve scale secimleri M1 minimumunda sistem dialoguna delege edilir.

- Clipboard image paste PNG/JPEG/WebP binary payloadlari canonical WriterAsset + ImageBlock command zincirine baglandi.
- TKO v1 optional asset index ve binary asset entry profile'i eklendi.
- Writer image read-model, lazy writer_get_asset IPC, Blob/ObjectURL render ve cleanup hatti eklendi.
- Gecersiz paste payload'inda browser default contenteditable paste sanitizer bypass'i kapatildi.
- Image insert undo/redo ve TKO round-trip regression kapsami eklendi.

- Clipboard runtime copy/cut/paste event hatti eklendi.
- Paste representation onceligi Internal Turkuaz MIME -> sanitized HTML -> plain text olarak baglandi.
- Clipboard payload schema/limit validation ve executable/resource HTML tag filtering eklendi.
- M1 paragraph-local paste icin external satir sonlari tek bosluga normalize edildi.
- Rust toolchain 1.98.1'e sabitlendi ve Windows desktop lib/bin output-name collision giderildi.
- Onceki CI rustfmt farklari canonical rustfmt outputuna gore normalize edildi.

- `ReplaceRangeWithStyledRuns` typed Writer command'i eklendi.
- Paragraph-local styled paste ve empty-fragment cut canonical mutation zincirine baglandi.
- Styled fragment Tauri DTO/IPC ve Desktop Tool/Service/Controller sinirlarina eklendi.
- Styled paste/cut tek undo entry olacak sekilde canonical command uzerinden calisir.
- Empty cut sonrasinda editable bos TextRun invariant'i korundu.
- Clipboard styled-fragment implementation, validation ve test matrix dokumanlari eklendi.

- Canonical primary section PageSettings Writer View ve Tauri DTO yuzeyine eklendi.
- Sabit CSS page geometry kaldirildi; width/height/margin twip -> CSS render conversion ile uretiliyor.
- %50-%200 session-only zoom, %10 step, statusbar kontrolu ve Ctrl/Cmd +/-/0 shortcutlari eklendi.
- `WriterLayoutService`, `FontCapabilityTool` ve merkezi font fallback profilleri eklendi.
- Font fallback canonical `fontFamily` degerini mutate etmeyecek sekilde ayrildi.
- Font substitution durumu statusbar read-modeline baglandi.
- ADR 0019, font/layout test matrisi ve implementation dokumani eklendi.

- Content fingerprint tabanli external-change detection eklendi.
- Cooperative sidecar file lock ve lock alinamadiginda read-only fallback eklendi.
- Read-only mutation guard Rust Service katmanina eklendi.
- Same-path Save external change durumunda explicit karar olmadan overwrite etmeyecek sekilde sertlestirildi.
- Reload From Disk / Keep Local Version / Save As protection banner akisi eklendi.
- 2 saniyelik frontend external-change poll ve statusbar lock/read-only durumu eklendi.
- Ctrl/Cmd+Shift+S Save As shortcut'i aktif edildi.
- ADR 0018, external-change test matrisi ve implementation dokumani eklendi.

- Autosave recovery TKO snapshot + YAML metadata akisi eklendi.
- Platform recovery root Tool ve per-document retention eklendi.
- Startup Recover / Compare / Discard recovery paneli eklendi.
- Recovery list/create/restore/compare/discard/clear Tauri IPC komutlari eklendi.
- 30 saniyelik dirty-revision autosave scheduler ve recovery revision deduplication eklendi.
- Explicit Save ve kullanici onayli discard akisi recovery cleanup ile baglandi.
- ADR 0017, recovery test matrisi ve implementation dokumani eklendi.

- Local `.tko` Open/Save akisi eklendi.
- TKO v1 `manifest.yml + content/writer.yml` ZIP+YAML serializer/deserializer eklendi.
- Canonical Writer domain serde bagimliligindan ayri Format DTO katmaninda tutuldu.
- ZIP Stored compression allowlist, traversal, directory, duplicate entry ve resource limit kontrolleri eklendi.
- Manifest/content id, revision ve schema mismatch validation eklendi.
- Save serializer current schema disindaki in-memory belgeyi reddedecek sekilde sertlestirildi.
- Desktop `WriterStorageService` ve `LocalFileTool` eklendi.
- Unix temp+sync+rename ve Windows backup+replace+rollback safe-save akisi eklendi.
- Relative save path icin current-directory parent normalization eklendi.
- Native Tauri Open/Save/Confirm dialog Tool eklendi.
- Dirty file baseline, discard guard ve Saved/Unsaved statusbar eklendi.
- Ctrl/Cmd+O, Ctrl/Cmd+S ve Ctrl/Cmd+N file commandlari aktif edildi.
- Tauri dialog capability minimum open/save/message izinleriyle guncellendi.
- ADR 0016, Local Open/Save implementation ve test matrisi eklendi.

- Writer ribbon baseline eklendi: Dosya/Giris/Ekle/Gorunum tab shell ve aktif Giris command bandi.
- CharacterStyle font family ve font size half-point alanlari typed patch zincirine eklendi.
- Caret typing-style Repository session state eklendi; canonical belge yalniz input sirasinda `InsertStyledText` ile mutate edilir.
- Paragraph left/center/right/justify alignment command, DTO ve renderer zinciri eklendi.
- Font family/size kontrolleri merkezi typography config'ine tasindi.
- Font select fokusundan once pending paragraph flush edilerek stale snapshot mutation riski azaltildi.
- Font/size select sonrasinda editor focus ve logical selection otomatik restore edildi.
- Ribbon tab/alignment metadata View icinden merkezi config ve Language katmanina tasindi.
- Typography/ribbon ADR, implementation dokumani ve test matrisi eklendi.

- IME-aware contenteditable paragraph surface eklendi.
- DOM selection ile Writer logical Unicode scalar offset mapper/restore Tool eklendi.
- CharacterStyle DTO, CharacterStylePatch ve ApplyCharacterStyle command zinciri eklendi.
- B/I/U toolbar ve Ctrl/Cmd+B/I/U shortcutlari aktif edildi.
- Paragraph input komple replace yerine style-preserving minimal Unicode diff kullanacak sekilde degistirildi.
- Rich-text ve IME ADR, implementation dokumani ve test matrisi eklendi.
- Paragraph/run View reconciliation stable `Index` omru ile caret/selection korumaya sertlestirildi.
- Tauri 2 Desktop shell crate eklendi.
- SolidJS + TypeScript + Vite UI baseline secildi ve ADR 0012 ile kaydedildi.
- Writer read-only paragraph/run DTO yuzeyi eklendi.
- Frontend Controller -> Service -> Repo -> Tool -> View -> Language katmanlari olusturuldu.
- Create, paragraph edit, split, merge, undo ve redo IPC koprusu eklendi.
- WriterEditorService icin tek history entry ureten batch command destegi eklendi.
- Minimum Tauri capability ve production/development CSP ayrimi eklendi.
- Frontend paragraph mutationlari serial queue ile siralandi.
- Pending paragraph blur/undo/redo commit davranisi eklendi.
- DOM UTF-16 offset -> Writer logical Unicode offset adaptoru ve regression testi eklendi.
- Desktop shell test matrix, implementation ve validation dokumani eklendi.


## 0.2.0 - Writer Domain

### Added

- `turkuaz-office-writer` Rust workspace modulu.
- Canonical WriterDocument, Section, Paragraph, TextRun, Table ve Image modeli.
- Stable NodeId ve logical selection modeli.
- InsertText, DeleteRange, SplitParagraph, MergeParagraph command seti.
- Character ve paragraph style commandlari.
- Image ve table insertion commandlari.
- Snapshot tabanli undo/redo ve monotonik revision semantigi.
- Writer repository, controller ve view DTO katmanlari.
- TKO v1 logical package profile.
- Writer domain integration testleri.
- ADR 0010 logical text offset ve ADR 0011 undo/redo baseline.
- Turkce ve Ingilizce Writer language YAML kaynaklari.

### Known Limits

- TKO ZIP/YAML byte archive encoding M1 Local Open/Save alt fazinda eklenmistir.
- Cross-section DeleteRange acik hata verir.
- Table hucre selection/edit commandlari siradaki editor fazina kalmistir.
- Desktop/Tauri UI bu milestone'un disindadir.


## 0.1.1 - Foundation Hardening

### Added

- Canonical native `.tko` format boundary.
- Document schema version type.
- SchemaMigrationService boundary.
- Font/layout, clipboard, print, localization, autosave/recovery docs.
- Untrusted document, file locking and distribution security docs.
- Offline-first sync contract.
- Plugin API capability contract.
- Accessibility and performance quality budgets.
- Licensing and third-party policy.
- ADR 0005-0009.

### Changed

- Roadmap M0 Foundation Hardening olarak tamamlandi.
- Definition of Done schema, security, accessibility ve performance kriterleriyle sertlestirildi.

## 0.1.0 - Foundation

- Monorepo iskeleti.
- Rust Core.
- Controller -> Service -> Repo/Tool -> View -> Language sinirlari.
- Temel Document Service ve InMemory Repository.
- Ilk mimari dokumantasyon ve CI foundation.
