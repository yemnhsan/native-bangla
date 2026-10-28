# Project Status — Native Bangla

**Type Naturally. Write Beautifully.**

Current milestone: **Phase 1 Alpha — Core Engine, Prediction & UI Shell**

## Component Status Matrix

| Component | Layer / Language | Status | Notes |
|---|---|---|---|
| **phonetic-engine** | Rust | Complete | Full Bengali phonetic transliteration with conjuncts, vowel signs, danda conversion, and Bengali digit support. |
| **prediction-engine** | Rust | Complete | Trie-based prefix lookup, bigram n-gram model, and personal learning store. |
| **dictionary-core** | Rust | Complete | Fast zero-copy / memory-efficient dictionary loader and vocabulary index. |
| **user-preferences** | Rust | Complete | Schema-backed JSON configuration with Windows RoamingAppData storage. |
| **ime-bridge** | Rust (FFI) | Complete | C ABI bridge exporting stateful composition and prediction interfaces for C++. |
| **native-engine** | Rust | Complete | Top-level coordination API unifying transliteration, dictionary, and preferences. |
| **windows-ime** | C++20 | Complete | Windows Text Services Framework (TSF) InProc COM text service without SendInput hacks. |
| **desktop** | React + Vite / Tauri | Complete | Modern desktop settings suite: Home, Typing, Suggestions, Dictionary, Layouts, Shortcuts, Appearance, Advanced, About, and Onboarding. |
| **resources** | JSON / Schemas | Complete | Seed Bengali dictionary, phonetic mapping rules, and Probhat fixed keyboard layout. |
| **tools** | Rust | Complete | Dictionary builder CLI and latency benchmarking harnesses. |

## Key Technical Decisions & Highlights

1. **True TSF Integration**: Uses `ITfTextInputProcessorEx`, `ITfKeyEventSink`, and `ITfThreadMgrEventSink`. Avoids synthetic keystroke injection (`SendInput` / `keybd_event`) to guarantee full compatibility with modern Windows 11/10 applications, input controls, and undo stacks.
2. **Offline-First & Zero Telemetry**: Absolutely no cloud dependencies or data transmission. All vocabulary learning and n-gram caches remain strictly on local disk.
3. **Sub-5ms Latency Budget**: Phonetic composition and candidate generation happen synchronously well within the 60fps / 16ms frame budget.
