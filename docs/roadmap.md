# ts2wasm Roadmap

大きな未実装領域を W0-W8 で整理する。

このファイルは **epic 一覧**であり、個別の実装作業は `issues/` で管理する。

## Current State (2026-05-13)

```
issues/ total: 147 (Open: 50, Done: 92, Doing: 5)
test262 coverage:
  total: 53,469 | executed: 9,359 | semantic_pass: 773 | negative_compile_pass: 640
  semantic coverage: ~1.45% | target: >90%
```

**Completed phases:** P13 (architecture decoupling), P14 (architecture hardening), P15 (test hardening)
**Current phase:** P16 (semantic correctness — 79 open feature issues across all layers)
**Target:** test262 >90% semantic coverage

## Issue structure

- issues are **single-layer**: each covers one of Frontend / IR / Runtime / Backend / Host / Coverage
- each issue is **substantial**: completing it significantly advances the project
- all 79 open issues resolved → test262 >90%
- see `issues/README.md` for format and commands

## Roadmap model

W0-W8 は厳密な直列フェーズではない。現在の主要な作業は W4/W5 (builtin semantics + language runtime)。

```txt
issues/        = 作業台帳（147件, 50 open）
docs/roadmap.md = 大きな未実装領域の分類
```

## Rules

- Roadmap item は epic であり、そのまま `issues/` にコピーしない。
- `issues/` の item は acceptance command と done evidence を必須にする。
- coverage / reference-coverage の結果から `issues/` を自動生成しない。

## Open feature issues by layer

| Layer | Issues | P1 | P2 | P3 | Description |
|-------|--------|----|----|----|-------------|
| runtime | 31 | 4 | 21 | 6 | WAT runtime functions for builtins and language features |
| ir | 14 | 2 | 8 | 4 | IR lowering, name resolution, state machines |
| coverage | 7 | 4 | 3 | 0 | test262 ramp, canary, negative verification, differential, perf |
| frontend | 5 | 0 | 4 | 1 | Syntax parsing for remaining constructs |
| semantics | 3 | 0 | 3 | 0 | Resolution and dispatch |
| backend | 2 | 1 | 1 | 0 | wasm-encoder parity |
| host | 2 | 1 | 1 | 0 | $262 harness, Node.js shim |
| compiler | 1 | 0 | 1 | 0 | Pipeline integration |

**Ready to work: 45 items** | **P1 open: 13 items**

## Gate overview

| Gate | Meaning | Status |
|------|---------|--------|
| Gate A | Stable runtime substrate | ✅ W0 done |
| Gate B | Standalone WASI execution | ✅ W1 done |
| Gate C | Parser does not block common fixtures | ✅ W2 done |
| Gate D | Known names and builtin dispatch are explicit | ✅ W3 done |
| Gate E | Core runtime builtins are implemented or precisely rejected | 🔄 W4 in progress (31 open runtime issues) |
| Gate F | JS/TS runtime semantics become coherent | 🔄 W5 in progress (14 open IR issues) |
| Gate G | test262 ramp is measurable and regression-safe | 🔄 W6 in progress (7 open coverage issues) |
| Gate H | Host capability boundary is auditable | ✅ W7 done |
| Post-Gate H | Optimization and large backend replacement | ⏳ W8 deferred |

---

## W0: Runtime substrate / ABI baseline ✅ COMPLETE

- [X] Minimal binary WASM emitter path for simple programs
- [X] Runtime core raw WAT reduction where it blocks maintainability
- [X] ABI contract document: logical values vs wire representation
- [X] ABI mismatch tests for `i64` logical value vs `i32` wire handle/value representation
- [X] Runtime value representation smoke tests under iwasm/WAMR

## W1: Standalone WASI baseline ✅ COMPLETE

- [X] WASI args/env/exit/clock/random host imports
- [X] Standalone iwasm/WAMR smoke test coverage
- [X] Capability manifest entries for every WASI import

## W2: Syntax acceptance and precise rejection ✅ COMPLETE

- [X] All common JS/TS syntax accepted or precisely rejected
- [X] JSX, decorators, module augmentation → precise diagnostics
- [X] TypeScript erasure (enum, namespace, type-only imports, etc.)
- [X] Remaining parser gaps tracked: hashbang (#474), import assertions (#475), shorthand (#443)

## W3: Name/call resolution and builtin dispatch ✅ COMPLETE

- [X] Core ECMAScript global builtin names registered
- [X] TypedArray constructor names (11 types)
- [X] Well-known symbols (iterator, toStringTag, hasInstance, toPrimitive, for, keyFor)
- [X] Builtin method dispatch table (program_builtins.rs)
- [ ] **Remaining:** test262 harness globals (#457, ~13,426 tests)

## W4: Builtin API semantics 🔄 IN PROGRESS

Goal: implement selected builtins after names and dispatch paths are explicit.

**Runtime issues (31 open):**

| Issue | Feature | test262 impact |
|-------|---------|----------------|
| #419-#420 | TypedArray constructors + methods | ~5,000 |
| #421-#422 | Map/Set iterator protocol + Set algebra | ~1,300 |
| #423 | WeakMap/WeakSet complete | ~300 |
| #424 | DataView complete | ~500 |
| #425 | ArrayBuffer/SharedArrayBuffer | ~500 |
| #426 | Object static methods | ~300 |
| #427 | Math builtins complete | ~500 |
| #428 | Number methods complete | ~300 |
| #429 | Date methods complete | ~500 |
| #430 | Error subclasses | ~500 |
| #431 | console API complete | ~200 |
| #432 | JSON replacer/reviver | ~200 |
| #433 | BigInt arithmetic complete | ~2,000 |
| #434 | Symbol.for/keyFor registry | ~300 |
| #435 | Atomics complete | ~1,500 |
| #436-#437 | Intl.DateTimeFormat + NumberFormat | ~3,500 |
| #438 | Proxy complete (13 traps) | ~3,000 |
| #439-#441 | RegExp exec/test + advanced | ~3,000 |
| #442 | eval/Function constructor | ~300 |
| #458-#459 | Function.prototype bind/call/apply/toString | ~500 |
| #460-#461 | String supplementary + static methods | ~800 |
| #462 | Object.prototype methods | ~500 |
| #463 | Boolean/Symbol.prototype | ~100 |
| #464 | Promise supplementary (any, withResolvers) | ~500 |
| #465-#466 | Iterator helpers + Array copying | ~500 |
| #467 | WeakRef/FinalizationRegistry | ~200 |
| #468 | Atomics.waitAsync | ~300 |
| #469-#470 | Map/Set supplementary | ~300 |
| #480 | Micro-task queue | ~500 |
| #481 | NativeError types | ~300 |
| #482 | Global object properties | ~500 |

## W5: Language runtime semantics 🔄 IN PROGRESS

Goal: runtime behavior should match the supported JS/TS subset.

**IR issues (14 open):**

| Issue | Feature | test262 impact |
|-------|---------|----------------|
| #404 | async/await Promise integration | ~1,300 |
| #405 | Generator functions (function*/yield) | ~800 |
| #406 | Well-known Symbol runtime wiring | ~500 |
| #407 | Proxy handler traps (basic) | ~3,000 |
| #408 | Dynamic import() | ~500 |
| #409 | Live module bindings | ~300 |
| #410 | Object shorthand/computed/method | ~200 |
| #411 | BigInt arithmetic complete | ~2,000 |
| #412 | Sparse array holes | ~200 |
| #415 | for-await-of | ~300 |
| #416 | Async generators | ~300 |
| #417 | Strict mode semantics | ~500 |
| #446 | Generator state-machine lowering | ~800 |
| #471-#473 | super.prop, import.meta, new.target | ~500 |
| #483 | Destructuring patterns | ~500 |

## W6: Coverage and regression infrastructure 🔄 IN PROGRESS

- [X] Ramp 500 → 2,000 → 10,000 → 30,000 → 53,445 with stable parallel execution
- [X] Regression detection: fail on build_pass / semantic_pass decrease
- [X] Delta reporting: feature-level and diagnostic-class pass/fail deltas
- [X] Coverage dashboard: trend graph, feature-level burn-down, diagnostic burn-down
- [X] Gate progress visualization
- [ ] **Remaining:** Full sampler ramp (#477), canary expansion (#455), negative classification (#456), differential infra (#478), perf gate (#479)

## W7: Host capability boundary ✅ COMPLETE

- [X] Full host import audit with `--emit-manifest`
- [X] Manifest golden tests for all supported fixtures
- [X] Host-deny test matrix expansion
- [X] Standalone assurance for Promise, Proxy, Reflect, TypedArray, WASI
- [X] CI gate for unexpected host imports
- [ ] **Remaining:** $262 harness (#451), Node.js shim (#452), WASI filesystem (#476)

## W8: Optimization and backend replacement ⏳ DEFERRED

**Entry condition**: semantic_diff coverage stable, test262 ramp regression-safe.

- [ ] Benchmark suite + performance regression tracker
- [ ] Typed fast path / packed array / devirtualization
- [ ] Property inline cache / dead code elimination / constant folding / inlining
- [ ] Full binary WASM emitter replacing WAT text dependency
- [ ] Replace giant WAT templates with typed writers
- [ ] ABI bridge cleanup after logical/wire contract is stable
- [ ] wasm-encoder backend parity (#453)
