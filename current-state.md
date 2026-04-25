# Current State

Last updated: 2026-04-26

この文書は、現在の実装状態と検証の事実だけを記録する。設計は `docs/` 側に置き、ここでは「今何が動くか」「何が未実装か」「何を確認すればよいか」を扱う。

## Current gate（このリポジトリが今要求する最小バー）

正本は `docs/11-shared-definitions.md` の Workstreams / Gates。実装・レビューでまず満たすのは次の組み合わせとする。

- **Gate A（テスト）**: `cargo fmt --all --check` と `cargo nextest run`（フル suite。重いテストを分離する場合は `docs/11` の filterset 方針に従う）。
- **Gate D（coverage artifact）**: `scripts/update_coverage_matrix.sh --check` が `artifacts/coverage/reference-coverage-matrix.md` を検証。
- **その他（B–C, E–G）**: ポリシーと checklist は `docs/11` / `docs/12-coding-standard.md`（§19）に記載。証拠コマンドは下記「Last verified commands」。

## Last verified commands（代表）

開発者がローカルで再現する際の最小セット（CI と揃える場合はワークフローを参照）。

```bash
cargo fmt --all --check
cargo nextest run
scripts/update_coverage_matrix.sh --check
scripts/check_scripts.sh
```

reference coverage を更新する場合（実測値を変えるとき）:

```bash
scripts/update_coverage_matrix.sh
# または単 suite: scripts/reference_coverage.sh test262 --limit 50
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
- 列 `build_pass` / `semantic_pass` は `scripts/reference_coverage.sh` の出力に対応（semantic-pass は Node + `iwasm` が利用可能な環境でのみ増分）。

## Implemented (high-level)

- minimal parser/frontend（`crates/cli`）
- WAT/WASM emitter と runtime subset（`crates/cli`）
- shared schema crate（`crates/shared`）: ABI/capability/test status
- reference coverage パイプライン（`scripts/reference_coverage.sh`, `scripts/update_coverage_matrix.sh`, `scripts/check_coverage_gate.sh`）
- generated coverage table（`artifacts/coverage/reference-coverage-matrix.md`）

## Known blockers / gaps

- TypeScript parser/checker integration は未実装
- 汎用 JavaScript semantic IR は未実装
- full wasm backend は未実装（現状は WAT 中心）
- test262 full differential 運用は未完（sample/ramp が中心）
- OOM/GC/UTF-8 完全対応は未完
- host-deny / capability manifest の「監査可能な」E2E は `docs/06` の required test classes に沿って拡張予定

## Next legal slice（実装単位の候補）

次に取り込みやすい縦スライスは、`docs/11` の workstream 順と open issue を優先する。具体的な ticket は `issues/` を参照。ここでは「次の一行」だけ固定しない（更新コストを避けるため）。

## Current Policy

- `docs/` は ADR/設計判断の保存先として扱う。
- 実装の現在地と検証手順の要約はこの `current-state.md` を正とする。
- coverage 実測値は `artifacts/coverage/reference-coverage-matrix.md` を正とする。
- project goal、gates、schema は `docs/11-shared-definitions.md` を正とし、他 doc で再定義しない。
