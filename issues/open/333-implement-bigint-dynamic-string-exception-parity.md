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
updated: 2026-04-30
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
- [ ] Preserve issue-280 behavior for supported dynamic decimal/binary/octal/hex strings
- [ ] Add Node/iwasm differential or exception-parity coverage for the new path

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
- [ ] Supported issue-280 dynamic string inputs remain Node/iwasm differential matches
- [ ] Docs/current-state/issues state the final exception or diagnostic boundary

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

Commits:

- Removed stdout write dependency from BigInt runtime helper (BigIntFromValue no longer requires Write capability)
- Replaced runtime abort marker with simple trap for unknown dynamic invalid/out-of-range strings
- Updated test expectations to check for trap instead of abort marker output
- Renamed fixture files from runtime-abort to runtime-trap
- Updated documentation to reflect runtime trap behavior without exception throwing

Progress:

- Preserved issue-280 source diagnostics for provable/literal-derived invalid
  dynamic string inputs.
- Removed runtime abort marker output to eliminate stdout write dependency
  from BigInt runtime helpers.
- Unknown dynamic invalid/out-of-range strings now trap at runtime without
  requiring host imports.
- Compatible JavaScript exception throwing remains unimplemented; this is
  documented as a known limitation in issue-333.

Validation result:

```text
cargo fmt --all --check: pass
cargo nextest run -E 'test(bigint)': pass
mise run update-issue-index -- --check: pass
mise run check issues: pass
```

Remaining risks:

- Compatible JavaScript exception throwing for unknown dynamic invalid/out-of-range
  BigInt(string) inputs remains unimplemented. This is a known limitation
  documented in issue-333 and requires full JavaScript exception object
  construction and throwing infrastructure in the runtime.
