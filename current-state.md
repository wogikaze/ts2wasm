# Current State

Last updated: 2026-04-29

この文書は、現在の実装状態と検証の事実だけを記録する。設計は `docs/` 側に置き、ここでは「今何が動くか」「何が未実装か」「何を確認すればよいか」を扱う。

## Current gate（このリポジトリが今要求する最小バー）

正本は `docs/11-shared-definitions.md` の Workstreams / Gates。実装・レビューでまず満たすのは次の組み合わせとする。

- **Gate A（テスト）**: `cargo fmt --all --check` と `cargo nextest run`（フル suite。重いテストを分離する場合は `docs/11` の filterset 方針に従う）。
- **Gate D（coverage artifact）**: `mise run update-coverage-matrix -- --check` が `artifacts/coverage/reference-coverage-matrix.md` を検証。
- **その他（B–C, E–G）**: ポリシーと checklist は `docs/11` / `docs/12-coding-standard.md`（§19）に記載。証拠コマンドは下記「Last verified commands」。

## Last verified commands（代表）

開発者がローカルで再現する際の最小セット（CI と揃える場合はワークフローを参照）。

```bash
cargo fmt --all --check
cargo nextest run
mise run update-coverage-matrix -- --check
mise run check-scripts
mise run check-fast-gate -- --skip-nextest
mise run check-manifest-imports
mise run check-fixture-catalog
mise run check-architecture-rules
mise run check-compiler-diagnostics
mise run update-issue-index -- --check
mise run check-issue-health
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
- `Array.prototype.push` supports the current direct array receiver boundary plus array-like object receivers through `obj.push = Array.prototype.push; obj.push(value)` and `Array.prototype.push.call(obj, ...)`, with Node/iwasm differential coverage. In addition, unused statement-form local-array `arr.push(value);` now has a narrow grow path that uses the GC allocation body size as array capacity, mutates in place when possible, and reallocates/copies when capacity is exhausted; `fixtures/core-semantics/array-push-recursive-growth.ts` covers the ABC451 depth-3 reducer.
- Module cache test now passes (require_cache_reuses_same_object_at_runtime_semantic_diff).
- class / node-api fixture と broader module fixtures は build 成功の確認は通過しているが、semantic parity は `m2_node_diff.rs` 側で未確定として明示している。Static named ES module import/export の narrow local fixture は Node/iwasm differential で一致。

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
- 列 `build_pass` / `semantic_pass` は `mise run reference-coverage` の出力に対応（semantic-pass は Node + `iwasm` が利用可能な環境でのみ増分）。
- issue 060 is closed against a fixed `test262` window: `TS2WASM_REFERENCE_ROOT=./reference mise run reference-coverage -- test262 --limit 17000 --detail` passed on 2026-04-29 with `unknown-unsupported=0` (executed=17000; build_pass=5; semantic_pass=3; unsupported=16994; blocked=1). Later stored matrix context includes test262 limit 18000, tsc limit 200, and tsgo limit 120. The exact assigned tsc root `./reference` lacks `TypeScript`, so tsc validation from that root remains a residual external-reference risk; existing tsc evidence used `/tmp/ts2wasm-issue060-reference`. Future unknown-unsupported expansion should use separate fixed-window ramp issues.

## Implemented (high-level)

- frontend crate（`crates/frontend`）: AST/span/diagnostic/token definitions plus lexer/parser implementation; integer numeric literal separators are accepted for decimal, binary, octal, and hexadecimal literals
- BigInt literal syntax is recognized by the frontend for decimal, binary, octal, and hexadecimal forms with `n` suffix. BigInt literal runtime values now lower to object-tagged heap BigInt values with Node/iwasm coverage for literal printing, `typeof`, `String(bigint)`, concatenation/template ToString, and truthiness for `0n` / non-zero values. `BigInt(...)` supports string literal inputs for signed decimal, unsigned binary/octal/hex, and empty/whitespace-to-zero forms, plus static boolean/integer number/unary-negative integer number/BigInt literal inputs and dynamic boolean/tagged-int/BigInt inputs. Dynamic definitely-string `BigInt(value)` inputs now lower through the runtime helper for ASCII-trimmed decimal strings with optional sign, unsigned `0b` / `0o` / `0x` prefixes, and empty/whitespace-to-zero, within the current single-limb/u64 BigInt representation; dynamic invalid/out-of-range strings trap until compatible runtime exception throwing is implemented. `BigInt.asIntN` / `BigInt.asUintN` fold literal BigInt inputs for literal bit widths `0..=64`, including the same supported number/BigInt literals when they are referenced through simple identifier-bound inputs before a later assignment invalidates the binding; they also lower dynamic bit-width inputs in `0..=64` over guarded signed-i64-backed BigInt values through runtime helpers with Node/iwasm coverage. BigInt arithmetic has compile-time folding for literal unary minus and literal `+`, `-`, `*`, `/`, `%`; dynamic unary minus and `+`, `-`, `*`, `/`, `%` over known BigInt locals and known-local/literal operand pairs have a signed-i64-backed runtime helper slice guarded by pre-lowering diagnostics for out-of-slice values, including values assigned through nested control flow and large dynamic multiplication results. Dynamic division/remainder by zero in that helper slice now builds and traps at runtime, with Node `RangeError` baseline coverage; compatible JS `RangeError` object throwing remains issue-260 work. BigInt/BigInt strict equality, abstract equality, and relational comparison now have Node/iwasm coverage for the current heap BigInt representation; literal BigInt/String abstract equality folds supported StringToBigInt forms and invalid strings to Node-compatible booleans; literal BigInt/Boolean abstract equality folds `false` as `0n` and `true` as `1n`; literal BigInt/Number abstract equality folds representable tagged-int number literals; literal BigInt/nullish abstract equality folds to Node-compatible false/true. Other statically visible mixed BigInt abstract equality/relational coercion is diagnosed and runtime-only mixed cases trap rather than silently returning false. Full multi-limb runtime arithmetic, broader number model edges (`NaN`, `Infinity`, `-0`, fractional), dynamic invalid-string exception parity, and dynamic mixed BigInt coercion remain unsupported and tracked by issues 260, 281, 282, and 280.
- compiler/driver crate（`crates/compiler`）: build pipeline, dump pipeline, AST validation, lowering orchestration
- TypeScript compiler API oracle: explicit `ts2wasm check <input.ts>` and `ts2wasm_frontend::check_typescript_file` type-check through the local TypeScript devDependency; the oracle also exposes binding/parameter/binary-expression type hints and number/string optimization candidates. Normal `build` does not invoke tsc.
- WAT/WASM emitter と runtime subset（`crates/backend-wasm`）
- shared schema crate（`crates/shared`）: ABI/capability/test status
- IR crate（`crates/ir`）: resolved/lowered IR
- Semantic HIR initial slice（`crates/ir::semantic`）: `ResolvedStmt` lowers to JS semantic operations such as `JsAdd`, `ToBoolean` branch conditions, property/index access, builtin calls, and method calls; backend still consumes `LoweredProgram`.
- CLI dump command: `ts2wasm dump` can emit tokens, AST, resolved AST, lowered IR, WAT, and AST pseudo-source via `--ast --unparse`
- Static ES module graph diagnostics are implemented for the compiler-side graph slice: parsed source-bearing static imports/re-exports are scanned from the entry file and reachable local relative modules, local `./` / `../` specifiers are resolved deterministically with `.ts` before `.js`, local cycles are represented by stable existing module IDs without recursive graph growth, bare or missing local specifiers produce source diagnostics before lowering/emission, and the compiler API exposes the graph's stable module IDs, canonical source paths, dependency edges, and dependency-first once-only initialization steps for downstream binding/lowering work.
- Static named ES module import/export execution is implemented for the narrow local `import { value } from "./static-entry-source";` plus source-side literal `export const value = 1;` slice. The build path lowers named imports to `PropertyGet(ModuleLoad { module_id }, export_name)`, attaches reachable local literal `export const` declarations to `LoweredProgram.modules` as explicit `LoweredStmt::Export` statements using the module graph IDs, emits dependency-first module initializers before top-level import reads, and has Node/iwasm differential coverage for direct import, alias import, importer lexical shadowing, and repeated imports from the same source module. Missing named exports from existing local modules have issue-233 diagnostic coverage at the imported name span. Live binding updates, default/namespace/dynamic imports, package resolution, and broader module body semantics remain out of scope.
- Destructuring binding runtime support is implemented for the current simple identifier-only subset: dense array declarations, object shorthand/identifier alias declarations, ordinary function parameter patterns, arrow parameter patterns, array elisions, array rest bindings, nested array/object bindings, static object-literal rest declarations, literal default initializers, and ordinary function whole-pattern parameter defaults have Node/iwasm differential coverage. Dynamic-source/parameter object rest binding, non-literal default initializers, and broad iterator semantics remain issue-251 unsupported diagnostics.
- Backend runtime-link planning now scans explicit lowered `ModuleInfo.statements`, so future ES module export statements select module export helpers through the runtime catalog, while empty module metadata does not select ES module export helpers. This is a link-plan contract only; runtime module execution parity remains tracked by issues 233 and 234.
- runtime-abi crate（`crates/runtime-abi`）: RawValue/layout/ABI
- reference coverage パイプライン（`mise run reference-coverage`, `mise run update-coverage-matrix`, `mise run update-coverage-matrix -- --check`）
- generated coverage table（`artifacts/coverage/reference-coverage-matrix.md`）
- issue queue index（`issues/index.md` の Ready/Blocked/Done 表は `mise run update-issue-index` が生成、`mise run check-issue-health` で整合検証。`mise run check-issue-index` は互換 alias）
- harness scripts（`mise run check-fast-gate`、`mise run check-manifest-imports`、`mise run check-test-records-schema`、`mise run check-fixture-catalog`、`mise run check-architecture-rules`、`mise run check-compiler-diagnostics`；pre-push は `.githooks/pre-push`）

## Known blockers / gaps

- **Crates module migration**: `crates/frontend`, `crates/ir`, `crates/runtime-abi`, and `crates/backend-wasm` code migrations are complete. Issue 026 is closed with full workspace nextest evidence.
- **CLI ownership**: `crates/cli` is now a thin binary/re-export wrapper for compiler APIs. Lexer/parser implementation has moved to `crates/frontend`; backend runtime emission is split into focused modules and no repo-owned source file currently exceeds the 2000-line architecture warning threshold.
- TypeScript compiler API の明示 type-check oracle と optimization hint 抽出は実装済み。production build pipeline は tsc を必須にしない。
- Generic JavaScript semantic IR is implemented as an initial validated HIR slice. The build pipeline validates supported HIR lowering opportunistically; broader backend consumption remains follow-up implementation work.
- typed IR dump (`ts2wasm dump --tir`) and optimizer dump (`ts2wasm dump --optimize`) are tracked by issues 204 and 205.
- full wasm backend は未実装（現状は WAT 中心）
- test262 full differential 運用は未完（sample/ramp が中心）
- GC 実装は未完。`$alloc_heap` は GC header、memory-headroom-aware allocation threshold hook、bounded 16-page minimum `memory.grow`、GC mark phase は module cache / class prototype globals / heap graph payload / heap closure capture slots / top-level local root table / active function call-frame root stack / backend temporary roots を走査し、sweep/free-list reuse と free-block splitting を行う。managed `$concat` allocation は transient/object-root/high-pressure/call-frame/closure allocation fixtures で通っている。OOM check と UTF-8 literal basic support は完了済みだが、UTF-16 parity / encode-decode helper は追跡対象。
- Ordinary `number` remains an integer-only subset. Tagged small-int numbers are still used for values in `ValueTag` payload range; issue 300 added a narrow heap-number path for `i32` integer values outside that range, storing an object-tagged heap payload with `HEAP_NUMBER_SENTINEL` and cached decimal bytes. This is verified by `fixtures/core-semantics/large-integer-number-boundary.ts` for `2 ** i <= 1000000000`, `String`/unary-plus round trip, Set identity, and numeric sort values up to `819264512`. Fractional values, `NaN`, `Infinity`, and `-0` remain outside this subset.
- ABC451 D is not sample-compatible yet. The rewritten fixture builds past the previous `NumberOutOfRange` blocker and the array-push/free-list reducers pass. Issue 304 raised the bounded standalone memory cap from 42 to 185 pages: the depth-8 ABC451 live-set reducer now prints Node-matching `292743` under `iwasm`, while the OOM regression still traps intentionally. Issue 308 added a memory-headroom-aware GC cadence slice that keeps the committed cap at 185 pages, grows in 16-page chunks when below the cap, and delays allocation-pressure GC until the bump result is within 12 pages of reserved memory; bounded depth-9 telemetry at 1,000,000 allocations improves from 834 collections / 196,941,253 sweep visits to 790 collections / 192,697,486 sweep visits. The depth-9 reducer still traps under the committed cap, and the full official sample inputs `10`, `69`, and `1099898` still require issue 300 follow-up before compatibility can be claimed.
- host-deny / capability manifest の base path は実装済み。`Math.random()` は WASI `random_get` を import し、manifest に `wasi.random: true` と audit reason を記録する。`docs/06` の required test classes に沿った監査範囲の拡張は継続対象。

Semantic gap tracking:

- class 系: `crates/cli/tests/m8_oop_classes.rs`（build_smoke）。semantic status は `crates/cli/tests/m2_node_diff.rs` の class gap アサーションで管理。
- module 系: `crates/cli/tests/m9_modules.rs`（build_smoke）。Static named ES module import/export の narrow local fixtures は `crates/cli/tests/m2_node_diff.rs` で Node/iwasm differential 一致。broader module semantic status は同ファイルの module gap アサーションで管理。
- node API 系: `crates/cli/tests/m10_node_apis.rs`（build_smoke）。semantic status は `crates/cli/tests/m2_node_diff.rs` の node_api gap アサーションで管理。
- `instanceof` now has Node differential coverage for ordinary class constructors, inherited class prototype chains, non-object left operands, and objects manually linked with `Object.setPrototypeOf`; custom `Symbol.hasInstance` remains out of scope.
- BigInt literals are parser-classified as explicit AST nodes and invalid non-separator forms such as fractional/exponent BigInt, invalid radix digits, and decimal leading-zero BigInt produce issue-244 diagnostics. Literal runtime values are implemented as object-tagged heap BigInts with a canonical sign/first-limb prefix plus cached decimal bytes for the current observable slice. Literal arithmetic folds in the resolver with arbitrary-size decimal math, then emits a BigInt literal heap object. Dynamic unary minus and `+`, `-`, `*`, `/`, `%` over known BigInt locals and known-local/literal operand pairs now lower to issue-260 runtime helpers with Node/iwasm differential coverage, but those helpers reconstruct through signed i64 and the existing first-limb/cached-decimal constructor only when the resolver proves operands/results are in slice; large dynamic values, control-flow-assigned out-of-slice values, dynamic large multiplication results, and dynamic mixed Number/BigInt arithmetic report issue-260 diagnostics. Dynamic division/remainder by zero in the signed-i64 helper slice lowers and traps at runtime; Node baseline for the same fixtures is `RangeError: Division by zero`, but compatible JS exception throwing is still open. BigInt/BigInt `===`, `!==`, `==`, `!=`, `<`, `<=`, `>`, and `>=` compare mathematical values through sign plus cached decimal magnitude. Literal BigInt/String `==` and `!=` fold through the supported StringToBigInt subset, including invalid-string false/true behavior; literal BigInt/Boolean `==` and `!=` fold through `false -> 0n` and `true -> 1n`; literal BigInt/Number `==` and `!=` fold for representable tagged-int number literals; literal BigInt/nullish `==` and `!=` fold to false/true. `BigInt(...)` now supports string literal inputs for signed decimal, unsigned binary/octal/hex, and empty/whitespace-to-zero forms; static boolean/integer number/unary-negative integer number/BigInt literal inputs; dynamic boolean/tagged-int/BigInt inputs; and dynamic definitely-string inputs in the same ASCII StringToBigInt subset within the current single-limb/u64 representation. Dynamic invalid/out-of-range strings trap until compatible runtime exception throwing is implemented. Known-BigInt `String(...)`, concatenation, and template interpolation use BigInt ToString without the `n` suffix, while `console.log(bigint)` preserves the display suffix. `BigInt.asIntN` / `BigInt.asUintN` fold literal BigInt inputs for literal bit widths `0..=64`, fold direct identifier-bound number/BigInt literal inputs when no intervening assignment invalidates the known value, and lower dynamic bit-width inputs in `0..=64` over guarded signed-i64-backed BigInt values through runtime helpers; non-BigInt value inputs remain issue-280 diagnostics. Other statically visible mixed BigInt abstract equality/relational coercion reports issue-linked diagnostics; runtime-only mixed cases trap. Full multi-limb runtime arithmetic, broader number edge equality/comparison, invalid dynamic StringToBigInt exception parity, and dynamic mixed BigInt coercion remain split across issues 260, 281, 282, and 280.
- Date support is a partial epoch-millisecond subset, not full Date support. `new Date(<epoch-ms integer>)` is supported for integer epoch literals, including unary-negative integer literals, and `Date.prototype.getTime()` / `Date.prototype.valueOf()` on those deterministic Date values have Node/iwasm differential coverage. `Date.now()` and no-argument `new Date()` now read WASI Preview 1 realtime clock through `wasi_snapshot_preview1.clock_time_get`, remain standalone under host-deny, and emit `wasi.clock.realtime: true` with source API reasons in the capability manifest. `Date.prototype.toString()` remains blocked on timezone/host formatting policy. Annex B legacy Date methods `getYear`, `setYear`, and `toGMTString` are explicitly diagnosed as issue-241 unsupported behavior in this partial subset.
- Partial feature semantics from historical done issues are tracked by dedicated open issues, not by the done queue. Arrow functions now have Node differential coverage for local binding calls, expression bodies, single-return block bodies, captured locals, lexical `this`, and a closure capture preserved across GC allocation pressure; escaping function values remain outside the current devirtualized local-arrow model. Ordinary non-arrow functions now have Node differential coverage for direct calls, object-receiver method calls through narrow object-literal function references, immutable outer-local capture by a non-escaping nested ordinary function, and basic `arguments.length` / indexed reads for zero, one, and multiple arguments; direct receiver-dependent calls, returned escaping ordinary closures, mutable captured ordinary closure environments, and top-level `arguments` are diagnosed with issue-linked unsupported messages. Rest parameter argument collection now has Node differential coverage for zero, one, and multiple extra arguments. Class constructor and instance-method receiver `this` now has Node differential coverage; class static block statement lists execute at class declaration time in source order for the supported static-method/class subset; non-derived instance private field initializers plus direct `this.#field` read/write use backend-internal private slots with Node/iwasm differential and GC-pressure coverage, direct non-derived instance private methods called as `this.#m()` inside the declaring class have Node/iwasm differential coverage, direct non-derived static private methods called as `this.#m()` from static methods or `Class.#m()` inside the declaring class have Node/iwasm differential coverage, direct non-derived instance private getters read as `this.#x` inside the declaring class have Node/iwasm differential coverage, and direct non-derived instance private setters assigned as `this.#x = value` inside the declaring class have Node/iwasm differential coverage. Ordinary backing-key access/enumeration remains issue-255 guarded while static private fields/accessors, derived-class private initialization, extracted/external private method/accessor access, and full brand-checking semantics remain issue-255; top-level/static/extracted/unknown class receiver forms and static-block `this`/`super` forms are diagnosed with issue-linked unsupported messages. Labeled break/continue now has parser, resolver, lowering, backend emission, invalid-label diagnostics, and Node differential coverage. Switch fall-through/default ordering, template literal interpolation, template interpolation around non-strict legacy octal string escapes, primitive abstract equality, and logical assignment for identifiers/static members/string-literal computed members now have Node differential coverage. Dynamic computed logical-assignment keys on identifier receivers now have Node differential coverage for single key evaluation and RHS short-circuiting; non-identifier logical-assignment targets remain tracked by issue 236. Annex B `[[IsHTMLDDA]]` test262 host hooks are intentionally unsupported browser compatibility forms and now report issue-237 diagnostics for equality, logical, `typeof`, `if`, logical-assignment, and Object.is emulates-undefined paths. Strict legacy octal escapes are diagnosed with issue-linked unsupported messages. String `trim`, `toUpperCase`, and `toLowerCase` now have Node differential coverage for the runtime's byte-oriented ASCII subset; Unicode whitespace/case folding remains outside the current UTF-8/UTF-16 parity model. `Math.random()` no longer uses a deterministic placeholder; it is backed by WASI `random_get`, while full fractional double parity remains tied to the broader number model. Object `ToPrimitive` remains tied to object-model follow-up work.
- The runtime ABI now defines the intended heap closure object contract for escaping ordinary functions/arrows: closure values are `OBJECT_TAG` heap values with a closure sentinel payload, `code_id`, immutable capture count, reserved flags, and raw capture slots. Lowering can represent a returned immutable ordinary closure as heap-closure creation IR and calls through a known heap-closure local as explicit heap-closure dispatch IR. Backend WAT now allocates immutable heap closure objects, dispatches zero- and one-argument returned ordinary closure calls, and marks closure capture slots during GC, with Node/iwasm differential coverage for returned string-reader, `makeAdder(4)(5) -> 9`, and a returned closure retaining a captured heap object across post-return allocation pressure. Mutable captured environments and broader closure dispatch forms remain out of scope.
- JSON has a documented supported-subset contract in issue 052. `JSON.parse` covers whitespace-trimmed primitives, supported strings, small integers, non-integer decimal/exponent number values in the current observable number subset, arrays/objects with nested supported values, ASCII escapes, UTF-16 `\uXXXX` escapes that map to Unicode scalar values, surrogate pairs encoded into the runtime's byte-backed UTF-8 string representation, lone surrogate escapes materialized as U+FFFD under that byte-backed contract, and many malformed-input rejection paths with a selected `SyntaxError: JSON.parse invalid JSON` runtime diagnostic before aborting. `JSON.stringify` covers representable primitives, arrays/objects including nested literals, supported string escaping, numeric/string/ignored/selected boxed `space` forms, boxed `space` edge cases classified by Node differential or issue-052e diagnostics, object-literal array replacers with string/numeric literal plus boxed Number/String property-list entries and selected static ignored entries, and function replacer callbacks for the currently supported value subset including root key, property keys, supported receiver/holder behavior, return filtering, and primitive transformations. Full JSON compatibility remains blocked on broader replacer/object-coercion gaps tracked by issue 052d.

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
Run `mise run update-issue-index` to refresh after adding, closing, or moving issues. The generated Ready queue in `issues/index.md` is the source of truth for current ordering.

## Current Policy

- `docs/` は ADR/設計判断の保存先として扱う。
- 実装の現在地と検証手順の要約はこの `current-state.md` を正とする。
- coverage 実測値は `artifacts/coverage/reference-coverage-matrix.md` を正とする。
- project goal、gates、schema は `docs/11-shared-definitions.md` を正とし、他 doc で再定義しない。
