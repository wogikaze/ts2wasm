# ts2wasm Roadmap

大きな未実装領域を W0-W8 で整理する。このファイルは **epic 一覧**であり、個別の実装作業は `TRACKING.yaml` で管理する。

## W0: Runtime substrate（Gate A precondition）

- [ ] Binary WASM emitter 拡充 (`crates/backend-wasm/src/binary_mvp.rs`)
- [ ] Runtime core の raw WAT 削減（`runtime_core_emitter_part[12].rs`）
- [ ] ABI 二重表現の解消（論理 `i64` vs wire `i32`）

## W1: Standalone WASI execution

- [ ] WASI args (`args_get`/`args_sizes_get`)
- [ ] WASI env (`environ_get`/`environ_sizes_get`)
- [ ] WASI proc_exit clean exit
- [ ] WASI clock resolution (`clock_res_get`)
- [ ] Standalone test coverage 拡充

## W2: Syntax coverage（parser gaps）

- [ ] RegExp literal flags (g, i, m, s, u, y, d) — ~45 test262 cases
- [ ] Expression forms: SequenceExpression, yield/generator, async/await, optional chaining, nullish coalescing
- [ ] Statement forms: with, debugger, Annex B block-level function hoisting
- [ ] Cover initializers / labelled function declarations
- [ ] JSX parsing
- [ ] Decorator parsing + parameter property parsing
- [ ] Parser-specific test coverage

## W3: Name resolution

- [ ] Register all ECMAScript global builtins (Symbol, Proxy, Reflect, Map, Set, WeakMap, WeakSet, Promise, Error types, ArrayBuffer, DataView, Atomics, Intl, globalThis, etc.)
- [ ] Register TypedArray constructors (Int8Array through Float64Array)
- [ ] Register well-known symbols (Symbol.iterator, toStringTag, hasInstance, toPrimitive, for, keyFor)
- [ ] Complete builtin method dispatch table (all String/Array/Object/Number/Function.prototype methods)
- [ ] Nested namespace/module resolution (A.B.C)
- [ ] Type-only imports / triple-slash directives / module augmentation

## W4: Runtime builtins

- [ ] Promise constructor + .then/.catch/.finally
- [ ] Promise.all / race / resolve / reject / allSettled / any / withResolvers
- [ ] async/await lowering and runtime
- [ ] Proxy constructor + 13 fundamental handler traps
- [ ] Proxy.revocable
- [ ] Reflect API (construct, apply, get, set, has, deleteProperty, etc.)
- [ ] TypedArray constructors (11 types) + methods
- [ ] ArrayBuffer / SharedArrayBuffer / DataView
- [ ] WeakMap / WeakSet
- [ ] Symbol constructor + well-known symbols
- [ ] Atomics / Intl
- [ ] String.prototype.replace/replaceAll full RegExp semantics
- [ ] String.prototype.matchAll
- [ ] Array.prototype.sort / reduce / reduceRight
- [ ] Build_smoke → semantic_diff upgrade for existing builtins

## W5: Runtime semantics

- [ ] Iterator protocol (Symbol.iterator for Array/String/Map/Set, Iterator.prototype)
- [ ] Well-known symbol wiring (hasInstance, toPrimitive, toStringTag, iterator)
- [ ] Proper completion records for all statement types
- [ ] Generator functions (function*, yield, yield*, Generator.prototype)
- [ ] async generators + for-await-of
- [ ] Object model completeness (property descriptors, seal/freeze, [[GetPrototypeOf]]/[[SetPrototypeOf]])
- [ ] ES module live binding updates + module namespace objects
- [ ] Dynamic import + circular dependency evaluation
- [ ] Mutable capture environments for escaping closures
- [ ] This binding (global this, strict-mode receiver)

## W6: test262 coverage ramp

- [ ] Ramp 500 → 2,000（parallel execution, caching）
- [ ] Ramp 2,000 → 10,000
- [ ] Ramp 10,000 → 30,000
- [ ] Ramp 30,000 → 53,445
- [ ] Regression detection (compare ramp runs, fail on regression)
- [ ] Delta reporting (feature-level pass/fail deltas)
- [ ] Coverage dashboard: trend graph + feature-level burn-down
- [ ] Gate progress visualization

## W7: Host capability boundary

- [ ] Full host import audit (--emit-manifest on all fixtures)
- [ ] Expand host-deny test matrix + manifest golden tests
- [ ] Standalone assurance for new features (Promise, Proxy, etc.)
- [ ] Capability review checklist in coding standard

## W8: Optimization（deferred to post-90% test262）

- [ ] Benchmark suite + regression tracker
- [ ] Typed fast path / packed array / devirtualization
- [ ] Property inline cache / dead code elimination / constant folding / inlining
- [ ] Full binary WASM emitter (replace WAT text dependency)
- [ ] Replace giant WAT templates with typed writers
- [ ] ABI bridge: fix logical vs wire representation
