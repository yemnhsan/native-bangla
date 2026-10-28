# Dictionary sources

Native Bangla ships only legally redistributable word lists.

| Dataset | Path | License | Notes |
|---------|------|---------|-------|
| Native Bangla core seed | `bn-core-seed.json` | MIT | Curated frequency list for development and tests. Not a complete lexicon. |
| Generated runtime index | `generated/` | MIT (derived) | Produced by `tools/dictionary-builder`. |

## Adding upstream dictionaries

1. Document the upstream name, URL, and license here.
2. Place the raw import under `resources/dictionaries/upstream/` (git-ignored if large).
3. Run `cargo run -p dictionary-builder -- --input ... --output resources/dictionaries/generated/bn-core.v1.json`.
4. Do not invent frequency statistics. If frequencies are unavailable, use a neutral default and document it.

## Frequency scale

Seed frequencies are relative integers for ranking only. They are not corpus percentages.
