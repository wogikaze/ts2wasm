# IR Layer Completion: HIR → MIR → WasmIR Semantic Analysis

このドキュメントは、IR layer（Semantic HIR、MIR、WasmIR）を「完成」とみなすための最終形、移行ゲート、並列実装 issue 分割を定義する。
進捗ログではなく、実装・レビュー・issue 起票のための canonical contract として扱う。

## Scope

対象範囲は次の compiler phase boundary である。

```text
Source
  -> AST
  -> BuiltinResolved AST
  -> Validated<HirProgram>
  -> Optimized HirProgram（任意）
  -> Validated<MirProgram>
  -> WasmModule / WasmInstr
  -> WAT または wasm binary
```

この文書で扱うもの:

- HIR が保持する JavaScript semantic operation
- HIR → MIR lowering と MIR validation
- MIR から backend emission へ渡る runtime ABI intent
- WasmIR typed backend module model
- 既存 `LoweredProgram` bridge から native MIR / WasmIR path へ移るための gate
- 並列実装できる issue slice と dependency graph

この文書で扱わないもの:

- 個別 builtin family の全面実装
- full JavaScript conformance の総合ロードマップ
- runtime ABI の tagged value layout 詳細
- host capability policy の詳細

それらは `docs/05-compatibility-and-semantics.md`、`docs/14-runtime-abi.md`、`docs/24-architecture-decoupling-and-llm-friendly-sizing.md`、`docs/language-reference/*.md` に委譲する。

## Current implementation snapshot

2026-05-12 時点で、この領域は「設計だけ」ではない。すでに operational な部分と、bridge / alias のまま残っている部分が混在している。

| Layer | Current state | Completion gap |
|---|---|---|
| HIR | `crates/ir/src/semantic.rs` に `HirProgram` / `HirStmt` / `HirExpr`、`lower_to_hir`、`validate_hir` がある。初期 slice は実装済み。 | 対応構文を P16 semantic correctness matrix と同期し、UnsupportedSyntax 境界を fixture 化する。 |
| HIR → MIR | `crates/ir/src/lowered/hir_to_mir.rs` の `lower_hir_to_mir` が現行 HIR variant を `LoweredProgram` 互換に下ろす。 | Span/metadata の損失、method call runtime intent、native MIR model への移行を詰める。 |
| MIR | `crates/ir/src/lowered/mir.rs` では `MirProgram = LoweredProgram` alias。`Validated<MirProgram>::new_mir` と `validate_mir` wrapper はある。 | Alias ではなく独立した `MirProgram` / `MirStmt` / `MirExpr` にする。 |
| MIR backend path | `emit_mir` / `emit_mir_wat` があり、`Validated<MirProgram>` を受け取れる。 | 現状は `mir_emit` bridge が standard emitter に委譲する。native MIR emitter は未完。 |
| WasmIR | `backend-core` に typed `WasmInstr` / `WasmModule` があり、`WatWriter::emit_module` と feature-gated `wasm-encoder` path がある。 | main build pipeline の全面入力にはまだなっていない。raw WAT legacy helper を段階移行する。 |

このため、完了判定は「IR 名が存在するか」ではなく、**default pipeline が validated phase boundary を通り、backend が native MIR / typed WasmIR を primary input として使えるか**で判断する。

## Final-state definition

完成形では、compiler は以下の境界を満たす。

```text
BuiltinResolved AST
  -> lower_to_hir
  -> validate_hir
  -> optimize_hir（optional, semantics preserving）
  -> lower_hir_to_mir
  -> validate_mir
  -> emit_mir_to_wasm_ir
  -> validate_wasm_module / wasm-tools validate
  -> WAT or wasm binary
```

最終状態の重要な性質:

1. **HIR は JS semantic source of truth** である。parser syntax、runtime ABI、wasm stack layout を持たない。
2. **MIR は runtime ABI intent** である。`RuntimeFn`、local/function/module IDs、control-flow shape、heap/value representation への lowering intent を持つ。
3. **WasmIR は backend output model** である。WAT string は output format であり、中間意味表現ではない。
4. **Backend public API は validated input だけを受け取る**。`Validated<HirProgram>`、`Validated<MirProgram>`、`ValidatedRuntimeLinkPlan` を phase boundary とする。
5. **Legacy `LoweredProgram` path は bridge / compatibility path** に格下げされ、default path ではなくなる。
6. **Semantic regression gate が pipeline switch を守る**。build pass だけでは default 化しない。

## HIR completion contract

### Responsibility

HIR は、JavaScript の observable semantics を backend から切り離す。

HIR が持つべき情報:

- local / function / builtin receiver の解決済み ID
- `ToBoolean`、`ToNumber`、`ToString`、`ToPropertyKey` などの abstract operation
- `JsAdd`、equality、relational、unary などの operator semantics
- property / index access と method receiver
- builtin / user function / method call の区別
- semantic control flow（truthy branch、loop、return、throw、completion boundary）
- diagnostics と optimizer のための source span / metadata

HIR が持ってはいけない情報:

- wasm local index / stack layout
- WAT string
- runtime helper function の string name
- host import / capability decision
- parser の raw identifier fallback

### Current supported slice

初期 HIR slice は以下を扱える。

- statements: `let`、assignment、expression statement、`if`、`while`、`return`
- expressions: `undefined` / `null` / boolean / number / bigint / string constants
- local load / builtin load
- `ToBoolean` / `!`
- add / equality / relational basics
- property get / computed index / array length
- builtin call / direct user function call / method call

未対応構文は silently fallback ではなく、`UnsupportedSyntax` で明示的に境界化する。

### Completion criteria

HIR layer は以下を満たしたとき完成とする。

- P16 semantic correctness matrix で、HIR が担当する構文・semantic operation の pass/mismatch が fixture 化されている。
- `validate_hir` が local/function ID、top-level return、truthy condition、function table index、receiver loss を検出する。
- HIR dump / unparse / snapshot tests が representative variant を覆う。
- HIR に variant を追加すると、dump、validator、HIR→MIR lowering、snapshot の更新が必須になる。
- unsupported boundary は diagnostic と coverage 集計に残り、無音で legacy lowering に吸収されない。

## MIR completion contract

### Responsibility

MIR は HIR の semantic operation を runtime ABI intent に変換する。

MIR が持つべき情報:

- runtime call: typed `RuntimeFn`
- locals / functions / modules / class prototypes / closure captures の runtime-level IDs
- control-flow shape: block、loop、branch、return、throw、try/catch/finally、switch
- value representation intent: RawValue / HeapPtr / logical JS value の境界
- builtin / method / user function call の runtime dispatch intent
- backend が wasm primitive に落とせる typed primitive operation

MIR が持ってはいけない情報:

- parser AST node
- unresolved source name
- HIR-only abstract semantic operation
- raw WAT string
- host capability policy decision

### Alias bridge policy

現状の `MirProgram = LoweredProgram` は有効な bridge だが、完成形ではない。

Alias bridge の扱い:

- `Validated<MirProgram>` backend path の smoke / compatibility に使える。
- `validate_mir` は native MIR validator が入るまで `validate_lowered` wrapper として扱う。
- `mir_emit` は native emitter が入るまで standard emitter への委譲 bridge として扱う。
- issue / docs では `MIR alias` と `native MIR` を区別して記述する。

### Native MIR completion criteria

Native MIR は以下を満たしたとき完成とする。

- `MirProgram` / `MirFunction` / `MirStmt` / `MirExpr` が alias ではない独立型として存在する。
- `From<LoweredProgram>` または compatibility bridge があり、移行中の backend parity を測れる。
- `lower_hir_to_mir` が native MIR を直接返す。
- `validate_mir` が native MIR の invariant を検査する。
- `emit_mir_wat` が native MIR を直接読める subset から始まり、bridge 委譲を段階的に縮小する。
- MIR dump / snapshot が HIR と別の runtime intent を示す。
- runtime call は string ではなく typed `RuntimeFn` を使う。

## WasmIR completion contract

### Responsibility

WasmIR は wasm module structure と instructions の typed representation である。

WasmIR が持つべき情報:

- imports / globals / memory / data segments / functions / exports
- typed wasm value types
- typed instructions (`WasmInstr`) と structured control-flow
- WAT writer と binary encoder に共有される module input

WasmIR が持ってはいけない情報:

- JS semantic decision
- HIR / MIR の unresolved runtime intent
- runtime catalog にない import
- arbitrary WAT string。ただし audited legacy escape hatch は migration 中のみ許可する。

### Completion criteria

WasmIR layer は以下を満たしたとき完成とする。

- backend の新規 emission code は `WasmModule` / `WasmInstr` / `WatWriter` helper を優先する。
- `WasmInstr::Raw` は audited legacy escape hatch として、用途・削減計画・test を持つ。
- representative module fixtures で WAT writer と wasm-encoder path の parity を確認する。
- generated WAT / wasm binary が validation tool を通る。
- runtime link plan / capability manifest と import emission が一致する。

## Semantic correctness gate

Default pipeline switch は semantic correctness gate の後に行う。

最低限の gate:

- semantic canary suite が pass する。
- function/call semantic suite が pass する。
- method receiver、arity、builtin call、dynamic call rejection が fixture 化されている。
- HIR lowering supported subset と UnsupportedSyntax boundary が feature matrix に記録されている。
- `build_pass` 増加だけでなく `semantic_pass` / mismatch / runtime_error の変化が比較されている。
- pipeline switch 前後で representative fixtures の stdout が Node reference と一致する。

## Migration gates

| Gate | Name | Exit criteria | Allows |
|---|---|---|---|
| C0 | Docs and tracking sync | `docs/13-ir-contracts.md` とこの文書が現コードを正しく表す。issue index が生成される。 | 並列実装開始 |
| C1 | P16 semantic correctness gate | canary + function/call suite + HIR support matrix が pass / recorded。 | default path switch issue を着手可能にする |
| C2 | Native MIR model | alias ではない MIR 型、compat bridge、validator skeleton、snapshots がある。 | native MIR emitter work |
| C3 | Native MIR emitter subset | selected MIR subset が bridge なしで WAT を出し、legacy path と parity を持つ。 | feature-gated pipeline path |
| C4 | Feature-gated HIR→MIR→emit path | CLI/compiler に opt-in path があり、fallback と比較できる。 | default switch rehearsal |
| C5 | Default path switch | P16 gate + C3/C4 parity が pass。fallback は diagnostic/compat mode のみ。 | LoweredProgram bridge 縮小 |
| C6 | Typed WasmIR backend expansion | selected runtime/emitter domain が typed WasmIR に移り、WAT/binary parity tests を持つ。 | raw WAT legacy helper 削減 |

## Parallel implementation lanes

以下の issue は `issues/` に起票済み。依存のないものは別 worktree で同時に進められる。

| Issue | Lane | Work | Depends on | Main conflict zone |
|---|---|---|---|---|
| `I-20260512-FNCSEM` | Semantic gate | Function/call semantic suite | none | fixtures, semantic tests |
| `I-20260512-HRSCVR` | Semantic gate | HIR support matrix and UnsupportedSyntax fixtures | none | `crates/ir/src/semantic.rs`, docs |
| `I-20260512-P6GATE` | Gate | P16 correctness gate baseline | `FNCSEM`, `HRSCVR` | coverage scripts, docs/current-state |
| `I-20260512-MRDATA` | MIR model | Native MIR type model and bridge | none | `crates/ir/src/lowered/mir.rs` |
| `I-20260512-MRVADT` | MIR validation | Native MIR validator, dump, snapshots | `MRDATA` | `crates/ir/src/lowered/*`, tests |
| `I-20260512-MREWAT` | MIR backend | Native MIR→WAT subset emitter | `MRVADT` | `crates/backend-wasm/src/mir_emit.rs` |
| `I-20260512-HMPATH` | Compiler pipeline | Feature-gated HIR→MIR→emit path / default switch rehearsal | `P6GATE`, `MREWAT` | compiler stages, CLI flags |
| `I-20260512-WASMDM` | WasmIR | Move one emitter/runtime domain to typed WasmIR | none | backend-wasm emitter/runtime domain |
| `I-20260512-WAENCD` | WasmIR | wasm-encoder parity fixtures for typed WasmModule | `WASMDM` | backend-core/backend-wasm tests |

Recommended first wave:

```text
worktree A: I-20260512-FNCSEM
worktree B: I-20260512-HRSCVR
worktree C: I-20260512-MRDATA
worktree D: I-20260512-WASMDM
```

Recommended second wave:

```text
worktree E: I-20260512-P6GATE  after A+B
worktree F: I-20260512-MRVADT  after C
worktree G: I-20260512-WAENCD  after D
```

Recommended third wave:

```text
worktree H: I-20260512-MREWAT  after F
worktree I: I-20260512-HMPATH  after E+H
```

## Issue acceptance command set

Use the smallest command set that proves the boundary touched by the issue.

HIR / MIR model and validator issues:

```bash
cargo test -p ts2wasm-ir --test hir_to_mir
cargo test -p ts2wasm-ir hir_to_mir
cargo test -p ts2wasm-ir validate_mir
```

Backend MIR / WasmIR issues:

```bash
cargo test -p ts2wasm-backend-core
cargo test -p ts2wasm-backend-wasm mir
cargo test -p ts2wasm-backend-wasm wasm_ir
```

Pipeline and semantic gate issues:

```bash
cargo test -p ts2wasm-cli --test m2_node_diff
cargo test -p ts2wasm-cli --test m6_builtin_methods
python3 scripts/gate/semantic-regression.py
python3 scripts/run/reference-coverage.py --compare-baseline
```

Issue hygiene:

```bash
python3 scripts/issue-lint.py
python3 scripts/issue-index.py
python3 scripts/check/tracking-consistency.py
```

Commands may differ by local runner availability. When a command cannot run because the checkout lacks workspace files or external tools, record that explicitly in the issue evidence instead of replacing it with a weaker claim.

## Review checklist

Before closing an IR-layer issue, confirm:

- the changed phase owns the decision it now makes;
- no parser AST type leaked into backend public API;
- no raw runtime function string replaced a typed `RuntimeFn`;
- `validate_*` rejects the invalid shape introduced by the feature;
- dump / snapshot output changed intentionally;
- semantic fixtures cover observable JS behavior, not only build success;
- docs distinguish current bridge state from final native state;
- issue evidence includes commands, exit status, and any unavailable-tool caveat.

## Completion statement

This field is complete when the default compiler path can be described as:

```text
validated semantic HIR
  -> validated native MIR
  -> typed WasmIR module
  -> WAT/binary output
```

and when `LoweredProgram` is no longer the primary semantic/backend boundary, but only a compatibility bridge or removed legacy type.
