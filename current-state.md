# Current State

Last updated: 2026-04-28

この文書は、現在の実装状態と検証の事実だけを記録する。設計は `docs/` 側に置き、ここでは「今何が動くか」「何が未実装か」「何を確認すればよいか」を扱う。

## Current gate（このリポジトリが今要求する最小バー）

正本は `docs/11-shared-definitions.md` の Workstreams / Gates。実装・レビューでまず満たすのは次の組み合わせとする。

- **Gate A（テスト）**: `cargo fmt --all --check` と `cargo nextest run`（フル suite。重いテストを分離する場合は `docs/11` の filterset 方針に従う）。
- **Gate D（coverage artifact）**: `scripts/manager update-coverage-matrix --check` が `artifacts/coverage/reference-coverage-matrix.md` を検証。
- **その他（B–C, E–G）**: ポリシーと checklist は `docs/11` / `docs/12-coding-standard.md`（§19）に記載。証拠コマンドは下記「Last verified commands」。

## Last verified commands（代表）

開発者がローカルで再現する際の最小セット（CI と揃える場合はワークフローを参照）。

```bash
cargo fmt --all --check
cargo nextest run
scripts/manager update-coverage-matrix --check
scripts/manager check-scripts
scripts/manager check-fast-gate --skip-nextest
scripts/manager check-manifest-imports
scripts/manager check-fixture-catalog
scripts/manager check-architecture-rules
scripts/manager check-compiler-diagnostics
scripts/manager update-issue-index --check
scripts/manager check-issue-health
```

reference coverage を更新する場合（実測値を変えるとき）:

```bash
scripts/manager update-coverage-matrix
# または単 suite: scripts/manager reference-coverage test262 --limit 50
```

## Snapshot

- Compiler/runtime は一部が実装済み。
- 最小 subset の TS/JS を WASI `.wasm` に変換し、`iwasm` 実行が可能。
- semantic-core の curated fixture は Node differential で一致。
- data-model の curated fixture（array/object basic）は Node differential で一致。
- Module cache test now passes (require_cache_reuses_same_object_at_runtime_semantic_diff).
- class / module / node-api fixture は build 成功の確認は通過しているが、semantic parity は `m2_node_diff.rs` 側で未確定として明示している。

## Fixture groups（curated / 回帰の目安）

| Group | Path prefix | 件数（目安） | 検証の種類 |
|------|-------------|-------------|------------|
| basics | `fixtures/basics-*` | 複数 | build + 必要に応じ differential |
| primitives / control flow | `fixtures/primitives-control-flow/` | 複数 | build / differential |
| core semantics | `fixtures/core-semantics/` | 複数 | Node differential（semantic-core） |
| arrays / objects | `fixtures/arrays-objects/` | 複数 | Node differential（data-model） |

正確なファイル数は `find fixtures/<dir> -type f | wc -l` で取得する。AGENTS の「19 fixtures」などの圧縮表記がある場合は、この表を優先する。

## Test classification

Tests are classified into three categories to distinguish build success from semantic compatibility:

- **build_smoke**: Tests that compilation succeeds (syntax parsing, name resolution, lowering to WASM). These do NOT verify runtime semantics.
- **semantic_diff**: Tests that Node.js and iwasm execution produce identical output (differential testing).
- **parser_smoke**: Tests that syntax can be parsed (not yet implemented).

**Build pass does NOT imply semantic compatibility.**

Test files follow naming conventions:
- `*_build_smoke()`: Build smoke tests (m7_control_flow.rs, m8_oop_classes.rs, m9_modules.rs, m10_node_apis.rs)
- `*_semantic_diff()`: Differential tests (m2_node_diff.rs)
- `m2_node_diff.rs`: Semantic differential tests for core fixtures

Compile-only tests for class/module/Node API are explicitly marked as build_smoke to avoid implying semantic support.

## Reference coverage（測定の正本）

- 生成テーブル: `artifacts/coverage/reference-coverage-matrix.md`
- ポリシーと列定義: `docs/15-coverage-matrix.md`
- 列 `build_pass` / `semantic_pass` は `scripts/manager reference-coverage` の出力に対応（semantic-pass は Node + `iwasm` が利用可能な環境でのみ増分）。

## Implemented (high-level)

- frontend crate（`crates/frontend`）: AST/span/diagnostic/token definitions plus lexer/parser implementation
- compiler/driver crate（`crates/compiler`）: build pipeline, dump pipeline, AST validation, lowering orchestration
- WAT/WASM emitter と runtime subset（`crates/backend-wasm`）
- shared schema crate（`crates/shared`）: ABI/capability/test status
- IR crate（`crates/ir`）: resolved/lowered IR
- CLI dump command: `ts2wasm dump` can emit tokens, AST, resolved AST, lowered IR, WAT, and AST pseudo-source via `--ast --unparse`
- runtime-abi crate（`crates/runtime-abi`）: RawValue/layout/ABI
- reference coverage パイプライン（`scripts/manager reference-coverage`, `scripts/manager update-coverage-matrix`, `scripts/manager update-coverage-matrix --check`）
- generated coverage table（`artifacts/coverage/reference-coverage-matrix.md`）
- issue queue index（`issues/index.md` の Ready/Blocked/Done 表は `scripts/manager update-issue-index` が生成、`scripts/manager check-issue-health` で整合検証。`scripts/manager check-issue-index` は互換 alias）
- harness scripts（`mise run check-fast-gate`、`mise run check-manifest-imports`、`mise run check-test-records-schema`、`mise run check-fixture-catalog`、`mise run check-architecture-rules`、`mise run check-compiler-diagnostics`；pre-push は `.githooks/pre-push`）

## Known blockers / gaps

- **Crates module migration**: `crates/frontend`, `crates/ir`, `crates/runtime-abi`, and `crates/backend-wasm` code migrations are complete. Issue 026 is closed with full workspace nextest evidence.
- **CLI ownership**: `crates/cli` is now a thin binary/re-export wrapper for compiler APIs. Lexer/parser implementation has moved to `crates/frontend`; backend runtime emission is split into focused modules and no repo-owned source file currently exceeds the 2000-line architecture warning threshold.
- TypeScript parser/checker integration は未実装
- 汎用 JavaScript semantic IR は未実装
- typed IR dump (`ts2wasm dump --tir`) and optimizer dump (`ts2wasm dump --optimize`) are tracked by issues 204 and 205.
- full wasm backend は未実装（現状は WAT 中心）
- test262 full differential 運用は未完（sample/ramp が中心）
- GC 実装は未完。OOM check と UTF-8 literal basic support は完了済みだが、UTF-16 parity / encode-decode helper は追跡対象。
- host-deny / capability manifest の base path は実装済み。`docs/06` の required test classes に沿った監査範囲の拡張は継続対象。

Semantic gap tracking:

- class 系: `crates/cli/tests/m8_oop_classes.rs`（build_smoke）。semantic status は `crates/cli/tests/m2_node_diff.rs` の class gap アサーションで管理。
- module 系: `crates/cli/tests/m9_modules.rs`（build_smoke）。semantic status は `crates/cli/tests/m2_node_diff.rs` の module gap アサーションで管理。
- node API 系: `crates/cli/tests/m10_node_apis.rs`（build_smoke）。semantic status は `crates/cli/tests/m2_node_diff.rs` の node_api gap アサーションで管理。
- Partial feature semantics from historical done issues are tracked by dedicated open issues, not by the done queue: `instanceof` prototype-chain semantics (207), switch fall-through (208), labeled break/continue (209), arrow closure and lexical `this` (210), `this` receiver binding (211), rest parameter argument collection (212), template literal interpolation (213), string method placeholders (214), and `Math.random` capability/randomness policy (215). Abstract equality now has primitive coercion coverage for nullish, boolean, number, and string values; object `ToPrimitive` remains tied to object-model follow-up work.

## Risk Management

既知のリスクと対応計画。

| リスク | 緊急度 | 影響 | 対応計画 | 状態 |
|---|---|---|---|---|
| TypeScript parser integration の複雑性 | 中 | 高 | 既存 parser を oracle として活用しつつ、段階的に置換 | 監視中 |
| GC 実装の複雑さ | 中 | 高 | 初期は arena + 明示 lifetime、段階的に mark-and-sweep | 計画中 |
| WASM 提案の進化による ABI 変更 | 低 | 中 | 論理 ABI を固定し、backend で表現を差し替える設計 | 設計済み |
| test262 カバレッジの達成困難 | 中 | 高 | 機能レベルの breakdown (issue 005) を優先 | 実行中 |
| Node host import の増大 | 中 | 高 | capability manifest と host-deny test で監査 | 実装中 |
| Reference repository の依存 | 低 | 中 | 参照 repository のハッシュ固定、local cache 検討 | 検討中 |

**リスク評価基準**:
- **緊急度**: 高 (即時対応)、中 (次回更新時)、低 (将来検討)
- **影響**: 高 (プロジェクト成功に致命的)、中 (主要機能に影響)、低 (軽微)

## Next Priority Steps

See `issues/index.md` for the auto-generated Ready queue and Blocked queue.
Run `scripts/manager update-issue-index` to refresh after adding, closing, or moving issues. The generated Ready queue in `issues/index.md` is the source of truth for current ordering.

## Current Policy

- `docs/` は ADR/設計判断の保存先として扱う。
- 実装の現在地と検証手順の要約はこの `current-state.md` を正とする。
- coverage 実測値は `artifacts/coverage/reference-coverage-matrix.md` を正とする。
- project goal、gates、schema は `docs/11-shared-definitions.md` を正とし、他 doc で再定義しない。
