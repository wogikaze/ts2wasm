# Next Architecture Design for ts2wasm

## Executive Summary

この設計書は、`ts2wasm` の次実装 wave を **Requirement → Task → Acceptance → Gate** に閉じるための実装契約である。対象の中心は **TypeScript Frontend Boundary Design** だが、repository の現状では frontend 境界は coverage triage、host capability、runtime ABI、reference corpus の再現性と分離して達成できない。そのため、本書は要求された 6 theme をすべて扱う。

現状の repository から確認できる主要事実は次である。

- project goal は、Node.js runtime への依存ではなく、TS/JS source を WASM に変換し、WASI/iwasm 上で動かし、Node.js oracle との差分で意味論を確認することである。これは `README.md`、`docs/04-compiler-architecture-and-runtime.md`、`docs/11-shared-definitions.md`、`current-state.md` に一致する。
- `crates/frontend` は lexer/parser/AST/diagnostic/TypeScript oracle を持つ。frontend crate 自体は `ir` / `backend` / `compiler` に依存していないが、`typescript_oracle` が同一 crate に残り、parser smoke と semantic/build smoke の境界は fixture/category と coverage runner 上でまだ完全に閉じていない。
- coverage は `scripts/run/reference-coverage.py`、`scripts/gate/coverage.py`、`scripts/gen/coverage-matrix.py`、`artifacts/coverage/reference-coverage-matrix.md` に実装がある。`build_pass` と `semantic_pass` は分離され、test262 の現行 matrix は `denominator=53469`、`build_coverage%=20.50`、`semantic_coverage%=11.73` を記録する。ただし、失敗を stable な outcome taxonomy と top-N reason report に固定する契約はまだ存在しない。
- capability manifest は `docs/11-shared-definitions.md`、`crates/shared/src/capability.rs`、`crates/runtime-catalog`、`crates/backend-wasm/src/capability_manifest.rs`、`scripts/check/manifest-imports.py` に存在する。manifest と wasm import の比較はあるが、`scripts/check/host-deny.py` は現在 info-only と自称しており、strict gate になっていない。
- runtime/GC は linear memory、RawValue、heap header、GC root table/call-frame roots、allocation pressure fixtures が存在する。だが root category の coverage contract と ABI snapshot coverage はまだ十分に task 化されていない。
- `reference/` は README のみで、test262 / TypeScript / typescript-go の checkout と commit lock が archive 内にない。coverage artifact は存在するが、reference corpus の固定・選択条件・lock hash を JSONL/report に埋める契約は未完成である。
- runtime ABI crate は `crates/runtime-abi` に分離され、compat snapshots もある。生成 WASM に runtime ABI metadata を埋め、互換性チェックで検証する契約はまだない。

次 wave の最小完成状態は次である。

1. `reference-coverage` が `CoverageOutcome` を stable に出力し、JSONL / JSON / Markdown の triage artifact が再現可能である。
2. Node host import は `CapabilityManifest` と `runtime-catalog` の宣言を通らない限り生成できず、standalone gate で未宣言 host import が失敗する。
3. `crates/frontend` は syntax-only boundary と TypeScript erasure boundary を明示し、parser smoke は build/semantic pass と混同されない。
4. GC root と runtime ABI layout は snapshot / fixture / differential test で保護される。
5. reference corpus は `reference/lock.json` と selection metadata によって再現できる。
6. 生成 WASM は `ts2wasm.abi` custom section に runtime ABI version と target profile を持つ。

## Repository Findings

### 現在できていること

- `cargo` workspace は `crates/cli`、`shared`、`source`、`diagnostic`、`syntax`、`resolve`、`semantics`、`frontend`、`ir`、`runtime-abi`、`runtime-catalog`、`backend-core`、`backend-wasm`、`compiler` を含む。`Cargo.toml` は edition 2024 を共有する。
- CLI は `build`、`check`、`server`、`dump` を持つ。`build` は `--emit-manifest` / `--emit-capabilities`、`--host-deny`、`--experimental-hir-mir`、`--experimental-hir-mir-compat-fallback` を受け付ける。
- compiler pipeline は `io::read_source` → test262 include preprocessor → type reference directive validation → lexer/parser → AST validator → module graph → static import lowering → name resolver → builtin resolver → semantic validator → legacy `LoweredProgram` or opt-in HIR/MIR → runtime gate → backend WAT/WASM emission という構成である。
- frontend parser は多数の ECMAScript/TypeScript syntax を AST として受け、`crates/frontend/tests/parser_snapshot.rs`、`crates/frontend/tests/parser_property.rs`、`crates/cli/tests/parser_ast_structures.rs`、`crates/cli/tests/parser_keywords.rs` で parser smoke を持つ。
- TypeScript erasure fixtures は `fixtures/basics-types` にあり、ambient declaration、interface、type alias、generic、satisfies、as assertion、const assertion などの erasure が catalog 上 `type-erasure` として記録されている。
- capability manifest は schema v1 を持ち、`standalone`、`wasi`、`node_host.imports`、`capability_reasons` を表現する。`scripts/check/manifest-imports.py` は manifest と wasm import の一致、manifest deterministic snapshot、expected build-fail fixtures を検査する。
- coverage runner は `test262`、`tsc`、`tsgo` を扱い、`--jsonl`、`--jobs`、`--sample`、`--category`、`--no-server`、`--no-semantic`、`--check-prerequisites` を持つ。
- `artifacts/coverage/reference-coverage-matrix.md` は generated source of truth として suite denominator、executed、build/semantic coverage、unsupported diagnostic code、unsupported feature を記録する。
- `crates/runtime-abi` は RawValue / ValueTag / Layout / compat snapshots を持ち、`crates/runtime-abi/tests/abi_invariants.rs` が tag/layout invariant を確認する。
- backend は `crates/backend-wasm/src/emitter/gc_roots.rs`、`runtime/core/memory.rs`、`runtime_*` modules、`runtime_link_plan.rs`、`capability_manifest.rs` などに分割され、GC root table、call-frame roots、backend temp roots、manifest generation が存在する。
- issue tracker は `issues/` と manager scripts で管理される。active issue には coverage expansion、host Node shim、WASI filesystem、runtime builtin、typed wasm encoder parity、test262 real harness loading block が存在する。

### 現在できていないこと

- `reference-coverage` は `build_pass` / `semantic_pass` を集計できるが、`semantic_mismatch`、`runtime_error`、`build_only`、`verified_negative_compile`、`negative_compile_mismatch` などを **stable enum** として出力する契約がない。
- triage artifact は matrix と JSONL に散在しており、top-N reason を stable JSON schema と Markdown で出す CLI option がない。
- `scripts/check/host-deny.py` は source regex と optional compile check を持つが、docstring が示す通り exit 0 の info-only gate である。未宣言 host import を strict に拒否する compile/build gate ではない。
- `crates/frontend` 内に `typescript_oracle` が存在し、production build と oracle usage の境界を script/architecture gate で検証していない。
- parser-only acceptance と semantic/build acceptance が fixture catalog、manager command、coverage record 上で完全には分離されていない。
- TypeScript erasure の「parse-only erased syntax」と「runtime-bearing TS syntax」の境界が `ErasureReport` のような実装 API として固定されていない。
- archive 内の `reference/` は README のみで、参照 corpus の commit lock と local path prerequisite がない。
- coverage JSONL/report には reference lock hash、selection seed、paths-file hash、category sampling condition が必須 metadata として入っていない。
- 生成 WASM に runtime ABI version / target profile / manifest schema version を埋める custom section がない。
- Runtime object/GC は複数 fixture と partial GC 実装があるが、root category と allocation path coverage が spec/test matrix として閉じていない。

### 中途半端に存在するもの

- HIR/MIR は opt-in と compat fallback があり、`docs/current-state.md` は P16 default gate が no-go と明記する。legacy lowering が主経路である。
- `host-deny` は fixture catalog を読むが、source-level regex が WASI-compatible idiom と Node-host-required idiom を混同し得る。compile check も strict failure ではない。
- `reference-coverage` は negative compile を検出し、Node oracle で verification する code path があるが、status と outcome が script 内に散在している。
- `docs/language-reference/typescript-features.md` など一部 language reference は現実より古い記述を含み得る。現状判断では `current-state.md`、`docs/current-state.md`、coverage artifact、fixtures/tests を優先する。
- runtime ABI は crate と snapshots を持つが、WASM artifact 内 metadata と target evolution policy は実装で固定されていない。
- `reference/README.md` は参照プロジェクトの URL を列挙するが、clone commit、denominator、local path の機械可読 lock ではない。

### 壊してはいけない前提

- `mise run check` / `python scripts/manager.py check` は lightweight smoke のまま維持する。reference corpus がない環境で default check を重くしてはならない。
- `cargo fmt --all --check` と `cargo nextest run` は Gate A の基本であり、既存 tests を regression させてはならない。
- `artifacts/coverage/reference-coverage-matrix.md` は generated artifact であり、手編集で coverage を主張してはならない。
- `build_pass` は conformance pass ではない。semantic compatibility は Node/iwasm differential または verified negative compile でのみ主張する。
- `crates/frontend` は `ts2wasm-ir`、`ts2wasm-backend-wasm`、`ts2wasm-compiler` に依存してはならない。
- production build pipeline は Node TypeScript compiler API を必須 dependency にしてはならない。oracle は `ts2wasm check`、triage、test/diff 用である。
- `CapabilityManifest` schema v1 と `ts2wasm_shared::capability::SCHEMA_VERSION` は migration policy に従って bump する。unknown host import を silent allow してはならない。
- `RawValue` / `ValueTag` / layout constants の変更は ABI snapshot と tests を通さなければならない。
- Node.js runtime を WASM 内に埋め込む設計に転換してはならない。
- Wasm GC / Component Model は target evolution として扱い、default backend を即時全面移行してはならない。

### 既存コード上の主要ファイル

- `crates/cli/src/main.rs`
- `crates/compiler/src/pipeline.rs`
- `crates/compiler/src/dump.rs`
- `crates/compiler/src/stages/parse.rs`
- `crates/compiler/src/stages/validate.rs`
- `crates/compiler/src/stages/runtime_gate.rs`
- `crates/frontend/src/lib.rs`
- `crates/frontend/src/parser.rs`
- `crates/frontend/src/lexer.rs`
- `crates/frontend/src/type_reference_directive.rs`
- `crates/frontend/src/typescript_oracle.rs`
- `crates/syntax/src/ast.rs`
- `crates/ir/src/lowered.rs`
- `crates/ir/src/semantic.rs`
- `crates/runtime-abi/src/value.rs`
- `crates/runtime-abi/src/layout.rs`
- `crates/runtime-catalog/src/capability.rs`
- `crates/runtime-catalog/src/host_import.rs`
- `crates/runtime-catalog/src/link_plan.rs`
- `crates/backend-wasm/src/capability_manifest.rs`
- `crates/backend-wasm/src/runtime_link_plan.rs`
- `crates/backend-wasm/src/runtime_fn.rs`
- `crates/backend-wasm/src/emitter/gc_roots.rs`
- `crates/backend-wasm/src/wasm_encoder_backend.rs`
- `scripts/manager.py`
- `scripts/run/reference-coverage.py`
- `scripts/gate/coverage.py`
- `scripts/gen/coverage-matrix.py`
- `scripts/check/manifest-imports.py`
- `scripts/check/host-deny.py`
- `scripts/check/architecture-rules.py`
- `scripts/check/test-records-schema.py`
- `scripts/dev/link-reference.py`

### 既存テスト上の主要ファイル

- `crates/frontend/tests/parser_snapshot.rs`
- `crates/frontend/tests/parser_property.rs`
- `crates/cli/tests/parser_ast_structures.rs`
- `crates/cli/tests/parser_keywords.rs`
- `crates/cli/tests/type_reference_directives.rs`
- `crates/cli/tests/differential_jsonl.rs`
- `crates/cli/tests/m1_iwasm.rs`
- `crates/cli/tests/m2_node_diff.rs`
- `crates/cli/tests/m11_host_deny.rs`
- `crates/backend-wasm/tests/host_import_capability.rs`
- `crates/backend-wasm/tests/manifest_snapshot_equality.rs`
- `crates/backend-wasm/tests/runtime_link_plan.rs`
- `crates/backend-wasm/tests/runtime_signature.rs`
- `crates/backend-wasm/tests/runtime_intrinsic_mapping.rs`
- `crates/runtime-catalog/tests/capability_registry.rs`
- `crates/runtime-catalog/tests/runtime_registry.rs`
- `crates/runtime-catalog/tests/link_plan_structural.rs`
- `crates/runtime-abi/tests/abi_invariants.rs`
- `fixtures/catalog.yaml`
- `fixtures/basics-types/*.ts`
- `fixtures/core-semantics/gc-*.ts`
- `fixtures/core-semantics/ordinary-function-closure-*.ts`
- `scripts/data/semantic-canary.txt`
- `rule-tests/no-dbg-macro-test.yml`

### 関連する既存 issue / docs / plans

- Docs: `README.md`, `current-state.md`, `docs/current-state.md`, `docs/03-api-and-host-capability.md`, `docs/04-compiler-architecture-and-runtime.md`, `docs/05-compatibility-and-semantics.md`, `docs/06-testing-and-coverage.md`, `docs/09-security-and-capability-model.md`, `docs/11-shared-definitions.md`, `docs/12-coding-standard.md`, `docs/13-ir-contracts.md`, `docs/14-runtime-abi.md`, `docs/15-coverage-matrix.md`, `docs/17-jsonl-test-record-schema.md`, `docs/23-coverage-runner-completeness.md`, `docs/24-architecture-decoupling-and-llm-friendly-sizing.md`, `docs/26-semantic-feature-matrix.md`, `docs/27-coverage-expansion-plan.md`, `docs/27-ir-layer-completion-plan.md`, `docs/language-reference/frontend-parser-wave.md`, `docs/language-reference/typescript-features.md`.
- Coverage artifact: `artifacts/coverage/reference-coverage-matrix.md`.
- Plans: `plans/457-harness-compiler-gaps.md`, `plans/5000-parser-syntax-coverage.md`, `plans/5004-runtime-builtins-coverage.md`, `.agents/plans/5029-direct-binary-emission.md`, `.agents/plans/5030-split-runtime-emitters.md`, `.agents/plans/5032-capability-detection.md`, `.agents/plans/5041-expr-fixture-coverage.md`, `.agents/plans/5043-split-parser.md`, `.agents/plans/5044-ambient-erasure.md`, `.agents/plans/5052-abi-memory-map.md`.
- Active issues observed through `scripts/issue-index.py`: `I-20260513-HRH6X9`, `I-20260513-47XE8P`, `I-20260513-BAKAE9`, `I-20260513-E7X3MQ`, `I-20260513-EMRHSY`, `I-20260513-HDW7PQ`, `I-20260513-W9X2Z8`, `I-20260513-5PGJNN`, `I-20260513-WHBN24`, `I-20260513-XJSRR2`, `I-20260513-C86NV6`, `I-20260512-WAENCD`, `I-20260513-HGGTXF`.

## Design Principles

1. **Frontend is syntax and parse-time erasure only**: `crates/frontend` may tokenize, parse, preserve spans, reject invalid syntax, and report TypeScript parse/erase facts. It must not resolve names, choose runtime helper functions, choose host capabilities, or make backend decisions.
2. **Coverage claims require outcome evidence**: build success, semantic match, runtime mismatch, negative compile verification, unsupported, and blocked are separate outcomes. Coverage tooling must not collapse them into one status.
3. **Capabilities are declared before emitted**: every wasm import pair must be derivable from a `RuntimeFn` / `HostImport` / `Capability` entry and must be present in `CapabilityManifest`. Unknown `host.*` import is a build/gate failure.
4. **Reference runs are reproducible or explicitly blocked**: a reference coverage run must record corpus lock hash and selection metadata. If reference corpus is absent, prerequisite commands must fail with a deterministic message, not silently change denominator.
5. **Runtime ABI is versioned at artifact boundary**: layout constants and generated WASM metadata must agree. ABI changes require snapshot updates and a visible version delta.
6. **Default remains conservative**: HIR/MIR, Wasm GC, Component Model, Node host shim, unsafe-fast, and full JS engine embedding stay opt-in or non-goals for this wave.
7. **Existing supported behavior is protected first**: each task includes regression commands for current parser tests, manifest tests, differential fixtures, or ABI tests before expanding behavior.

## Non-goals

- JavaScript 全仕様の完全実装はこの設計書のゴールではない。
- TypeScript checker の完全再実装はこの設計書のゴールではない。
- Node.js runtime を WASM 内に埋め込まない。
- QuickJS / Javy のような full JS engine に委譲しない。
- Node.js 完全互換 host shim をこの wave で完成させない。
- `build_pass` を semantic conformance として扱わない。
- 既存 supported subset を壊して coverage だけ増やすことはしない。
- Wasm GC / Component Model に即時全面移行しない。
- reference corpus が存在しない環境で full test262 を default gate にしない。
- host-deny を source regex だけで判定しない。最終判定は generated wasm import と manifest に基づく。
- ABI layout constant を silent に変更しない。
- parser smoke fixture を runtime semantic pass として扱わない。

## Current Architecture Map

```text
TS/JS source
  ├─ test262 include preprocessor              crates/compiler/src/test262_preprocessor.rs
  ├─ type reference directive validation       crates/frontend/src/type_reference_directive.rs
  ├─ lexer/parser                              crates/frontend/src/lexer.rs, parser.rs
  ├─ syntax AST                                crates/syntax/src/ast.rs
  ├─ AST validator                             crates/compiler/src/stages/parse.rs
  ├─ module graph / static imports             crates/compiler/src/module_graph.rs, stages/static_imports.rs
  ├─ name resolver                             crates/compiler/src/stages/name_resolve.rs
  ├─ builtin resolver                          crates/compiler/src/stages/builtin_resolve.rs
  ├─ semantic validator                        crates/compiler/src/stages/semantic_validate.rs
  ├─ lowering                                  crates/ir/src/lowered.rs or crates/ir/src/semantic.rs
  ├─ runtime gate                              crates/compiler/src/stages/runtime_gate.rs
  ├─ runtime link plan                         crates/backend-wasm/src/runtime_link_plan.rs
  ├─ capability manifest                       crates/backend-wasm/src/capability_manifest.rs
  ├─ WAT / wasm backend                        crates/backend-wasm/src/*.rs
  └─ output wasm + optional manifest           crates/compiler/src/io/*.rs
```

### 現在できていること

- single-file and narrow module graph flow exists.
- `BuildPipelineOptions` controls `host_deny` and HIR/MIR mode.
- `validate_host_deny` rejects `backend::has_node_host_imports(lowered)` when `host_deny` is true.
- manifest generation happens from validated runtime link plan before WAT emission in legacy path.
- HIR/MIR path exists but is opt-in and not the default.
- CLI `dump` exposes tokens/AST/resolved/TIR/lowered/WAT/unparse flows.

### 現在できていないこと

- pipeline output does not include ABI metadata custom section by default.
- `host_deny` option value is accepted as a CLI string but effectively becomes boolean presence.
- frontend boundary is not encoded as a repository architecture rule beyond dependency shape.
- parser-only evidence is not first-class in coverage matrix.
- reference corpus lock is not part of pipeline or coverage metadata.

### 中途半端に存在するもの

- `crates/frontend/src/typescript_oracle.rs` is useful for `check` but makes frontend crate not purely compile-independent from Node in implementation terms.
- `runtime-catalog` owns many runtime function/capability facts, while backend still contains some string/capability logic.
- `scripts/check/architecture-rules.py` enforces size/coupling limits, but not frontend-specific forbidden dependency and forbidden symbol rules.

### 壊してはいけない前提

- Current legacy lowering remains default until HIR/MIR gate is explicitly green.
- `--emit-manifest` output must remain deterministic for existing manifest snapshot tests.
- `ts2wasm check` may continue to use TypeScript compiler oracle, but `ts2wasm build` must not require it.

### 既存コード上の主要ファイル

- `crates/compiler/src/pipeline.rs`
- `crates/compiler/src/stages/parse.rs`
- `crates/compiler/src/stages/runtime_gate.rs`
- `crates/frontend/src/lib.rs`
- `crates/frontend/src/typescript_oracle.rs`
- `crates/backend-wasm/src/runtime_link_plan.rs`
- `crates/backend-wasm/src/capability_manifest.rs`

### 既存テスト上の主要ファイル

- `crates/cli/tests/parser_ast_structures.rs`
- `crates/cli/tests/type_reference_directives.rs`
- `crates/backend-wasm/tests/manifest_snapshot_equality.rs`
- `crates/backend-wasm/tests/runtime_link_plan.rs`

### 関連する既存 issue / docs / plans

- `docs/04-compiler-architecture-and-runtime.md`
- `docs/13-ir-contracts.md`
- `docs/24-architecture-decoupling-and-llm-friendly-sizing.md`
- `.agents/plans/5043-split-parser.md`
- `.agents/plans/5029-direct-binary-emission.md`
- `.agents/plans/5030-split-runtime-emitters.md`
- `I-20260512-WAENCD`
- `I-20260513-HGGTXF`

## Theme 1: Coverage Strategy / test262 Triage

### 現在できていること

- `scripts/run/reference-coverage.py` runs `test262`, `tsc`, and `tsgo` with `--limit`, `--paths-file`, `--path-filter`, `--jsonl`, `--jobs`, `--sample`, `--category`, `--no-server`, `--no-semantic`, `--check-prerequisites`.
- `scripts/manager.py reference-coverage test262` sets audited defaults for semantic test262 coverage: `TS2WASM_TEST262_NODE_ORACLE=always` and `TS2WASM_DISABLE_TEST262_PREPROCESSOR_STUBS=1` unless `--no-semantic` is passed.
- `scripts/check/test-records-schema.py` validates canonical JSONL records and coverage-runner `build_pass` extension.
- `scripts/gate/coverage.py` and `scripts/gen/coverage-matrix.py` produce/check coverage metrics.
- `artifacts/coverage/reference-coverage-matrix.md` records suite-level build/semantic coverage and top unsupported diagnostic/feature counts.

### 現在できていないこと

- `CoverageOutcome` enum does not exist as a single source of truth.
- `semantic_mismatch` and `runtime_error` are not stable top-level report categories across JSONL, summary JSON, Markdown, and matrix.
- top-N failure reason report has no stable JSON schema.
- server batch and `--no-server` legacy path parity is not a gate for selected cases.
- frontend parser attribution is mixed with generic `UnsupportedSyntax` / `unknown-unsupported:parser` records.

### 中途半端に存在するもの

- negative compile classification functions exist in `reference-coverage.py`, but final record status/outcome naming is script-local.
- unsupported feature tables exist in matrix, but they are aggregated after the run and not a versioned triage artifact.
- `scripts/run/reference-triage.py` can inspect one case, but it does not replace stable suite-level triage artifact.

### 壊してはいけない前提

- `build_pass` must remain separate from semantic conformance.
- `unsupported` / `blocked` must retain `reason` and `tracking`.
- no reference corpus dependency may be added to default `python scripts/manager.py check`.
- coverage matrix remains generated and not hand-edited.

### 既存コード上の主要ファイル

- `scripts/run/reference-coverage.py`
- `scripts/gate/coverage.py`
- `scripts/gen/coverage-matrix.py`
- `scripts/check/test-records-schema.py`
- `scripts/manager.py`
- `crates/shared/src/test_status.rs`

### 既存テスト上の主要ファイル

- `crates/cli/tests/differential_jsonl.rs`
- `scripts/check/test-records-schema.py --self-test`
- `artifacts/coverage/reference-coverage-matrix.md`
- `scripts/data/semantic-canary.txt`

### 関連する既存 issue / docs / plans

- `docs/15-coverage-matrix.md`
- `docs/17-jsonl-test-record-schema.md`
- `docs/23-coverage-runner-completeness.md`
- `docs/26-semantic-feature-matrix.md`
- `docs/27-coverage-expansion-plan.md`
- `plans/457-harness-compiler-gaps.md`
- `plans/5000-parser-syntax-coverage.md`
- `I-20260513-WHBN24`
- `I-20260513-HDW7PQ`
- coverage expansion doing issues: `I-20260513-HRH6X9`, `I-20260513-47XE8P`, `I-20260513-BAKAE9`, `I-20260513-E7X3MQ`, `I-20260513-EMRHSY`

### Completion Criteria

Theme 1 is complete when:

- every JSONL record produced by `reference-coverage --jsonl` has a stable `outcome` field from `CoverageOutcome`;
- top-N triage JSON and Markdown can be emitted from the same run;
- `scripts/check/coverage-triage-schema.py` validates triage JSON;
- server and legacy runner paths produce identical `outcome` values for a fixed paths file;
- parser/frontend-related unsupported records are attributable to `parser-syntax`, `typescript-erasure`, `frontend-unsupported`, or `frontend-invalid-source` instead of generic unknown parser bucket.

### REQ-COV-001: Coverage outcome taxonomy

Priority: P0

Rationale:
`build_pass`, `semantic_pass`, `unsupported`, and `blocked` exist, but implementation work needs a stable taxonomy that distinguishes semantic mismatches, runtime errors, build-only successes, verified negative compile passes, and unverified negative tests. Without this, test262 triage cannot be decomposed into issues reliably.

Affected files:
- `scripts/run/reference-coverage.py`
- `scripts/lib/coverage_outcome.py` new
- `scripts/check/test-records-schema.py`
- `scripts/gate/coverage.py`
- `docs/17-jsonl-test-record-schema.md`
- `docs/23-coverage-runner-completeness.md`

Implementation outline:
1. Add `scripts/lib/coverage_outcome.py` with enum-like constants:
   - `semantic_match`
   - `semantic_mismatch`
   - `runtime_error`
   - `build_only`
   - `verified_negative_compile`
   - `negative_compile_mismatch`
   - `negative_runtime_unverified`
   - `unsupported`
   - `blocked`
   - `internal_failure`
2. Add function `classify_coverage_outcome(record: dict) -> str` and keep it pure: classification uses only record fields, not global counters.
3. Extend every `reference-coverage --jsonl` record with `outcome`, `phase`, `diagnostic_code`, `feature`, `tracking`, `semantic_checked`, `node_exit_status`, `iwasm_exit_status` when known.
4. Update `scripts/check/test-records-schema.py` to allow and validate `outcome` for coverage records. It must reject unknown outcomes.
5. Document the outcome table in `docs/17-jsonl-test-record-schema.md` and state that `status` remains backward-compatible while `outcome` is coverage-specific.

Tests:
- Add unit self-test cases in `scripts/check/test-records-schema.py --self-test` for all outcome values.
- Add Python unit test file `scripts/tests/test_coverage_outcome.py` or self-test mode in `scripts/lib/coverage_outcome.py` covering pass, mismatch, runtime error, build-only, verified negative, negative mismatch, unsupported, blocked, internal failure.
- Update `crates/cli/tests/differential_jsonl.rs` expected JSONL assertions to check `outcome` for at least one semantic match and one unsupported record.

Acceptance commands:
```bash
python scripts/lib/coverage_outcome.py --self-test
python scripts/manager.py check records -- --self-test
python scripts/manager.py reference-coverage test262 --jsonl --sample 10 --jobs 1 --no-dashboard-data > /tmp/ts2wasm-test262.jsonl
python scripts/manager.py check records -- /tmp/ts2wasm-test262.jsonl
```

Done definition:

- [ ] `CoverageOutcome` values are defined in one Python module and used by `reference-coverage.py`.
- [ ] Every JSONL record from `reference-coverage --jsonl` includes `outcome`.
- [ ] schema validation rejects unknown `outcome`.
- [ ] documentation lists all outcome values and their meaning.
- [ ] existing `status` field remains backward-compatible.

Depends on:

- none

Non-goals:

- Increasing test262 pass count.
- Rewriting the full coverage runner.
- Changing canonical `TestRecord.status` values in `crates/shared/src/test_status.rs`.

Risk:

- Existing scripts that parse JSONL may ignore unknown fields safely; scripts that assume exact field sets may need updates.

### REQ-COV-002: Stable triage JSON and Markdown reports

Priority: P0

Rationale:
A coding agent needs stable output to create issues. Matrix tables show aggregate counts but not a deterministic list of top reasons, sample cases, or exact phase attribution.

Affected files:
- `scripts/run/reference-coverage.py`
- `scripts/report/coverage_triage.py` new
- `scripts/check/coverage-triage-schema.py` new
- `scripts/manager.py`
- `mise.toml`
- `artifacts/coverage/README.md` new or update if present
- `docs/15-coverage-matrix.md`

Implementation outline:
1. Add CLI options to `reference-coverage.py`:
   - `--out-json PATH`
   - `--out-md PATH`
   - `--top-reasons N` default `20`
2. Create triage JSON schema:
   ```json
   {
     "schema_version": 1,
     "suite": "test262",
     "generated_at_utc": "2026-05-14T00:00:00Z",
     "command": "python scripts/manager.py reference-coverage test262 ...",
     "reference": {
       "lock_sha256": "...",
       "root": "reference",
       "suite_commit": "..."
     },
     "selection": {
       "mode": "sample|limit|paths-file|all",
       "limit": null,
       "sample": 10,
       "category": null,
       "paths_file_sha256": null
     },
     "summary": {
       "executed": 10,
       "outcomes": {"semantic_match": 3, "unsupported": 7}
     },
     "top_reasons": [
       {
         "rank": 1,
         "outcome": "unsupported",
         "phase": "parser",
         "diagnostic_code": "SyntaxError",
         "feature": "parser-syntax",
         "tracking": "feature:parser-syntax",
         "count": 7,
         "sample_cases": ["language/statements/..."]
       }
     ]
   }
   ```
3. Stable sort `top_reasons` by `count DESC`, then `outcome`, `phase`, `diagnostic_code`, `feature`, `tracking` lexicographically.
4. Markdown report must include command, reference lock, selection, outcome table, top reason table, and sample cases. It must not include full stdout/stderr logs.
5. Add manager command `coverage-triage-schema` or check alias `check coverage-triage`.

Tests:
- Add self-test fixture JSON in `scripts/check/coverage-triage-schema.py --self-test`.
- Add deterministic sorting test in `scripts/report/coverage_triage.py --self-test`.
- Run `reference-coverage test262 --sample 10` and validate both JSON and Markdown exist.

Acceptance commands:
```bash
python scripts/report/coverage_triage.py --self-test
python scripts/check/coverage-triage-schema.py --self-test
python scripts/manager.py reference-coverage test262 --jsonl --sample 10 --jobs 1 --no-dashboard-data --out-json /tmp/ts2wasm-triage.json --out-md /tmp/ts2wasm-triage.md --top-reasons 10 > /tmp/ts2wasm-test262.jsonl
python scripts/check/coverage-triage-schema.py /tmp/ts2wasm-triage.json
test -s /tmp/ts2wasm-triage.md
```

Done definition:

- [ ] `--out-json`, `--out-md`, `--top-reasons` are implemented.
- [ ] triage JSON schema validates with a repository script.
- [ ] Markdown report is deterministic for the same JSON input.
- [ ] top reasons include `outcome`, `phase`, `diagnostic_code`, `feature`, `tracking`, `count`, and `sample_cases`.
- [ ] generated artifacts do not change coverage matrix semantics.

Depends on:

- REQ-COV-001
- REQ-REF-002 for reference metadata fields; until REQ-REF-002 lands, fields may be `null` but schema keys must exist.

Non-goals:

- HTML dashboard redesign.
- Full failure minimization.
- New issue auto-creation.

Risk:

- Long test262 runs can create large reports; cap `sample_cases` to 5 per reason by default.

### REQ-COV-003: Server/legacy runner parity for outcome classification

Priority: P1

Rationale:
`reference-coverage.py` has server batch and `--no-server` paths. If they classify the same case differently, triage data becomes non-reproducible.

Affected files:
- `scripts/run/reference-coverage.py`
- `scripts/gate/coverage.py`
- `scripts/data/coverage-parity-test262.txt` new
- `docs/23-coverage-runner-completeness.md`

Implementation outline:
1. Add deterministic paths file `scripts/data/coverage-parity-test262.txt` with 20 cases covering semantic match, unsupported parser, unsupported name resolution, build-only, verified negative compile, blocked-like metadata, and runtime mismatch if available.
2. Add `scripts/gate/coverage.py --check-runner-parity PATHS_FILE` or a new `scripts/check/coverage-runner-parity.py`.
3. The parity check runs:
   - server mode: `reference-coverage test262 --jsonl --paths-file PATHS --jobs 1 --no-dashboard-data`
   - legacy mode: same plus `--no-server`
4. Compare `suite`, `case`, `status`, `outcome`, `diagnostic_code`, `feature`, and `tracking`. Timing fields are ignored.
5. Fail with a diff listing first 10 mismatched cases.

Tests:
- Add a self-test for comparing two small JSONL files with one intentional mismatch.
- Add parity check to `python scripts/manager.py check coverage-parity` alias, but do not include it in default lightweight check.

Acceptance commands:
```bash
python scripts/check/coverage-runner-parity.py --self-test
python scripts/check/coverage-runner-parity.py --suite test262 --paths-file scripts/data/coverage-parity-test262.txt
```

Done definition:

- [ ] fixed paths file exists and is documented.
- [ ] server and legacy paths produce equal `outcome` for fixed cases.
- [ ] mismatch output includes case path and differing fields.
- [ ] command is available through `scripts/manager.py check coverage-parity`.

Depends on:

- REQ-COV-001

Non-goals:

- Making `--no-server` performance equal to server mode.
- Full corpus parity on every CI run.

Risk:

- Some cases may be unavailable when reference corpus is absent; command must fail with deterministic prerequisite message from REQ-REF-001.

### REQ-COV-004: Frontend-attributed coverage reasons

Priority: P1

Rationale:
Current unsupported feature buckets include `unknown-unsupported:parser` and generic parser syntax. For TypeScript frontend boundary work, syntax parser, TypeScript erasure, invalid source, and later semantic unsupported must be distinguishable.

Affected files:
- `scripts/run/reference-coverage.py`
- `crates/frontend/src/diagnostic.rs`
- `crates/compiler/src/stages/parse.rs`
- `crates/compiler/src/stages/semantic_validate.rs`
- `docs/15-coverage-matrix.md`
- `docs/language-reference/frontend-parser-wave.md`

Implementation outline:
1. Define coverage feature labels:
   - `parser-syntax`
   - `typescript-erasure`
   - `frontend-invalid-source`
   - `frontend-unsupported`
   - `name-resolution`
   - `runtime-subset`
2. Ensure diagnostics emitted by lexer/parser/AST validator/type-reference directive/type-erasure validation carry enough `phase` or message token for `reference-coverage.py` to map into the labels.
3. Update unsupported classifier to map `DiagCode::SyntaxError` with phase `parser` to `parser-syntax`, TypeScript erasure diagnostics to `typescript-erasure`, and explicit invalid test source to `frontend-invalid-source`.
4. Add feature label counts to triage JSON from REQ-COV-002.

Tests:
- Add a parser syntax invalid fixture and expected feature label.
- Add a TypeScript erasure unsupported fixture, for example runtime-bearing namespace/module augmentation, and expected `typescript-erasure`.
- Add JSONL schema test that `feature` is required for `outcome=unsupported`.

Acceptance commands:
```bash
python scripts/manager.py reference-coverage test262 --jsonl --sample 20 --category 'language/statements' --jobs 1 --no-dashboard-data > /tmp/ts2wasm-parser-sample.jsonl
python scripts/manager.py check records -- /tmp/ts2wasm-parser-sample.jsonl
python scripts/report/coverage_triage.py --input-jsonl /tmp/ts2wasm-parser-sample.jsonl --out-json /tmp/ts2wasm-parser-triage.json --out-md /tmp/ts2wasm-parser-triage.md --top-reasons 10
python scripts/check/coverage-triage-schema.py /tmp/ts2wasm-parser-triage.json
```

Done definition:

- [ ] parser/frontend unsupported records no longer use `unknown-unsupported:parser` when phase and code identify a frontend source.
- [ ] TypeScript erased syntax failures are labeled `typescript-erasure`.
- [ ] invalid source is not counted as missing runtime support.
- [ ] coverage matrix generation preserves existing aggregate columns while accepting new labels.

Depends on:

- REQ-COV-001
- REQ-COV-002
- REQ-FE-003

Non-goals:

- Zeroing all parser syntax unsupported cases.
- Implementing new syntax semantics.

Risk:

- Diagnostic messages currently used by classifiers may be brittle; prefer explicit `phase` and diagnostic code where possible.

## Theme 2: Host Capability Boundary

### 現在できていること

- `docs/11-shared-definitions.md` defines capability manifest schema v1 and migration policy.
- `docs/03-api-and-host-capability.md` defines standalone WASI vs Node host split and requires per-function host imports such as `host.timer.setTimeout`, not monolithic `node_fs_all`.
- `crates/shared/src/capability.rs` defines manifest structures and schema version.
- `crates/runtime-catalog/src/capability.rs`, `host_import.rs`, `link_plan.rs`, and `runtime_fn.rs` define catalog concepts.
- `crates/backend-wasm/src/capability_manifest.rs` emits canonical manifest from runtime link plan.
- `scripts/check/manifest-imports.py` compares manifest imports with wasm import section and checks deterministic manifest output.
- `crates/backend-wasm/tests/host_import_capability.rs` checks runtime functions with imports have capabilities.

### 現在できていないこと

- `scripts/check/host-deny.py` is info-only and exits 0 after reporting.
- Unknown host imports are not rejected by a single manifest/import hard gate in all relevant commands.
- `host_deny` CLI option does not encode target policy beyond boolean presence.
- Runtime catalog is not yet the only place where host import module/name strings can originate.

### 中途半端に存在するもの

- Source regex host analysis exists but treats `process.env`, `fs`, and date formatting patterns broadly, while docs allow some Node idioms to lower to WASI without Node host.
- Manifest/import comparison compiles representative fixtures but does not validate all fixture catalog standalone/host classifications.
- `validate_host_deny` rejects `backend::has_node_host_imports(lowered)` but does not inspect generated wasm and manifest after emission.

### 壊してはいけない前提

- WASI-compatible idioms such as `console.log`, stdin read, realtime clock, and random must not be reclassified as Node host solely because source contains Node-like names.
- `CapabilityManifest.node_host.imports` must use `host.<domain>.<function>` and be deterministic.
- Node host shim expansion must be opt-in and audited.

### 既存コード上の主要ファイル

- `docs/03-api-and-host-capability.md`
- `docs/11-shared-definitions.md`
- `crates/shared/src/capability.rs`
- `crates/runtime-catalog/src/capability.rs`
- `crates/runtime-catalog/src/host_import.rs`
- `crates/backend-wasm/src/capability_manifest.rs`
- `crates/backend-wasm/src/runtime_link_plan.rs`
- `scripts/check/manifest-imports.py`
- `scripts/check/host-deny.py`

### 既存テスト上の主要ファイル

- `crates/backend-wasm/tests/host_import_capability.rs`
- `crates/backend-wasm/tests/manifest_snapshot_equality.rs`
- `crates/compiler/tests/manifest_snapshot.rs`
- `crates/cli/tests/m11_host_deny.rs`
- `crates/runtime-catalog/tests/capability_registry.rs`

### 関連する既存 issue / docs / plans

- `I-20260513-W9X2Z8`
- `I-20260513-5PGJNN`
- `.agents/plans/5032-capability-detection.md`
- `docs/09-security-and-capability-model.md`

### Completion Criteria

Theme 2 is complete when:

- `python scripts/manager.py check manifest -- --all` fails if manifest imports and wasm imports differ;
- `python scripts/manager.py check host -- --strict --compile` fails for any standalone fixture that generates `node_host.required=true` or wasm module import `"host"`;
- all host import strings are declared in runtime catalog structures and covered by registry tests;
- fixture catalog can distinguish `host_imports: deny`, `host_imports: allow-wasi`, and `host_imports: allow-node-host`.

### REQ-CAP-001: Manifest/import equivalence is a hard gate

Priority: P0

Rationale:
The repository already compares manifest imports and wasm imports. The next boundary requires this to be the authoritative hard gate: no host import may exist without a manifest declaration, and no manifest import may be stale.

Affected files:
- `scripts/check/manifest-imports.py`
- `crates/backend-wasm/src/capability_manifest.rs`
- `crates/backend-wasm/src/runtime_link_plan.rs`
- `crates/shared/src/capability.rs`
- `crates/backend-wasm/tests/manifest_snapshot_equality.rs`

Implementation outline:
1. Add helper in Python checker to parse import pairs as `{module, name}` from both manifest and `wasm-tools print`.
2. Treat mismatch as exit 1 for all fixtures passed to `--all` and default deterministic fixture set.
3. Add manifest validation for `node_host.required == any(import.module == "host")` and `standalone == false` when host imports exist.
4. Add failure output:
   ```text
   check_manifest_imports: mismatch fixture=...
     wasm_only=[("host","timer.setTimeout")]
     manifest_only=[]
   ```
5. Keep deterministic snapshot check for repeated builds.

Tests:
- Existing backend manifest tests remain.
- Add synthetic manifest parser unit in `scripts/check/manifest-imports.py --self-test` with one `wasm_only` and one `manifest_only` case.
- Add compile check for `fixtures/builtins-and-io/console-log.ts` showing standalone true and no host import.

Acceptance commands:
```bash
python scripts/check/manifest-imports.py --self-test
python scripts/manager.py check manifest -- --all
cargo nextest run -p ts2wasm-backend-wasm --test manifest_snapshot_equality
```

Done definition:

- [ ] manifest/wasm import mismatch exits nonzero.
- [ ] `node_host.required` and `standalone` are validated against actual imports.
- [ ] deterministic manifest snapshot still passes.
- [ ] failure output lists exact import pairs.

Depends on:

- none

Non-goals:

- Implementing new Node host APIs.
- Changing manifest schema version unless a field is added.

Risk:

- Requires `wasm-tools`; check must keep existing toolchain prerequisite behavior.

### REQ-CAP-002: Strict standalone host-deny compile gate

Priority: P0

Rationale:
Security/capability boundary cannot rely on an info-only script. Standalone fixtures must fail the gate if generated wasm imports module `host` or manifest requires node host.

Affected files:
- `scripts/check/host-deny.py`
- `fixtures/catalog.yaml`
- `scripts/manager.py`
- `mise.toml`
- `crates/compiler/src/stages/runtime_gate.rs`
- `crates/cli/tests/m11_host_deny.rs`

Implementation outline:
1. Add `--strict` to `scripts/check/host-deny.py`.
2. Extend `fixtures/catalog.yaml` host policy values:
   - `host_imports: deny`
   - `host_imports: allow-wasi`
   - `host_imports: allow-node-host`
3. In strict mode, compile all fixtures with `category` not in `negative`, `parser`, `test-infrastructure` and host policy `deny` or `allow-wasi`.
4. For each compiled fixture, emit manifest to temp path and validate:
   - wasm import module `host` absent;
   - manifest `node_host.required == false`;
   - manifest `standalone == true`.
5. Source regex may remain as advisory output but must not be final verdict in strict mode.
6. Add manager alias `check host` to pass through strict options; do not make `--strict --compile` part of default smoke unless toolchain is present.

Tests:
- Update `crates/cli/tests/m11_host_deny.rs` to assert CLI build with `--host-deny deny` rejects known host-required fixture.
- Add script self-test with fake manifest/wasm import pairs.
- Add catalog validation that every compilable fixture has host policy inherited from directory or explicit fixture entry.

Acceptance commands:
```bash
python scripts/check/host-deny.py --self-test
python scripts/manager.py check host -- --strict --compile
cargo nextest run -p ts2wasm-cli --test m11_host_deny
```

Done definition:

- [ ] `--strict` exists and exits nonzero on host import violation.
- [ ] strict verdict uses generated wasm/manifest, not source regex alone.
- [ ] fixture catalog has explicit or inherited host policy for every compilable fixture.
- [ ] `m11_host_deny` still covers compiler runtime gate behavior.

Depends on:

- REQ-CAP-001

Non-goals:

- Full Node.js compatibility.
- Static classification of every Node API by source regex.

Risk:

- Compiling all fixtures can be slow; allow `--fixture PATH` and `--limit N` for local debugging, but strict CI command must have an all-fixture mode.

### REQ-CAP-003: Runtime catalog owns host import strings and reasons

Priority: P1

Rationale:
Capability boundary becomes fragile if backend modules can create raw `"host"` imports without a catalog entry. Host import names and capability reasons must be centralized.

Affected files:
- `crates/runtime-catalog/src/host_import.rs`
- `crates/runtime-catalog/src/capability.rs`
- `crates/runtime-catalog/src/runtime_fn.rs`
- `crates/backend-wasm/src/runtime_link_plan.rs`
- `crates/backend-wasm/src/runtime/host/catalog.rs`
- `crates/backend-wasm/tests/host_import_capability.rs`
- `scripts/check/architecture-rules.py`

Implementation outline:
1. Define `HostImportSpec { module: &'static str, name: &'static str, capability: Capability, reason: &'static str }` in `runtime-catalog`.
2. Runtime function specs that require host import must reference `HostImportSpec`, not duplicate module/name strings.
3. Add architecture rule that raw string literal `"host"` is allowed only in:
   - `crates/runtime-catalog/src/host_import.rs`
   - tests that assert output
   - scripts that parse wasm output
4. Manifest generation uses `HostImportSpec` to populate `node_host.imports` and `capability_reasons`.
5. Existing tests assert every host import has exactly one capability and reason.

Tests:
- Update `crates/runtime-catalog/tests/capability_registry.rs` for `HostImportSpec` uniqueness.
- Update `crates/backend-wasm/tests/host_import_capability.rs` for one-to-one host import to manifest reason.
- Add architecture rule fixture in `rule-tests/` if ast-grep is suitable, otherwise Python architecture-rule self-test.

Acceptance commands:
```bash
cargo nextest run -p ts2wasm-runtime-catalog
cargo nextest run -p ts2wasm-backend-wasm --test host_import_capability
python scripts/manager.py check architecture
```

Done definition:

- [ ] no production backend file constructs raw host import string outside runtime catalog.
- [ ] each host import has capability and reason.
- [ ] manifest reasons are generated from catalog data.
- [ ] architecture rule catches forbidden raw host strings.

Depends on:

- REQ-CAP-001

Non-goals:

- Adding new host import domains.
- Implementing Node host shim runtime.

Risk:

- Existing backend runtime host modules may need mechanical migration; keep migration small by introducing adapter functions first.

## Theme 3: TypeScript Frontend Boundary

### 現在できていること

- `crates/frontend` contains lexer, parser, AST re-exports, diagnostics, resolver module, type reference directive validation, and TypeScript oracle integration.
- `crates/frontend/Cargo.toml` depends on `shared`, `source`, `diagnostic`, and `syntax`, but not `ir`, `backend-wasm`, or `compiler`.
- `crates/syntax/src/ast.rs` contains broad ECMAScript/TypeScript-ish AST variants, including imports/exports, functions, classes, enums, ambient declarations, arrows, optional chaining, array holes/spread, object properties, and more.
- parser tests cover snapshots, property/no-panic, CLI AST structures, parser keywords, and type reference directives.
- `fixtures/basics-types` catalog includes TypeScript erasure fixtures: ambient declaration erasure, interface erasure, type alias erasure, generic erasure, satisfies/const/as assertion erasure, and optimization hints.
- `docs/language-reference/frontend-parser-wave.md` explicitly states parser acceptance is not runtime semantic claim.
- `current-state.md` states production build pipeline does not require tsc and TypeScript compiler API is an oracle/check path.

### 現在できていないこと

- There is no explicit `FrontendBoundary` architecture check verifying forbidden dependencies and forbidden symbol usage.
- There is no parser-smoke manager command that only accepts parser/unparse and never performs lowering/backend/semantic claim.
- TypeScript erasure is not exposed as a concrete API/report such as `ErasureReport` with `ErasureKind` values.
- Runtime-bearing TS syntax diagnostics are not consistently separated from plain parser syntax errors in coverage.
- TypeScript oracle is still in `crates/frontend` and re-exported; production build isolation is a policy, not a checked invariant.

### 中途半端に存在するもの

- `validate_type_reference_directives` is in frontend and used before lexer/parser in compiler pipeline.
- `semantic::validate_typescript_call_arity` runs for TypeScript path in semantic validation, so some TS-specific checks occur after frontend.
- `docs/language-reference/typescript-features.md` lists many TS features but may be stale relative to `fixtures/basics-types` and current-state.
- `crates/frontend/src/resolver.rs` exists inside frontend, while actual name resolution stage in compiler/IR exists elsewhere. This name overlap can confuse boundary ownership.

### 壊してはいけない前提

- Parser acceptance must not imply runtime support.
- Type-only syntax must be erased without changing runtime semantics.
- Runtime-bearing TypeScript syntax must either lower explicitly or emit `UnsupportedTypeScriptSyntax` with span/feature/tracking.
- `ts2wasm build` must not require Node or TypeScript compiler API.
- Existing parser and type-erasure fixtures must continue to pass.

### 既存コード上の主要ファイル

- `crates/frontend/Cargo.toml`
- `crates/frontend/src/lib.rs`
- `crates/frontend/src/parser.rs`
- `crates/frontend/src/lexer.rs`
- `crates/frontend/src/diagnostic.rs`
- `crates/frontend/src/type_reference_directive.rs`
- `crates/frontend/src/typescript_oracle.rs`
- `crates/syntax/src/ast.rs`
- `crates/compiler/src/stages/parse.rs`
- `crates/compiler/src/stages/semantic_validate.rs`
- `crates/compiler/src/dump.rs`
- `scripts/check/architecture-rules.py`

### 既存テスト上の主要ファイル

- `crates/frontend/tests/parser_snapshot.rs`
- `crates/frontend/tests/parser_property.rs`
- `crates/cli/tests/parser_ast_structures.rs`
- `crates/cli/tests/parser_keywords.rs`
- `crates/cli/tests/type_reference_directives.rs`
- `fixtures/basics-types/*.ts`
- `fixtures/catalog.yaml`

### 関連する既存 issue / docs / plans

- `docs/05-compatibility-and-semantics.md`
- `docs/language-reference/frontend-parser-wave.md`
- `docs/language-reference/typescript-features.md`
- `plans/5000-parser-syntax-coverage.md`
- `.agents/plans/5043-split-parser.md`
- `.agents/plans/5044-ambient-erasure.md`
- `I-20260513-XJSRR2` for downstream destructuring lowering

### Completion Criteria

Theme 3 is complete when:

- `python scripts/manager.py check frontend-boundary` proves `crates/frontend` has no forbidden crate dependencies and no production frontend path makes runtime/capability/backend decisions;
- parser smoke command validates parser fixtures without lowering or claiming semantic pass;
- TypeScript erased syntax emits a concrete `ErasureReport` and runtime-bearing TS forms fail with `UnsupportedTypeScriptSyntax` and feature label;
- `ts2wasm build` path cannot call TypeScript oracle, while `ts2wasm check` may;
- coverage triage can attribute frontend failures to parser syntax or TypeScript erasure.

### REQ-FE-001: Frontend crate syntax-only contract

Priority: P0

Rationale:
The central architecture boundary is that frontend parses syntax and preserves spans. Name resolution, builtin resolution, host capability decisions, runtime helper selection, and backend lowering must happen after frontend.

Affected files:
- `crates/frontend/Cargo.toml`
- `crates/frontend/src/lib.rs`
- `crates/frontend/src/parser.rs`
- `crates/frontend/src/type_reference_directive.rs`
- `crates/frontend/src/typescript_oracle.rs`
- `scripts/check/frontend-boundary.py` new
- `scripts/check/architecture-rules.py`
- `scripts/manager.py`
- `docs/language-reference/frontend-parser-wave.md`

Implementation outline:
1. Add `scripts/check/frontend-boundary.py` with checks:
   - `crates/frontend/Cargo.toml` must not contain dependencies whose package path is `../ir`, `../backend-wasm`, `../backend-core`, `../compiler`, `../runtime-catalog`, `../runtime-abi`.
   - Production modules under `crates/frontend/src` except `typescript_oracle.rs` must not contain strings `RuntimeFn`, `Capability`, `HostImport`, `Lowered`, `Wasm`, `emit_wat`, `build_runtime_link_plan`.
   - `typescript_oracle` exports are allowed only behind check/oracle boundary; script reports their public re-export until REQ-FE-004 decides migration.
2. Add manager alias `check frontend-boundary`.
3. Document boundary in `docs/language-reference/frontend-parser-wave.md`: frontend output is `ts2wasm_syntax::Stmt/Expr` plus diagnostics/spans, not semantic IR.
4. Add a CI-friendly self-test for the script using temp files.

Tests:
- `python scripts/check/frontend-boundary.py --self-test`.
- `python scripts/manager.py check frontend-boundary`.
- Existing frontend parser tests.

Acceptance commands:
```bash
python scripts/check/frontend-boundary.py --self-test
python scripts/manager.py check frontend-boundary
cargo nextest run -p ts2wasm-frontend
```

Done definition:

- [ ] forbidden dependencies are checked mechanically.
- [ ] forbidden runtime/backend/capability symbols in frontend production modules fail the check.
- [ ] manager exposes `check frontend-boundary`.
- [ ] frontend docs define syntax-only boundary.

Depends on:

- none

Non-goals:

- Moving all existing modules out of frontend immediately.
- Implementing new syntax.

Risk:

- Simple string checks can false positive in comments/tests; script must ignore `#[cfg(test)]` blocks only if robust, otherwise allow explicit path allowlist.

### REQ-FE-002: Parser smoke is a first-class non-semantic fixture class

Priority: P0

Rationale:
Parser acceptance is necessary for coverage expansion but must not be confused with build or semantic compatibility. The repository already has parser tests and a `parser` fixture category; it needs a manager command and record format for parser-only evidence.

Affected files:
- `fixtures/catalog.yaml`
- `scripts/check/parser-smoke.py` new
- `scripts/manager.py`
- `mise.toml`
- `crates/cli/src/main.rs`
- `crates/compiler/src/dump.rs`
- `docs/language-reference/frontend-parser-wave.md`
- `docs/15-coverage-matrix.md`

Implementation outline:
1. Add `scripts/check/parser-smoke.py` that reads `fixtures/catalog.yaml`, selects directories with `category: parser`, and runs:
   ```bash
   cargo run -q -p ts2wasm-cli -- dump --ast --unparse <fixture>
   ```
2. The script must not call `build`, `wasm-tools`, `iwasm`, or Node.
3. Parser smoke result records must use target `parser-ast` and status `pass` / `fail` / `unsupported` with feature `parser-syntax` when JSON output is requested.
4. Add `python scripts/manager.py check parser-smoke` alias.
5. Add at least one parser-only fixture if none exist in catalog, for example `fixtures/parser/ts-type-only-syntax.ts`, and ensure it is not included in semantic fixture checks.

Tests:
- Parser smoke script self-test with two temp fixture entries.
- Existing parser unit and CLI tests.
- Fixture catalog validation updated to accept parser-only directory.

Acceptance commands:
```bash
python scripts/check/parser-smoke.py --self-test
python scripts/manager.py check parser-smoke
cargo nextest run -p ts2wasm-cli --test parser_ast_structures --test parser_keywords
```

Done definition:

- [ ] parser-smoke command exists and does not build wasm.
- [ ] parser fixtures are selected by catalog category, not ad hoc paths.
- [ ] parser smoke failures cannot be counted as semantic failures.
- [ ] docs state parser acceptance is non-semantic evidence.

Depends on:

- REQ-FE-001

Non-goals:

- Full ECMAScript parser completion.
- test262 full syntax sweep.

Risk:

- Existing catalog may have semantic fixtures under syntax-like directories; script must use category, not path names.

### REQ-FE-003: TypeScript erasure report and unsupported boundary

Priority: P0

Rationale:
TypeScript syntax splits into erased type-only forms and runtime-bearing forms. The current implementation has fixtures and diagnostics but no stable report/API proving what was erased and what was rejected.

Affected files:
- `crates/frontend/src/typescript_erasure.rs` new
- `crates/frontend/src/lib.rs`
- `crates/frontend/src/parser.rs`
- `crates/syntax/src/ast.rs`
- `crates/compiler/src/stages/parse.rs`
- `crates/compiler/src/dump.rs`
- `crates/cli/src/main.rs`
- `fixtures/basics-types/*.ts`
- `crates/cli/tests/type_reference_directives.rs`
- `docs/05-compatibility-and-semantics.md`

Implementation outline:
1. Add public frontend module `typescript_erasure` with:
   ```rust
   pub struct ErasureReport {
       pub erased: Vec<ErasedSyntax>,
       pub unsupported: Vec<UnsupportedTsSyntax>,
   }
   pub struct ErasedSyntax {
       pub kind: ErasureKind,
       pub span: Span,
   }
   pub enum ErasureKind {
       TypeAnnotation,
       InterfaceDecl,
       TypeAliasDecl,
       AmbientDecl,
       AmbientNamespace,
       GenericParams,
       AsAssertion,
       Satisfies,
       ConstAssertion,
       TypeOnlyImport,
   }
   pub struct UnsupportedTsSyntax {
       pub feature: &'static str,
       pub span: Span,
       pub diagnostic_code: DiagCode,
       pub tracking: &'static str,
   }
   pub fn collect_erasure_report(program: &[Stmt]) -> ErasureReport;
   ```
2. Parser keeps erased syntax source spans in AST nodes or parser side-channel sufficient for the report. If current AST cannot store some erased spans, start with forms present in `fixtures/basics-types` and explicitly list missing forms as unsupported.
3. Runtime-bearing TS syntax that is not supported emits `DiagCode::UnsupportedTypeScriptSyntax` with `feature` one of:
   - `enum`
   - `namespace-runtime`
   - `declaration-emit`
   - `module-augmentation`
   - `type-only-import-runtime`
4. Add CLI dump option `ts2wasm dump --ast --erasure-report <file>` or extend `dump --ast` JSON/text output with `--erasure-report` to print stable JSON.
5. Update docs to state exact current erasure set and unsupported runtime-bearing set.

Tests:
- Add frontend unit tests for `collect_erasure_report` on type annotation, interface, type alias, ambient declaration, satisfies/as/const/generic fixtures.
- Add CLI test `typescript_erasure_report.rs` checking stable JSON keys.
- Update `type_reference_directives` tests for unsupported feature labels.

Acceptance commands:
```bash
cargo nextest run -p ts2wasm-frontend erasure
cargo nextest run -p ts2wasm-cli --test type_reference_directives
cargo run -q -p ts2wasm-cli -- dump --ast --erasure-report fixtures/basics-types/ambient-erasure-comprehensive.ts > /tmp/ts2wasm-erasure.json
python -m json.tool /tmp/ts2wasm-erasure.json >/dev/null
```

Done definition:

- [ ] `ErasureReport`, `ErasedSyntax`, `ErasureKind`, and `UnsupportedTsSyntax` exist.
- [ ] supported erased forms produce report entries with spans.
- [ ] unsupported runtime-bearing TS forms produce `UnsupportedTypeScriptSyntax` with feature/tracking.
- [ ] CLI can emit stable erasure report JSON.
- [ ] existing type-erasure fixtures still build or pass their current checks.

Depends on:

- REQ-FE-001

Non-goals:

- Implementing TypeScript checker.
- Supporting every TypeScript runtime-bearing feature.
- Changing runtime semantics based on type annotations.

Risk:

- Existing parser may discard spans for erased syntax; implement limited report first for current fixtures rather than broad AST redesign.

### REQ-FE-004: TypeScript oracle isolation

Priority: P1

Rationale:
TypeScript compiler API is valuable as oracle but production build must not depend on Node. This must be tested mechanically because `typescript_oracle` is currently in frontend and re-exported.

Affected files:
- `crates/frontend/src/typescript_oracle.rs`
- `crates/frontend/src/lib.rs`
- `crates/cli/src/main.rs`
- `crates/compiler/src/pipeline.rs`
- `scripts/check/typescript-oracle-boundary.py` new
- `scripts/manager.py`
- `docs/04-compiler-architecture-and-runtime.md`
- `docs/05-compatibility-and-semantics.md`

Implementation outline:
1. Add `scripts/check/typescript-oracle-boundary.py` that scans production build path files:
   - `crates/compiler/src/pipeline.rs`
   - `crates/compiler/src/stages/*.rs`
   - `crates/backend-wasm/src/**/*.rs`
   - `crates/ir/src/**/*.rs`
   and fails if they call `collect_typescript_diagnostics`, `check_typescript_file`, `typescript-oracle.js`, or `Command::new("node")`.
2. Allow oracle usage in:
   - `crates/frontend/src/typescript_oracle.rs`
   - CLI `check` command path
   - tests
   - scripts/check and scripts/run triage
3. Add manager alias `check typescript-oracle-boundary` and include it in `check frontend-boundary` composite.
4. Optional migration: stop re-exporting oracle from `crates/frontend/src/lib.rs` and require explicit module path, but only if tests can migrate safely.

Tests:
- Script self-test with temporary allowed/forbidden files.
- CLI `ts2wasm check` test remains allowed.
- Build fixture test with `PATH` hiding `node` should still compile a simple file if cargo binary is already built.

Acceptance commands:
```bash
python scripts/check/typescript-oracle-boundary.py --self-test
python scripts/manager.py check typescript-oracle-boundary
cargo nextest run -p ts2wasm-cli --test type_reference_directives
```

Done definition:

- [ ] production build path files cannot call TypeScript oracle.
- [ ] `ts2wasm check` remains able to use oracle.
- [ ] manager exposes the boundary check.
- [ ] docs state oracle is non-production.

Depends on:

- REQ-FE-001

Non-goals:

- Removing TypeScript oracle entirely.
- Replacing TypeScript checker.

Risk:

- Static grep may miss indirect calls; combine forbidden symbol scan with allowed path list and review in code comments.

### REQ-FE-005: Frontend reference-window smoke

Priority: P1

Rationale:
Parser/frontend work needs a small reproducible reference window to catch regressions without requiring full test262. This aligns with docs/language-reference/frontend-parser-wave.md.

Affected files:
- `scripts/data/frontend-reference-window.txt` new
- `scripts/check/frontend-reference-window.py` new
- `scripts/manager.py`
- `scripts/run/reference-coverage.py`
- `docs/language-reference/frontend-parser-wave.md`

Implementation outline:
1. Add `scripts/data/frontend-reference-window.txt` with 50 reference paths grouped by comments:
   - source text/comments
   - identifiers/literals
   - ASI/statements
   - functions/arrows/classes
   - modules
   - TypeScript erased syntax where available in tsc suite
2. Add `scripts/check/frontend-reference-window.py` that requires reference prerequisites and runs `reference-coverage` using `--paths-file`.
3. The check accepts outcomes `semantic_match`, `build_only`, `verified_negative_compile`, `unsupported` only if unsupported feature is frontend-related and tracked. It fails on `internal_failure` and untracked frontend unsupported.
4. The command is not part of default check.

Tests:
- Script self-test for parsing window file and checking allowed outcome policy.
- Use REQ-REF-001 prerequisite message when reference corpus is absent.

Acceptance commands:
```bash
python scripts/check/frontend-reference-window.py --self-test
python scripts/manager.py check frontend-reference-window
```

Done definition:

- [ ] deterministic frontend reference paths file exists.
- [ ] checker fails on untracked parser/frontend unsupported outcomes.
- [ ] checker records accepted outcomes by case.
- [ ] command is manager-accessible but not default lightweight check.

Depends on:

- REQ-COV-001
- REQ-COV-004
- REQ-REF-001

Non-goals:

- Full test262 parser conformance.
- Full TypeScript suite execution.

Risk:

- Reference corpus may be absent in local archive; prerequisite check must explain exactly how to sync.

## Theme 4: Runtime Object/GC

### 現在できていること

- Runtime ABI layout constants exist in `crates/runtime-abi/src/layout.rs` and are tested by `crates/runtime-abi/tests/abi_invariants.rs`.
- `crates/backend-wasm/src/emitter/gc_roots.rs` emits static root table initialization, call-frame root stack, root mirror slots, activation frame push/pop, and backend temp root clearing.
- GC/object pressure fixtures exist: `fixtures/core-semantics/gc-call-frame-root.ts`, `gc-high-pressure-root.ts`, `gc-object-root.ts`, `gc-transient-allocation.ts`, `closure-gc-call-frame-root.ts`, `ordinary-function-closure-gc-pressure.ts`, `private-class-field-internal-slot-gc.ts`.
- Current-state records partial mark-and-sweep and closure capture marking behavior.

### 現在できていないこと

- Allocation sites do not have a repository-level coverage matrix proving every allocation path has root protection.
- Root categories are not encoded as a testable enum/report.
- ABI snapshot tests do not yet explicitly snapshot every GC header/root stack/object/closure layout constant used by backend.
- Coverage triage does not separate runtime OOM/GC corruption from semantic mismatch.

### 中途半端に存在するもの

- GC root emission exists in backend but root liveness contract is implicit in WAT emission.
- Runtime layout constants have invariant tests, but compat snapshots are not integrated with generated WASM metadata.
- GC pressure fixtures cover selected cases but not an explicit root category matrix.

### 壊してはいけない前提

- Existing GC pressure and closure fixtures must remain differential-pass.
- Heap object tags and ValueTag low-bit encoding must not change without ABI snapshot and metadata update.
- GC failures must be treated as runtime/ABI failures, not generic semantic unsupported.

### 既存コード上の主要ファイル

- `crates/runtime-abi/src/layout.rs`
- `crates/runtime-abi/src/value.rs`
- `crates/backend-wasm/src/emitter/gc_roots.rs`
- `crates/backend-wasm/src/runtime/core/memory.rs`
- `crates/backend-wasm/src/runtime/object/emit.rs`
- `crates/backend-wasm/src/runtime_core.rs`
- `crates/backend-wasm/src/runtime_objects.rs`

### 既存テスト上の主要ファイル

- `crates/runtime-abi/tests/abi_invariants.rs`
- `crates/cli/tests/m2_node_diff.rs`
- `fixtures/core-semantics/gc-*.ts`
- `fixtures/core-semantics/ordinary-function-closure-*.ts`
- `fixtures/core-semantics/private-class-field-internal-slot-gc.ts`

### 関連する既存 issue / docs / plans

- `docs/04-compiler-architecture-and-runtime.md`
- `docs/14-runtime-abi.md`
- `.agents/plans/5052-abi-memory-map.md`
- `plans/5004-runtime-builtins-coverage.md`

### Completion Criteria

Theme 4 is complete when:

- a GC root category report/test exists and covers `TopLevelLocal`, `CallFrame`, `BackendTemp`, `ModuleCache`, `ClassPrototype`, `ClosureCapture`;
- all existing GC pressure fixtures pass Node/iwasm differential;
- ABI snapshot includes GC header/root stack/object/closure constants;
- coverage outcome taxonomy can classify runtime GC/ABI failures as `runtime_error` or `internal_failure` with phase `runtime`/`abi`.

### REQ-RT-001: GC root category coverage contract

Priority: P1

Rationale:
The backend already emits multiple root mechanisms. A future coding agent must know which root categories are required and which fixtures prove them.

Affected files:
- `crates/backend-wasm/src/emitter/gc_roots.rs`
- `crates/backend-wasm/src/emitter/functions.rs`
- `crates/backend-wasm/src/runtime/core/memory.rs`
- `crates/runtime-abi/src/layout.rs`
- `fixtures/core-semantics/*.ts`
- `crates/cli/tests/m2_node_diff.rs`
- `docs/14-runtime-abi.md`

Implementation outline:
1. Define root category names in `docs/14-runtime-abi.md` and backend comments:
   - `TopLevelLocal`
   - `CallFrame`
   - `BackendTemp`
   - `ModuleCache`
   - `ClassPrototype`
   - `ClosureCapture`
2. Add `scripts/check/gc-root-fixtures.py` with a mapping:
   ```json
   {
     "TopLevelLocal": "fixtures/core-semantics/gc-object-root.ts",
     "CallFrame": "fixtures/core-semantics/gc-call-frame-root.ts",
     "BackendTemp": "fixtures/core-semantics/gc-transient-allocation.ts",
     "ClosureCapture": "fixtures/core-semantics/ordinary-function-closure-gc-pressure.ts",
     "ClassPrototype": "fixtures/core-semantics/private-class-field-internal-slot-gc.ts"
   }
   ```
   `ModuleCache` may use an existing module fixture or new `fixtures/core-semantics/gc-module-cache-root.ts` if needed.
3. Ensure each mapped fixture is in `fixtures/catalog.yaml` and in differential test selection.
4. Add a backend unit test or snapshot that `gc_root_slot_count`, call-frame stack, and backend temp clearing emit expected WAT snippets for one representative lowered program.

Tests:
- `scripts/check/gc-root-fixtures.py --self-test`.
- Differential fixture tests for mapped fixtures.
- Backend WAT snippet test for root table/call-frame emission.

Acceptance commands:
```bash
python scripts/check/gc-root-fixtures.py --self-test
python scripts/check/gc-root-fixtures.py --run-differential
cargo nextest run -p ts2wasm-cli --test m2_node_diff
cargo nextest run -p ts2wasm-backend-wasm gc_root
```

Done definition:

- [ ] every required root category has a named fixture or explicit documented blocker.
- [ ] mapped fixtures are in catalog and differential selection.
- [ ] backend root emission has at least one direct test.
- [ ] docs define root categories and failure mode.

Depends on:

- none

Non-goals:

- Full moving GC.
- Wasm GC backend migration.

Risk:

- Some categories may require new fixture support; if unsupported, create precise issue and mark category blocked, not silently absent.

### REQ-RT-002: Runtime layout and ABI snapshot coverage

Priority: P1

Rationale:
Runtime object/GC correctness depends on layout constants matching backend WAT. Existing invariant tests are necessary but should snapshot all cross-crate constants relevant to object/GC/closure.

Affected files:
- `crates/runtime-abi/src/layout.rs`
- `crates/runtime-abi/src/value.rs`
- `crates/runtime-abi/compat/v1-snapshot.txt`
- `crates/runtime-abi/compat/v2-snapshot.txt`
- `crates/runtime-abi/tests/abi_invariants.rs`
- `crates/backend-wasm/src/runtime/core/memory.rs`
- `docs/14-runtime-abi.md`

Implementation outline:
1. Add `RuntimeAbiSnapshot` dump function or test helper listing:
   - `ValueTag::*`
   - `Layout::GC_HEADER_SIZE`
   - `Layout::GC_MARK_BIT`
   - `Layout::GC_KIND_*`
   - object/array/string header sizes
   - closure sentinel/layout offsets
   - call-frame root stack constants
   - module cache constants
2. Update compat snapshot files and tests to compare exact text.
3. Require intentional version bump or snapshot update comment when a constant changes.
4. Add docs paragraph: backend must import constants from `ts2wasm_runtime_abi`, not duplicate numeric literals when practical.

Tests:
- Existing `abi_invariants.rs` plus snapshot test `runtime_abi_snapshot_matches`.
- Backend test that representative emitted WAT contains constants from runtime ABI snapshot, not stale values, for GC header size and root stack size.

Acceptance commands:
```bash
cargo nextest run -p ts2wasm-runtime-abi
cargo nextest run -p ts2wasm-backend-wasm runtime_signature
```

Done definition:

- [ ] snapshot includes GC/object/closure/root constants.
- [ ] snapshot test fails on changed constants.
- [ ] docs state update/version policy.
- [ ] backend tests cover at least two cross-crate constants.

Depends on:

- none

Non-goals:

- Changing RawValue encoding.
- Adding new heap object kinds.

Risk:

- Snapshot churn; require reviewers to distinguish intentional ABI change from formatting change.

### REQ-RT-003: Runtime/GC failure mode classification in coverage

Priority: P2

Rationale:
When GC or runtime ABI fails during reference coverage, it must not be classified as syntax unsupported. It is either runtime error or internal failure.

Affected files:
- `scripts/run/reference-coverage.py`
- `scripts/lib/coverage_outcome.py`
- `scripts/report/coverage_triage.py`
- `docs/15-coverage-matrix.md`

Implementation outline:
1. Map iwasm trap, wasm validation failure after successful compile, and runtime ABI assertion messages to `outcome=runtime_error` unless the compiler itself panics, which is `internal_failure`.
2. Add `phase=runtime` for iwasm trap and `phase=abi` for metadata/layout mismatch if detected by ABI metadata gate.
3. Include first stderr line in triage sample but cap to 200 chars.

Tests:
- Add synthetic JSONL classification self-test for iwasm trap and wasm validation failure.
- Add one intentionally blocked/negative fixture only if safe; do not introduce crashing tests into normal nextest.

Acceptance commands:
```bash
python scripts/lib/coverage_outcome.py --self-test
python scripts/report/coverage_triage.py --self-test
```

Done definition:

- [ ] iwasm trap maps to `runtime_error`.
- [ ] compiler panic/internal invariant maps to `internal_failure`.
- [ ] runtime/ABI phase is visible in triage JSON.

Depends on:

- REQ-COV-001
- REQ-COV-002

Non-goals:

- Fixing all runtime errors.
- Adding crash-prone fixtures to default gates.

Risk:

- Tool stderr formats may vary; keep classification based on runner result type first, stderr regex second.

## Theme 5: Reference Corpus Reproducibility

### 現在できていること

- `reference/README.md` lists relevant upstream repositories and paths.
- `scripts/dev/link-reference.py` exists as a worktree/reference helper.
- `reference-coverage.py --check-prerequisites` exists.
- coverage matrix records denominator and evidence command.
- current-state recognizes reference repository dependency as a risk and mentions hash fixing/local cache as a consideration.

### 現在できていないこと

- `reference/lock.json` does not exist in the archive.
- `reference/test262`, `reference/typescript`, and `reference/typescript-go` are not present in the archive.
- Reference suite commit, denominator, and expected path are not machine-checked.
- JSONL/triage/matrix outputs do not require reference lock hash and selection metadata.

### 中途半端に存在するもの

- Coverage artifact has denominator and evidence command but not the exact corpus commit.
- `--sample` and `--paths-file` exist but metadata about seed/file hash is not required in outputs.
- `reference/README.md` provides URLs but no deterministic checkout instructions.

### 壊してはいけない前提

- Repository archive without reference corpora must still run lightweight checks.
- Full reference coverage must fail early with deterministic prerequisite error if corpus is absent.
- Denominator changes must be visible through lock/schema update, not silent.

### 既存コード上の主要ファイル

- `reference/README.md`
- `scripts/dev/link-reference.py`
- `scripts/run/reference-coverage.py`
- `scripts/gen/coverage-matrix.py`
- `scripts/manager.py`

### 既存テスト上の主要ファイル

- `artifacts/coverage/reference-coverage-matrix.md`
- `scripts/check/test-records-schema.py`

### 関連する既存 issue / docs / plans

- `docs/15-coverage-matrix.md`
- `docs/23-coverage-runner-completeness.md`
- `plans/457-harness-compiler-gaps.md`
- `I-20260513-HDW7PQ`

### Completion Criteria

Theme 5 is complete when:

- `reference/lock.json` exists and validates suite name, upstream URL, commit, expected path, denominator, and optional checksum;
- `python scripts/dev/sync-reference.py --check` validates local corpus against the lock or exits with exact missing path/commit guidance;
- coverage JSONL/triage/matrix include `reference_lock_sha256` and selection metadata;
- deterministic subset files exist for coverage parity and frontend windows.

### REQ-REF-001: Reference lockfile and prerequisite checker

Priority: P0

Rationale:
Coverage percentages are only meaningful against a fixed corpus. The archive shows reference README but no checked-out corpus or lockfile.

Affected files:
- `reference/lock.json` new
- `scripts/dev/sync-reference.py` new or extend `scripts/dev/link-reference.py`
- `scripts/run/reference-coverage.py`
- `scripts/manager.py`
- `docs/15-coverage-matrix.md`
- `reference/README.md`

Implementation outline:
1. Add `reference/lock.json` schema:
   ```json
   {
     "schema_version": 1,
     "suites": {
       "test262": {
         "repo": "https://github.com/tc39/test262.git",
         "commit": "<pinned-sha>",
         "path": "reference/test262",
         "case_glob": "test/**/*.js",
         "denominator": 53469
       },
       "tsc": {
         "repo": "https://github.com/microsoft/TypeScript.git",
         "commit": "<pinned-sha>",
         "path": "reference/typescript",
         "case_glob": "tests/cases/compiler/**/*.ts",
         "denominator": 6419
       },
       "tsgo": {
         "repo": "https://github.com/microsoft/typescript-go.git",
         "commit": "<pinned-sha>",
         "path": "reference/typescript-go",
         "case_glob": "testdata/tests/**",
         "denominator": 166
       }
     }
   }
   ```
2. Add `scripts/dev/sync-reference.py --check`:
   - validates lock schema;
   - checks paths exist;
   - if `.git` exists, checks HEAD commit equals lock;
   - counts denominator from glob and compares lock denominator;
   - if absent, exits 2 with `reference prerequisite missing: suite=test262 path=reference/test262 expected_commit=...`.
3. Update `reference-coverage.py --check-prerequisites` to call the same validation.
4. Update `reference/README.md` with clone/sync commands using lockfile.

Tests:
- `scripts/dev/sync-reference.py --self-test` using temp fake suite.
- Missing corpus deterministic error test.
- Lock JSON schema validation.

Acceptance commands:
```bash
python scripts/dev/sync-reference.py --self-test
python scripts/dev/sync-reference.py --check
python scripts/manager.py reference-coverage test262 --check-prerequisites
```

Done definition:

- [ ] `reference/lock.json` exists and is schema-valid.
- [ ] prerequisite checker validates path, commit when available, glob denominator.
- [ ] missing corpus error is deterministic and actionable.
- [ ] `reference-coverage --check-prerequisites` uses the checker.

Depends on:

- none

Non-goals:

- Downloading corpora in default checks.
- Vendoring full test262/TypeScript corpora into repository.

Risk:

- Pinning commits requires maintainers to select real SHAs; until selected, use placeholder only in draft branch is not acceptable for done.

### REQ-REF-002: Coverage outputs include reference and selection metadata

Priority: P0

Rationale:
A JSONL or triage artifact must say exactly which corpus and subset produced it. Without this, coverage deltas cannot be reproduced.

Affected files:
- `scripts/run/reference-coverage.py`
- `scripts/report/coverage_triage.py`
- `scripts/gen/coverage-matrix.py`
- `scripts/check/test-records-schema.py`
- `docs/17-jsonl-test-record-schema.md`
- `docs/15-coverage-matrix.md`

Implementation outline:
1. Compute `reference_lock_sha256` as SHA-256 of canonicalized `reference/lock.json`.
2. Add per-record or run-header fields to JSONL. Because JSONL currently emits one object per test, add fields to every record:
   - `reference_lock_sha256`
   - `reference_suite_commit`
   - `selection_mode`
   - `selection_seed`
   - `paths_file_sha256`
   - `path_filter`
   - `sample_category`
3. For `--sample`, use deterministic default seed `0` unless `--sample-seed N` is provided. Add `--sample-seed` option.
4. Triage JSON summary from REQ-COV-002 includes same metadata under `reference` and `selection`.
5. Coverage matrix evidence includes lock hash suffix, for example:
   ```text
   `mise run reference-coverage -- test262` lock=<sha256:12>
   ```

Tests:
- JSONL schema self-test with metadata.
- Two `--sample 10 --sample-seed 123` runs produce identical selected case list.
- Different seed changes case list when enough candidates exist.

Acceptance commands:
```bash
python scripts/manager.py reference-coverage test262 --jsonl --sample 10 --sample-seed 123 --jobs 1 --no-dashboard-data > /tmp/ts2wasm-seeded-a.jsonl
python scripts/manager.py reference-coverage test262 --jsonl --sample 10 --sample-seed 123 --jobs 1 --no-dashboard-data > /tmp/ts2wasm-seeded-b.jsonl
python scripts/manager.py check records -- /tmp/ts2wasm-seeded-a.jsonl
diff -u <(jq -r '.case' /tmp/ts2wasm-seeded-a.jsonl) <(jq -r '.case' /tmp/ts2wasm-seeded-b.jsonl)
```

Done definition:

- [ ] every coverage JSONL record includes reference lock and selection metadata.
- [ ] triage JSON includes the same metadata.
- [ ] sample seed is explicit and deterministic.
- [ ] coverage matrix evidence records lock hash.

Depends on:

- REQ-REF-001
- REQ-COV-001
- REQ-COV-002

Non-goals:

- Recomputing historical artifacts.
- Storing full selected case lists in coverage matrix.

Risk:

- `jq` may not be installed in all environments; schema test should not depend on `jq`, but acceptance may use it for developer convenience. Provide Python fallback in docs.

### REQ-REF-003: Deterministic reference subsets

Priority: P1

Rationale:
Small stable windows are necessary for parity, frontend, and semantic canary checks.

Affected files:
- `scripts/data/coverage-parity-test262.txt`
- `scripts/data/frontend-reference-window.txt`
- `scripts/data/semantic-canary.txt`
- `scripts/check/reference-subsets.py` new
- `fixtures/catalog.yaml`
- `docs/23-coverage-runner-completeness.md`

Implementation outline:
1. Add `scripts/check/reference-subsets.py` that validates each paths file:
   - no duplicate paths;
   - sorted lexicographically within section;
   - all paths exist under locked reference suite when corpus is present;
   - comments use `# group: <name>` format.
2. Add command to check all deterministic subset files.
3. Link subset purpose in docs.

Tests:
- Self-test with duplicate path and unsorted path.
- Run checker against repository subset files.

Acceptance commands:
```bash
python scripts/check/reference-subsets.py --self-test
python scripts/check/reference-subsets.py --all
```

Done definition:

- [ ] deterministic subset checker exists.
- [ ] coverage parity and frontend window files pass it.
- [ ] checker integrates with reference prerequisite logic when corpus is present.

Depends on:

- REQ-REF-001
- REQ-COV-003
- REQ-FE-005

Non-goals:

- Replacing full coverage matrix.
- Automatically selecting optimal subsets.

Risk:

- Reference corpus absence must not fail subset syntax validation; only existence validation is skipped or reported as prerequisite missing.

## Theme 6: ABI Stability / Target Evolution

### 現在できていること

- `crates/runtime-abi` defines `WasmTaggedJsWire`, `ValueTag`, `TaggedValue`, `HeapPtr`, `LocalRawValue`, `Layout`, and compat snapshots.
- `docs/14-runtime-abi.md` defines RawValue and layout constants, plus BigInt, heap number, module cache, symbol registry, GC headers, and closure concepts.
- `docs/04-compiler-architecture-and-runtime.md` distinguishes logical ABI and wire format and warns against implicit mixing.
- `crates/backend-core/src/wasm_ir.rs` and `crates/backend-wasm/src/wasm_encoder_backend.rs` support typed wasm/binary direction.
- `I-20260512-WAENCD` tracks typed wasm encoder parity fixtures.

### 現在できていないこと

- Generated WASM does not carry explicit `runtime_abi_version` / target profile metadata.
- Target evolution is not represented as a checked `TargetProfile` enum.
- Wasm GC / Component Model are future concepts in docs but not gated by target profile.
- ABI compat snapshots do not connect to generated artifacts.

### 中途半端に存在するもの

- HIR/MIR and typed WasmIR exist but are opt-in / partial.
- `runtime_signature` tests exist but do not parse a custom metadata section.
- manifest schema has versioning, but runtime ABI metadata is separate and absent from artifacts.

### 壊してはいけない前提

- Default target remains `wasm32-wasi` linear-memory MVP-compatible backend.
- Wasm GC / Component Model must not silently affect default emitted WASM.
- Runtime ABI changes require snapshot updates and visible metadata version bump.

### 既存コード上の主要ファイル

- `crates/runtime-abi/src/consts.rs`
- `crates/runtime-abi/src/layout.rs`
- `crates/runtime-abi/src/value.rs`
- `crates/backend-core/src/wasm_ir.rs`
- `crates/backend-wasm/src/wasm_encoder_backend.rs`
- `crates/backend-wasm/src/wasm_binary.rs`
- `crates/compiler/src/pipeline.rs`
- `crates/cli/src/main.rs`

### 既存テスト上の主要ファイル

- `crates/runtime-abi/tests/abi_invariants.rs`
- `crates/backend-wasm/tests/runtime_signature.rs`
- `crates/backend-wasm/tests/runtime_intrinsic_mapping.rs`
- `crates/backend-wasm/tests/manifest_snapshot_equality.rs`

### 関連する既存 issue / docs / plans

- `docs/14-runtime-abi.md`
- `docs/13-ir-contracts.md`
- `docs/27-ir-layer-completion-plan.md`
- `.agents/plans/5029-direct-binary-emission.md`
- `.agents/plans/5052-abi-memory-map.md`
- `I-20260512-WAENCD`

### Completion Criteria

Theme 6 is complete when:

- emitted WASM has custom section `ts2wasm.abi` with runtime ABI version, target profile, raw value encoding, manifest schema version, and compiler crate version;
- backend tests parse that section and compare it to `ts2wasm-runtime-abi` constants;
- ABI compat snapshot is enforced;
- target profile prevents default builds from emitting Wasm GC / Component Model features unless explicitly selected.

### REQ-ABI-001: Generated WASM embeds ABI metadata

Priority: P0

Rationale:
Runtime ABI cannot be stable if artifacts do not declare which ABI they use. Metadata also lets host tools reject incompatible WASM before execution.

Affected files:
- `crates/runtime-abi/src/consts.rs`
- `crates/backend-wasm/src/wasm_binary.rs`
- `crates/backend-wasm/src/wasm_encoder_backend.rs`
- `crates/compiler/src/io/write_output.rs`
- `crates/backend-wasm/tests/wasm_abi_metadata.rs` new
- `docs/14-runtime-abi.md`

Implementation outline:
1. Add constants:
   ```rust
   pub const RUNTIME_ABI_VERSION: u32 = 2;
   pub const RAW_VALUE_ENCODING: &str = "i32-low3-tagged";
   pub const ABI_METADATA_SECTION: &str = "ts2wasm.abi";
   ```
2. Define metadata JSON payload:
   ```json
   {
     "schema_version": 1,
     "runtime_abi_version": 2,
     "target_profile": "wasm32-wasi",
     "raw_value_encoding": "i32-low3-tagged",
     "capability_manifest_schema": 1,
     "compiler": {
       "crate": "ts2wasm-cli",
       "version": "0.1.0"
     }
   }
   ```
3. Emit custom section `ts2wasm.abi` in generated binary. For WAT-to-wasm output, add custom section after encoding step; if WAT path cannot insert section, route through wasm-encoder append or post-process binary.
4. Add parser helper in test using `wasmparser` or `wasm-tools` output to read custom section.
5. Ensure `--emit-manifest` manifest target matches metadata `target_profile`.

Tests:
- Build `fixtures/basics-hello/hello.ts`, parse custom section, assert fields.
- Metadata absent or malformed test fixture should fail parser helper.
- Ensure WAT text snapshots remain unaffected where they intentionally test WAT only.

Acceptance commands:
```bash
cargo nextest run -p ts2wasm-backend-wasm --test wasm_abi_metadata
cargo run -q -p ts2wasm-cli -- build fixtures/basics-hello/hello.ts -o /tmp/hello.wasm --emit-manifest /tmp/hello.manifest.json
wasm-tools validate /tmp/hello.wasm
```

Done definition:

- [ ] generated wasm contains `ts2wasm.abi` custom section.
- [ ] section JSON includes required fields.
- [ ] backend test validates fields against runtime/shared constants.
- [ ] manifest target and metadata target profile agree.

Depends on:

- REQ-CAP-001 for manifest schema consistency.
- REQ-RT-002 for ABI version constants.

Non-goals:

- Component Model metadata.
- Host-side loader implementation.

Risk:

- Current output path may use WAT conversion that drops custom section; implement post-encode append with wasm-encoder if necessary.

### REQ-ABI-002: Runtime ABI compatibility snapshot gate

Priority: P0

Rationale:
ABI stability requires that changes to ValueTag/Layout are intentional and visible. Existing snapshots must become a gate tied to runtime ABI version.

Affected files:
- `crates/runtime-abi/src/consts.rs`
- `crates/runtime-abi/src/layout.rs`
- `crates/runtime-abi/src/value.rs`
- `crates/runtime-abi/compat/v*-snapshot.txt`
- `crates/runtime-abi/tests/abi_invariants.rs`
- `docs/14-runtime-abi.md`

Implementation outline:
1. Add test helper `dump_runtime_abi_snapshot() -> String`.
2. Compare output to `compat/v{RUNTIME_ABI_VERSION}-snapshot.txt`.
3. If constants change without `RUNTIME_ABI_VERSION` bump and snapshot update, tests fail.
4. Add docs migration rule:
   - non-breaking doc-only changes do not bump version;
   - value tag/layout/custom section schema changes bump runtime ABI version;
   - manifest schema changes bump `SCHEMA_VERSION`, not runtime ABI version unless emitted metadata schema changes.

Tests:
- Runtime ABI snapshot test.
- Existing invariant tests.

Acceptance commands:
```bash
cargo nextest run -p ts2wasm-runtime-abi
```

Done definition:

- [ ] snapshot generated by test matches checked-in file.
- [ ] runtime ABI version is included in snapshot.
- [ ] docs define bump policy.

Depends on:

- REQ-RT-002

Non-goals:

- Backward compatibility loader.
- Maintaining multiple runtime implementations.

Risk:

- Version bump policy can be over-triggered; document exact breaking fields.

### REQ-ABI-003: TargetProfile boundary for future Wasm targets

Priority: P1

Rationale:
Future Wasm GC / Component Model support must not silently change default output. A target profile enum creates a boundary for backend and capability decisions.

Affected files:
- `crates/compiler/src/target.rs` new
- `crates/compiler/src/pipeline.rs`
- `crates/cli/src/main.rs`
- `crates/backend-wasm/src/wasm_encoder_backend.rs`
- `crates/backend-core/src/wasm_ir.rs`
- `docs/04-compiler-architecture-and-runtime.md`
- `docs/14-runtime-abi.md`

Implementation outline:
1. Add enum:
   ```rust
   pub enum TargetProfile {
       Wasm32Wasi,
       Wasm32WasiNodeHost,
       Wasm32WasiGcExperimental,
       Wasm32ComponentExperimental,
   }
   ```
2. Add CLI option `--target-profile <wasm32-wasi|wasm32-wasi-node-host|wasm32-wasi-gc-experimental|wasm32-component-experimental>` default `wasm32-wasi`.
3. Store `target_profile` in `BuildPipelineOptions`.
4. Backend rejects GC/reference-types/component features unless matching experimental profile is selected.
5. ABI metadata from REQ-ABI-001 uses `target_profile` string.
6. Manifest target remains `wasm32-wasi` for default and `wasm32-wasi+node-host` for node host profile.

Tests:
- CLI parse test for all target profiles.
- Default build metadata says `wasm32-wasi`.
- Attempt to use experimental feature in default profile fails with diagnostic `UnsupportedRuntimeSubset` and phase `target-profile`.

Acceptance commands:
```bash
cargo nextest run -p ts2wasm-cli target_profile
cargo nextest run -p ts2wasm-compiler target_profile
cargo nextest run -p ts2wasm-backend-wasm --test wasm_abi_metadata
```

Done definition:

- [ ] `TargetProfile` exists and is carried through build options.
- [ ] CLI option parses all supported strings and defaults to `wasm32-wasi`.
- [ ] default profile rejects experimental Wasm GC/Component features.
- [ ] ABI metadata records selected profile.

Depends on:

- REQ-ABI-001

Non-goals:

- Implementing Wasm GC backend.
- Implementing Component Model backend.

Risk:

- Adding target option touches CLI snapshots/tests; keep default behavior unchanged.

## Cross-theme Dependency Graph

```text
REQ-REF-001 ─┬─> REQ-REF-002 ─┬─> REQ-COV-002
             │                └─> REQ-REF-003
             └─> REQ-FE-005

REQ-COV-001 ─┬─> REQ-COV-002 ──> REQ-COV-004
             ├─> REQ-COV-003
             └─> REQ-RT-003

REQ-FE-001 ─┬─> REQ-FE-002
             ├─> REQ-FE-003 ──> REQ-COV-004
             └─> REQ-FE-004

REQ-CAP-001 ─┬─> REQ-CAP-002
              ├─> REQ-CAP-003
              └─> REQ-ABI-001

REQ-RT-002 ─┬─> REQ-ABI-001
            └─> REQ-ABI-002

REQ-ABI-001 ──> REQ-ABI-003
```

Critical P0 path:

```text
REQ-COV-001 + REQ-COV-002
REQ-CAP-001 + REQ-CAP-002
REQ-FE-001 + REQ-FE-002 + REQ-FE-003
REQ-REF-001 + REQ-REF-002
REQ-ABI-001 + REQ-ABI-002
```

## Task Decomposition

### TASK-COV-001: Implement CoverageOutcome and JSONL schema

Priority: P0

Source requirements:
- REQ-COV-001
- REQ-RT-003

Goal:
Add a stable coverage outcome taxonomy and make `reference-coverage --jsonl` records validate against it.

Files:
- `scripts/lib/coverage_outcome.py` new
- `scripts/run/reference-coverage.py`
- `scripts/check/test-records-schema.py`
- `scripts/gate/coverage.py`
- `docs/17-jsonl-test-record-schema.md`

Steps:
1. Implement `CoverageOutcome` constants and `classify_coverage_outcome(record)`.
2. Add self-test covering all outcomes.
3. Update JSONL record creation in `reference-coverage.py` to add `outcome`.
4. Update schema validation to reject unknown outcomes.
5. Update docs with outcome table.
6. Keep runner-parity code paths unchanged; TASK-COV-003 owns parity checking.

Tests:
- `coverage_outcome.py --self-test`
- `check-test-records-schema.py --self-test`
- sample test262 JSONL validation

Acceptance:
```bash
python scripts/lib/coverage_outcome.py --self-test
python scripts/manager.py check records -- --self-test
python scripts/manager.py reference-coverage test262 --jsonl --sample 10 --jobs 1 --no-dashboard-data > /tmp/ts2wasm-test262.jsonl
python scripts/manager.py check records -- /tmp/ts2wasm-test262.jsonl
```

Completion proves:

- REQ-COV-001 done item 1
- REQ-COV-001 done item 2
- REQ-COV-001 done item 3
- REQ-RT-003 done item 1
- REQ-RT-003 done item 2

### TASK-COV-002: Add stable coverage triage artifacts

Priority: P0

Source requirements:
- REQ-COV-002
- REQ-COV-004
- REQ-REF-002

Goal:
Generate deterministic JSON and Markdown triage reports from reference coverage runs.

Files:
- `scripts/report/coverage_triage.py` new
- `scripts/check/coverage-triage-schema.py` new
- `scripts/run/reference-coverage.py`
- `scripts/manager.py`
- `mise.toml`
- `docs/15-coverage-matrix.md`

Steps:
1. Implement triage aggregator and stable top-N sorting.
2. Add schema validator with self-test.
3. Add `--out-json`, `--out-md`, `--top-reasons` to `reference-coverage.py`.
4. Include reference/selection metadata fields, initially nullable until TASK-REF-002.
5. Map frontend-specific features from diagnostics where available.
6. Document schema and output location.

Tests:
- triage self-test
- schema self-test
- sample test262 run with JSON/Markdown outputs

Acceptance:
```bash
python scripts/report/coverage_triage.py --self-test
python scripts/check/coverage-triage-schema.py --self-test
python scripts/manager.py reference-coverage test262 --jsonl --sample 10 --jobs 1 --no-dashboard-data --out-json /tmp/ts2wasm-triage.json --out-md /tmp/ts2wasm-triage.md --top-reasons 10 > /tmp/ts2wasm-test262.jsonl
python scripts/check/coverage-triage-schema.py /tmp/ts2wasm-triage.json
test -s /tmp/ts2wasm-triage.md
```

Completion proves:

- REQ-COV-002 done items 1-5
- REQ-COV-004 done items 1-4 after TASK-FE-003 labels are present
- REQ-REF-002 triage metadata portion

### TASK-COV-003: Implement coverage runner parity check

Priority: P1

Source requirements:
- REQ-COV-003

Goal:
Prove that server batch and legacy `--no-server` coverage paths classify fixed reference cases identically.

Files:
- `scripts/check/coverage-runner-parity.py` new
- `scripts/data/coverage-parity-test262.txt` new
- `scripts/run/reference-coverage.py`
- `scripts/manager.py`
- `docs/23-coverage-runner-completeness.md`

Steps:
1. Add fixed paths file with representative semantic, unsupported, negative, and build-only cases.
2. Implement parity checker that runs server and legacy modes with the same paths file.
3. Compare `suite`, `case`, `status`, `outcome`, `diagnostic_code`, `feature`, and `tracking`; ignore timing/log fields.
4. Print first 10 mismatches with field-level diff and exit nonzero.
5. Add manager alias `check coverage-parity`.
6. Use REQ-REF-001 prerequisite checker for missing reference corpus.

Tests:
- `coverage-runner-parity.py --self-test` with synthetic JSONL mismatch.
- Reference-window parity run when corpus is present.

Acceptance:
```bash
python scripts/check/coverage-runner-parity.py --self-test
python scripts/check/coverage-runner-parity.py --suite test262 --paths-file scripts/data/coverage-parity-test262.txt
```

Completion proves:

- REQ-COV-003 done items 1-4

### TASK-CAP-001: Harden manifest/import equivalence and catalog invariants

Priority: P0

Source requirements:
- REQ-CAP-001
- REQ-CAP-003

Goal:
Make manifest/import mismatch a hard failure and ensure host import strings originate from runtime catalog.

Files:
- `scripts/check/manifest-imports.py`
- `crates/runtime-catalog/src/host_import.rs`
- `crates/runtime-catalog/src/capability.rs`
- `crates/backend-wasm/src/capability_manifest.rs`
- `crates/backend-wasm/src/runtime_link_plan.rs`
- `crates/backend-wasm/tests/host_import_capability.rs`
- `crates/runtime-catalog/tests/capability_registry.rs`
- `scripts/check/architecture-rules.py`

Steps:
1. Add manifest import parser self-test and strict mismatch output.
2. Validate `node_host.required` and `standalone` against actual imports.
3. Introduce `HostImportSpec` in runtime catalog.
4. Migrate backend manifest generation to catalog host import specs.
5. Add architecture rule for raw `"host"` strings outside allowlist.

Tests:
- manifest import self-test
- runtime catalog tests
- backend host capability tests
- architecture rule check

Acceptance:
```bash
python scripts/check/manifest-imports.py --self-test
python scripts/manager.py check manifest -- --all
cargo nextest run -p ts2wasm-runtime-catalog
cargo nextest run -p ts2wasm-backend-wasm --test host_import_capability --test manifest_snapshot_equality
python scripts/manager.py check architecture
```

Completion proves:

- REQ-CAP-001 done items 1-4
- REQ-CAP-003 done items 1-4

### TASK-CAP-002: Implement strict standalone host-deny gate

Priority: P0

Source requirements:
- REQ-CAP-002

Goal:
Convert host-deny from info-only reporting to a strict compile/manifest/wasm gate for standalone fixtures.

Files:
- `scripts/check/host-deny.py`
- `fixtures/catalog.yaml`
- `scripts/manager.py`
- `mise.toml`
- `crates/cli/tests/m11_host_deny.rs`

Steps:
1. Add `--strict`, `--fixture`, `--limit`, and `--self-test` options.
2. Add fixture catalog host policy values and validation.
3. In strict mode, compile fixtures, emit manifest, inspect wasm import section.
4. Fail on host import for `deny` or `allow-wasi` fixtures.
5. Keep regex scan as advisory only.
6. Update CLI host-deny test if option semantics require.

Tests:
- host-deny self-test
- strict compile gate
- CLI m11 host-deny test

Acceptance:
```bash
python scripts/check/host-deny.py --self-test
python scripts/manager.py check host -- --strict --compile
cargo nextest run -p ts2wasm-cli --test m11_host_deny
```

Completion proves:

- REQ-CAP-002 done items 1-4

### TASK-FE-001: Add frontend boundary and oracle isolation checks

Priority: P0

Source requirements:
- REQ-FE-001
- REQ-FE-004

Goal:
Mechanically enforce syntax-only frontend dependency and production-build oracle isolation.

Files:
- `scripts/check/frontend-boundary.py` new
- `scripts/check/typescript-oracle-boundary.py` new
- `scripts/check/architecture-rules.py`
- `scripts/manager.py`
- `crates/frontend/Cargo.toml`
- `crates/frontend/src/lib.rs`
- `docs/language-reference/frontend-parser-wave.md`
- `docs/04-compiler-architecture-and-runtime.md`

Steps:
1. Implement frontend dependency/symbol boundary script.
2. Implement TypeScript oracle boundary script.
3. Add manager aliases `check frontend-boundary` and `check typescript-oracle-boundary`.
4. Update docs with allowed/forbidden responsibilities.
5. If necessary, stop re-exporting oracle from frontend lib or document a temporary allowlist.

Tests:
- both script self-tests
- frontend nextest
- type reference directive CLI tests

Acceptance:
```bash
python scripts/check/frontend-boundary.py --self-test
python scripts/check/typescript-oracle-boundary.py --self-test
python scripts/manager.py check frontend-boundary
python scripts/manager.py check typescript-oracle-boundary
cargo nextest run -p ts2wasm-frontend
cargo nextest run -p ts2wasm-cli --test type_reference_directives
```

Completion proves:

- REQ-FE-001 done items 1-4
- REQ-FE-004 done items 1-4

### TASK-FE-002: Implement parser-smoke and frontend reference window

Priority: P0

Source requirements:
- REQ-FE-002
- REQ-FE-005
- REQ-REF-003

Goal:
Separate parser-only evidence from semantic/build evidence and provide a deterministic frontend reference window.

Files:
- `scripts/check/parser-smoke.py` new
- `scripts/check/frontend-reference-window.py` new
- `scripts/data/frontend-reference-window.txt` new
- `fixtures/catalog.yaml`
- `scripts/manager.py`
- `docs/language-reference/frontend-parser-wave.md`

Steps:
1. Add parser-smoke script that runs only `dump --ast --unparse`.
2. Add parser fixture category validation and at least one parser-only fixture if needed.
3. Add frontend reference window paths file.
4. Add frontend reference checker using `reference-coverage --paths-file`.
5. Integrate with reference prerequisite checker from TASK-REF-001.

Tests:
- parser-smoke self-test
- parser smoke command
- frontend reference window self-test
- CLI parser tests

Acceptance:
```bash
python scripts/check/parser-smoke.py --self-test
python scripts/manager.py check parser-smoke
python scripts/check/frontend-reference-window.py --self-test
python scripts/manager.py check frontend-reference-window
cargo nextest run -p ts2wasm-cli --test parser_ast_structures --test parser_keywords
```

Completion proves:

- REQ-FE-002 done items 1-4
- REQ-FE-005 done items 1-4
- REQ-REF-003 subset use for frontend window

### TASK-FE-003: Implement TypeScript erasure report and diagnostics

Priority: P0

Source requirements:
- REQ-FE-003
- REQ-COV-004

Goal:
Expose TypeScript erasure as a stable frontend report and label unsupported runtime-bearing TS forms.

Files:
- `crates/frontend/src/typescript_erasure.rs` new
- `crates/frontend/src/lib.rs`
- `crates/frontend/src/parser.rs`
- `crates/syntax/src/ast.rs`
- `crates/compiler/src/stages/parse.rs`
- `crates/compiler/src/dump.rs`
- `crates/cli/src/main.rs`
- `fixtures/basics-types/*.ts`
- `docs/05-compatibility-and-semantics.md`

Steps:
1. Add `ErasureReport`, `ErasedSyntax`, `ErasureKind`, `UnsupportedTsSyntax`.
2. Collect report for existing type-erasure fixture forms.
3. Add CLI `dump --erasure-report` stable JSON output.
4. Emit feature/tracking for unsupported runtime-bearing TS syntax.
5. Update coverage classifier to map TS erasure diagnostics.
6. Update docs with supported erasure set.

Tests:
- frontend erasure tests
- CLI erasure report test
- type reference directives tests
- sample dump JSON validation

Acceptance:
```bash
cargo nextest run -p ts2wasm-frontend erasure
cargo nextest run -p ts2wasm-cli --test type_reference_directives
cargo run -q -p ts2wasm-cli -- dump --ast --erasure-report fixtures/basics-types/ambient-erasure-comprehensive.ts > /tmp/ts2wasm-erasure.json
python -m json.tool /tmp/ts2wasm-erasure.json >/dev/null
```

Completion proves:

- REQ-FE-003 done items 1-5
- REQ-COV-004 TypeScript-erasure label items

### TASK-RT-001: Add GC root contract checks and differential mapping

Priority: P1

Source requirements:
- REQ-RT-001
- REQ-RT-003

Goal:
Make GC root category coverage explicit and classify runtime/GC failures correctly.

Files:
- `scripts/check/gc-root-fixtures.py` new
- `fixtures/catalog.yaml`
- `fixtures/core-semantics/*.ts`
- `crates/backend-wasm/src/emitter/gc_roots.rs`
- `crates/backend-wasm/tests/gc_roots.rs` new or update existing
- `scripts/lib/coverage_outcome.py`
- `docs/14-runtime-abi.md`

Steps:
1. Define root categories in docs.
2. Add fixture mapping checker.
3. Ensure all mapped fixtures are cataloged and differential-selected.
4. Add backend WAT root snippet test.
5. Update outcome classification for runtime trap/internal failure.

Tests:
- gc-root fixture checker self-test
- differential fixture command
- backend gc root test

Acceptance:
```bash
python scripts/check/gc-root-fixtures.py --self-test
python scripts/check/gc-root-fixtures.py --run-differential
cargo nextest run -p ts2wasm-cli --test m2_node_diff
cargo nextest run -p ts2wasm-backend-wasm gc_root
```

Completion proves:

- REQ-RT-001 done items 1-4
- REQ-RT-003 done items 1-3

### TASK-RT-002: Extend runtime ABI snapshots for GC/object layout

Priority: P1

Source requirements:
- REQ-RT-002
- REQ-ABI-002

Goal:
Snapshot all runtime ABI constants that backend and generated WASM depend on.

Files:
- `crates/runtime-abi/src/consts.rs`
- `crates/runtime-abi/src/layout.rs`
- `crates/runtime-abi/src/value.rs`
- `crates/runtime-abi/compat/v*-snapshot.txt`
- `crates/runtime-abi/tests/abi_invariants.rs`
- `docs/14-runtime-abi.md`

Steps:
1. Implement `dump_runtime_abi_snapshot()` test helper.
2. Include ValueTag, GC/object/closure/root/module-cache constants.
3. Compare against versioned snapshot file.
4. Add version bump policy docs.
5. Add backend constant cross-check if practical.

Tests:
- runtime ABI nextest
- backend runtime signature tests

Acceptance:
```bash
cargo nextest run -p ts2wasm-runtime-abi
cargo nextest run -p ts2wasm-backend-wasm runtime_signature
```

Completion proves:

- REQ-RT-002 done items 1-4
- REQ-ABI-002 done items 1-3

### TASK-REF-001: Add reference lockfile and prerequisite checker

Priority: P0

Source requirements:
- REQ-REF-001

Goal:
Pin reference corpora and make missing/mismatched corpus failures deterministic.

Files:
- `reference/lock.json` new
- `scripts/dev/sync-reference.py` new
- `scripts/dev/link-reference.py`
- `scripts/run/reference-coverage.py`
- `scripts/manager.py`
- `reference/README.md`
- `docs/15-coverage-matrix.md`

Steps:
1. Add lockfile schema and real pinned commits/denominators.
2. Implement checker self-test.
3. Validate local paths, commits, and glob counts.
4. Hook `reference-coverage --check-prerequisites` into checker.
5. Document sync/check commands.

Tests:
- checker self-test
- missing corpus deterministic error
- prerequisite command

Acceptance:
```bash
python scripts/dev/sync-reference.py --self-test
python scripts/dev/sync-reference.py --check
python scripts/manager.py reference-coverage test262 --check-prerequisites
```

Completion proves:

- REQ-REF-001 done items 1-4

### TASK-REF-002: Add reference metadata and deterministic selection to coverage

Priority: P0

Source requirements:
- REQ-REF-002
- REQ-REF-003
- REQ-COV-002

Goal:
Ensure coverage JSONL/triage/matrix outputs identify corpus lock and selected subset.

Files:
- `scripts/run/reference-coverage.py`
- `scripts/report/coverage_triage.py`
- `scripts/gen/coverage-matrix.py`
- `scripts/check/reference-subsets.py` new
- `scripts/data/coverage-parity-test262.txt`
- `scripts/data/frontend-reference-window.txt`
- `docs/17-jsonl-test-record-schema.md`

Steps:
1. Compute lock hash and suite commit from `reference/lock.json`.
2. Add record fields for reference and selection metadata.
3. Add `--sample-seed` and deterministic sampling.
4. Add reference subset checker.
5. Update triage and matrix evidence.

Tests:
- seeded sample repeatability
- record schema validation
- subset checker self-test

Acceptance:
```bash
python scripts/check/reference-subsets.py --self-test
python scripts/check/reference-subsets.py --all
python scripts/manager.py reference-coverage test262 --jsonl --sample 10 --sample-seed 123 --jobs 1 --no-dashboard-data > /tmp/ts2wasm-seeded-a.jsonl
python scripts/manager.py reference-coverage test262 --jsonl --sample 10 --sample-seed 123 --jobs 1 --no-dashboard-data > /tmp/ts2wasm-seeded-b.jsonl
python scripts/manager.py check records -- /tmp/ts2wasm-seeded-a.jsonl
python - <<'PY'
import json
from pathlib import Path

def cases(path):
    return [json.loads(line)['case'] for line in Path(path).read_text().splitlines() if line.strip()]
assert cases('/tmp/ts2wasm-seeded-a.jsonl') == cases('/tmp/ts2wasm-seeded-b.jsonl')
PY
```

Completion proves:

- REQ-REF-002 done items 1-4
- REQ-REF-003 done items 1-3
- REQ-COV-002 metadata keys

### TASK-ABI-001: Embed and validate WASM ABI metadata

Priority: P0

Source requirements:
- REQ-ABI-001
- REQ-ABI-003

Goal:
Emit `ts2wasm.abi` custom section and verify it against runtime/shared constants and target profile.

Files:
- `crates/runtime-abi/src/consts.rs`
- `crates/backend-wasm/src/wasm_binary.rs`
- `crates/backend-wasm/src/wasm_encoder_backend.rs`
- `crates/compiler/src/io/write_output.rs`
- `crates/compiler/src/target.rs` new
- `crates/compiler/src/pipeline.rs`
- `crates/cli/src/main.rs`
- `crates/backend-wasm/tests/wasm_abi_metadata.rs` new
- `docs/14-runtime-abi.md`

Steps:
1. Add runtime ABI metadata constants.
2. Add `TargetProfile` enum and CLI option with default.
3. Carry target profile through build options.
4. Emit custom section JSON in generated WASM.
5. Parse and assert metadata in backend tests.
6. Ensure manifest target and metadata target agree.

Tests:
- wasm ABI metadata test
- target profile CLI/compiler tests
- wasm validation command

Acceptance:
```bash
cargo nextest run -p ts2wasm-backend-wasm --test wasm_abi_metadata
cargo nextest run -p ts2wasm-cli target_profile
cargo nextest run -p ts2wasm-compiler target_profile
cargo run -q -p ts2wasm-cli -- build fixtures/basics-hello/hello.ts -o /tmp/hello.wasm --emit-manifest /tmp/hello.manifest.json
wasm-tools validate /tmp/hello.wasm
```

Completion proves:

- REQ-ABI-001 done items 1-4
- REQ-ABI-003 done items 1-4

### TASK-ABI-002: Connect ABI snapshot gate to generated metadata

Priority: P0

Source requirements:
- REQ-ABI-002
- REQ-RT-002

Goal:
Ensure runtime ABI version in snapshots and generated WASM metadata is the same source of truth.

Files:
- `crates/runtime-abi/src/consts.rs`
- `crates/runtime-abi/tests/abi_invariants.rs`
- `crates/runtime-abi/compat/v*-snapshot.txt`
- `crates/backend-wasm/tests/wasm_abi_metadata.rs`
- `docs/14-runtime-abi.md`

Steps:
1. Include `RUNTIME_ABI_VERSION` in ABI snapshot.
2. Read same constant in wasm metadata test.
3. Add failing test case for mismatch using synthetic metadata parser helper.
4. Document bump procedure.

Tests:
- runtime ABI tests
- wasm metadata tests

Acceptance:
```bash
cargo nextest run -p ts2wasm-runtime-abi
cargo nextest run -p ts2wasm-backend-wasm --test wasm_abi_metadata
```

Completion proves:

- REQ-ABI-002 done items 1-3
- REQ-RT-002 snapshot/version integration

## Traceability Matrix

| Requirement | Tasks | Acceptance command | Completion evidence |
|---|---|---|---|
| REQ-COV-001 | TASK-COV-001 | `python scripts/lib/coverage_outcome.py --self-test`; `python scripts/manager.py check records -- --self-test`; sample `reference-coverage --jsonl` validation | JSONL records include valid `outcome`; schema rejects unknown outcomes |
| REQ-COV-002 | TASK-COV-002, TASK-REF-002 | `python scripts/report/coverage_triage.py --self-test`; `python scripts/check/coverage-triage-schema.py /tmp/ts2wasm-triage.json` | deterministic triage JSON/Markdown with top reasons and metadata |
| REQ-COV-003 | TASK-COV-003 | `python scripts/check/coverage-runner-parity.py --suite test262 --paths-file scripts/data/coverage-parity-test262.txt` | server/legacy JSONL outcomes match for fixed paths |
| REQ-COV-004 | TASK-COV-002, TASK-FE-003 | parser sample triage command with schema validation | frontend unsupported records have parser/erasure feature labels |
| REQ-CAP-001 | TASK-CAP-001 | `python scripts/manager.py check manifest -- --all` | manifest and wasm import pairs match; mismatch exits nonzero |
| REQ-CAP-002 | TASK-CAP-002 | `python scripts/manager.py check host -- --strict --compile`; `cargo nextest run -p ts2wasm-cli --test m11_host_deny` | standalone fixtures have no host import and manifest `standalone=true` |
| REQ-CAP-003 | TASK-CAP-001 | `cargo nextest run -p ts2wasm-runtime-catalog`; `python scripts/manager.py check architecture` | host import strings originate from catalog and have capability/reason |
| REQ-FE-001 | TASK-FE-001 | `python scripts/manager.py check frontend-boundary`; `cargo nextest run -p ts2wasm-frontend` | frontend has no forbidden dependencies/symbols |
| REQ-FE-002 | TASK-FE-002 | `python scripts/manager.py check parser-smoke`; CLI parser tests | parser fixtures run AST/unparse only, no wasm build |
| REQ-FE-003 | TASK-FE-003 | `cargo run -q -p ts2wasm-cli -- dump --ast --erasure-report fixtures/basics-types/ambient-erasure-comprehensive.ts` | stable erasure report JSON and TS unsupported feature labels |
| REQ-FE-004 | TASK-FE-001 | `python scripts/manager.py check typescript-oracle-boundary` | production build path cannot call TS oracle |
| REQ-FE-005 | TASK-FE-002 | `python scripts/manager.py check frontend-reference-window` | fixed frontend reference window has accepted tracked outcomes |
| REQ-RT-001 | TASK-RT-001 | `python scripts/check/gc-root-fixtures.py --run-differential`; `cargo nextest run -p ts2wasm-backend-wasm gc_root` | each GC root category has fixture/test evidence |
| REQ-RT-002 | TASK-RT-002, TASK-ABI-002 | `cargo nextest run -p ts2wasm-runtime-abi` | runtime ABI snapshot includes GC/object/closure/root constants |
| REQ-RT-003 | TASK-COV-001, TASK-RT-001 | `python scripts/lib/coverage_outcome.py --self-test`; triage self-test | runtime trap/internal failure map to `runtime_error`/`internal_failure` |
| REQ-REF-001 | TASK-REF-001 | `python scripts/dev/sync-reference.py --check`; `python scripts/manager.py reference-coverage test262 --check-prerequisites` | lockfile validates paths/commits/denominator or deterministic missing error |
| REQ-REF-002 | TASK-REF-002 | seeded sample repeatability commands; records schema validation | JSONL/triage/matrix include lock hash and selection metadata |
| REQ-REF-003 | TASK-REF-002, TASK-FE-002 | `python scripts/check/reference-subsets.py --all` | deterministic subset files are sorted, unique, and valid |
| REQ-ABI-001 | TASK-ABI-001 | `cargo nextest run -p ts2wasm-backend-wasm --test wasm_abi_metadata`; `wasm-tools validate /tmp/hello.wasm` | generated wasm contains valid `ts2wasm.abi` custom section |
| REQ-ABI-002 | TASK-RT-002, TASK-ABI-002 | `cargo nextest run -p ts2wasm-runtime-abi`; wasm metadata test | runtime ABI version is snapshotted and matches generated metadata |
| REQ-ABI-003 | TASK-ABI-001 | `cargo nextest run -p ts2wasm-cli target_profile`; `cargo nextest run -p ts2wasm-compiler target_profile` | target profile is parsed, defaulted, and enforced |

## Gates

### Gate A: Coverage observability complete

Commands:
```bash
python scripts/lib/coverage_outcome.py --self-test
python scripts/report/coverage_triage.py --self-test
python scripts/check/coverage-triage-schema.py --self-test
python scripts/manager.py reference-coverage test262 --jsonl --sample 20 --sample-seed 0 --jobs 1 --no-dashboard-data --out-json /tmp/ts2wasm-test262-triage.json --out-md /tmp/ts2wasm-test262-triage.md --top-reasons 10 > /tmp/ts2wasm-test262.jsonl
python scripts/manager.py check records -- /tmp/ts2wasm-test262.jsonl
python scripts/check/coverage-triage-schema.py /tmp/ts2wasm-test262-triage.json
python scripts/check/coverage-runner-parity.py --self-test
python scripts/check/coverage-runner-parity.py --suite test262 --paths-file scripts/data/coverage-parity-test262.txt
```

Pass criteria:

- all JSONL records include valid `outcome`;
- triage JSON validates schema;
- Markdown report exists and is non-empty;
- `top_reasons` order is stable.

Executable status:

- Made executable by TASK-COV-001, TASK-COV-002, TASK-COV-003, TASK-REF-002.

### Gate B: Capability boundary enforced

Commands:
```bash
python scripts/check/manifest-imports.py --self-test
python scripts/manager.py check manifest -- --all
python scripts/check/host-deny.py --self-test
python scripts/manager.py check host -- --strict --compile
cargo nextest run -p ts2wasm-runtime-catalog
cargo nextest run -p ts2wasm-backend-wasm --test host_import_capability --test manifest_snapshot_equality
cargo nextest run -p ts2wasm-cli --test m11_host_deny
```

Pass criteria:

- manifest imports and wasm imports match exactly;
- standalone/allow-wasi fixtures generate no `host` wasm import;
- runtime catalog owns all host import capability/reason data;
- CLI host-deny still rejects host-required output.

Executable status:

- Existing commands partially executable now; strict host gate and self-tests made executable by TASK-CAP-001 and TASK-CAP-002.

### Gate C: TypeScript frontend boundary enforced

Commands:
```bash
python scripts/check/frontend-boundary.py --self-test
python scripts/check/typescript-oracle-boundary.py --self-test
python scripts/manager.py check frontend-boundary
python scripts/manager.py check typescript-oracle-boundary
python scripts/manager.py check parser-smoke
cargo nextest run -p ts2wasm-frontend
cargo nextest run -p ts2wasm-cli --test parser_ast_structures --test parser_keywords --test type_reference_directives
cargo run -q -p ts2wasm-cli -- dump --ast --erasure-report fixtures/basics-types/ambient-erasure-comprehensive.ts > /tmp/ts2wasm-erasure.json
python -m json.tool /tmp/ts2wasm-erasure.json >/dev/null
```

Pass criteria:

- frontend crate has no forbidden dependencies;
- production build path cannot call TS oracle;
- parser smoke does not invoke build/backend/runtime;
- erasure report JSON is valid and includes supported erased forms.

Executable status:

- Made executable by TASK-FE-001, TASK-FE-002, TASK-FE-003.

### Gate D: Reference corpus reproducible

Commands:
```bash
python scripts/dev/sync-reference.py --self-test
python scripts/dev/sync-reference.py --check
python scripts/check/reference-subsets.py --self-test
python scripts/check/reference-subsets.py --all
python scripts/manager.py reference-coverage test262 --check-prerequisites
python scripts/manager.py update-coverage-matrix -- --check
```

Pass criteria:

- `reference/lock.json` schema is valid;
- local reference corpus matches pinned commit and denominator, or missing corpus error is deterministic;
- deterministic subset files are unique/sorted;
- coverage matrix check still passes.

Executable status:

- Made executable by TASK-REF-001 and TASK-REF-002.

### Gate E: Runtime ABI and GC boundary protected

Commands:
```bash
python scripts/check/gc-root-fixtures.py --self-test
python scripts/check/gc-root-fixtures.py --run-differential
cargo nextest run -p ts2wasm-runtime-abi
cargo nextest run -p ts2wasm-backend-wasm runtime_signature
cargo nextest run -p ts2wasm-cli --test m2_node_diff
```

Pass criteria:

- every required GC root category has fixture/test evidence or explicit blocker;
- runtime ABI snapshot matches checked-in version;
- existing GC pressure differential fixtures pass.

Executable status:

- Made executable by TASK-RT-001 and TASK-RT-002.

### Gate F: ABI metadata and target profile complete

Commands:
```bash
cargo nextest run -p ts2wasm-backend-wasm --test wasm_abi_metadata
cargo nextest run -p ts2wasm-cli target_profile
cargo nextest run -p ts2wasm-compiler target_profile
cargo run -q -p ts2wasm-cli -- build fixtures/basics-hello/hello.ts -o /tmp/hello.wasm --emit-manifest /tmp/hello.manifest.json
wasm-tools validate /tmp/hello.wasm
```

Pass criteria:

- generated wasm contains valid `ts2wasm.abi` custom section;
- metadata runtime ABI version matches snapshot constant;
- target profile defaults to `wasm32-wasi` and rejects experimental features by default;
- manifest target and metadata target agree.

Executable status:

- Made executable by TASK-ABI-001 and TASK-ABI-002.

### Gate G: Integrated next-wave readiness

Commands:
```bash
cargo fmt --all --check
cargo nextest run
python scripts/manager.py gate-fast -- --skip-nextest
python scripts/manager.py check architecture
python scripts/manager.py check manifest -- --all
python scripts/manager.py check frontend-boundary
python scripts/manager.py check parser-smoke
```

Pass criteria:

- no existing supported behavior regresses;
- architecture sizing/coupling remains within repository rules;
- manifest/capability and frontend boundary gates pass;
- parser smoke remains separate from semantic claims.

Executable status:

- Existing commands plus TASK-CAP-001, TASK-FE-001, TASK-FE-002.

## Open Questions

1. Should `typescript_oracle` move from `crates/frontend` to a dedicated `crates/frontend-oracle` or `crates/tooling-oracle` crate? This is not required for this wave if REQ-FE-004 enforces production isolation.
2. What exact upstream commits should populate `reference/lock.json`? This requires maintainer selection from current reference corpus used to generate `artifacts/coverage/reference-coverage-matrix.md`.
3. Should `coverage-triage` live as a standalone `scripts/report/coverage_triage.py` or inside `scripts/run/reference-coverage.py`? Requirement only fixes CLI behavior and schema, not file organization beyond affected files.
4. Should target profile be part of capability manifest schema v1 or only ABI metadata? This design requires metadata now; manifest schema bump is only required if manifest itself adds target-profile fields.
5. Should parser-smoke JSON records reuse canonical `TestRecord.status=pass` with `target=parser-ast`, or define a separate parser record schema? This design chooses canonical status plus `target=parser-ast` to minimize schema expansion.
6. How should `ModuleCache` GC root category be proven if existing module fixtures do not allocate through module cache? REQ-RT-001 allows a new fixture or explicit blocker, but Gate E requires one of those outcomes.

## Appendix: Files Investigated

### Root and project configuration

- `README.md`
- `current-state.md`
- `Cargo.toml`
- `mise.toml`
- `package.json`
- `reference/README.md`

### Docs

- `docs/current-state.md`
- `docs/03-api-and-host-capability.md`
- `docs/04-compiler-architecture-and-runtime.md`
- `docs/05-compatibility-and-semantics.md`
- `docs/06-testing-and-coverage.md`
- `docs/09-security-and-capability-model.md`
- `docs/11-shared-definitions.md`
- `docs/12-coding-standard.md`
- `docs/13-ir-contracts.md`
- `docs/14-runtime-abi.md`
- `docs/15-coverage-matrix.md`
- `docs/17-jsonl-test-record-schema.md`
- `docs/23-coverage-runner-completeness.md`
- `docs/24-architecture-decoupling-and-llm-friendly-sizing.md`
- `docs/26-semantic-feature-matrix.md`
- `docs/27-coverage-expansion-plan.md`
- `docs/27-ir-layer-completion-plan.md`
- `docs/language-reference/frontend-parser-wave.md`
- `docs/language-reference/typescript-features.md`

### Coverage artifacts and scripts

- `artifacts/coverage/reference-coverage-matrix.md`
- `scripts/manager.py`
- `scripts/run/reference-coverage.py`
- `scripts/run/reference-triage.py`
- `scripts/gate/coverage.py`
- `scripts/gen/coverage-matrix.py`
- `scripts/check/test-records-schema.py`
- `scripts/check/manifest-imports.py`
- `scripts/check/host-deny.py`
- `scripts/check/architecture-rules.py`
- `scripts/dev/link-reference.py`

### Crates

- `crates/frontend/Cargo.toml`
- `crates/frontend/src/lib.rs`
- `crates/frontend/src/parser.rs`
- `crates/frontend/src/lexer.rs`
- `crates/frontend/src/diagnostic.rs`
- `crates/frontend/src/type_reference_directive.rs`
- `crates/frontend/src/typescript_oracle.rs`
- `crates/syntax/src/ast.rs`
- `crates/compiler/Cargo.toml`
- `crates/compiler/src/pipeline.rs`
- `crates/compiler/src/dump.rs`
- `crates/compiler/src/stages/parse.rs`
- `crates/compiler/src/stages/validate.rs`
- `crates/compiler/src/stages/runtime_gate.rs`
- `crates/cli/src/main.rs`
- `crates/ir/src/lowered.rs`
- `crates/ir/src/semantic.rs`
- `crates/backend-core/src/wasm_ir.rs`
- `crates/backend-wasm/src/capability_manifest.rs`
- `crates/backend-wasm/src/runtime_link_plan.rs`
- `crates/backend-wasm/src/runtime_fn.rs`
- `crates/backend-wasm/src/emitter/gc_roots.rs`
- `crates/backend-wasm/src/wasm_encoder_backend.rs`
- `crates/backend-wasm/src/wasm_binary.rs`
- `crates/runtime-abi/src/consts.rs`
- `crates/runtime-abi/src/layout.rs`
- `crates/runtime-abi/src/value.rs`
- `crates/runtime-catalog/src/capability.rs`
- `crates/runtime-catalog/src/host_import.rs`
- `crates/runtime-catalog/src/link_plan.rs`

### Tests and fixtures

- `crates/frontend/tests/parser_snapshot.rs`
- `crates/frontend/tests/parser_property.rs`
- `crates/cli/tests/parser_ast_structures.rs`
- `crates/cli/tests/parser_keywords.rs`
- `crates/cli/tests/type_reference_directives.rs`
- `crates/cli/tests/differential_jsonl.rs`
- `crates/cli/tests/m1_iwasm.rs`
- `crates/cli/tests/m2_node_diff.rs`
- `crates/cli/tests/m11_host_deny.rs`
- `crates/backend-wasm/tests/host_import_capability.rs`
- `crates/backend-wasm/tests/manifest_snapshot_equality.rs`
- `crates/backend-wasm/tests/runtime_link_plan.rs`
- `crates/backend-wasm/tests/runtime_signature.rs`
- `crates/backend-wasm/tests/runtime_intrinsic_mapping.rs`
- `crates/runtime-catalog/tests/capability_registry.rs`
- `crates/runtime-catalog/tests/runtime_registry.rs`
- `crates/runtime-catalog/tests/link_plan_structural.rs`
- `crates/runtime-abi/tests/abi_invariants.rs`
- `fixtures/catalog.yaml`
- `fixtures/basics-types/ambient-erasure-comprehensive.ts`
- `fixtures/basics-types/type-alias-erasure.ts`
- `fixtures/core-semantics/gc-call-frame-root.ts`
- `fixtures/core-semantics/gc-high-pressure-root.ts`
- `fixtures/core-semantics/gc-object-root.ts`
- `fixtures/core-semantics/gc-transient-allocation.ts`
- `fixtures/core-semantics/ordinary-function-closure-gc-pressure.ts`
- `fixtures/core-semantics/private-class-field-internal-slot-gc.ts`
- `scripts/data/semantic-canary.txt`

### Issues and plans

- `issues/README.md`
- `issue-views/index.json` generated during investigation via `python3 scripts/issue-index.py`
- `plans/457-harness-compiler-gaps.md`
- `plans/5000-parser-syntax-coverage.md`
- `plans/5004-runtime-builtins-coverage.md`
- `.agents/plans/5029-direct-binary-emission.md`
- `.agents/plans/5030-split-runtime-emitters.md`
- `.agents/plans/5032-capability-detection.md`
- `.agents/plans/5041-expr-fixture-coverage.md`
- `.agents/plans/5043-split-parser.md`
- `.agents/plans/5044-ambient-erasure.md`
- `.agents/plans/5052-abi-memory-map.md`

## Self-check

- [x] Every requirement has affected files
- [x] Every requirement has tests
- [x] Every requirement has acceptance commands
- [x] Every task maps to at least one requirement
- [x] Every requirement maps to at least one task
- [x] Every gate is executable or has a task that makes it executable
- [x] No task is purely vague research
- [x] Non-goals are explicit
- [x] Existing supported behavior is protected
- [x] The design can be considered complete when all tasks are done
