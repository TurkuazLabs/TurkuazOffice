# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/07-quality/suite-code-reuse-audit.md
# 📌 Amac: Turkuaz Office mevcut ortak altyapi, tekrar ve olasi olu kod risklerini somut kanit ve kalite aksiyonlariyla izler
# 📌 Modul - FileType: Quality - Markdown
# Version: 0.1.0
# Aciklama: Kod tabanindaki tekrarlari, dogrulanmamis silme adaylarini, uygulanacak testleri ve devreye alma kriterlerini belirler

Bagimli Oldugu Katman: Controller | Service | Repo | Tool | View | Language | Config

# Suite Shared-Code Audit - 2026-10-09

Referans baseline: TurkuazLabs/TurkuazOffice `main` 46e5aa70f0c9fe5363e63118b53b3cc73801e2bc.
Kapsam: GitHub uzerinden secili source, config, docs ve PR metadata incelemesi.
Kisit: Tam repository dependency graph, canli Windows/Linux startup profili ve Knip/cargo-machete taramasi bu denetimde calistirilmadi.
Karar: Buradaki riskler, otomatik olarak "silinebilir olu kod" sonucunu vermez.

## Gozlemler

| Kod / kontrat | Kanit | Risk turu | Karar |
| --- | --- | --- | --- |
| `apps/desktop/src/config/keyboard.ts` | `WRITER_SHORTCUT_ACTIONS`, `WRITER_ARIA_SHORTCUTS`, `WRITER_SHORTCUT_HINTS`, Sheet esdegerleri ayri tablolar | Metadata tekrar/drift riski | Etkin komutlar icin tek typed descriptor kaynagi |
| `apps/desktop/src/services/keyboard-shortcut.service.ts` ve `sheet-keyboard-shortcut.service.ts` | Iki resolver ortak modifier/kucuk harf normalizasyonu kullaniyor | Sinirli davranis tekrari | Yalniz normalization ortak; command policy modul bazli |
| `apps/desktop/src/views/writer-shell.tsx` ve `sheet-shell.tsx` | Ikisi de `window.addEventListener("keydown")` kurup temizliyor | Olay baglama tekrari | Once parity testi, sonra gercek gereksinim varsa ortak adapter |
| `apps/desktop/src/views/suite-titlebar.tsx` ve `suite-icon.tsx` | Writer/Sheet icin mevcut ortak baslik ve ikon View'u var | Mevcut dogru reuse | Ikinci shared header/icon kutuphanesi ekleme |
| `apps/desktop/src/config/ipc-commands.ts` | Tauri IPC adlari tek merkezi tabloya alinmis | Mevcut dogru reuse | Ayrica ikinci IPC registry olusturma |
| `apps/desktop/src/config/app-container.ts` | Iki modulun repository, service ve Tool instance'lari tek yerde olusturuluyor | Startup/memory aday maliyeti | Gercek sure/memory olcmeden lazy composition yok |
| `apps/desktop/src/services/writer-session.service.ts` | 1321 satir ve file/edit/print/selection/IME akislari | Sorumluluk yogunlasmasi | Odakli Service'lere sadece testli ve gercek ihtiyac varsa bol |
| `apps/desktop/src/services/sheet-session.service.ts` | 978 satir ve cell/table/query/chart/format akislari | Sorumluluk yogunlasmasi | Siklikla degisen akislar icin parcali refactor oncesi test |
| `apps/desktop/src/views/sheet-functions-sidebar.tsx` | Local `search` state yalniz function catalog'u filtreliyor | Yanlis soyutlama riski | Belge Find motoru olarak yeniden kullanma |
| `apps/desktop/tsconfig.json` ve `apps/web/tsconfig.json` | `strict` var; `noUnusedLocals` ve `noUnusedParameters` yok | Olasi erisilemeyen yerel kod | Ilk compiler raporundan sonra staged gate |
| `Cargo.toml` | Workspace Rust lints ve shared crates var | Bagimlilik/feature birikimi olasigi | Mevcut Cargo lint + onayli periodic audit |
| `docs/05-roadmap/roadmap.md`, `config/project.yml`, `apps/desktop/src/main.tsx`, `docs/08-implementation/desktop-suite-entrypoints-v0.5.0.md`, PR #33 | Runtime in-window, roadmap ve config in-window diyor; mevcut desktop suite entrypoints dokumani ve PR #33 ayri process hedefliyor | Mimari karar ve dokumantasyon uyumsuzlugu | PR #33 oncesinde tek hedef sec, gercek runtime'a gore dokumani duzelt |
| PR #32 | Gercek WASM Chromium smoke eklemeyi hedefliyor; acik | Tamamlanmamis kalite kapisi | Bugun zaten varmis gibi kabul etme |
| PR #57 | AltGr korumasi acik PR'da; main'de degil | Kopya bugfix riski | Ayni korumayi ayri ikinci implementation olarak ekleme |

## Bu turda kanitlanamayanlar

- Kesin unused source/exports/dependencies sayisi.
- Dinamik Tauri command invocation, WASM generated artifact, conditional feature ve test fixture kullanimlari dahil tum entrypoint erisimi.
- Tekrar eden kodun runtime bundle olcekli maliyeti.
- Bir Service'i parcalamanin performans/net kod azaltma kazanimi.
- Native Search/Replace motorunun halihazirda var oldugu: incelenen Writer/Sheet Desktop Service + keyboard kontratinda bir belge Find/Replace komutu tespit edilmedi; bu tum git gecmisinin tarandigi anlamina gelmez.

## Aksiyon oncelikleri

### P0 - Yeni shortcut ozelligi oncesi
1. Mevcut kisa yol descriptor/action/hint/aria tablosunu cikar ve 1:1 bagla.
2. PR #57 ile ortak modifier guard'in gelecegini kontrol et; ayni AltGr logic'ini bir kez daha yazma.
3. Menu/toolbar/key event'in gercek handler'a, handler'in Service'e ve gerekiyorsa Core mutation'a kadar ulasmasini testle.
4. PR #33 icin "tek pencere mi ayri process mi" kontratini onceden sec; roadmap guncellenmeden birlestirme.

### P1 - Olasi olu kodu dogrula
1. Her TS entrypoint'i belirle: Desktop main, Web main, test, config, generated WASM/IPC ve lazy import.
2. Desktop/Web TS compiler'a `noUnusedLocals` ve `noUnusedParameters` acildiginda kac ve hangi hata ciktigini raporla; mevcut CI'yi bozma.
3. Knip icin Desktop ve Web entrypoint, Tauri IPC string bridge ve generated modul/config girislerini acikca tanimla. Ilk calismasi rapor odakli ve non-blocking olsun.
4. Rust icin mevcut Cargo dependency lint sonuclarini kaydet. cargo-machete karsilastirmasini opsiyonel/periodic yap; proc-macro/build-script/feature gated false-positive adaylarini teyit et.
5. Silebilecegin her dosya/export icin "production imports 0 + dynamic entrypoints 0 + ABI/API consumer 0 + tests/CI green" kaniti bulunsun.
6. Public API'yi kullanan Pro/harici kullanicilar icin sadece internal reachability ile silme karari verme.

### P2 - Calisan Find/Replace
1. `Ctrl+F` ve gercek Find UI ayni PR'da, Worker/Service/Core arama semantics ve status/focus regression ile gelir.
2. Writer Unicode scalar/paragraph/mixed style offset'lerini korur; Sheet sparse cell/formula/visible value semantics ayri belirlenir.
3. `Ctrl+H` replacement mutation, undo/redo, dirty revision, file protection, read-only, IME ve type check'ler olmadan acilmaz.
4. Yeni command registry, sadece gercek iki production handler ortak oldugunda shared olur.

## Ilk uygulanan dilim - Strict TypeScript + keyboard metadata parity

- `apps/desktop/src/config/keyboard-contract.test.ts` Writer/Sheet komut kimligi benzersizligini, kisa yol gorunen yazilarini ve ARIA eslemelerini Vitest ile dogrular.
- 2026-10-09 CI advisory ilk olcum: Web 0; Desktop 1 TS6133. Tek bulgu `apps/desktop/src/services/sheet-session.service.test.ts` dosyasindaki kullanilmayan chart mock callback parametresidir; test davranisi degismeden parametre kaldirildi.
- Ayni CI'nin ikinci olcumu: Web 0 ve Desktop 0. Bu sayilar yalniz TS6133/TS6192/TS6196 sembol diagnostikleridir; unused exports/dependencies taramasi degildir.
- `apps/desktop/tsconfig.json` ve `apps/web/tsconfig.json` icin `noUnusedLocals` ve `noUnusedParameters` artik `true`. Mevcut `npm run build` quality gate bu hatalarda dogrudan fail verir.
- Gecici advisory script ve CI adimlari, strict gate'de gereksiz tekrar olusturacagi icin ayni PR icinde kaldirildi. Final PR'da yeni npm paketi veya ayri arka plan tarayici yoktur.
- Knip, Rust Cargo dependency ve dinamik Tauri/WASM/Pro referanslari henuz tam taranmadigi icin, bu alanlarin silinmesi onaylanmamistir.

## Denetim raporu kontrol listesi

- [ ] `entrypoint-graph` olusturuldu ve dinamik/generator/Tauri/WASM referanslari manuel teyit edildi.
- [ ] Var olan komutlarin UI action -> Controller -> Service -> Core/Tool path haritasi yazildi.
- [ ] Shortcut/hint/aria cakismalari icin unit test ve TR klavye regression var.
- [ ] `noUnusedLocals`, `noUnusedParameters` hatalari raporlandi.
- [ ] Knip raporu false-positive siniflarina ayrildi.
- [ ] Rust Cargo unused-dependency raporu ve macro/feature istisnalari kaydedildi.
- [ ] Kullanilmayan her kod silme PR'si runtime ve CI ile ayri onaylandi.
- [ ] PR #33 roadmap uyumu netlestirildi.
- [ ] Find/Replace baska yerde yalniz kisa yol olarak degil tam calisan feature olarak test edildi.

## Referanslar

- Architecture decision: `docs/06-adr/0021-suite-command-reuse-boundary.md`
- Roadmap: `docs/05-roadmap/roadmap.md`
- TypeScript compiler: https://www.typescriptlang.org/tsconfig/noUnusedLocals.html
- Knip production analysis: https://knip.dev/typescript/unused-dependencies
- Knip issue validation: https://knip.dev/guides/handling-issues
- Cargo dependency lints: https://doc.rust-lang.org/stable/cargo/reference/lints.html
- cargo-machete limitations: https://github.com/bnjbvr/cargo-machete
