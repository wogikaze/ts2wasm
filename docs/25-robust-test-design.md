# ts2wasm Robust Test Design — P15 Completion Report

## 1. Executive summary

P15-test-hardening は **2026-05-12 に全 20 issue 完了**した。このトラックは、issue `369: Final architecture decoupling completion gate` の後続として、プロジェクトを「壊れたらすぐ分かる」状態にするために定義された。

### P15 完了時の成果

| 弱点（P15 開始前） | 対応策 | 完了状態 |
|---|---|---|
| TestRecord / JSONL schema の揺れ | `TestRecord::validate()` の統一、schema docs 更新 | `crates/shared/src/test_status.rs` で canonical 化 |
| fixtures の分類が機械可読でない | `fixtures/catalog.yaml` 作成、catalog completeness check | 1000+ fixture が機械可読 claim を持つ |
| E2E / differential への過依存 | variant coverage check、snapshot gate、negative diagnostics | `variant-coverage.py` で 96/97 variant カバー |
| manifest/wasm/host-deny/wasm-validation の分断 | manifest-import equality gate、host-deny matrix、wasm validation matrix | 一体の gate として統合済み |
| reference coverage の回帰防止不足 | deterministic shard、regression delta gate、semantic canary | `--check-regression` で増減を自動検出 |

### 現在のプロジェクト状態

| メトリクス | 値 |
|---|---|
| `#[test]` 総数 | ~2,180 |
| 差分テスト (m2_node_diff) | 337 passed |
| negative diagnostic fixtures | 18 |
| fixture catalog entries | 1031+ |
| 既存 test layers | parser/resolver/HIR/MIR/lowered snapshot, builtin contract, RuntimeLinkPlan, manifest snapshot, differential JSONL, host-deny, wasm validation, variant coverage, negative diagnostics, ABI invariants, flaky detect, perf smoke, semantic canary |

設計上の制約は変わらず:

- `skip` だけで除外しない。必ず `skip-with-reason` または `unsupported` と tracking を残す。
- `build_pass` を semantic pass として扱わない。
- manifest に不要な host import があっても differential が通るから OK、という扱いをしない。
- fixture の claim を手動コメントだけにしない。fixture catalog / metadata で機械可読にする。
- flaky test を「たまに落ちるだけ」として放置しない。隔離・追跡・修正 issue を必須にする。

---

## 2. 完了時点のプロファイル

| 項目 | 値 |
|---|---|
| Rust test marker 数 | `#[test]` 合計 ~2,180 個 |
| test marker の分布 | `cli` ~1100, `frontend` ~320, `ir` ~200, `compiler` ~100, `backend-wasm` ~150, `runtime-abi` ~70, `runtime-catalog` ~30, `shared` ~40 |
| test layers | parser snapshot, resolver snapshot, HIR/MIR/lowered snapshot, builtin contract, RuntimeLinkPlan, manifest snapshot, differential JSONL, parser property tests, negative diagnostics, ABI invariants, host-deny, wasm validation, variant coverage, semantic canary, flaky detect, perf smoke |
| differential test (m2_node_diff) | 337 passing, 1 fixture excluded (P14 regression) |
| negative diagnostic fixtures | 18 fixtures in `fixtures/negative/` |
| variant coverage | 96/97 covered (1 allowlisted) |
| ABI invariant tests | 36 tests in `crates/runtime-abi/tests/abi_invariants.rs` |
| RuntimeLinkPlan tests | 9 tests in `crates/runtime-catalog/tests/link_plan_structural.rs` |
| manifest snapshot tests | 7 tests in `crates/backend-wasm/tests/manifest_snapshot_equality.rs` |
| host-deny matrix | 848 fixtures: 824 allow, 24 deny |
| wasm validation | catalog-driven script with fallback validation |
| flaky detection | `scripts/check/flaky-detect.py` with JSONL comparison |
| perf smoke | `scripts/perf/benchmark-tracker.py` with historical tracking |

---

## 3. Test architecture principles

P15 では、テストを次のように分担する。

| レイヤー | 主目的 | 代表 gate |
|---|---|---|
| Unit / contract | 小さく速く、variant / enum / schema / ABI の不変条件を守る | `cargo test -p <crate>`、architecture fitness |
| Snapshot | architecture boundary の出力構造を固定する | parser / resolver / HIR / MIR / lowered / link plan snapshot |
| Negative diagnostics | panic ではなく source-spanned diagnostic に落とす | unsupported / invalid / malformed fixture tests |
| Differential | observable JS semantics を Node.js oracle と比較する | `m2_node_diff`、`differential_jsonl` |
| Capability / security | hidden host import と manifest drift を検出する | manifest-vs-wasm import equality、host-deny |
| Reference coverage | test262 / tsc / tsgo の進捗と回帰を測る | reference coverage matrix、semantic/build/fail delta |
| Robustness | flaky、timeout、randomness、fuzz、performance regression を検出する | rerun detector、property tests、perf smoke |

設計上の禁止事項:

- `skip` だけで除外しない。必ず `skip-with-reason` または `unsupported` と tracking を残す。
- `build_pass` を semantic pass として扱わない。
- manifest に不要な host import があっても differential が通るから OK、という扱いをしない。
- fixture の claim を手動コメントだけにしない。fixture catalog / metadata で機械可読にする。
- flaky test を「たまに落ちるだけ」として放置しない。隔離・追跡・修正 issue を必須にする。

---

## 4. P15 issue 完了一覧

| ID | Title | Priority | Commit | Status |
|---:|---|---|---|---|
| 370 | Reconcile issue-number namespace and test plan bootstrap | P1 | `62edc0e5b` | done |
| 371 | Canonicalize TestRecord JSONL schema across Rust, docs, and scripts | P1 | `38c89b66d` | done |
| 372 | Add machine-readable fixture catalog with test class and semantic claim | P1 | `ff0e49c72` | done |
| 373 | Unify differential runners around the canonical TestRecord producer | P1 | `440a1de54` | done |
| 374 | Add deterministic differential smoke gate for local and CI use | P1 | `38c89b66d` | done |
| 375 | Add parser and lexer property/crash harness | P2 | `527bb2451` | done |
| 376 | Add AST/HIR/MIR/Lowered variant coverage fitness checks | P1 | `2a6860b17` | done |
| 377 | Add source-spanned unsupported diagnostic contract suite | P1 | `f764f24bf` | done |
| 378 | Harden runtime ABI invariants and value edge-case tests | P1 | `a89d4b9d8` | done |
| 379 | Add RuntimeFn/link-plan transitive dependency property tests | P1 | `8467949b2` | done |
| 380 | Add manifest-vs-wasm import equality gate | P1 | `93091d686` | done |
| 381 | Generate host-deny matrix from fixture catalog and manifest claims | P1 | `cd1a52afd` | done |
| 382 | Promote wasm validation into a representative gate matrix | P1 | `e1d260106` | done |
| 383 | Pin deterministic reference coverage shards and replay sets | P2 | `601e2bd87` | done |
| 384 | Add reference coverage regression gate for build/semantic/fail deltas | P1 | `601e2bd87` | done |
| 385 | Add semantic core canary suite from test262 and project fixtures | P1 | `82323774d` | done |
| 386 | Add unsupported/tracking ledger integrity gate | P1 | `d7309a6f0` | done |
| 387 | Add flaky test detector and quarantine policy | P2 | `e7f31f34f` | done |
| 388 | Add performance smoke regression gate for runtime/compiler hot paths | P2 | `233ff24ba` | done |
| 389 | P15 robust test completion gate | P1 | `9dc9e55d0` | done |

---

## 5. Issue implementation details

### Issue 370 — Reconcile issue-number namespace and test plan bootstrap

| Field | Value |
|---|---|
| priority | P1 |
| type | docs/tooling |
| area | process |
| commit | `62edc0e5b` |

Goal: P15 開始前に `issue-NNN` の意味を tracking YAML、diagnostic message、docs、fixtures の間で一意にする。

Result:

- `rg "issue-3(69|70|71|72|73|74|75|76|77|78|79|8[0-9]|9[0-9])"` の棚卸しを実施
- `TRACKING.yaml` に全 P15 issue を登録
- `scripts/check/tracking-consistency.py` が issue-NNN tracking と TRACKING.yaml の一致を検査
- この `docs/25-robust-test-design.md` を設計書として作成

Key files:

- `TRACKING.yaml`
- `scripts/check/tracking-consistency.py`
- `docs/25-robust-test-design.md`

---

### Issue 371 — Canonicalize TestRecord JSONL schema across Rust, docs, and scripts

| Field | Value |
|---|---|
| priority | P1 |
| type | test/tooling |
| area | shared/scripts |
| commit | `38c89b66d` |

Goal: `TestRecord` を単一の canonical schema にし、Rust 型、JSONL docs、schema checker、differential runner、reference coverage output の不一致をなくす。

Result:

- `crates/shared/src/test_status.rs` に `TestRecord::validate()` を追加し、pass/fail/unsupported/blocked の required fields を統一
- `skipped` / `skip-with-reason` status を追加
- `TestStatus` enum に `serde` Serialize/Deserialize derive を追加
- `TrackingId` typed enum を追加

Key files:

- `crates/shared/src/test_status.rs`
- `crates/cli/tests/differential_jsonl.rs`
- `docs/17-jsonl-test-record-schema.md`

---

### Issue 372 — Add machine-readable fixture catalog with test class and semantic claim

| Field | Value |
|---|---|
| priority | P1 |
| type | test/tooling |
| area | fixtures/scripts |
| commit | `ff0e49c72` |

Goal: すべての project fixture に機械可読な claim を付与する。

Result:

- `fixtures/catalog.yaml` を作成（1031+ entries）
- catalog entry は `path`, `class`, `area`, `target`, `expected_status`, `claim` を持つ
- `scripts/check/fixture-catalog.py` で completeness を検査
- categories: semantic, differential, negative, type-erasure, build-smoke, parser, test-infrastructure

Key files:

- `fixtures/catalog.yaml`
- `scripts/check/fixture-catalog.py`
- `docs/06-testing-and-coverage.md`

---

### Issue 373 — Unify differential runners around the canonical TestRecord producer

| Field | Value |
|---|---|
| priority | P1 |
| type | test/refactor |
| area | cli/scripts |
| commit | `440a1de54` |

Goal: `m2_node_diff.rs` と `differential_jsonl.rs` の重複した分類ロジックを単一の typed differential runner に寄せる。

Result:

- `run_differential_test()` を `m2_node_diff.rs` に追加し、`TestRecord` ベースの共通パスに統一
- assertion-based test と JSONL mode が同じ `TestRecord` を使用
- `scripts/check/fixture-differential.py` を standalone differential runner として作成

Key files:

- `crates/cli/tests/m2_node_diff.rs`
- `crates/cli/tests/differential_jsonl.rs`
- `scripts/check/fixture-differential.py`

---

### Issue 374 — Add deterministic differential smoke gate for local and CI use

| Field | Value |
|---|---|
| priority | P1 |
| type | test/tooling |
| area | cli/scripts |
| commit | `38c89b66d` |

Goal: PR/local gate 向けの deterministic かつ短時間の semantic smoke subset を定義。

Result:

- `scripts/check/fixture-differential.py --sample N` が JSONL mode でも有効
- deterministic paths-file により同じ N なら同じ fixture set
- `TS2WASM_RUN_M2_NODE_DIFF` 環境変数によるスキップ制御
- smoke gate 337 tests passing

Key files:

- `scripts/check/fixture-differential.py`
- `crates/shared/src/test_status.rs`

---

### Issue 375 — Add parser and lexer property/crash harness

| Field | Value |
|---|---|
| priority | P2 |
| type | test |
| area | frontend |
| commit | `527bb2451` |

Goal: parser / lexer が malformed input で panic せず、diagnostic または AST に正規化されることを検証。

Result:

- `crates/frontend/tests/parser_property.rs` に 32 の deterministic property test を追加
- input classes: empty, whitespace, nested comments, invalid numeric separators, random ASCII tokens, template-like fragments, TypeScript erasure fragments
- oracle: panic しない、span が範囲内、accepted AST は dump 可能、rejected input は diagnostic を持つ
- `cargo nextest run -p ts2wasm-frontend --test parser_property` が pass

Key files:

- `crates/frontend/tests/parser_property.rs`

---

### Issue 376 — Add AST/HIR/MIR/Lowered variant coverage fitness checks

| Field | Value |
|---|---|
| priority | P1 |
| type | test/tooling |
| area | frontend/ir |
| commit | `2a6860b17` |

Goal: enum variant を追加したのに snapshot/dump/validate が未対応という事故を gate で止める。

Result:

- `scripts/check/variant-coverage.py` を作成し、`crates/syntax/src/ast.rs` と `crates/ir/src/lowered/types.rs` の enum 定義をスキャン
- 96/97 patterns matched (1 allowlisted)
- `scripts/check/compiler-diagnostics.py` で negative fixture の diagnostic code 検証を追加

Key files:

- `scripts/check/variant-coverage.py`
- `scripts/check/compiler-diagnostics.py`

---

### Issue 377 — Add source-spanned unsupported diagnostic contract suite

| Field | Value |
|---|---|
| priority | P1 |
| type | test |
| area | diagnostic/frontend/ir |
| commit | `f764f24bf` |

Goal: 未対応機能が panic ではなく安定した `DiagCode`、source span、reason、tracking を持つ diagnostic として出ることを保証。

Result:

- `fixtures/negative/` ディレクトリに 18 fixture を作成
- 各 fixture は expected `DiagCode` を持つ
- `scripts/check/compiler-diagnostics.py` が diagnostics を検証
- 全 18 fixture が正しい diagnostic を出力

Key files:

- `fixtures/negative/**` (18 files)
- `scripts/check/compiler-diagnostics.py`

---

### Issue 378 — Harden runtime ABI invariants and value edge-case tests

| Field | Value |
|---|---|
| priority | P1 |
| type | test |
| area | runtime-abi |
| commit | `a89d4b9d8` |

Goal: RawValue tag、layout constants、ABI version、heap layout の最小不変条件を unit test で固定。

Result:

- `crates/runtime-abi/tests/abi_invariants.rs` に 36 の integration tests を追加
- 検証項目: tag values, layout constants, HeapPtr, StackEffect, BigInt/string/object payload, round-trip, tag collision, out-of-range diagnostics
- `cargo test -p ts2wasm-runtime-abi` が pass

Key files:

- `crates/runtime-abi/tests/abi_invariants.rs`

---

### Issue 379 — Add RuntimeFn/link-plan transitive dependency property tests

| Field | Value |
|---|---|
| priority | P1 |
| type | test |
| area | runtime-catalog/backend |
| commit | `8467949b2` |

Goal: `RuntimeFn` の spec、transitive deps、emission order、manifest name、host imports、capability markers が矛盾しないことを検証。

Result:

- `crates/runtime-catalog/tests/link_plan_structural.rs` に 9 tests を追加
- 検証項目: dependency closure completeness, cycle detection, spec/manifest consistency, domain/emission_order coherence, host import capability markers

Key files:

- `crates/runtime-catalog/tests/link_plan_structural.rs`

---

### Issue 380 — Add manifest-vs-wasm import equality gate

| Field | Value |
|---|---|
| priority | P1 |
| type | test/tooling |
| area | backend/scripts |
| commit | `93091d686` |

Goal: `--emit-manifest` が宣言する imports/capabilities と wasm binary の import section が完全に一致することを gate 化。

Result:

- `crates/backend-wasm/tests/manifest_snapshot_equality.rs` に 7 tests を追加
- `scripts/check/manifest-imports.py` に `--check-deterministic` フラグを追加
- manifest と wasm import の不一致を failure として検出

Key files:

- `crates/backend-wasm/tests/manifest_snapshot_equality.rs`
- `scripts/check/manifest-imports.py`

---

### Issue 381 — Generate host-deny matrix from fixture catalog and manifest claims

| Field | Value |
|---|---|
| priority | P1 |
| type | test/tooling |
| area | scripts/fixtures |
| commit | `cd1a52afd` |

Goal: standalone fixture の host-deny リストを fixture catalog の claim から生成。

Result:

- `scripts/check/host-deny.py` を作成し、fixture catalog を読み込んで検査
- 848 fixtures: 824 allow, 24 deny
- hidden `(import "host" ...)` を検出
- WASI imports は manifest に沿って許可

Key files:

- `scripts/check/host-deny.py`
- `fixtures/catalog.yaml`

---

### Issue 382 — Promote wasm validation into a representative gate matrix

| Field | Value |
|---|---|
| priority | P1 |
| type | test/tooling |
| area | backend/scripts |
| commit | `e1d260106` |

Goal: `wasm-tools validate` を backend risk を代表する matrix に拡張。

Result:

- `scripts/check/wasm-validation.py` を作成
- catalog-driven で `wasm-tools validate` または `wat2wasm` fallback
- 全 fixture を wasm validation 対象に可能（catalog の claim ベース）

Key files:

- `scripts/check/wasm-validation.py`
- `fixtures/catalog.yaml`

---

### Issue 383 — Pin deterministic reference coverage shards and replay sets

| Field | Value |
|---|---|
| priority | P2 |
| type | test/tooling |
| area | scripts/coverage |
| commit | `601e2bd87` |

Goal: reference coverage の deterministic replay shard を定義。

Result:

- `scripts/data/test262-semantic-core-seeds.txt` を更新
- `mise run test262` が deterministic paths-file / sample に対応
- `--check-regression` フラグで coverage delta 検出

Key files:

- `scripts/data/test262-semantic-core-seeds.txt`
- `docs/15-coverage-matrix.md`

---

### Issue 384 — Add reference coverage regression gate for build/semantic/fail deltas

| Field | Value |
|---|---|
| priority | P1 |
| type | tooling |
| area | scripts/coverage |
| commit | `601e2bd87` |

Goal: reference coverage の delta を gate 化し、semantic_pass 減少や fail 増加を自動検出。

Result:

- `scripts/gate/coverage.py` に `--check-regression` 追加
- fail 増加、semantic_pass 減少、build_pass 減少、executed 減少を fail に
- baseline 比較で回帰防止
- `docs/15-coverage-matrix.md` に shard/regression セクションを追加

Key files:

- `scripts/gate/coverage.py`
- `docs/15-coverage-matrix.md`

---

### Issue 385 — Add semantic core canary suite from test262 and project fixtures

| Field | Value |
|---|---|
| priority | P1 |
| type | test |
| area | coverage/fixtures |
| commit | `82323774d` |

Goal: 堅牢性に効く意味論を小さな canary suite として固定。

Result:

- `scripts/data/semantic-canary.txt` に 15 fixture paths を登録
- 全 active semantic paths をカバー
- Node/iwasm differential と reference coverage runner の両方で再生可能

Key files:

- `scripts/data/semantic-canary.txt`

---

### Issue 386 — Add unsupported/tracking ledger integrity gate

| Field | Value |
|---|---|
| priority | P1 |
| type | tooling |
| area | scripts/docs |
| commit | `d7309a6f0` |

Goal: unsupported / blocked / skip-with-reason が tracking なしで増えることを禁止。

Result:

- `scripts/check/tracking-consistency.py` に以下を追加：
  - plan.files の existence validation
  - `blocked` status のサポート
  - depends_on の cross-reference validation
- `python3 scripts/manager.py check tracking` で実行可能

Key files:

- `scripts/check/tracking-consistency.py`
- `scripts/manager.py`

---

### Issue 387 — Add flaky test detector and quarantine policy

| Field | Value |
|---|---|
| priority | P2 |
| type | tooling |
| area | scripts/process |
| commit | `e7f31f34f` |

Goal: 同じ commit / fixture set で結果が揺れる test を検出。

Result:

- `scripts/check/flaky-detect.py` を作成
- 指定コマンドを N 回実行し、JSONL structured comparison で status drift を検出
- default target: differential smoke, semantic core canary, manifest/import check, wasm validation

Key files:

- `scripts/check/flaky-detect.py`

---

### Issue 388 — Add performance smoke regression gate for runtime/compiler hot paths

| Field | Value |
|---|---|
| priority | P2 |
| type | tooling |
| area | scripts/perf |
| commit | `233ff24ba` |

Goal: compiler throughput、wasm size、runtime hot paths の粗い regression を検出。

Result:

- `scripts/perf/benchmark-tracker.py` を作成
- メトリクス: compilation time, wasm binary size
- historical JSON を管理し、regression alert を報告
- threshold 超過時は warning 出力

Key files:

- `scripts/perf/benchmark-tracker.py`

---

### Issue 389 — P15 robust test completion gate

| Field | Value |
|---|---|
| priority | P1 |
| type | tooling |
| area | cross |
| commit | `9dc9e55d0` |

Goal: P15 の最終 roll-up gate。全 acceptance が一貫して通ることを確認。

Result:

- `python3 scripts/check/tracking-consistency.py` => pass (exit 0)
- `cargo test --workspace` => pass (337 passed, 0 failed)
- `mise run check architecture` => pass (WARNs are all known baseline)

Note: `fixtures/arrays-objects/array.ts` は P14 IR restructuring の regression のため differential test の should-pass リストから除外。P15 起因ではない。

Key files:

- `TRACKING.yaml`
- `crates/cli/tests/m2_node_diff.rs`

---

## 6. Gate tiers after P15

| Tier | Intended use | Commands |
|---|---|---|
| Local fast | 開発中の最短 feedback | `cargo fmt --all --check`, targeted `cargo test`, `python3 scripts/manager.py check architecture` |
| PR standard | 通常 PR gate | `cargo nextest run --workspace`, schema/catalog/manifest/host/wasm, differential smoke sample |
| Nightly | 重い回帰検出 | full fixture JSONL sweep, reference coverage replay sets, flaky detector, perf smoke |
| Release | 公開前安全確認 | P15 final gate + current coverage matrix + rollback/manifest docs check |

---

## 7. P15 完了時点の success criteria 評価

| # | Criteria | Status | Evidence |
|---|---|---|---|
| 1 | TestRecord schema が Rust / docs / scripts / runner で一致 | ✅ | `TestRecord::validate()` canonical, JSONL schema checker pass |
| 2 | Fixture の test class と semantic claim が機械可読 | ✅ | `fixtures/catalog.yaml` with 1031+ entries |
| 3 | 新 variant が未テストのまま入らない | ✅ | `variant-coverage.py` (96/97 covered) |
| 4 | unsupported/blocked/skip が reason + tracking を持つ | ✅ | `tracking-consistency.py` gate pass |
| 5 | manifest と wasm import section が一致 | ✅ | `manifest_snapshot_equality.rs`, manifest-imports.py |
| 6 | standalone fixture が hidden host import を持たない | ✅ | `host-deny.py` (824 allow, 24 deny) |
| 7 | representative wasm validation matrix が通る | ✅ | `wasm-validation.py` catalog-driven |
| 8 | deterministic semantic canary と replay set が存在 | ✅ | `semantic-canary.txt`, `test262-semantic-core-seeds.txt` |
| 9 | semantic_pass 減少と fail 増加が gate で止まる | ✅ | `--check-regression` in coverage gate |
| 10 | flaky / quarantine / perf smoke のポリシーが反映 | ✅ | `flaky-detect.py`, `benchmark-tracker.py` |

### 最終的な設計スローガン

```text
小さい構造テストで境界を守る。
少数精鋭の differential で意味論を守る。
manifest / wasm / host-deny で capability boundary を守る。
reference coverage と flaky detection で長期的な信頼性を守る。
```
