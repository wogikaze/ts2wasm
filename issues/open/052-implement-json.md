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

2026-04-28:

- Implemented a `JSON.stringify` argument continuation slice for integer numeric `space` values with null/undefined replacer values, including the JSON clamp to 10 spaces.
- Added lowering-time validation so unsupported `JSON.stringify` replacer forms and unsupported `space` forms report diagnostics instead of emitting a malformed runtime call.
- Added Node differential coverage in `fixtures/builtins-and-io/json-stringify-space.ts`; Node and iwasm both print:

```text
{
  "a": 1,
  "b": 2
}
[
  1,
  2
]
```

- Pre-change gap check with `/tmp/ts2wasm-json-stringify-space.ts` showed `JSON.stringify(obj, null, 2)` failed during WAT assembly because lowering passed all three arguments to a one-argument `$json_stringify` helper.
- Validation passed:
  - `cargo fmt --all --check`
  - `cargo nextest run -E 'test(json)'`
  - `cargo nextest run -p ts2wasm-cli json`
  - `node fixtures/builtins-and-io/json-stringify-space.ts`
  - `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-space.ts -o /tmp/ts2wasm-json-stringify-space.wasm && iwasm /tmp/ts2wasm-json-stringify-space.wasm`
  - `scripts/manager check-issue-health`
  - `scripts/manager check-agent-state`
- Full `cargo nextest run` was skipped for this PROGRESS slice because the issue remains open and the assignment requires the JSON-filtered nextest commands plus direct Node/iwasm evidence.
- Remaining gaps before close: arbitrary non-integer JSON number representation, non-ASCII `\uXXXX`/surrogate handling, nested object literal value preservation for `JSON.stringify`, full replacer semantics, string `space` semantics, and throw-compatible parse diagnostics remain outside this slice.

2026-04-28:

- Implemented a `JSON.stringify` string `space` continuation slice for null/undefined replacer values, carrying a clamped gap string through runtime indentation emission.
- Progress commit: `49f07b5`.
- String `space` now supports simple ASCII prefix strings and clamps the gap to ECMAScript's 10-character limit.
- Added Node differential coverage in `fixtures/builtins-and-io/json-stringify-space-string.ts`; Node and iwasm both print:

```text
{
>>"a": 1,
>>"b": 2
}
[
abcdefghij1
]
```

- Pre-change gap check with `/tmp/ts2wasm-json-string-space.ts` showed Node printed a prefix-indented JSON object, while ts2wasm rejected the third argument with `UnsupportedSyntax: JSON.stringify space currently supports integer numeric values`.
- Validation passed:
  - `cargo fmt --all --check`
  - `cargo nextest run -E 'test(json)'`
  - `cargo nextest run -p ts2wasm-cli json`
  - `node fixtures/builtins-and-io/json-stringify-space-string.ts`
  - `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-space-string.ts -o /tmp/ts2wasm-json-stringify-space-string.wasm && iwasm /tmp/ts2wasm-json-stringify-space-string.wasm`
  - `scripts/manager check-issue-health`
  - `scripts/manager check-agent-state`
- Remaining gaps before close: arbitrary non-integer JSON number representation, non-ASCII `\uXXXX`/surrogate handling, nested object literal value preservation for `JSON.stringify`, full replacer semantics, and throw-compatible parse diagnostics remain outside this slice.

2026-04-28:

- Implemented a `JSON.stringify` nested object/array literal preservation slice so aggregate literal children use separate backend temporaries from their containing literal.
- Progress commit: `bc15c89`.
- Added Node differential coverage in `fixtures/builtins-and-io/json-stringify-nested-object.ts` for `JSON.stringify({ a: { b: 2 }, c: [3] })`.
- Pre-change gap check with `/tmp/ts2wasm-json-stringify-nested-object.ts` showed Node printed `{"a":{"b":2},"c":[3]}`, while iwasm printed `undefined`.
- Direct evidence for the new fixture:
  - `node fixtures/builtins-and-io/json-stringify-nested-object.ts` prints `{"a":{"b":2},"c":[3]}`.
  - `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-nested-object.ts -o /tmp/ts2wasm-json-stringify-nested-object.wasm && iwasm /tmp/ts2wasm-json-stringify-nested-object.wasm` prints `{"a":{"b":2},"c":[3]}`.
- Validation passed:
  - `cargo fmt --all --check`
  - `cargo nextest run -E 'test(json)'`
  - `cargo nextest run -p ts2wasm-cli json`
- Remaining gaps before close: arbitrary non-integer JSON number representation, non-ASCII `\uXXXX`/surrogate handling, full replacer semantics, and throw-compatible parse diagnostics remain outside this slice.

2026-04-28:

- Added precise issue-052 lowering diagnostics for unsupported `JSON.stringify` replacer forms:
  - function replacer callbacks, including declared function identifiers and arrow function literals;
  - array replacer property lists.
- Preserved accepted null/undefined replacer behavior and numeric/string `space` handling through the existing JSON fixture set.
- Added diagnostic fixtures:
  - `fixtures/builtins-and-io/json-stringify-replacer-function-unsupported.ts`
  - `fixtures/builtins-and-io/json-stringify-replacer-array-unsupported.ts`
- Direct build evidence:
  - `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-replacer-function-unsupported.ts -o /tmp/ts2wasm-json-replacer-function.wasm` rejects with `[UnsupportedSyntax] issue-052: JSON.stringify function replacer callbacks are not supported yet ... at 59..89`.
  - `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-replacer-array-unsupported.ts -o /tmp/ts2wasm-json-replacer-array.wasm` rejects with `[UnsupportedSyntax] issue-052: JSON.stringify array replacer property lists are not supported yet ... at 12..49`.
- Validation passed:
  - `cargo fmt --all --check`
  - `cargo nextest run -E 'test(json)'`
  - `cargo nextest run -p ts2wasm-cli json`
- Remaining gaps before close: arbitrary non-integer JSON number representation, non-ASCII `\uXXXX`/surrogate handling, full replacer semantics, and throw-compatible parse diagnostics remain outside this slice.

2026-04-28:

- Implemented a `JSON.parse` invalid-literal diagnostics slice so top-level and nested `true`/`false`/`null` parsing requires exact keyword bytes instead of accepting any same-length token with the same first character.
- Added rejection coverage in `fixtures/builtins-and-io/json-parse-invalid-literal.ts` for `JSON.parse("turd")`.
- Pre-change gap check with `/tmp/ts2wasm-json-invalid-literal.ts` showed Node rejected `turd` with a JSON `SyntaxError` and status 1, while iwasm accepted the same parse and printed `accepted` with status 0.
- Direct evidence for the new fixture:
  - `node fixtures/builtins-and-io/json-parse-invalid-literal.ts` rejects with a JSON `SyntaxError` and status 1.
  - `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-invalid-literal.ts -o /tmp/ts2wasm-json-parse-invalid-literal.wasm && iwasm /tmp/ts2wasm-json-parse-invalid-literal.wasm` rejects with `Exception: unreachable` and status 1.
- Validation passed:
  - `cargo fmt --all --check`
  - `cargo nextest run -E 'test(json)'`
  - `cargo nextest run -p ts2wasm-cli json`
  - `scripts/manager check-issue-health`
  - `scripts/manager check-agent-state`
  - `cargo nextest run`
- Remaining gaps before close: arbitrary non-integer JSON number representation, non-ASCII `\uXXXX`/surrogate handling, full replacer semantics, and broader throw-compatible parse diagnostics remain outside this slice.

2026-04-28:

- Implemented a narrow `JSON.parse` invalid-number diagnostics slice that rejects leading-zero number tokens instead of accepting them as small integers.
- Covered top-level, array-value, and object-value parse paths with rejection fixtures:
  - `fixtures/builtins-and-io/json-parse-invalid-number-leading-zero.ts` for `JSON.parse('01')`
  - `fixtures/builtins-and-io/json-parse-invalid-number-leading-zero-array.ts` for `JSON.parse('[01]')`
  - `fixtures/builtins-and-io/json-parse-invalid-number-leading-zero-object.ts` for `JSON.parse('{"a":01}')`
- Pre-change gap checks showed Node rejected all three leading-zero forms with JSON `SyntaxError`, while iwasm accepted each fixture and printed `accepted` with status 0.
- Incomplete number probes were already rejected by both Node and iwasm:
  - `JSON.parse('1.')`
  - `JSON.parse('1e')`
  - `JSON.parse('-')`
- Direct evidence for the new fixtures:
  - `node fixtures/builtins-and-io/json-parse-invalid-number-leading-zero.ts` rejects with a JSON `SyntaxError` and status 1.
  - `node fixtures/builtins-and-io/json-parse-invalid-number-leading-zero-array.ts` rejects with a JSON `SyntaxError` and status 1.
  - `node fixtures/builtins-and-io/json-parse-invalid-number-leading-zero-object.ts` rejects with a JSON `SyntaxError` and status 1.
  - `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-invalid-number-leading-zero.ts -o /tmp/ts2wasm-json-invalid-number-leading-zero.wasm && iwasm /tmp/ts2wasm-json-invalid-number-leading-zero.wasm` rejects with `Exception: unreachable` and status 1.
  - `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-invalid-number-leading-zero-array.ts -o /tmp/ts2wasm-json-invalid-number-leading-zero-array.wasm && iwasm /tmp/ts2wasm-json-invalid-number-leading-zero-array.wasm` rejects with `Exception: unreachable` and status 1.
  - `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-invalid-number-leading-zero-object.ts -o /tmp/ts2wasm-json-invalid-number-leading-zero-object.wasm && iwasm /tmp/ts2wasm-json-invalid-number-leading-zero-object.wasm` rejects with `Exception: unreachable` and status 1.
- Validation passed:
  - `cargo fmt --all --check`
  - `cargo nextest run -E 'test(json)'`
  - `cargo nextest run -p ts2wasm-cli json`
  - `scripts/manager check-issue-health`
  - `scripts/manager check-agent-state`
  - `cargo nextest run`
- Remaining gaps before close: arbitrary non-integer JSON number representation, non-ASCII `\uXXXX`/surrogate handling, full replacer semantics, and broader throw-compatible parse diagnostics remain outside this slice.

2026-04-28:

- Added explicit `JSON.parse` unicode escape diagnostic regression coverage for the current narrow runtime behavior.
- Progress commit: `b034261`.
- Covered:
  - top-level invalid unicode escape hex with `fixtures/builtins-and-io/json-parse-invalid-unicode-escape.ts` (`JSON.parse('"\\u00G0"')`): Node rejects with a JSON `SyntaxError`; iwasm rejects with `Exception: unreachable`.
  - array string value unsupported non-ASCII unicode escape with `fixtures/builtins-and-io/json-parse-unsupported-unicode-array.ts` (`JSON.parse('["\\u00e9"]')`): Node accepts; iwasm rejects with `Exception: unreachable` instead of producing a wrong single-byte string.
  - object string value unsupported surrogate unicode escape with `fixtures/builtins-and-io/json-parse-unsupported-unicode-object.ts` (`JSON.parse('{"s":"\\ud800"}')`): Node accepts; iwasm rejects with `Exception: unreachable` instead of producing a wrong string.
- The existing runtime `$json_parse_unicode_escape_byte` helper already rejected invalid hex and code points above the current ASCII string representation; this slice pins that behavior in `crates/cli/tests/m2_node_diff.rs`.
- Validation passed:
  - `cargo fmt --all --check`
  - `cargo nextest run -E 'test(json)'`
  - `cargo nextest run -p ts2wasm-cli json`
  - `node fixtures/builtins-and-io/json-parse-invalid-unicode-escape.ts` (expected JSON `SyntaxError`, status 1)
  - `node fixtures/builtins-and-io/json-parse-unsupported-unicode-array.ts` (expected accept, status 0)
  - `node fixtures/builtins-and-io/json-parse-unsupported-unicode-object.ts` (expected accept, status 0)
  - `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-invalid-unicode-escape.ts -o /tmp/ts2wasm-json-parse-invalid-unicode-escape.wasm`
  - `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-unsupported-unicode-array.ts -o /tmp/ts2wasm-json-parse-unsupported-unicode-array.wasm`
  - `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-unsupported-unicode-object.ts -o /tmp/ts2wasm-json-parse-unsupported-unicode-object.wasm`
  - `iwasm /tmp/ts2wasm-json-parse-invalid-unicode-escape.wasm` (expected `Exception: unreachable`, status 1)
  - `iwasm /tmp/ts2wasm-json-parse-unsupported-unicode-array.wasm` (expected `Exception: unreachable`, status 1)
  - `iwasm /tmp/ts2wasm-json-parse-unsupported-unicode-object.wasm` (expected `Exception: unreachable`, status 1)
  - `scripts/manager check-issue-health`
  - `scripts/manager check-agent-state`
  - `cargo nextest run`
- Additional gate note: `cargo clippy --all-targets --all-features -- -D warnings` was run and failed on pre-existing `clippy::assertions_on_constants` diagnostics in `crates/runtime-abi/src/layout.rs`, outside this child assignment's allowed files.
- Remaining gaps before close: arbitrary non-integer JSON number representation, full UTF-16/non-ASCII string representation, full surrogate-pair support, full replacer semantics, and broader throw-compatible parse diagnostics remain outside this slice.

2026-04-28:

- Added focused Node differential regression coverage for reading properties from object elements inside a parsed JSON array in `fixtures/builtins-and-io/json-parse-array-object-properties.ts`, covering `JSON.parse('[{"n":1},{"n":2}]')`.
- Direct probe before adding the fixture showed the current runtime already handled this narrow continuation slice, so this child records PROGRESS as regression coverage only and made no backend/runtime changes.
- Node and iwasm both print:

```text
2
1
2
```

- Validation passed:
  - `cargo fmt --all --check`
  - `cargo nextest run -E 'test(json)'`
  - `cargo nextest run -p ts2wasm-cli json`
  - `node fixtures/builtins-and-io/json-parse-array-object-properties.ts`
  - `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-array-object-properties.ts -o /tmp/ts2wasm-json-parse-array-object-properties.wasm && iwasm /tmp/ts2wasm-json-parse-array-object-properties.wasm`
  - `scripts/manager check-issue-health`
  - `scripts/manager check-agent-state`
  - `cargo nextest run`
- Full validation report for run `052-json-array-object-20260428T074900Z` is recorded under `reports/runs/052-json-array-object-20260428T074900Z/`.
- Remaining gaps before close: arbitrary non-integer JSON number representation, full UTF-16/non-ASCII string representation, full surrogate-pair support, full replacer semantics, and broader throw-compatible parse diagnostics remain outside this slice.

2026-04-28:

- Added explicit `JSON.parse` non-integer number diagnostic regression coverage for the current tagged small-int runtime behavior.
- Covered:
  - top-level unsupported non-integer number with `fixtures/builtins-and-io/json-parse-unsupported-noninteger-number.ts` (`JSON.parse("1.5")`);
  - array value unsupported non-integer number with `fixtures/builtins-and-io/json-parse-unsupported-noninteger-number-array.ts` (`JSON.parse("[1.5]")`);
  - object value unsupported non-integer number with `fixtures/builtins-and-io/json-parse-unsupported-noninteger-number-object.ts` (`JSON.parse("{\"n\":1.5}")`).
- Pre-change probe using `/tmp/ts2wasm-json-noninteger-probe.ts` showed Node accepted all three forms with status 0, while iwasm rejected the first parsed non-integer value with `Exception: unreachable` and status 1.
- The existing `$json_parse_number_value` helper already traps when a decimal/exponent number cannot be reduced to an integer-valued tagged small-int, so this slice pins the behavior in `crates/cli/tests/m2_node_diff.rs` rather than changing backend runtime code.
- Direct evidence for the new fixtures:
  - `node fixtures/builtins-and-io/json-parse-unsupported-noninteger-number.ts` accepts with status 0 and prints `accepted`.
  - `node fixtures/builtins-and-io/json-parse-unsupported-noninteger-number-array.ts` accepts with status 0 and prints `accepted`.
  - `node fixtures/builtins-and-io/json-parse-unsupported-noninteger-number-object.ts` accepts with status 0 and prints `accepted`.
  - `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-unsupported-noninteger-number.ts -o /tmp/ts2wasm-json-noninteger-number.wasm && iwasm /tmp/ts2wasm-json-noninteger-number.wasm` rejects with `Exception: unreachable` and status 1.
  - `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-unsupported-noninteger-number-array.ts -o /tmp/ts2wasm-json-noninteger-number-array.wasm && iwasm /tmp/ts2wasm-json-noninteger-number-array.wasm` rejects with `Exception: unreachable` and status 1.
  - `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-unsupported-noninteger-number-object.ts -o /tmp/ts2wasm-json-noninteger-number-object.wasm && iwasm /tmp/ts2wasm-json-noninteger-number-object.wasm` rejects with `Exception: unreachable` and status 1.
- Remaining gaps before close: arbitrary non-integer JSON number representation, full UTF-16/non-ASCII string representation, full surrogate-pair support, full replacer semantics, and broader throw-compatible parse diagnostics remain outside this slice.

2026-04-28:

- Added focused Node differential regression coverage for nested `JSON.stringify` object/array literal value preservation in `fixtures/builtins-and-io/json-stringify-nested-array-object.ts`, covering `JSON.stringify({ a: [{ b: 1 }], c: { d: [2] } })`.
- Progress commit: `5a992b1`.
- Direct probe of the pre-existing assigned minimal fixture showed the current runtime already matches Node:
  - `node fixtures/builtins-and-io/json-stringify-nested-object.ts` prints `{"a":{"b":2},"c":[3]}`.
  - `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-nested-object.ts -o /tmp/ts2wasm-json-stringify-nested-object.current.wasm` and `iwasm /tmp/ts2wasm-json-stringify-nested-object.current.wasm` print `{"a":{"b":2},"c":[3]}`.
- Direct evidence for the new fixture:
  - `node fixtures/builtins-and-io/json-stringify-nested-array-object.ts` prints `{"a":[{"b":1}],"c":{"d":[2]}}`.
  - `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-nested-array-object.ts -o /tmp/ts2wasm-json-stringify-nested-array-object.wasm` and `iwasm /tmp/ts2wasm-json-stringify-nested-array-object.wasm` print `{"a":[{"b":1}],"c":{"d":[2]}}`.
- Validation passed:
  - `cargo fmt --all --check`
  - `cargo nextest run -E 'test(json)'`
  - `cargo nextest run -p ts2wasm-cli json`
  - `scripts/manager update-issue-index --check`
  - `scripts/manager check-issue-health`
  - `scripts/manager check-agent-state`
  - `cargo nextest run`
- Full validation report for run `052-json-stringify-nested-20260428T080100Z` is recorded under `reports/runs/052-json-stringify-nested-20260428T080100Z/`.
- Remaining gaps before close: arbitrary non-integer JSON number representation, full UTF-16/non-ASCII string representation, full surrogate-pair support, full replacer semantics, and broader throw-compatible parse diagnostics remain outside this slice.

2026-04-28:

- Added explicit `JSON.parse` surrogate diagnostic regression coverage for the current narrow runtime behavior.
- Covered:
  - lone low surrogate with `fixtures/builtins-and-io/json-parse-unsupported-surrogate-low.ts` (`JSON.parse('"\\udc00"')`);
  - surrogate pair with `fixtures/builtins-and-io/json-parse-unsupported-surrogate-pair.ts` (`JSON.parse('"\\ud83d\\ude00"')`).
- Pre-change probes showed Node accepts both forms while iwasm already rejects both with `Exception: unreachable`; this slice records PROGRESS as regression coverage only and made no backend/runtime changes.
- Direct evidence for the new fixtures:
  - `node fixtures/builtins-and-io/json-parse-unsupported-surrogate-low.ts` accepts with status 0 and prints `accepted`.
  - `node fixtures/builtins-and-io/json-parse-unsupported-surrogate-pair.ts` accepts with status 0 and prints `accepted`.
  - `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-unsupported-surrogate-low.ts -o /tmp/ts2wasm-json-parse-unsupported-surrogate-low.wasm && iwasm /tmp/ts2wasm-json-parse-unsupported-surrogate-low.wasm` rejects with `Exception: unreachable` and status 1.
  - `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-unsupported-surrogate-pair.ts -o /tmp/ts2wasm-json-parse-unsupported-surrogate-pair.wasm && iwasm /tmp/ts2wasm-json-parse-unsupported-surrogate-pair.wasm` rejects with `Exception: unreachable` and status 1.
- Validation passed:
  - `cargo fmt --all --check`
  - `cargo nextest run -E 'test(json)'`
  - `cargo nextest run -p ts2wasm-cli json`
  - `scripts/manager update-issue-index --check`
  - `scripts/manager check-issue-health`
  - `scripts/manager check-agent-state`
  - `cargo nextest run`
- Remaining gaps before close: arbitrary non-integer JSON number representation, full UTF-16/non-ASCII string representation, full surrogate-pair support, full replacer semantics, and broader throw-compatible parse diagnostics remain outside this slice.

2026-04-28:

- Implemented a narrow `JSON.stringify` array replacer slice for object-literal property filtering with a single string-literal property-list entry.
- Progress commit: `2fa4ae2`.
- Added Node differential coverage in `fixtures/builtins-and-io/json-stringify-replacer-array.ts` for `JSON.stringify({ a: 1, b: 2 }, ["a"])`; Node and iwasm both print:

```text
{"a":1}
```

- Preserved unsupported diagnostics for function replacers and array replacer contents/forms outside this slice by keeping `fixtures/builtins-and-io/json-stringify-replacer-function-unsupported.ts` and changing `fixtures/builtins-and-io/json-stringify-replacer-array-unsupported.ts` to cover a non-string property-list entry.
- Pre-change reproduction of the former guarded array fixture showed ts2wasm rejected `JSON.stringify({ a: 1, b: 2 }, ["a"])` with `issue-052: JSON.stringify array replacer property lists are not supported yet`, while Node printed `{"a":1}`.
- Validation passed:
  - `cargo fmt --all --check`
  - `cargo nextest run -E 'test(json)'`
  - `cargo nextest run -p ts2wasm-cli json`
  - `node fixtures/builtins-and-io/json-stringify-replacer-array.ts`
  - `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-replacer-array.ts -o /tmp/ts2wasm-json-replacer-array.wasm`
  - `iwasm /tmp/ts2wasm-json-replacer-array.wasm`
  - `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-replacer-array-unsupported.ts -o /tmp/ts2wasm-json-replacer-array-unsupported.wasm` (expected `UnsupportedSyntax`, status 1)
  - `scripts/manager check-issue-health`
  - `scripts/manager check-agent-state`
  - `cargo nextest run`
- Full validation report for run `052-json-replacer-array-20260428T083349Z` is recorded under `reports/runs/052-json-replacer-array-20260428T083349Z/`.
- Remaining gaps before close: arbitrary non-integer JSON number representation, full UTF-16/non-ASCII string representation, full surrogate-pair support, broader replacer semantics beyond the single string-literal object-literal subset, and broader throw-compatible parse diagnostics remain outside this slice.

2026-04-28:

- Implemented a narrow `JSON.stringify` array replacer continuation slice for object-literal property filtering with multiple string-literal property-list entries.
- Progress commit: `3e3c4ae`.
- Added Node differential coverage in `fixtures/builtins-and-io/json-stringify-replacer-array-multikey.ts` for `JSON.stringify({ a: 1, b: 2, c: 3 }, ["c", "a"])`; Node and iwasm both print:

```text
{"c":3,"a":1}
```

- Preserved unsupported diagnostics for function replacers and unsupported array replacer contents/forms; `fixtures/builtins-and-io/json-stringify-replacer-array-unsupported.ts` still rejects a non-string property-list entry with an issue-052 `UnsupportedSyntax` diagnostic.
- Pre-change reproduction with `/tmp/ts2wasm-json-replacer-array-multikey.ts` showed Node printed `{"c":3,"a":1}`, while ts2wasm rejected the same source with `issue-052: JSON.stringify array replacer property lists outside the single string-literal object subset are not supported yet`.
- Validation passed:
  - `cargo fmt --all --check`
  - `cargo nextest run -E 'test(json)'`
  - `cargo nextest run -p ts2wasm-cli json`
  - `node fixtures/builtins-and-io/json-stringify-replacer-array-multikey.ts`
  - `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-replacer-array-multikey.ts -o /tmp/ts2wasm-json-replacer-array-multikey.wasm`
  - `iwasm /tmp/ts2wasm-json-replacer-array-multikey.wasm`
  - `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-replacer-array-unsupported.ts -o /tmp/ts2wasm-json-replacer-array-unsupported.wasm` (expected `UnsupportedSyntax`, status 1)
  - `scripts/manager update-issue-index --check`
  - `scripts/manager check-issue-health`
  - `scripts/manager check-agent-state`
  - `cargo nextest run`
- Remaining gaps before close: arbitrary non-integer JSON number representation, full UTF-16/non-ASCII string representation, full surrogate-pair support, broader replacer semantics beyond the string-literal object-literal subset, and broader throw-compatible parse diagnostics remain outside this slice.

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
