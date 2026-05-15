# ts2wasm Roadmap

大きな未実装領域を W0-W8 で整理する。

このファイルは **epic 一覧**であり、個別の実装作業は `issues/` で管理する。

## Current State (2026-05-15)

```
issues/ open: 29 (done: 316, total: 345)
  P0: 4 | P1: 3 | P2: 20 | P3: 2
  ready to work: 27
test262 coverage (full run, server mode):
  total: 53,469 | executed: 49,811 | build_pass: 9,481 | semantic_pass: 9,481
  differential_pass: 5,823 | negative_compile_pass: 3,658 | negative_mismatch: 39
  unsupported: 30,375 | fail: 12,734 | blocked: 879
  build coverage: 17.73% | semantic coverage: 17.73%
  duration: ~28 min (1,693s)
```

Migration from legacy issue tracker (#419-#483, 50 open) to new issues/ format (29 open) completed.
First full test262 run completed: 49,811/53,469 cases executed, 9,481 semantic pass.

**Current waves:** W4 (builtins, 20 runtime open), W5 (runtime semantics, 3 IR open), W6 (coverage infra, 7 open)
**Target:** unsupported (30,375) → 0, fail (12,734) → 0, blocked (879) → 0, semantic coverage >90%

## Roadmap model

W0-W8 は厳密な直列フェーズではない。現在の主要な作業は W4/W5 (builtin semantics + language runtime)。

```txt
issues/        = 作業台帳（29 open）
docs/roadmap.md = 大きな未実装領域の分類
```

## Rules

- Roadmap item は epic であり、そのまま `issues/` にコピーしない。
- `issues/` の item は acceptance command と done evidence を必須にする。
- coverage / reference-coverage の結果から `issues/` を自動生成しない。

## Open feature issues by layer

| Layer | Issues | P1 | P2 | P3 | Description |
|-------|--------|----|----|----|-------------|
| runtime | 20 | 2 | 17 | 1 | WAT runtime functions for builtins and language features |
| coverage | 7 | 0 | 6 | 1 | test262 ramp, parity, negative verification, diagnostics |
| ir | 3 | 0 | 3 | 0 | IR lowering, name resolution, state machines |
| frontend | 1 | 1 | 0 | 0 | Syntax parsing for remaining constructs |
| backend | 1 | 0 | 1 | 0 | ABI metadata, manifest output |
| compiler | 1 | 0 | 1 | 0 | Pipeline integration (harness loading) |
| cli | 1 | 0 | 1 | 0 | Target selection, diagnostics |
| abi | 1 | 0 | 1 | 0 | Runtime ABI constants |

**Ready to work: 27 items** | **P1 open: 3 items** (Map/Set, Promise/async, frontend comma expr)
**P0 epics:** 4 (ABI metadata, parity unsupported elimination, semantic enablement, coverage scale)

## Gate overview

| Gate | Meaning | Status |
|------|---------|--------|
| Gate A | Stable runtime substrate | ✅ W0 done |
| Gate B | Standalone WASI execution | ✅ W1 done |
| Gate C | Parser does not block common fixtures | ✅ W2 done |
| Gate D | Known names and builtin dispatch are explicit | ✅ W3 done |
| Gate E | Core runtime builtins are implemented or precisely rejected | 🔄 W4 in progress (20 open runtime issues) |
| Gate F | JS/TS runtime semantics become coherent | 🔄 W5 in progress (3 open IR issues) |
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
- [X] Remaining parser gaps tracked (hashbang, import assertions, shorthand)

## W3: Name/call resolution and builtin dispatch ✅ COMPLETE

- [X] Core ECMAScript global builtin names registered
- [X] TypedArray constructor names (11 types)
- [X] Well-known symbols (iterator, toStringTag, hasInstance, toPrimitive, for, keyFor)
- [X] Builtin method dispatch table (program_builtins.rs)
- [ ] **Remaining:** test262 harness globals (I-20260513-HDW7PQ, ~13,426 tests)

## W4: Builtin API semantics 🔄 IN PROGRESS

Goal: implement selected builtins after names and dispatch paths are explicit.

**Runtime issues (20 open):**

| Issue | Feature | Priority | test262 impact |
|-------|---------|----------|----------------|
| I-20260515-SB83JG | Map, Set, WeakMap | P1 | ~1,600 |
| I-20260515-V28WKG | Promise and async function support | P1 | ~1,800 |
| I-20260513-Q7B4E8 | RegExp advanced (named groups, lookaround, flags) | P2 | ~3,000 |
| I-20260513-BQTVQV | WeakRef and FinalizationRegistry | P2 | ~200 |
| I-20260513-DCRMZJ | String.prototype.normalize (full Unicode) | P2 | ~300 |
| I-20260513-WBEJBE | First-class Function object model | P2 | ~500 |
| I-20260514-E2TCH8 | Intl.NumberFormat full locale data | P2 | ~1,500 |
| I-20260514-28R5VE | Function.prototype.toString source | P2 | ~300 |
| I-20260514-9P5WP5 | console API full parity | P2 | ~200 |
| I-20260514-SB757Q | Intl.DateTimeFormat full locale data | P2 | ~2,000 |
| I-20260515-WPZXJA | Object/GC triage labels and trap classes | P2 | infra |
| I-20260515-7M5QEV | Symbol and well-known symbols | P2 | ~300 |
| I-20260515-J2X65E | TypedArray, ArrayBuffer, DataView | P2 | ~6,000 |
| I-20260515-RRWK59 | BigInt arithmetic + BigInt typed arrays | P2 | ~2,000 |
| I-20260515-73QE5A | Date UTC coercion gap | P2 | ~500 |
| I-20260515-EWCSK9 | Array.prototype.push mismatch fix | P2 | ~100 |
| I-20260515-7HM9PK | Unicode normalize strategy decision | P2 | arch |
| I-20260515-M2MQS8 | console WAT runtime (group, timer, counter) | P2 | ~100 |
| I-20260515-CWN7F3 | Runtime-subset unsupported diagnostics | P2 | infra |
| I-20260515-8NP365 | WeakSet | P3 | ~100 |

## W5: Language runtime semantics 🔄 IN PROGRESS

Goal: runtime behavior should match the supported JS/TS subset.

**IR issues (3 open):**

| Issue | Feature | Priority | test262 impact |
|-------|---------|----------|----------------|
| I-20260515-PMTJTQ | Frontend: parse comma expression in Object.entries shard | P1 | ~50 |
| I-20260513-HDW7PQ | Enable real test262 harness loading | P2 | ~13,426 |

Covered by W4 runtime issues:
- async/await Promise lowering (I-20260515-V28WKG)
- Symbol runtime wiring (I-20260515-7M5QEV)
- BigInt lowering (I-20260515-RRWK59)

## W6: Coverage and regression infrastructure 🔄 IN PROGRESS

- [X] Full test262 suite run: 49,811/53,469 executed, ~28 min
- [X] Server-mode test262 harness (JSONL batch, parallel compile + semantic)
- [X] Semantic checking enabled (semantic_pass=9,481)
- [X] Regression detection: fail on build_pass / semantic_pass decrease
- [X] Coverage dashboard data pipeline
- [ ] **Reduce unsupported (30,375) and fail (12,734):** I-20260515-ENB7EJ (P0 epic), W4 builtins, W5 runtime semantics
- [ ] Resolve blocked (879): cross-realm, SharedArrayBuffer, module import, evalScript
- [ ] Resolve negative_compile_mismatch (39)
- [ ] Differential test infrastructure (I-20260513-WHBN24, P2)
- [ ] Negative compile verification for unverified cases (I-20260515-HFPCFC, P3)

## W7: Host capability boundary ✅ COMPLETE

- [X] Full host import audit with `--emit-manifest`
- [X] Manifest golden tests for all supported fixtures
- [X] Host-deny test matrix expansion
- [X] Standalone assurance for Promise, Proxy, Reflect, TypedArray, WASI
- [X] CI gate for unexpected host imports

## W8: Optimization and backend replacement ⏳ DEFERRED

**Entry condition**: semantic_diff coverage stable, test262 ramp regression-safe.

- [ ] Benchmark suite + performance regression tracker
- [ ] Typed fast path / packed array / devirtualization
- [ ] Property inline cache / dead code elimination / constant folding / inlining
- [ ] Full binary WASM emitter replacing WAT text dependency
- [ ] Replace giant WAT templates with typed writers
- [ ] ABI bridge cleanup after logical/wire contract is stable
- [ ] wasm-encoder backend parity
