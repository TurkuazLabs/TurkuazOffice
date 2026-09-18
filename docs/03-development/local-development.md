# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/docs/03-development/local-development.md
# 📌 Amac: Windows ve Linux local gelistirme akisinin temelini tanimlar
# 📌 Modul - FileType: Docs - Markdown
# Version: 0.2.0
# Aciklama: Windows ve Linux local gelistirme akisinin temelini tanimlar

Bagimli Oldugu Katman: Documentation

# Local Development

## Foundation gereksinimi

Rust stable toolchain yeterlidir. Foundation Core dis dependency kullanmaz.

## Kontrol

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Desktop milestone

Desktop Tauri kurulumu acildiginda Node LTS, pnpm ve platforma ozel Tauri prerequisite'leri eklenecektir. Bu dokuman o milestone ile birlikte exact komutlarla guncellenecektir.

## Linux

Linux build dependency listesi distro bazinda pinlenmeden once CI runner uzerinde dogrulanir.


## Desktop M1 prerequisites

### Ortak

- Rust 1.85.0 workspace baseline.
- Node.js 24 baseline CI runtime.
- Desktop frontend dependency kurulumu `apps/desktop` icinde yapilir.

### Windows

Tauri 2 icin Microsoft C++ Build Tools, Windows SDK ve WebView2 development/runtime gereksinimleri saglanmalidir. Windows build ilk olarak local `npm run tauri:dev` ile smoke test edilir.

### Debian/Ubuntu Linux

Tauri resmi prerequisite baseline'i:

```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

### Desktop frontend

```bash
cd apps/desktop
npm install --no-audit --no-fund
npm run build
npm run tauri:dev
```

M1 paketinde package lock henuz uretilmemistir; ilk network-enabled dependency resolve sonrasinda lockfile commit edilerek CI `npm ci` moduna gecirilecektir.
