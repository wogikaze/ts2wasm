# ts2wasm Roadmap

大きな未実装領域を W0-W8 で整理する。

このファイルは **epic 一覧**であり、個別の実装作業は `issues/` で管理する。

## Current State (2026-05-15)

```
issues/ open: 29 (done: 332, total: 361)
  P0: 6 | P1: 3 | P2: 19 | P3: 1
  ready to work: 26
test262 coverage (full run, server mode):
  total: 53,469 | executed: 49,811 | build_pass: 9,481 | semantic_pass: 9,481
  differential_pass: 5,823 | negative_compile_pass: 3,658 | negative_mismatch: 39
  unsupported: 30,375 | fail: 12,734 | blocked: 879
  build coverage: 17.73% | semantic coverage: 17.73%
  duration: ~28 min (1,693s)
```

Migration from legacy issue tracker (#419-#483, 50 open) to new issues/ format completed.
First full test262 run completed: 49,811/53,469 cases executed, 9,481 semantic pass.

**Current waves:** W4 (builtins, 12 runtime open), W5 (runtime semantics, 6 IR open, 3 frontend open), W6 (coverage infra, 5 open)
**Target:** unsupported (30,375) → 0, fail (12,734) → 0, blocked (879) → 0, semantic coverage >90%

## Roadmap model

W0-W8 は厳密な直列フェーズではない。現在の主要な作業は W4/W5 (builtin semantics + language runtime)。

```txt
issues/        = 作業台帳（29 open, 332 done）
docs/roadmap.md = 大きな未実装領域の分類
```

## Rules

- Roadmap item は epic であり、そのまま `issues/` にコピーしない。
- `issues/` の item は acceptance command と done evidence を必須にする。
- coverage / reference-coverage の結果から `issues/` を自動生成しない。

## Open feature issues by layer

| Layer | Issues | P1 | P2 | P3 | Description |
|-------|--------|----|----|----|-------------|
| runtime | 12 | 0 | 11 | 1 | WAT runtime functions for builtins and language features |
| ir | 6 | 0 | 6 | 0 | IR lowering, spread/rest, instanceof, closure capture |
| coverage | 5 | 0 | 4 | 1 | test262 ramp, parity, differential, negative verify |
| frontend | 3 | 1 | 2 | 0 | Syntax parsing: comma expr, rest/spread, remaining gaps |
| backend | 3 | 0 | 2 | 0 | ABI metadata, manifest output, target selection |
| compiler | 1 | 0 | 1 | 0 | Pipeline integration (harness loading) |
| abi | 1 | 0 | 1 | 0 | Runtime ABI constants |
| cli | 1 | 0 | 1 | 0 | WAMR semantic runner |

**Ready to work: 26 items** | **P1 open: 3 items** (comma expr frontend, WAMR runner, sampled semantic)
**P0 epics:** 6 (ABI metadata, parity unsupported elimination, ~44k failure elimination, ESM support, WAT runtime funcs, backend arch)

## Gate overview

| Gate | Meaning | Status |
|------|---------|--------|
| Gate A | Stable runtime substrate | ✅ W0 done |
| Gate B | Standalone WASI execution | ✅ W1 done |
| Gate C | Parser does not block common fixtures | ✅ W2 done |
| Gate D | Known names and builtin dispatch are explicit | ✅ W3 done |
| Gate E | Core runtime builtins are implemented or precisely rejected | 🔄 W4 in progress (12 open runtime issues) |
| Gate F | JS/TS runtime semantics become coherent | 🔄 W5 in progress (6 open IR issues, 3 frontend) |
| Gate G | test262 ramp is measurable and regression-safe | 🔄 W6 in progress (5 open coverage issues) |
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

**Runtime issues (12 open, excl. epic):**

| Issue | Feature | Priority | test262 impact |
|-------|---------|----------|----------------|
| I-20260515-D2MJSY | Persistent WAMR semantic runner | P1 | infra |
| I-20260513-Q7B4E8 | RegExp advanced (named groups, lookaround, flags) | P2 | ~3,000 |
| I-20260513-BQTVQV | WeakRef and FinalizationRegistry | P2 | ~200 |
| I-20260514-E2TCH8 | Intl.NumberFormat full locale data | P2 | ~1,500 |
| I-20260514-28R5VE | Function.prototype.toString source | P2 | ~300 |
| I-20260514-SB757Q | Intl.DateTimeFormat full locale data | P2 | ~2,000 |
| I-20260515-J2X65E | TypedArray, ArrayBuffer, DataView | P2 | ~6,000 |
| I-20260515-C539AG | Reflect.* methods | P2 | ~2,000 |
| I-20260515-5HVE2K | Array.prototype.push mismatch | P2 | ~100 |
| I-20260515-Q46FKJ | BigInt arithmetic + BigInt typed arrays | P2 | ~2,000 |
| I-20260515-48B77E | WeakSet | P3 | ~100 |

## W5: Language runtime semantics 🔄 IN PROGRESS

Goal: runtime behavior should match the supported JS/TS subset.

**IR / Frontend issues (9 open, excl. epic):**

| Issue | Feature | Layer | Priority | test262 impact |
|-------|---------|-------|----------|----------------|
| I-20260515-PMTJTQ | Parse comma expression statement | frontend | P1 | ~50 |
| I-20260513-HDW7PQ | Enable real test262 harness loading | compiler | P2 | ~13,426 |
| I-20260515-FGC8MS | Spread/rest lowering in call/array/destructuring | IR | P2 | ~1,200 |
| I-20260515-GAX7YV | instanceof dynamic class constructors | IR | P2 | ~1,500 |
| I-20260515-35H4XD | Nested function args/this capture (issue-062d/e) | IR | P2 | ~800 |
| I-20260515-PSG76B | Accept rest/spread `...` in expression contexts | frontend | P2 | ~1,500 |

## W6: Coverage and regression infrastructure 🔄 IN PROGRESS

- [X] Full test262 suite run: 49,811/53,469 executed, ~28 min
- [X] Server-mode test262 harness (JSONL batch, parallel compile + semantic)
- [X] Semantic checking enabled (semantic_pass=9,481)
- [X] Regression detection: fail on build_pass / semantic_pass decrease
- [X] Coverage dashboard data pipeline
- [ ] **Reduce unsupported (30,375) + fail (12,734) + blocked (879) to zero:**
      I-20260515-7N7MWQ (P0 epic, 7 workstreams, wave1a/wave1b/wave2/wave3 in parallel)
      Priority order: Wave1a (builtins+parser) → Wave1b (name resolution) → Wave2 (compiler features) → Wave3 (backend+correctness)
- [ ] **Server-mode executor pipeline optimization (performance bottleneck)**
      - WAMR VMcore persistent runner is 819x faster than iwasm CLI per-call (10ms vs 582ms)
      - `_wamr_queue` + `semantic_executor` interaction under-optimized in current server mode
      - `--no-server` (legacy subprocess mode) is still faster in practice due to executor overhead
      - `process_one_test` fallback is slow (per-file subprocess spawn)
      - Required: executor pipeline rewrite to match legacy mode throughput, then apply WAMR runner gain
- [ ] Resolve negative_compile_mismatch (39)
- [ ] Differential test infrastructure (I-20260513-WHBN24, P2)

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
