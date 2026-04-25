# Current State

Last updated: 2026-04-25

この文書は、現在の実装状態だけを短く記録する。設計は `docs/` 側に置き、ここでは「今何が動くか」「何が未実装か」「何を確認すればよいか」だけを扱う。

## Snapshot

- Compiler/runtime は 一部が実装済み。
- 最小 subset の TS/JS を WASI `.wasm` に変換し、`iwasm` 実行が可能。
- semantic-core の curated fixture は Node differential で一致。
- data-model の curated fixture（array/object basic）は Node differential で一致。

## Stable Checks

```bash
cargo fmt --all --check
cargo nextest run
scripts/update_coverage_matrix.sh --check
scripts/check_scripts.sh
```

## Implemented (high-level)

- minimal parser/frontend (`crates/cli`)
- WAT/WASM emitter と runtime subset (`crates/cli`)
- shared schema crate (`crates/shared`): ABI/capability/test status
- reference coverage gate (`scripts/reference_coverage.sh`, `scripts/update_coverage_matrix.sh`)
- generated coverage table (`artifacts/coverage/reference-coverage-matrix.md`)

## Known Gaps

- TypeScript parser/checker integration は未実装
- 汎用 JavaScript semantic IR は未実装
- full wasm backend は未実装（現状は WAT 中心）
- test262 full differential 運用は未完（sample/ramp が中心）
- OOM/GC/UTF-8 完全対応は未完

## Current Policy

- `docs/` は ADR/設計判断の保存先として扱う。
- 実装の現在地はこの `current-state.md` を正とする。
- coverage 実測値は `artifacts/coverage/reference-coverage-matrix.md` を正とする。
