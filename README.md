# Native Bangla

**Type Naturally. Write Beautifully.**

Native Bangla is a free, offline-first, intelligent Bengali keyboard and Windows IME. It combines Avro-style phonetic typing and traditional fixed layouts with fast word prediction, modern design, and a privacy-first architecture.

## Product

| | |
|---|---|
| Product | Native Bangla |
| Short name | Native |
| Input engine | Native Engine |
| Prediction | Native Predict |
| Windows app | Native Bangla |

## Architecture

Hybrid native stack:

- **Native Engine** — C++20 Text Services Framework (TSF) IME for system-wide typing
- **Native Predict** — Rust phonetic, dictionary, and prediction crates
- **Desktop** — Tauri 2 + React settings, onboarding, tray, and dictionary management

The IME process never depends on the settings UI being open. Core typing works fully offline.

```
apps/desktop          Tauri + React settings application
native/windows-ime    C++ TSF text service
crates/               Rust linguistic engines + FFI bridge
packages/             Shared TypeScript UI, config, types
resources/            Dictionaries, fonts, layouts, icons
tools/                Dictionary builder, benchmarks, packaging
tests/                Cross-cutting integration & compatibility
docs/                 Architecture, development, security
```

## Quick start

### Prerequisites

- Windows 10/11 x64
- [Rust](https://rustup.rs/) (stable)
- Node.js 20+
- Visual Studio 2022 with Desktop C++ / Windows SDK (for the IME DLL)
- [WiX Toolset](https://wixtoolset.org/) (for installer packaging)

### Build Rust engines

```powershell
cargo build --workspace --release
cargo test --workspace
cargo bench -p native-predict --bench prediction_bench
```

### Build desktop app

```powershell
cd apps/desktop
npm install
npm run tauri dev
```

### Build Windows IME

```powershell
cmake -S native/windows-ime -B build/ime -A x64
cmake --build build/ime --config Release
```

See [docs/development/getting-started.md](docs/development/getting-started.md) for full setup, signing, and registration.

## Status

See [docs/status.md](docs/status.md) for phase completion and known platform constraints.

## License

MIT — see [LICENSE](LICENSE). Dictionary datasets retain their upstream licenses; see [resources/dictionaries/SOURCES.md](resources/dictionaries/SOURCES.md).

## Privacy

Native Bangla does not upload typed text. Prediction and personalization stay on-device. Password and secure fields suspend learning and suggestions. Details: [docs/security/privacy.md](docs/security/privacy.md).
