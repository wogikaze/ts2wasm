---
id: 052
title: "Implement JSON"
type: feature
area: runtime/builtins
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Implement JSON.parse and JSON.stringify.

## Problem

JSON is not implemented. It is essential for data serialization.

## Desired final state

`JSON.parse()` and `JSON.stringify()` work correctly.

## Scope

In scope:

- [ ] Implement JSON.parse
- [ ] Implement JSON.stringify
- [ ] Add fixtures for JSON behavior

Out of scope:

- Full JSON spec compliance (start with common cases)

## Affected paths

Expected:

- `crates/backend-wasm/src/` (runtime builtins)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] JSON.parse works correctly
- [ ] JSON.stringify works correctly
- [ ] Fixtures cover JSON behavior
- [ ] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/json-test.ts -o /tmp/test.wasm
iwasm /tmp/test.wasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Note: fixtures/builtins-and-io/json-*.ts exist but may not be fully implemented.

## Progress evidence

2026-04-28:

- Implemented a first runtime slice for `JSON.stringify` covering small-int primitives, ASCII strings, arrays, and flat object serialization.
- Implemented a first runtime slice for `JSON.parse` covering whitespace-trimmed primitives, ASCII strings, and flat objects with string keys and small-int/string/boolean/null values.
- Added Node differential coverage for the existing JSON fixtures in `crates/cli/tests/m2_node_diff.rs`.
- Existing fixture evidence:
  - `fixtures/builtins-and-io/json-stringify.ts`: Node and iwasm both print `{"a":1,"b":2}`.
  - `fixtures/builtins-and-io/json-parse.ts`: Node and iwasm both print `1`.
- Remaining gaps before close: escaped strings, decimals/exponents, nested parse values, arrays in `JSON.parse`, replacer/space arguments, and throw-compatible parse diagnostics remain outside this first slice.

2026-04-28:

- Implemented a next runtime slice for `JSON.parse` covering top-level arrays with small-int, ASCII string, boolean, and null elements.
- Added Node differential coverage in `fixtures/builtins-and-io/json-parse-array.ts`; Node and iwasm both print:

```text
5
1
two
true
false
null
```

- Validation passed:
  - `cargo fmt --all --check`
  - `cargo nextest run -p ts2wasm-cli json`
  - `cargo nextest run -E 'test(json)'`
  - `cargo run -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-array.ts -o /tmp/ts2wasm-json-parse-array.wasm && iwasm /tmp/ts2wasm-json-parse-array.wasm`
  - `node fixtures/builtins-and-io/json-parse-array.ts`
  - `cargo nextest run`
  - `scripts/manager check-issue-health`
  - `scripts/manager check-agent-state`
  - `python -m jsonschema -i reports/runs/052-json-next-20260428T013435Z/test_report.json .agents/state/schemas/test_report.schema.json`
  - `scripts/manager check-repo-smoke`
- Remaining gaps before close: escaped strings, decimals/exponents, nested arrays/objects in parsed values, arrays inside parsed object values, object elements inside parsed arrays, replacer/space arguments, and throw-compatible parse diagnostics remain outside this slice.

2026-04-28:

- Implemented a nested `JSON.parse` continuation slice for arrays containing nested arrays.
- Added Node differential coverage in `fixtures/builtins-and-io/json-parse-nested-array.ts`; Node and iwasm both print:

```text
2
2
2
3
```

- Reproduced the pre-change gap with the same nested-array case using `/tmp/ts2wasm-json-nested-array.ts`: Node printed `2`, `2`, `2`, `3`, while iwasm printed four `undefined` lines.
- Validation passed:
  - `cargo fmt --all --check`
  - `cargo nextest run -E 'test(json)'`
  - `cargo nextest run -p ts2wasm-cli json`
  - `cargo run -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-nested-array.ts -o /tmp/ts2wasm-json-parse-nested-array.wasm`
  - `iwasm /tmp/ts2wasm-json-parse-nested-array.wasm`
  - `node fixtures/builtins-and-io/json-parse-nested-array.ts`
  - `scripts/manager check-issue-health`
  - `scripts/manager check-agent-state`
- Full `cargo nextest run` was skipped for this PROGRESS slice because the change is scoped to the JSON runtime helper and the assignment only requires full nextest before merge when the runtime parsing change is broad enough to justify it. The JSON-targeted nextest filters and direct Node/iwasm fixture evidence passed.
- Remaining gaps before close: escaped strings, decimals/exponents, nested objects, arrays inside parsed object values, object elements inside parsed arrays, `JSON.stringify` replacer/space arguments, and throw-compatible parse diagnostics remain outside this slice.

2026-04-28:

- Added Node differential coverage for `JSON.parse` object values containing an array and a nested object with its own array in `fixtures/builtins-and-io/json-parse-object-nested.ts`.
- Direct Node/iwasm evidence for the new fixture both print:

```text
2
2
3
4
```

- Pre-change gap check with `/tmp/ts2wasm-json-object-nested.ts` showed the current runtime already handled this narrow object-value continuation, so this child slice records the behavior as regression coverage rather than changing backend runtime code.
- Validation passed:
  - `cargo fmt --all --check`
  - `cargo nextest run -E 'test(json)'`
  - `cargo nextest run -p ts2wasm-cli json`
  - `node fixtures/builtins-and-io/json-parse-object-nested.ts`
  - `cargo run -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-object-nested.ts -o /tmp/ts2wasm-json-parse-object-nested.wasm && iwasm /tmp/ts2wasm-json-parse-object-nested.wasm`
  - `scripts/manager check-issue-health`
  - `scripts/manager check-agent-state`
- Full `cargo nextest run` was skipped for this PROGRESS slice because no runtime parser code changed; the assignment-specific JSON filters and direct Node/iwasm fixture evidence passed.
- Remaining gaps before close: escaped strings, decimals/exponents, object elements inside parsed arrays, stricter top-level/trailing-token parse validation, `JSON.stringify` replacer/space arguments, and throw-compatible parse diagnostics remain outside this slice.

2026-04-28:

- Implemented a top-level `JSON.parse` validation slice that rejects non-whitespace trailing tokens after a parsed object, array, string, literal, or integer number.
- Added rejection coverage in `fixtures/builtins-and-io/json-parse-trailing-invalid.ts`; Node rejects the fixture with a JSON `SyntaxError`, and iwasm now rejects it with an `unreachable` trap instead of accepting the parsed prefix and printing `unreachable`.
- Pre-change gap check with `/tmp/ts2wasm-json-trailing-invalid.ts` showed the previous runtime accepted `JSON.parse('{"a":1} trailing')`, printed `unreachable`, and exited with status 0.
- Validation passed:
  - `cargo fmt --all --check`
  - `cargo nextest run -E 'test(json)'`
  - `cargo nextest run -p ts2wasm-cli json`
  - `node fixtures/builtins-and-io/json-parse-trailing-invalid.ts` (expected rejection: status 1, JSON `SyntaxError`)
  - `cargo run -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-trailing-invalid.ts -o /tmp/ts2wasm-json-parse-trailing-invalid.wasm && iwasm /tmp/ts2wasm-json-parse-trailing-invalid.wasm` (expected rejection at iwasm: `Exception: unreachable`, status 1)
  - `scripts/manager check-issue-health`
  - `scripts/manager check-agent-state`
- Remaining gaps before close: escaped strings, decimals/exponents, stricter incomplete-token validation, object elements inside parsed arrays as explicit regression coverage, `JSON.stringify` replacer/space arguments, and throw-compatible parse diagnostics remain outside this slice.

2026-04-28:

- Implemented a `JSON.parse` escaped-string continuation slice for standard single-byte escapes in parsed strings: `\"`, `\\`, `\/`, `\b`, `\f`, `\n`, `\r`, and `\t`.
- Added an escape-aware string skipper so top-level strings, object keys, object string values, array string values, and nested container scanning do not terminate early on escaped quotes.
- Added Node differential coverage:
  - `fixtures/builtins-and-io/json-parse-escaped-string.ts`: Node and iwasm both print `a"b`.
  - `fixtures/builtins-and-io/json-parse-escaped-nested.ts`: Node and iwasm both print:

```text
x"y
c\d
```

- Pre-change gap check with `/tmp/ts2wasm-json-escaped-string.ts` showed Node printed `a"b`, while iwasm rejected the same case with `Exception: unreachable`.
- Validation passed:
  - `cargo fmt --all --check`
  - `cargo nextest run -E 'test(json)'`
  - `cargo nextest run -p ts2wasm-cli json`
  - `node fixtures/builtins-and-io/json-parse-escaped-string.ts`
  - `cargo run -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-escaped-string.ts -o /tmp/ts2wasm-json-parse-escaped-string.wasm && iwasm /tmp/ts2wasm-json-parse-escaped-string.wasm`
  - `node fixtures/builtins-and-io/json-parse-escaped-nested.ts`
  - `cargo run -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-escaped-nested.ts -o /tmp/ts2wasm-json-parse-escaped-nested.wasm && iwasm /tmp/ts2wasm-json-parse-escaped-nested.wasm`
  - `scripts/manager check-issue-health`
  - `scripts/manager check-agent-state`
- Full `cargo nextest run` was skipped for this PROGRESS slice because the issue remains open and the assignment allows focused validation for narrow runtime progress.
- Remaining gaps before close: decimal/exponent number parsing, `\uXXXX` string escapes, stricter incomplete-token validation, explicit object-elements-inside-arrays coverage, `JSON.stringify` replacer/space arguments, and throw-compatible parse diagnostics remain outside this slice.

2026-04-28:

- Implemented a `JSON.parse` number continuation slice for integer-valued decimal/exponent forms representable by the current tagged small-int runtime: `1.0`, `1e2`, `-2.5e1`, and `120e-1`.
- Added shared runtime helpers for parsing/skipping JSON numbers across top-level, object-value, and array-value parse paths.
- Added Node differential coverage in `fixtures/builtins-and-io/json-parse-number-decimal-exponent.ts`; Node and iwasm both print:

```text
1
100
-25
12
```

- Pre-change gap check with `/tmp/ts2wasm-json-number-dec-exp.ts` showed Node printed `1`, `100`, `-25`, `12`, while iwasm printed four `undefined` lines.
- Validation passed:
  - `cargo fmt --all --check`
  - `cargo nextest run -E 'test(json)'`
  - `cargo nextest run -p ts2wasm-cli json`
  - `node fixtures/builtins-and-io/json-parse-number-decimal-exponent.ts`
  - `cargo run -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-number-decimal-exponent.ts -o /tmp/ts2wasm-json-parse-number-decimal-exponent.wasm && iwasm /tmp/ts2wasm-json-parse-number-decimal-exponent.wasm`
- Full `cargo nextest run` was skipped for this PROGRESS slice because the issue remains open and the assignment allows focused validation for narrow runtime progress.
- Remaining gaps before close: arbitrary non-integer JSON number representation, `\uXXXX` string escapes, stricter incomplete-token validation, explicit object-elements-inside-arrays coverage, `JSON.stringify` replacer/space arguments, and throw-compatible parse diagnostics remain outside this slice.

2026-04-28:

- Implemented a `JSON.parse` unicode escape continuation slice for `\uXXXX` sequences that decode to the current runtime's supported single-byte ASCII string representation.
- Progress commit: `645da75`.
- Added shared runtime hex decoding for parsed JSON strings and wired it into object keys, object string values, array string values, nested container scanning, and top-level parsed strings through the existing string parse/skip paths.
- Added Node differential coverage in `fixtures/builtins-and-io/json-parse-unicode-escape.ts`; Node and iwasm both print:

```text
AZ
x/y
```

- Pre-change gap check with `/tmp/ts2wasm-json-unicode-escape.ts` showed Node printed `AZ` and `x/y`, while iwasm printed two `undefined` lines.
- Validation passed:
  - `cargo fmt --all --check`
  - `cargo nextest run -E 'test(json)'`
  - `cargo nextest run -p ts2wasm-cli json`
  - `node fixtures/builtins-and-io/json-parse-unicode-escape.ts`
  - `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-unicode-escape.ts -o /tmp/ts2wasm-json-parse-unicode-escape.wasm && iwasm /tmp/ts2wasm-json-parse-unicode-escape.wasm`
  - `scripts/manager check-issue-health`
  - `scripts/manager check-agent-state`
- Full `cargo nextest run` was skipped for this PROGRESS slice because the issue remains open and the assignment explicitly allows focused validated progress.
- Remaining gaps before close: arbitrary non-integer JSON number representation, non-ASCII `\uXXXX`/surrogate handling, stricter incomplete-token validation, explicit object-elements-inside-arrays coverage, `JSON.stringify` replacer/space arguments, and throw-compatible parse diagnostics remain outside this slice.

2026-04-28:

- Added explicit Node differential regression coverage for `JSON.parse` arrays containing object elements in `fixtures/builtins-and-io/json-parse-array-object.ts`, covering `JSON.parse('[{"a":1},{"b":[2]}]')`.
- Direct runtime check before adding the fixture showed the current runtime already handled this narrow continuation slice, so this child records the behavior as regression coverage rather than changing backend runtime code.
- Node and iwasm both print:

```text
2
1
1
2
```

- Validation passed:
  - `cargo fmt --all --check`
  - `cargo nextest run -E 'test(json)'`
  - `cargo nextest run -p ts2wasm-cli json`
  - `node fixtures/builtins-and-io/json-parse-array-object.ts`
  - `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-array-object.ts -o /tmp/ts2wasm-json-parse-array-object.wasm && iwasm /tmp/ts2wasm-json-parse-array-object.wasm`
  - `scripts/manager check-issue-health`
  - `scripts/manager check-agent-state`
- Full `cargo nextest run` was skipped for this PROGRESS slice because no runtime parser code changed and the assignment explicitly allows focused validated progress.
- Remaining gaps before close: arbitrary non-integer JSON number representation, non-ASCII `\uXXXX`/surrogate handling, stricter incomplete-token validation, `JSON.stringify` replacer/space arguments, and throw-compatible parse diagnostics remain outside this slice.

2026-04-28:

- Implemented a `JSON.parse` incomplete-token validation slice that rejects invalid top-level parse failures instead of silently returning `undefined`, covering empty input and incomplete object/array/string/number parse paths through the existing parse helpers.
- Added Node/iwasm rejection coverage in `fixtures/builtins-and-io/json-parse-incomplete-object.ts` for `JSON.parse('{"a":1')`.
- Pre-change gap check with `/tmp/ts2wasm-json-incomplete-object.ts` showed Node rejected the case with a JSON `SyntaxError` and status 1, while iwasm accepted the program with status 0.
- Direct evidence for the new fixture:
  - `node fixtures/builtins-and-io/json-parse-incomplete-object.ts` rejects with a JSON `SyntaxError` and status 1.
  - `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-incomplete-object.ts -o /tmp/ts2wasm-json-parse-incomplete-object.wasm && iwasm /tmp/ts2wasm-json-parse-incomplete-object.wasm` rejects with `Exception: unreachable` and status 1.
- Validation passed:
  - `cargo fmt --all --check`
  - `cargo nextest run -E 'test(json)'`
  - `cargo nextest run -p ts2wasm-cli json`
  - `scripts/manager check-issue-health`
  - `scripts/manager check-agent-state`
  - `cargo nextest run`
- Remaining gaps before close: arbitrary non-integer JSON number representation, non-ASCII `\uXXXX`/surrogate handling, `JSON.stringify` replacer/space arguments, and throw-compatible parse diagnostics remain outside this slice.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
