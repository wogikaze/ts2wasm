---
id: 390
title: "Allow or rewrite Test262 harness undefined binding name"
type: bug
area: frontend/parser
class: ready
priority: P2
depends_on: [389]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

After issue 389 aligned `reference-triage` with Test262 harness-preprocessed
input, the representative Array.map Test262 case no longer fails on the closed
issue-273 function-expression initializer. It now fails earlier in the harness
shim at `var undefined = void 0;`.

## Problem

`reference-triage` reports:

```text
UnsupportedSyntax / issue-247: expected binding identifier or pattern, got Some(Undefined)
```

The failing harness source is:

```js
var undefined = void 0;
```

The lexer tokenizes `undefined` as the keyword token `Undefined`, and the parser
rejects it where a binding identifier is expected. This leaves the Test262
representative path blocked by a closed issue number instead of a live parser or
harness-preprocessing issue.

## Desired final state

- Test262 harness input for
  `reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-2-19.js`
  no longer fails at `var undefined = void 0;` with closed issue 247.
- The compiler either accepts this sloppy-mode harness binding where appropriate
  or the Test262 preprocessor rewrites the harness binding safely for the wasm
  target.
- `mise run reference-triage -- test262 reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-2-19.js`
  reaches the next real blocker or passes.

## Scope

In scope:

- Parser or Test262 harness preprocessing support for the `undefined` binding
  in the generated harness input.
- A narrow regression fixture or triage evidence proving the representative path
  advances beyond this binding.

Out of scope:

- Direct function-expression spread calls with `this` / `arguments`, still
  tracked by issue 274 when reached.
- Full generic Array.map runtime array-like behavior, tracked by issue 388.

## Acceptance criteria

- [ ] `mise run reference-triage -- test262 reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-2-19.js` no longer reports issue 247 at `var undefined = void 0;`.
- [ ] Any newly exposed blocker is represented by an open issue with evidence.
- [ ] `cargo nextest run -p ts2wasm-cli array_map` still passes.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli array_map
mise run reference-triage -- test262 reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-2-19.js
mise run update-issue-index -- --check
mise run check issues
```

## Completion evidence

Fill when moving to `done/`.
