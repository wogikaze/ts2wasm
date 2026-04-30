---
id: 333
title: "Implement BigInt dynamic string exception parity"
type: feature
area: runtime/builtins
class: implementation-ready
priority: P2
depends_on: [280]
blocks: []
created: 2026-04-30
updated: 2026-05-01
---

## Summary

Close the remaining dynamic `BigInt(value)` string edge that issue 280 left out
of the supported runtime subset: unknown runtime strings that are invalid or
outside the current BigInt representation currently trap instead of reporting a
Node-compatible exception.

## Problem

`BigInt(value)` supports definitely-string dynamic inputs for the current
StringToBigInt subset, and literal-derived invalid/out-of-range dynamic strings
stay on source diagnostics. Unknown runtime string contents can still reach the
runtime helper and trap when parsing fails or the value is outside the current
single-limb/u64 representation.

Problem: unknown dynamic invalid/out-of-range `BigInt(string)` inputs trap at
runtime instead of producing a compatible JavaScript exception or a documented
source diagnostic.

## Current failure

Representative shape:

```sh
tmp=/tmp/ts2wasm-333-dynamic-bigint-string.ts
cat > "$tmp" <<'TS'
let value = "bad";
if (Date.now() < 0) {
  value = "10";
}
console.log(BigInt(value));
TS
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-333-dynamic-bigint-string.wasm
node "$tmp"
iwasm /tmp/ts2wasm-333-dynamic-bigint-string.wasm
```

Node throws a JavaScript exception for invalid StringToBigInt input. The current
runtime helper reaches the unsupported trap path for unknown invalid strings.

## Desired final state

Unknown dynamic string inputs to `BigInt(value)` either produce Node-compatible
JavaScript exceptions for invalid/out-of-range values or are rejected by a
source-spanned diagnostic linked to this issue when the compiler can prove the
input is outside the supported runtime representation.

## Scope

In scope:

- [ ] Handle unknown dynamic invalid decimal string inputs to `BigInt(value)`
- [ ] Handle unknown dynamic out-of-range string inputs for the current BigInt representation
- [x] Preserve issue-280 behavior for supported dynamic decimal/binary/octal/hex strings
- [x] Add source-diagnostic coverage for provable literal-derived dynamic invalid/out-of-range strings

Out of scope:

- Full multi-limb BigInt arithmetic
- Broader number model gaps such as `NaN`, `Infinity`, `-0`, and fractional values
- Object `ToPrimitive` for arbitrary objects

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/*bigint*`
- `docs/language-reference/javascript-features.md`
- `current-state.md`

Do not touch:

- parser BigInt literal syntax
- unrelated builtin families
- issue 311 arguments-object files

## Acceptance criteria

- [ ] Unknown invalid dynamic `BigInt(string)` input no longer traps without a tracked compatible exception result or source diagnostic
- [ ] Unknown out-of-range dynamic `BigInt(string)` input no longer traps without a tracked compatible exception result or source diagnostic
- [x] Supported issue-280 dynamic string inputs remain Node/iwasm differential matches
- [x] Docs/current-state/issues state the current diagnostic boundary

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(bigint) or test(node_diff)'
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo test -p ts2wasm-cli --test m2_node_diff bigint
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] updated: `docs/language-reference/javascript-features.md`

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none

## Notes

Issue 280 owns the supported dynamic builtin subset. This follow-up owns only
the runtime exception/diagnostic parity for unknown dynamic string contents that
cannot be classified safely before lowering.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending progress commit in child branch

Progress:

- Added issue-333 source diagnostics for literal-derived dynamic
  `BigInt(string)` values that are provably invalid or outside the runtime
  helper's current single-limb/u64 representation.
- Added `fixtures/core-semantics/bigint-builtin-dynamic-out-of-range-string-unsupported.ts`.
- Preserved source-spanned issue-280 diagnostics for static invalid string
  literals and nullish dynamic inputs.

Validation result:

```text
cargo fmt --all --check: pass
cargo nextest run -E 'test(bigint) or test(node_diff)': 173 passed, 1 failed
  - unrelated known failure: abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
    timed out under iwasm after 30.418s
mise run update-issue-index -- --check: pass
mise run check issues: pass
```

Remaining risks:

- Truly unknown runtime invalid/out-of-range strings can still reach
  `$bigint_from_string` and trap because compatible JavaScript exception
  construction/throwing is not implemented in this slice.
