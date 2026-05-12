# ts2wasm Robust Test Design Plan

## 1. Executive summary

この計画は、issue `369: Final architecture decoupling completion gate` の後続として、プロジェクトを「壊れたらすぐ分かる」状態にするための **P15-test-hardening** トラックを定義する。

既存プロジェクトには、parser snapshot、resolver snapshot、HIR/MIR/lowered snapshot、RuntimeLinkPlan、manifest snapshot、Node/iwasm differential、test262 reference coverage、host-deny、architecture checks の土台がある。一方で、現在の弱点は次の 5 点に集約できる。

1. **TestRecord / JSONL schema に揺れがある。** 共有 Rust 型、`docs/17-jsonl-test-record-schema.md`、`scripts/check/test-records-schema.py`、`crates/cli/tests/differential_jsonl.rs` の期待値が完全には一致していない。
2. **fixtures の分類が機械可読ではない。** `parser_smoke` / `build_smoke` / `semantic_diff` の意図は docs にあるが、fixture 単位で claim と tracking を強制する catalog が弱い。
3. **E2E / differential に依存しすぎる危険が残る。** issue 345 で snapshot 層は入っているが、AST/HIR/MIR/Lowered/RuntimeFn/HostImport の variant coverage を gate 化していない。
4. **manifest / wasm import / host-deny / wasm validation が一体の gate になっていない。** 個別 script は存在するが、release-level の境界保証には足りない。
5. **reference coverage の回帰防止と flaky 検出が不足している。** semantic pass を伸ばす前に、selected shard の再現性、分類の安定性、fail 増加検出を固める必要がある。

この設計では、P15 を「新機能追加」ではなく **test architecture の補強** として扱う。ECMAScript full compliance を求めず、test262 全件 pass も要求しない。目的は、今後の feature vertical slice が parser / resolver / IR / runtime catalog / backend / manifest / differential / coverage のどこを壊しても、短いフィードバックで検出できるようにすること。

---

## 2. 現状観測メモ

アップロードされた archive snapshot から確認した事項:

| 項目 | 観測 |
|---|---|
| Rust test marker 数 | `#[test]` は合計 1,566 個程度 |
| test marker の分布 | `cli` 922、`frontend` 293、`ir` 137、`compiler` 81、`backend-wasm` 72、`runtime-abi` 31、`shared` 24 など |
| 既存 test layers | parser snapshot、resolver snapshot、HIR/MIR/lowered snapshot、builtin contract、RuntimeLinkPlan、manifest snapshot、differential JSONL、host-deny、wasm validation script |
| current-state の test262 | executed 9,359、build_pass 864、semantic_pass 773、semantic coverage 1.45% |
| gate の現状 | `fast-gate.py` は fmt / tracking / assert-true / architecture / nextest が中心。manifest import / host-deny / wasm validation / JSONL schema / reference coverage regression は独立 check として存在 |
| archive の制約 | root `Cargo.toml` と `fixtures/` は snapshot に含まれていないため、実コマンド実行ではなく構造検査ベースで設計した |

注意: archive 内には `issue-369` / `issue-370` などの文字列が BigInt diagnostic 用に残っている。ユーザー指定では `369` が architecture final gate であり、次を `370` から作成予定なので、最初の issue で **issue ID namespace の衝突を棚卸し** する。

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

## 4. P15 issue overview

| ID | Title | Priority | Type | Area | Roadmap | Depends on |
|---:|---|---|---|---|---|---|
| 370 | Reconcile issue-number namespace and test plan bootstrap | P1 | docs/tooling | process | P15-test-hardening | 369 |
| 371 | Canonicalize TestRecord JSONL schema across Rust, docs, and scripts | P1 | test/tooling | shared/scripts | P15-schema | 370 |
| 372 | Add machine-readable fixture catalog with test class and semantic claim | P1 | test/tooling | fixtures/scripts | P15-fixtures | 371 |
| 373 | Unify differential runners around the canonical TestRecord producer | P1 | test/refactor | cli/scripts | P15-differential | 371,372 |
| 374 | Add deterministic differential smoke gate for local and CI use | P1 | test/tooling | cli/scripts | P15-differential | 373 |
| 375 | Add parser and lexer property/crash harness | P2 | test | frontend | P15-property | 371 |
| 376 | Add AST/HIR/MIR/Lowered variant coverage fitness checks | P1 | test/tooling | frontend/ir | P15-boundary | 345,371 |
| 377 | Add source-spanned unsupported diagnostic contract suite | P1 | test | diagnostic/frontend/ir | P15-negative | 371,372 |
| 378 | Harden runtime ABI invariants and value edge-case tests | P1 | test | runtime-abi | P15-runtime | 371 |
| 379 | Add RuntimeFn/link-plan transitive dependency property tests | P1 | test | runtime-catalog/backend | P15-link-plan | 378 |
| 380 | Add manifest-vs-wasm import equality gate | P1 | test/tooling | backend/scripts | P15-capability | 379 |
| 381 | Generate host-deny matrix from fixture catalog and manifest claims | P1 | test/tooling | scripts/fixtures | P15-capability | 372,380 |
| 382 | Promote wasm validation into a representative gate matrix | P1 | test/tooling | backend/scripts | P15-backend | 372,380 |
| 383 | Pin deterministic reference coverage shards and replay sets | P2 | test/tooling | scripts/coverage | P15-coverage | 371 |
| 384 | Add reference coverage regression gate for build/semantic/fail deltas | P1 | tooling | scripts/coverage | P15-coverage | 383 |
| 385 | Add semantic core canary suite from test262 and project fixtures | P1 | test | coverage/fixtures | P15-canary | 372,383 |
| 386 | Add unsupported/tracking ledger integrity gate | P1 | tooling | scripts/docs | P15-tracking | 371,372,384 |
| 387 | Add flaky test detector and quarantine policy | P2 | tooling | scripts/process | P15-flaky | 374,385 |
| 388 | Add performance smoke regression gate for runtime/compiler hot paths | P2 | tooling | scripts/perf | P15-performance | 374,382 |
| 389 | P15 robust test completion gate | P1 | tooling | cross | P15-final-gate | 370-388 |

---

## 5. Detailed issue designs

### Issue 370 — Reconcile issue-number namespace and test plan bootstrap

| Field | Value |
|---|---|
| priority | P1 |
| type | docs/tooling |
| area | process |
| roadmap | P15-test-hardening |
| depends_on | 369 |

Goal: P15 を開始する前に、`issue-NNN` の意味を tracking YAML、diagnostic message、docs、fixtures の間で一意にする。archive snapshot では BigInt diagnostic に `issue-369` / `issue-370` が残っているため、新規 issue 370 以降との衝突を明示的に解決する。

Acceptance:

- `rg "issue-3(69|70|71|72|73|74|75|76|77|78|79|8[0-9]|9[0-9])" crates docs fixtures scripts` の棚卸し結果を doc に残す。
- `docs/25-robust-test-design.md` か同等の P15 設計書が存在する。
- diagnostic 内の `issue-NNN` が実 tracker に対応するか、`feature:<label>` に置換される方針が決まっている。
- `python3 scripts/manager.py check tracking` が P15 issue id と diagnostic tracking の衝突を検出できる設計になっている。

Non-goals:

- BigInt semantics を修正しない。
- issue YAML をこの issue だけで全生成しない。
- 過去 issue の実装内容を再定義しない。

Plan files:

- `docs/25-robust-test-design.md`
- `docs/current-state.md`
- `scripts/check/tracking-consistency.py`
- `crates/**`

---

### Issue 371 — Canonicalize TestRecord JSONL schema across Rust, docs, and scripts

| Field | Value |
|---|---|
| priority | P1 |
| type | test/tooling |
| area | shared/scripts |
| roadmap | P15-schema |
| depends_on | 370 |

Goal: `TestRecord` を単一の canonical schema にし、Rust 型、JSONL docs、schema checker、differential runner、reference coverage output の不一致をなくす。

Current drift to resolve:

- `crates/shared/src/test_status.rs` は `pass/fail/unsupported/blocked/skip-with-reason` を持つ。
- `docs/17-jsonl-test-record-schema.md` も同じ 5 status を記述している。
- `scripts/check/test-records-schema.py` は `build_pass` を許可し、`target=wasm-iwasm` や `expected/actual/oracle` を `pass` に要求している。
- `crates/cli/tests/differential_jsonl.rs` は `target=wasm32-wasi` かつ `pass` の `expected/actual` を `None` にしている。

Acceptance:

- `docs/17-jsonl-test-record-schema.md` が canonical schema として更新される。
- `crates/shared/src/test_status.rs` の `TestRecord::validate()` が docs と一致する。
- `scripts/check/test-records-schema.py --self-test` が Rust schema と同じ status / required fields を検査する。
- `cargo test -p ts2wasm-shared test_status`
- `cargo nextest run -p ts2wasm-cli --test differential_jsonl differential_jsonl_quick_check_formats`
- `python3 scripts/manager.py check records -- --self-test`

Non-goals:

- reference coverage の pass 数を増やさない。
- fixture の意味論分類そのものは 372 で扱う。

Plan files:

- `crates/shared/src/test_status.rs`
- `crates/cli/tests/differential_jsonl.rs`
- `scripts/check/test-records-schema.py`
- `docs/17-jsonl-test-record-schema.md`
- `docs/11-shared-definitions.md`

---

### Issue 372 — Add machine-readable fixture catalog with test class and semantic claim

| Field | Value |
|---|---|
| priority | P1 |
| type | test/tooling |
| area | fixtures/scripts |
| roadmap | P15-fixtures |
| depends_on | 371 |

Goal: すべての project fixture に、`parser_smoke` / `build_smoke` / `semantic_diff` / `negative_diagnostic` / `host_deny` / `wasm_validate` の claim を機械可読に付与する。これにより、build smoke を semantic pass と誤認しない。

Acceptance:

- fixture catalog 形式を定義する。候補: `fixtures/catalog.jsonl` または `fixtures/catalog.yaml`。
- catalog entry は最低限 `path`, `class`, `area`, `target`, `expected_status`, `tracking`, `claim` を持つ。
- `scripts/check/fixture-catalog.py` が top-level naming だけでなく catalog completeness を検査する。
- `docs/06-testing-and-coverage.md` の `parser_smoke` / `build_smoke` / `semantic_diff` 定義と catalog class が一致する。
- `python3 scripts/manager.py check fixtures`
- `python3 scripts/manager.py check records -- --self-test`

Non-goals:

- fixture 内容の大規模 rewrite はしない。
- test262 全件に project fixture catalog を適用しない。

Plan files:

- `fixtures/catalog.*`
- `scripts/check/fixture-catalog.py`
- `docs/06-testing-and-coverage.md`
- `docs/15-coverage-matrix.md`

---

### Issue 373 — Unify differential runners around the canonical TestRecord producer

| Field | Value |
|---|---|
| priority | P1 |
| type | test/refactor |
| area | cli/scripts |
| roadmap | P15-differential |
| depends_on | 371, 372 |

Goal: `m2_node_diff.rs` と `differential_jsonl.rs` の重複した分類ロジックを、単一の typed differential runner に寄せる。assertion mode と JSONL mode が同じ `TestRecord` を使うようにする。

Acceptance:

- Node oracle / ts2wasm build / iwasm run / stdout diff / diagnostic extraction が shared helper に集約される。
- assertion-based test は `TestRecord` を検査して fail する。
- JSONL mode は同じ `TestRecord` を stdout に出す。
- `feature_label_from_diag` の label が catalog / tracking と整合する。
- `cargo nextest run -p ts2wasm-cli --test differential_jsonl differential_jsonl_quick_check_formats`
- `cargo nextest run -p ts2wasm-cli --test m2_node_diff --no-fail-fast`
- `python3 scripts/manager.py check differential -- --jsonl --sample 25`

Non-goals:

- m2 fixture の pass 数を増やさない。
- Node/iwasm unavailable 環境を pass 扱いしない。

Plan files:

- `crates/cli/tests/m2_node_diff.rs`
- `crates/cli/tests/differential_jsonl.rs`
- `crates/cli/tests/common/**`
- `crates/shared/src/test_status.rs`

---

### Issue 374 — Add deterministic differential smoke gate for local and CI use

| Field | Value |
|---|---|
| priority | P1 |
| type | test/tooling |
| area | cli/scripts |
| roadmap | P15-differential |
| depends_on | 373 |

Goal: full differential sweep は重いので、PR/local gate 向けに deterministic かつ短時間の semantic smoke subset を定義する。subset は fixture catalog から生成し、Node/iwasm がある環境では必ず意味論比較を行う。

Acceptance:

- `scripts/check/fixture-differential.py --sample N` が JSONL mode でも有効になる。
- deterministic seed / paths-file により、同じ N なら同じ fixture set を走らせる。
- `TS2WASM_RUN_M2_NODE_DIFF` の default-skip と smoke gate の関係を整理する。
- `python3 scripts/manager.py check differential -- --sample 25`
- `python3 scripts/manager.py check differential -- --jsonl --sample 25 | python3 scripts/manager.py check records -`
- `docs/06-testing-and-coverage.md` に local / PR / nightly の使い分けを追記する。

Non-goals:

- 全 fixture を毎 PR で走らせない。
- iwasm 未導入環境で semantic pass を主張しない。

Plan files:

- `scripts/check/fixture-differential.py`
- `crates/cli/tests/differential_jsonl.rs`
- `docs/06-testing-and-coverage.md`
- `fixtures/catalog.*`

---

### Issue 375 — Add parser and lexer property/crash harness

| Field | Value |
|---|---|
| priority | P2 |
| type | test |
| area | frontend |
| roadmap | P15-property |
| depends_on | 371 |

Goal: parser / lexer が malformed input、Unicode、numeric separators、comments、nested syntax で panic せず、diagnostic または AST に正規化されることを検証する。最初は external fuzzer ではなく deterministic corpus + small generator で始める。

Acceptance:

- `crates/frontend/tests/parser_property.rs` または同等の deterministic property test を追加する。
- input classes: empty, whitespace, nested comments, invalid numeric separators, random ASCII tokens, template-like fragments, TypeScript erasure fragments。
- oracle: panic しない、span が範囲内、accepted AST は dump 可能、rejected input は diagnostic を持つ。
- `cargo nextest run -p ts2wasm-frontend --test parser_property`
- `cargo nextest run -p ts2wasm-frontend --test parser_snapshot`

Non-goals:

- coverage-guided fuzzing infrastructure の完全導入はしない。
- 生成 input の ECMAScript 妥当性は要求しない。

Plan files:

- `crates/frontend/tests/parser_property.rs`
- `crates/frontend/src/lexer*.rs`
- `crates/frontend/src/parser/**`
- `docs/06-testing-and-coverage.md`

---

### Issue 376 — Add AST/HIR/MIR/Lowered variant coverage fitness checks

| Field | Value |
|---|---|
| priority | P1 |
| type | test/tooling |
| area | frontend/ir |
| roadmap | P15-boundary |
| depends_on | 345, 371 |

Goal: enum variant を追加したのに snapshot/dump/validate が未対応、というリファクタ事故を gate で止める。`LoweredExpr` の validate coverage は既存 architecture check にあるため、AST / Resolved / HIR / MIR / LoweredStmt まで広げる。

Acceptance:

- `scripts/check/variant-coverage.py` か architecture check 拡張で、主要 enum variant と dump/validate/snapshot coverage の対応を確認する。
- 対象: AST Expr/Stmt、ResolvedExpr/Stmt、HIR、MIR、LoweredExpr/Stmt、RuntimeFn、HostImport。
- 未対応 variant は allowlist + reason + owner が必須。
- `python3 scripts/manager.py check architecture`
- `python3 scripts/manager.py check variant-coverage` または `architecture` に統合。
- `cargo nextest run -p ts2wasm-frontend --test parser_snapshot`
- `cargo nextest run -p ts2wasm-ir --test hir_snapshot --test mir_snapshot --test lowered_snapshot --test resolver_snapshot`

Non-goals:

- すべての variant に semantic_diff fixture を要求しない。
- HIR/MIR の設計変更自体は行わない。

Plan files:

- `scripts/check/architecture-rules.py`
- `scripts/check/variant-coverage.py`
- `crates/frontend/tests/parser_snapshot.rs`
- `crates/ir/tests/**`
- `docs/24-architecture-decoupling-and-llm-friendly-sizing.md`

---

### Issue 377 — Add source-spanned unsupported diagnostic contract suite

| Field | Value |
|---|---|
| priority | P1 |
| type | test |
| area | diagnostic/frontend/ir |
| roadmap | P15-negative |
| depends_on | 371, 372 |

Goal: 未対応機能や不正入力が panic / unwrap / generic error ではなく、安定した `DiagCode`、source span、reason、tracking を持つ diagnostic として出ることを保証する。

Acceptance:

- negative fixture catalog を作る。各 fixture は expected `DiagCode`, span policy, tracking を持つ。
- `UnsupportedSyntax`, `UnsupportedBuiltin`, `UnresolvedName`, `ArityMismatch`, `NumberOutOfRange`, `InvalidTopLevelReturn`, `InvariantViolation not allowed` などを明示テストする。
- diagnostic message に `issue-NNN` がある場合は tracking ledger と一致する。
- `cargo nextest run -p ts2wasm-cli --test command_contract build_invalid_ts_exits_failure`
- `cargo nextest run -p ts2wasm-cli --test dump_cli dump_ast_reports_invalid_numeric_literal_separator`
- `python3 scripts/manager.py check diagnostics`

Non-goals:

- diagnostic 文言の長文完全一致を増やしすぎない。code/span/tracking を主に固定する。
- 未対応機能を実装済みにしない。

Plan files:

- `fixtures/negative/**`
- `fixtures/catalog.*`
- `crates/cli/tests/command_contract.rs`
- `crates/cli/tests/dump_cli.rs`
- `scripts/check/compiler-diagnostics.py`
- `crates/diagnostic/src/lib.rs`

---

### Issue 378 — Harden runtime ABI invariants and value edge-case tests

| Field | Value |
|---|---|
| priority | P1 |
| type | test |
| area | runtime-abi |
| roadmap | P15-runtime |
| depends_on | 371 |

Goal: RawValue tag、layout constants、ABI version、heap layout、BigInt/string/object payload の最小不変条件を unit test と golden snapshot で固定する。runtime bug が differential まで行かないと見つからない状態を減らす。

Acceptance:

- `crates/runtime-abi/src/layout.rs` / `value.rs` の golden snapshot が ABI version と layout diff を明確に示す。
- edge cases: `undefined/null/bool/small-int/string pointer/object pointer/bigint pointer` の round-trip、tag collision、out-of-range number diagnostic。
- ABI layout snapshot 変更時は docs/14 の versioning policy 更新を要求する。
- `cargo test -p ts2wasm-runtime-abi`
- `grep -R "abi_layout_golden_snapshot" crates/runtime-abi docs/14-runtime-abi.md`

Non-goals:

- f64 / NaN / Infinity representation を新規実装しない。
- multi-limb BigInt full arithmetic を実装しない。

Plan files:

- `crates/runtime-abi/src/layout.rs`
- `crates/runtime-abi/src/value.rs`
- `crates/runtime-abi/src/consts.rs`
- `docs/14-runtime-abi.md`

---

### Issue 379 — Add RuntimeFn/link-plan transitive dependency property tests

| Field | Value |
|---|---|
| priority | P1 |
| type | test |
| area | runtime-catalog/backend |
| roadmap | P15-link-plan |
| depends_on | 378 |

Goal: `RuntimeFn` の spec、transitive deps、emission order、manifest name、host imports、capability markers が互いに矛盾しないことを property/contract test にする。

Acceptance:

- すべての `RuntimeFn` について、spec entry、manifest name、domain、emission_order、dependency closure が存在する。
- dependency closure に cycle がない。
- Host import を要求する RuntimeFn は capability marker を持つ。
- capability marker を持つ RuntimeFn は manifest に反映される。
- `cargo test -p ts2wasm-runtime-catalog`
- `cargo nextest run -p ts2wasm-backend-wasm --test runtime_link_plan`
- `cargo nextest run -p ts2wasm-backend-wasm --test host_import_capability`
- `python3 scripts/manager.py check runtimefn`

Non-goals:

- RuntimeFn を追加実装しない。
- WAT implementation の中身の意味論はこの issue では検証しない。

Plan files:

- `crates/runtime-catalog/src/**`
- `crates/backend-wasm/tests/runtime_link_plan.rs`
- `crates/backend-wasm/tests/host_import_capability.rs`
- `scripts/check/runtimefn-invariants.py`

---

### Issue 380 — Add manifest-vs-wasm import equality gate

| Field | Value |
|---|---|
| priority | P1 |
| type | test/tooling |
| area | backend/scripts |
| roadmap | P15-capability |
| depends_on | 379 |

Goal: `--emit-manifest` が宣言する imports/capabilities と、実際に生成された wasm binary の import section が完全に一致することを gate 化する。standalone なのに hidden `host.*` が入る事故を防ぐ。

Acceptance:

- representative fixture set で wasm import section を抽出し、manifest の `wasi` / `node_host.imports` と照合する。
- manifest にあるが wasm にない import、wasm にあるが manifest にない import の両方を failure にする。
- `scripts/check/manifest-imports.py` が fixture catalog を参照できる。
- `python3 scripts/manager.py check manifest`
- `cargo nextest run -p ts2wasm-compiler --test manifest_snapshot`
- `grep -R "Manifest" crates/compiler/tests crates/backend-wasm/tests scripts/check/manifest-imports.py`

Non-goals:

- manifest schema version の破壊的変更はしない。
- Node host shim implementation は変更しない。

Plan files:

- `scripts/check/manifest-imports.py`
- `crates/compiler/tests/manifest_snapshot.rs`
- `crates/backend-wasm/tests/host_import_capability.rs`
- `docs/11-shared-definitions.md`

---

### Issue 381 — Generate host-deny matrix from fixture catalog and manifest claims

| Field | Value |
|---|---|
| priority | P1 |
| type | test/tooling |
| area | scripts/fixtures |
| roadmap | P15-capability |
| depends_on | 372, 380 |

Goal: standalone 対象 fixture の host-deny リストを手動配列ではなく、fixture catalog の `standalone: true` / `host_deny: true` claim から生成する。

Acceptance:

- `scripts/check/host-deny.py` が fixture catalog を読み、standalone claim のある fixture を検査する。
- hidden `(import "host" ...)` を検出する。
- WASI imports は manifest に沿って許可される。
- `TS2WASM_HOST_FREE_FIXTURES` など既存 override の扱いを明文化する。
- `python3 scripts/manager.py check host`
- `cargo nextest run -p ts2wasm-cli --test m11_host_deny`

Non-goals:

- host-required fixture を standalone に変えない。
- host API のセキュリティモデル全体の再設計はしない。

Plan files:

- `scripts/check/host-deny.py`
- `fixtures/catalog.*`
- `crates/cli/tests/m11_host_deny.rs`
- `docs/09-security-and-capability-model.md`

---

### Issue 382 — Promote wasm validation into a representative gate matrix

| Field | Value |
|---|---|
| priority | P1 |
| type | test/tooling |
| area | backend/scripts |
| roadmap | P15-backend |
| depends_on | 372, 380 |

Goal: `wasm-tools validate` を 3 fixture の smoke ではなく、backend risk を代表する matrix に拡張する。WAT/binary emission が構造的に壊れたら differential 前に検出する。

Acceptance:

- fixture catalog に `wasm_validate: true` claim を追加できる。
- matrix includes: empty/minimal、console.log、function call、loop、if/try、object、array、string、module、host-required fixture の代表。
- wasm validation failure は build failure と区別して report される。
- `python3 scripts/manager.py check wasm`
- `cargo nextest run -p ts2wasm-backend-wasm`

Non-goals:

- すべての fixture を wasm validation 対象にしない。
- wasm-encoder backend を default にしない。

Plan files:

- `scripts/check/wasm-validation.py`
- `fixtures/catalog.*`
- `crates/backend-wasm/**`
- `docs/06-testing-and-coverage.md`

---

### Issue 383 — Pin deterministic reference coverage shards and replay sets

| Field | Value |
|---|---|
| priority | P2 |
| type | test/tooling |
| area | scripts/coverage |
| roadmap | P15-coverage |
| depends_on | 371 |

Goal: reference coverage を毎回違うサンプルで揺らさず、semantic canary / parser canary / negative canary の replayable shard を定義する。

Acceptance:

- `scripts/data/test262-semantic-core-seeds.txt` または追加 paths-file が deterministic replay set として使われる。
- `--paths-file` / `--path-filter` / `--sample` の優先順位と seed policy を docs に明記する。
- `python3 scripts/manager.py reference-coverage test262 --jobs 1 --paths-file scripts/data/test262-semantic-core-seeds.txt --jsonl`
- `python3 scripts/manager.py reference-coverage test262 --jobs 1 --path-filter language/statements --json`
- selected subset は canonical ramp 行を置換しない、という docs/15 のルールを維持する。

Non-goals:

- full test262 を毎 PR で走らせない。
- selected subset の pass 率を全体 conformance として公表しない。

Plan files:

- `scripts/data/test262-semantic-core-seeds.txt`
- `scripts/run/reference-coverage.py`
- `docs/15-coverage-matrix.md`
- `docs/current-state.md`

---

### Issue 384 — Add reference coverage regression gate for build/semantic/fail deltas

| Field | Value |
|---|---|
| priority | P1 |
| type | tooling |
| area | scripts/coverage |
| roadmap | P15-coverage |
| depends_on | 383 |

Goal: reference coverage の `executed`, `build_pass`, `semantic_pass`, `fail`, `unsupported`, `blocked` の delta を gate 化し、semantic_pass 減少や fail 増加を自動検出する。

Acceptance:

- `scripts/gate/coverage.py` が current matrix / baseline matrix を安定して比較する。
- fail 増加、semantic_pass 減少、build_pass 減少、executed 減少を fail にする。
- selected subset の replay result も JSONL schema checker を通る。
- `python3 scripts/manager.py check coverage <baseline> <current>` の使い方を docs 化する。
- `python3 scripts/manager.py update-coverage-matrix -- --check`
- `python3 scripts/manager.py reference-coverage test262 --jobs 1 --path-filter language/statements --jsonl | python3 scripts/manager.py check records -`

Non-goals:

- coverage 数字の改善をこの issue で要求しない。
- unsupported を pass として扱わない。

Plan files:

- `scripts/gate/coverage.py`
- `scripts/gen/coverage-matrix.py`
- `scripts/run/reference-coverage.py`
- `docs/15-coverage-matrix.md`

---

### Issue 385 — Add semantic core canary suite from test262 and project fixtures

| Field | Value |
|---|---|
| priority | P1 |
| type | test |
| area | coverage/fixtures |
| roadmap | P15-canary |
| depends_on | 372, 383 |

Goal: `null/undefined`, equality, truthiness, numeric edge, string/array/object core, completion records, exceptions, host boundary など、堅牢性に効く意味論を小さな canary suite として固定する。

Acceptance:

- `scripts/data/test262-semantic-core-seeds.txt` に分類済み seeds を持つ。
- project fixtures catalog に semantic core の代表 fixture が `semantic_diff` として明示される。
- canary suite は Node/iwasm differential と reference coverage runner の両方で再生できる。
- `python3 scripts/manager.py reference-coverage test262 --jobs 1 --paths-file scripts/data/test262-semantic-core-seeds.txt --jsonl | python3 scripts/manager.py check records -`
- `python3 scripts/manager.py check differential -- --jsonl --sample 25`

Non-goals:

- test262 90% をこの issue で目指さない。
- Promise/Proxy/Intl など deferred area を canary 必須にはしない。

Plan files:

- `scripts/data/test262-semantic-core-seeds.txt`
- `fixtures/catalog.*`
- `docs/06-testing-and-coverage.md`
- `docs/current-state.md`

---

### Issue 386 — Add unsupported/tracking ledger integrity gate

| Field | Value |
|---|---|
| priority | P1 |
| type | tooling |
| area | scripts/docs |
| roadmap | P15-tracking |
| depends_on | 371, 372, 384 |

Goal: unsupported / blocked / skip-with-reason が tracking なしで増えることを禁止する。diagnostic message、JSONL record、fixture catalog、issue tracker の不一致を検出する。

Acceptance:

- `scripts/check/tracking-consistency.py` が TestRecord JSONL、fixture catalog、diagnostic `issue-NNN`、docs の tracking refs を検査する。
- `unsupported` / `blocked` / `skip-with-reason` は reason + tracking 必須。
- `issue-NNN` は open/done tracker のどちらかに存在する。
- stale `issue-NNN` が存在する場合は allowlist + reason を要求する。
- `python3 scripts/manager.py check tracking`
- `python3 scripts/manager.py check records -- --self-test`
- `rg "skip\(|ignore =" crates scripts docs` の棚卸し結果が docs に記録される。

Non-goals:

- issue YAML をこの issue で自動生成しない。
- unsupported 件数を減らすことは目的ではない。

Plan files:

- `scripts/check/tracking-consistency.py`
- `fixtures/catalog.*`
- `docs/done-tracking.yaml`
- `docs/17-jsonl-test-record-schema.md`

---

### Issue 387 — Add flaky test detector and quarantine policy

| Field | Value |
|---|---|
| priority | P2 |
| type | tooling |
| area | scripts/process |
| roadmap | P15-flaky |
| depends_on | 374, 385 |

Goal: 同じ commit / 同じ fixture set で結果が揺れる test を検出し、quarantine する場合にも reason / owner / expiry を要求する。

Acceptance:

- `scripts/check/flaky-detect.py` か manager command を追加し、指定 test を N 回実行して status drift を検出する。
- default target: differential smoke, semantic core canary, manifest/import check, wasm validation。
- quarantine file は `path`, `test`, `reason`, `tracking`, `expires`, `owner` を持つ。
- quarantine された test は pass として扱わず、report に分離される。
- `python3 scripts/manager.py check flaky -- --runs 3 --suite semantic-core`
- `python3 scripts/manager.py check tracking`

Non-goals:

- flaky を自動修正しない。
- すべての long-running test を毎 PR で N 回走らせない。

Plan files:

- `scripts/check/flaky-detect.py`
- `scripts/manager.py`
- `docs/06-testing-and-coverage.md`
- `fixtures/quarantine.*`

---

### Issue 388 — Add performance smoke regression gate for runtime/compiler hot paths

| Field | Value |
|---|---|
| priority | P2 |
| type | tooling |
| area | scripts/perf |
| roadmap | P15-performance |
| depends_on | 374, 382 |

Goal: correctness-preserving の範囲で、compiler throughput、wasm size、runtime hot paths の粗い regression を検出する。最初は厳密な benchmark ではなく smoke gate として設計する。

Acceptance:

- `scripts/perf/benchmark-tracker.py` の output schema を docs/11 benchmark policy と一致させる。
- benchmark targets: parser large input, lowering representative fixture, backend emission, iwasm execution for semantic core。
- metrics: duration, p95, wasm size, host import count, fixture count。
- threshold は最初は warning、P15 final gate で error 化対象を限定する。
- `python3 scripts/manager.py benchmark-tracker -- --json`
- `python3 scripts/manager.py repo-metrics`

Non-goals:

- W8 optimization を始めない。
- microbenchmark の絶対性能を外部比較しない。

Plan files:

- `scripts/perf/benchmark-tracker.py`
- `scripts/run/repo-metrics.py`
- `docs/11-shared-definitions.md`
- `docs/07-performance-and-optimization.md`

---

### Issue 389 — P15 robust test completion gate

| Field | Value |
|---|---|
| priority | P1 |
| type | tooling |
| area | cross |
| roadmap | P15-final-gate |
| depends_on | 370-388 |

Goal: P15 の最終 roll-up gate。schema、fixture catalog、boundary snapshots、negative diagnostics、RuntimeFn/link-plan、manifest/import equality、host-deny、wasm validation、differential smoke、semantic canary、coverage regression、tracking、flaky policy、perf smoke が一貫して通ることを確認する。

Acceptance:

- `cargo test --workspace`
- `cargo nextest run --workspace`
- `python3 scripts/manager.py check architecture`
- `python3 scripts/manager.py check records -- --self-test`
- `python3 scripts/manager.py check fixtures`
- `python3 scripts/manager.py check diagnostics`
- `python3 scripts/manager.py check manifest`
- `python3 scripts/manager.py check host`
- `python3 scripts/manager.py check wasm`
- `python3 scripts/manager.py check differential -- --jsonl --sample 25`
- `python3 scripts/manager.py reference-coverage test262 --jobs 1 --paths-file scripts/data/test262-semantic-core-seeds.txt --jsonl | python3 scripts/manager.py check records -`
- `python3 scripts/manager.py update-coverage-matrix -- --check`
- `python3 scripts/manager.py check tracking`
- `python3 scripts/manager.py benchmark-tracker -- --json`
- P15 docs に local / PR / nightly / release gate の使い分けが記載されている。

Non-goals:

- No requirement for full ECMAScript compliance.
- No requirement that every test262 test passes.
- No requirement that wasm-encoder is the default backend.
- No requirement to make all perf warnings hard failures.

Plan files:

- `crates/**`
- `fixtures/**`
- `scripts/**`
- `docs/**`

---

## 6. Proposed gate tiers after P15

| Tier | Intended use | Commands |
|---|---|---|
| Local fast | 開発中の最短 feedback | `cargo fmt --all --check`, targeted `cargo test`, `python3 scripts/manager.py check architecture` |
| PR standard | 通常 PR gate | `cargo nextest run --workspace`, schema/catalog/manifest/host/wasm, differential smoke sample |
| Nightly | 重い回帰検出 | full fixture JSONL sweep, reference coverage replay sets, flaky detector, perf smoke |
| Release | 公開前安全確認 | P15 final gate + current coverage matrix + rollback/manifest docs check |

---

## 7. YAML 化時の注意

- この doc は issue YAML ではない。YAML 化時は `id`, `title`, `priority`, `type`, `area`, `status`, `created`, `updated`, `roadmap`, `depends_on`, `acceptance`, `non_goals`, `plan`, `notes` に展開する。
- `created` / `updated` は `2026-05-12` を初期値にできる。
- `depends_on: 370-388` のような範囲表記は YAML では展開する。
- `issue-NNN` tracking と issue id の衝突は 370 を最初に解決してから YAML 化する。
- acceptance command は repo root での実行を前提にする。archive snapshot には root `Cargo.toml` / `fixtures/` が含まれていないため、実 repo で最終検証する。

---

## 8. Success criteria

P15 完了時点で満たすべき状態:

1. TestRecord schema が Rust / docs / scripts / runner で一致している。
2. Fixture ごとの test class と semantic claim が機械可読である。
3. 新しい AST/HIR/MIR/Lowered/RuntimeFn/HostImport variant が未テストのまま入らない。
4. unsupported / blocked / skip-with-reason が必ず reason + tracking を持つ。
5. manifest と wasm import section が一致する。
6. standalone claim の fixture が hidden host import を持たない。
7. representative wasm validation matrix が通る。
8. deterministic semantic canary と reference replay set が存在する。
9. semantic_pass 減少と fail 増加が gate で止まる。
10. flaky / quarantine / perf smoke の運用ポリシーが docs と scripts に反映されている。

最終的な設計スローガン:

```text
小さい構造テストで境界を守る。
少数精鋭の differential で意味論を守る。
manifest / wasm / host-deny で capability boundary を守る。
reference coverage と flaky detection で長期的な信頼性を守る。
```
