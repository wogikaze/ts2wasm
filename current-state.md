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
scripts/gen/coverage-matrix.sh --check
scripts/check/shell-syntax.sh
scripts/check_fast_gate.sh --skip-nextest
scripts/check_manifest_imports.sh
scripts/check_fixture_catalog.sh
scripts/check_architecture_rules.sh
scripts/check_compiler_diagnostics.sh
scripts/update_issue_index.sh --check
scripts/check_issue_index.sh
```

reference coverage を更新する場合（実測値を変えるとき）:

```bash
scripts/gen/coverage-matrix.sh
# または単 suite: scripts/run/reference-coverage.sh test262 --limit 50
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

- minimal parser/frontend（`crates/cli`）
- WAT/WASM emitter と runtime subset（`crates/cli`）
- shared schema crate（`crates/shared`）: ABI/capability/test status
- reference coverage パイプライン（`scripts/run/reference-coverage.sh`, `scripts/gen/coverage-matrix.sh`, `scripts/gate/coverage.sh`）
- generated coverage table（`artifacts/coverage/reference-coverage-matrix.md`）
- issue queue index（`issues/index.md` の Ready/Blocked/Done 表は `scripts/update_issue_index.sh` が生成、`scripts/check_issue_index.sh` で整合検証）
- harness scripts（`scripts/check_fast_gate.sh`、`check_manifest_imports.sh`、`check_test_records_schema.sh`、`check_fixture_catalog.sh`、`check_architecture_rules.sh`、`check_compiler_diagnostics.sh`；pre-push は `.githooks/pre-push`）

## Known blockers / gaps

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

## Next legal slice（実装単位の候補）

次に取り込みやすい縦スライスは、`docs/11` の workstream 順と open issue を優先する。具体的な ticket は `issues/` を参照。ここでは「次の一行」だけ固定しない（更新コストを避けるため）。

## Next Priority Slice（優先度順の具体的な次ステップ）

AI エージェントや自律開発ループでのタスク選択のため、現在の Ready queue から優先度順に以下を推奨。

1. **issues/open/002-emit-canonical-capability-manifest-schema.md** (P0)
   - capability manifest schema の正本を定義
   - Gate C の前提条件

2. **issues/open/012-fix-computed-property-semantics-bug.md** (P0)
   - 意味論バグ修正
   - 既存 fixture で再現可能

3. **issues/open/013-implement-heap-oom-check.md** (P0)
   - runtime safety critical
   - memory safety 関連

4. **issues/open/004-reclassify-compile-only-compatibility-tests.md** (P0)
   - coverage 測定の正確性改善
   - Gate D の前提条件

5. **issues/open/005-add-fine-grained-unsupported-feature-breakdown.md** (P0)
   - coverage breakdown の改善
   - 機能レベルのトラッキング

6. **issues/open/010-extract-frontend-module-from-crates-cli.md** (P1)
   - frontend モジュールの分離
   - TypeScript integration の前提

## Current Policy

- `docs/` は ADR/設計判断の保存先として扱う。
- 実装の現在地と検証手順の要約はこの `current-state.md` を正とする。
- coverage 実測値は `artifacts/coverage/reference-coverage-matrix.md` を正とする。
- project goal、gates、schema は `docs/11-shared-definitions.md` を正とし、他 doc で再定義しない。
