# Getting Started with Native Bangla Development

**Native Bangla — Type Naturally. Write Beautifully.**

This guide covers building, testing, and developing Native Bangla on Windows 10/11.

## Prerequisites

- **Windows 10 / 11 64-bit** (Build 19041 or newer)
- **Rust Toolchain** (1.75+ stable): [rustup.rs](https://rustup.rs/)
- **Node.js** (v20+ LTS) and npm
- **C++ Compiler & CMake** (3.20+): Visual Studio 2022 Build Tools (with Desktop C++ & Windows 10/11 SDK) or LLVM-MinGW
- **Git**

## Repository Structure

```
native-bangla/
├── apps/
│   └── desktop/            # Tauri / React settings & onboarding app
├── crates/
│   ├── phonetic-engine/    # Transliteration rules and session parser
│   ├── prediction-engine/  # Trie lookup, bigram model, personalization
│   ├── dictionary-core/    # Zero-copy lexicon loading & indexing
│   ├── user-preferences/   # Typed settings storage
│   ├── ime-bridge/         # C ABI FFI bridge for C++ TSF
│   └── native-engine/      # High-level orchestrator
├── native/
│   └── windows-ime/        # C++20 Windows Text Services Framework IME DLL
├── resources/
│   ├── dictionaries/       # Seed Bengali lexicon and corpora info
│   └── layouts/            # Phonetic and fixed keyboard layout definitions
├── tools/
│   ├── dictionary-builder/ # Utility to compile raw wordlists into optimized dictionaries
│   └── benchmarking/       # Latency benchmark suite
└── docs/                   # Architecture, privacy, status, and guides
```

## 1. Building the Rust Crates

To check and compile all Rust workspace crates:

```powershell
# Build workspace
cargo build --workspace

# Run test suite
cargo test --workspace

# Run benchmarks
cargo bench -p phonetic-engine
cargo bench -p prediction-engine
```

## 2. Running the Desktop Settings UI

The desktop application is located in `apps/desktop`:

```powershell
cd apps/desktop

# Install frontend dependencies
npm install

# Start Vite live development server
npm run dev

# Run unit tests
npm test

# Build production bundle
npm run build
```

## 3. Building the Windows IME DLL

The Windows IME text service is located in `native/windows-ime`:

```powershell
# Configure CMake
cmake -S native/windows-ime -B build/ime -A x64

# Build Release DLL
cmake --build build/ime --config Release
```

The resulting `NativeBanglaIME.dll` can be registered into Windows TSF using an elevated PowerShell terminal:

```powershell
# Register the IME COM server and language profile
regsvr32 build/ime/Release/NativeBanglaIME.dll

# To unregister:
regsvr32 /u build/ime/Release/NativeBanglaIME.dll
```

Once registered, Native Bangla appears in the Windows Language Bar / Input Switcher (`Win + Space`).
