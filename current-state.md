# Current State

Last updated: 2026-04-26

この文書は、現在の実装状態と検証の事実だけを記録する。設計は `docs/` 側に置き、ここでは「今何が動くか」「何が未実装か」「何を確認すればよいか」を扱う。

## Current gate（このリポジトリが今要求する最小バー）

正本は `docs/11-shared-definitions.md` の Workstreams / Gates。実装・レビューでまず満たすのは次の組み合わせとする。

- **Gate A（テスト）**: `cargo fmt --all --check` と `cargo nextest run`（フル suite。重いテストを分離する場合は `docs/11` の filterset 方針に従う）。
- **Gate D（coverage artifact）**: `scripts/gen/coverage-matrix.sh --check` が `artifacts/coverage/reference-coverage-matrix.md` を検証。
- **その他（B–C, E–G）**: ポリシーと checklist は `docs/11` / `docs/12-coding-standard.md`（§19）に記載。証拠コマンドは下記「Last verified commands」。

## Last verified commands（代表）

開発者がローカルで再現する際の最小セット（CI と揃える場合はワークフローを参照）。

```bash
cargo fmt --all --check
cargo nextest run
mise run check-coverage-matrix
mise run check-shell-syntax
mise run check-fast-gate --skip-nextest
mise run check-manifest-imports
mise run check-fixture-catalog
mise run check-architecture-rules
mise run check-compiler-diagnostics
mise run update-issue-index --check
mise run check-issue-index
```

reference coverage を更新する場合（実測値を変えるとき）:

```bash
mise run update-coverage-matrix
# または単 suite: mise run reference-coverage -- test262 --limit 50
```

## Snapshot

- Compiler/runtime は一部が実装済み。
- 最小 subset の TS/JS を WASI `.wasm` に変換し、`iwasm` 実行が可能。
- semantic-core の curated fixture は Node differential で一致。
- data-model の curated fixture（array/object basic）は Node differential で一致。

## Fixture groups（curated / 回帰の目安）

| Group | Path prefix | 件数（目安） | 検証の種類 |
|------|-------------|-------------|------------|
| basics | `fixtures/basics-*` | 複数 | build + 必要に応じ differential |
| primitives / control flow | `fixtures/primitives-control-flow/` | 複数 | build / differential |
| core semantics | `fixtures/core-semantics/` | 複数 | Node differential（semantic-core） |
| arrays / objects | `fixtures/arrays-objects/` | 複数 | Node differential（data-model） |

正確なファイル数は `find fixtures/<dir> -type f | wc -l` で取得する。AGENTS の「19 fixtures」などの圧縮表記がある場合は、この表を優先する。

## Reference coverage（測定の正本）

- 生成テーブル: `artifacts/coverage/reference-coverage-matrix.md`
- ポリシーと列定義: `docs/15-coverage-matrix.md`
- 列 `build_pass` / `semantic_pass` は `scripts/run/reference-coverage.sh` の出力に対応（semantic-pass は Node + `iwasm` が利用可能な環境でのみ増分）。

## Implemented (high-level)

- minimal parser/frontend（`crates/frontend` - 実装済み）
- WAT/WASM emitter と runtime subset（`crates/cli` - `crates/backend-wasm` へ移行中）
- shared schema crate（`crates/shared`）: ABI/capability/test status
- IR crate（`crates/ir` - 実装済み）: resolved/lowered IR
- runtime-abi crate（`crates/runtime-abi` - 実装済み）: RawValue/layout/ABI
- reference coverage パイプライン（`mise run reference-coverage`, `mise run update-coverage-matrix`, `mise run check-coverage`）
- generated coverage table（`artifacts/coverage/reference-coverage-matrix.md`）
- issue queue index（`issues/index.md` の Ready/Blocked/Done 表は `mise run update-issue-index` が生成、`mise run check-issue-index` で整合検証）
- harness scripts（`mise run check-fast-gate`、`mise run check-manifest-imports`、`mise run check-test-records-schema`、`mise run check-fixture-catalog`、`mise run check-architecture-rules`、`mise run check-compiler-diagnostics`；pre-push は `.githooks/pre-push`）

## Known blockers / gaps

- **Crates module migration**: `crates/runtime-abi`, `crates/ir`, `crates/frontend` は実装済み。`crates/backend-wasm` への移行が進行中（issue 026）。
- TypeScript parser/checker integration は未実装
- 汎用 JavaScript semantic IR は未実装
- full wasm backend は未実装（現状は WAT 中心）
- test262 full differential 運用は未完（sample/ramp が中心）
- OOM/GC/UTF-8 完全対応は未完
- host-deny / capability manifest の「監査可能な」E2E は `docs/06` の required test classes に沿って拡張予定

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

Based on current open issues and workstream progress, the next priority slices are:

1. **P0 - Capability Manifest**: issues/open/002-emit-canonical-capability-manifest-schema.md
   - Emit capability manifest as JSON output
   - Validate manifest against emitted WAT imports

2. **P0 - Computed Property Semantics**: issues/open/012-fix-computed-property-semantics-bug.md
   - Fix computed property access to work on all objects
   - Add differential test for computed property

3. **P0 - Heap OOM Check**: issues/open/013-implement-heap-oom-check.md
   - Runtime safety critical
   - Memory safety related

4. **P0 - Reclassify Compile-only Tests**: issues/open/004-reclassify-compile-only-compatibility-tests.md
   - Improve coverage measurement accuracy
   - Prerequisite for Gate D

5. **P1 - Frontend Module Extraction**: issues/open/010-extract-frontend-module-from-crates-cli.md
   - Extract frontend module from crates/cli
   - Establish clear frontend/semantic/backend boundaries

6. **P1 - TypeScript Parser Integration**: issues/open/019-integrate-typescript-parser-checker.md
   - Integrate TypeScript compiler API
   - Extract type information for optimization

7. **P1 - IR Validation Passes**: issues/open/020c-add-ir-validation-passes-and-document-contracts.md
   - Add IR validation passes
   - Document IR contracts

See issues/index.md for complete issue queue and status.

## Current Policy

- `docs/` は ADR/設計判断の保存先として扱う。
- 実装の現在地と検証手順の要約はこの `current-state.md` を正とする。
- coverage 実測値は `artifacts/coverage/reference-coverage-matrix.md` を正とする。
- project goal、gates、schema は `docs/11-shared-definitions.md` を正とし、他 doc で再定義しない。
