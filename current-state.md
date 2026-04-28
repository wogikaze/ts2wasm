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
- issue 060 progress classified current `unknown-unsupported` coverage windows into concrete feature labels; stored matrix rows now include test262 limit 15000, tsc limit 200, and tsgo limit 120. The test262 limit-15000 row has zero `unknown-unsupported`; no new classifier labels or follow-up issues were needed beyond existing labels. The exact assigned tsc root `/home/wogikaze/wgkz/ts2wasm/reference` lacks `TypeScript`, so the stored tsc row was refreshed with the existing `/tmp/ts2wasm-issue060-reference` checkout.

## Implemented (high-level)

- frontend crate（`crates/frontend`）: AST/span/diagnostic/token definitions plus lexer/parser implementation
- compiler/driver crate（`crates/compiler`）: build pipeline, dump pipeline, AST validation, lowering orchestration
- TypeScript compiler API oracle: explicit `ts2wasm check <input.ts>` and `ts2wasm_frontend::check_typescript_file` type-check through the local TypeScript devDependency; the oracle also exposes binding/parameter/binary-expression type hints and number/string optimization candidates. Normal `build` does not invoke tsc.
- WAT/WASM emitter と runtime subset（`crates/backend-wasm`）
- shared schema crate（`crates/shared`）: ABI/capability/test status
- IR crate（`crates/ir`）: resolved/lowered IR
- Semantic HIR initial slice（`crates/ir::semantic`）: `ResolvedStmt` lowers to JS semantic operations such as `JsAdd`, `ToBoolean` branch conditions, property/index access, builtin calls, and method calls; backend still consumes `LoweredProgram`.
- CLI dump command: `ts2wasm dump` can emit tokens, AST, resolved AST, lowered IR, WAT, and AST pseudo-source via `--ast --unparse`
- Static ES module graph diagnostics are implemented for the compiler-side graph slice: parsed source-bearing static imports/re-exports are scanned from the entry file and reachable local relative modules, local `./` / `../` specifiers are resolved deterministically with `.ts` before `.js`, local cycles are represented by stable existing module IDs without recursive graph growth, bare or missing local specifiers produce source diagnostics before lowering/emission, and the compiler API exposes the graph's stable module IDs, canonical source paths, dependency edges, and dependency-first once-only initialization steps for downstream binding/lowering work.
- Static named ES module import build progress exists for the narrow local `import { value } from "./static-entry-source";` plus source-side literal `export const value = 1;` slice. The compiler rewrites that resolved graph-backed import into a buildable local binding so `fixtures/module-system/static-entry.ts` and alias form `fixtures/module-system/static-entry-alias.ts` emit WASM; missing named exports from existing local modules now have issue-233 diagnostic coverage at the imported name span. Broader static module execution semantics and module initialization/runtime parity remain tracked by issues 233 and 234.
- Backend runtime-link planning now scans explicit lowered `ModuleInfo.statements`, so future ES module export statements select module export helpers through the runtime catalog, while empty module metadata does not select ES module export helpers. This is a link-plan contract only; runtime module execution parity remains tracked by issues 233 and 234.
- runtime-abi crate（`crates/runtime-abi`）: RawValue/layout/ABI
- reference coverage パイプライン（`scripts/manager reference-coverage`, `scripts/manager update-coverage-matrix`, `scripts/manager update-coverage-matrix --check`）
- generated coverage table（`artifacts/coverage/reference-coverage-matrix.md`）
- issue queue index（`issues/index.md` の Ready/Blocked/Done 表は `scripts/manager update-issue-index` が生成、`scripts/manager check-issue-health` で整合検証。`scripts/manager check-issue-index` は互換 alias）
- harness scripts（`mise run check-fast-gate`、`mise run check-manifest-imports`、`mise run check-test-records-schema`、`mise run check-fixture-catalog`、`mise run check-architecture-rules`、`mise run check-compiler-diagnostics`；pre-push は `.githooks/pre-push`）

## Known blockers / gaps

- **Crates module migration**: `crates/frontend`, `crates/ir`, `crates/runtime-abi`, and `crates/backend-wasm` code migrations are complete. Issue 026 is closed with full workspace nextest evidence.
- **CLI ownership**: `crates/cli` is now a thin binary/re-export wrapper for compiler APIs. Lexer/parser implementation has moved to `crates/frontend`; backend runtime emission is split into focused modules and no repo-owned source file currently exceeds the 2000-line architecture warning threshold.
- TypeScript compiler API の明示 type-check oracle と optimization hint 抽出は実装済み。production build pipeline は tsc を必須にしない。
- Generic JavaScript semantic IR is implemented as an initial validated HIR slice. The build pipeline validates supported HIR lowering opportunistically; broader backend consumption remains follow-up implementation work.
- typed IR dump (`ts2wasm dump --tir`) and optimizer dump (`ts2wasm dump --optimize`) are tracked by issues 204 and 205.
- full wasm backend は未実装（現状は WAT 中心）
- test262 full differential 運用は未完（sample/ramp が中心）
- GC 実装は未完。`$alloc_heap` は GC header、allocation threshold hook、bounded `memory.grow` を持ち、GC mark phase は module cache / class prototype globals / heap graph payload / top-level local root table / active function call-frame root stack / backend temporary roots を走査し、sweep/free-list reuse と managed `$concat` allocation は transient/object-root/high-pressure/call-frame/closure allocation fixtures で通っている。OOM check と UTF-8 literal basic support は完了済みだが、UTF-16 parity / encode-decode helper は追跡対象。
- host-deny / capability manifest の base path は実装済み。`Math.random()` は WASI `random_get` を import し、manifest に `wasi.random: true` と audit reason を記録する。`docs/06` の required test classes に沿った監査範囲の拡張は継続対象。

Semantic gap tracking:

- class 系: `crates/cli/tests/m8_oop_classes.rs`（build_smoke）。semantic status は `crates/cli/tests/m2_node_diff.rs` の class gap アサーションで管理。
- module 系: `crates/cli/tests/m9_modules.rs`（build_smoke）。semantic status は `crates/cli/tests/m2_node_diff.rs` の module gap アサーションで管理。
- node API 系: `crates/cli/tests/m10_node_apis.rs`（build_smoke）。semantic status は `crates/cli/tests/m2_node_diff.rs` の node_api gap アサーションで管理。
- `instanceof` now has Node differential coverage for ordinary class constructors, inherited class prototype chains, non-object left operands, and objects manually linked with `Object.setPrototypeOf`; custom `Symbol.hasInstance` remains out of scope.
- Partial feature semantics from historical done issues are tracked by dedicated open issues, not by the done queue. Arrow functions now have Node differential coverage for local binding calls, expression bodies, single-return block bodies, captured locals, lexical `this`, and a closure capture preserved across GC allocation pressure; escaping function values remain outside the current devirtualized local-arrow model. Rest parameter argument collection now has Node differential coverage for zero, one, and multiple extra arguments. Class constructor and instance-method receiver `this` now has Node differential coverage; top-level/static/extracted receiver forms are diagnosed with issue-linked unsupported messages. Labeled break/continue now has parser, resolver, lowering, backend emission, invalid-label diagnostics, and Node differential coverage. Switch fall-through/default ordering, template literal interpolation, template interpolation around non-strict legacy octal string escapes, primitive abstract equality, and logical assignment for identifiers/static members/string-literal computed members now have Node differential coverage. Dynamic computed logical-assignment keys on identifier receivers now have Node differential coverage for single key evaluation and RHS short-circuiting; non-identifier logical-assignment targets remain tracked by issue 236, and Annex B `[[IsHTMLDDA]]` compatibility remains tracked by issue 237. Strict legacy octal escapes are diagnosed with issue-linked unsupported messages. String `trim`, `toUpperCase`, and `toLowerCase` now have Node differential coverage for the runtime's byte-oriented ASCII subset; Unicode whitespace/case folding remains outside the current UTF-8/UTF-16 parity model. `Math.random()` no longer uses a deterministic placeholder; it is backed by WASI `random_get`, while full fractional double parity remains tied to the broader number model. Object `ToPrimitive` remains tied to object-model follow-up work.

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
