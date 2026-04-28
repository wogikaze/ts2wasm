# Cycle Report: 052 JSON.stringify Replacer Array

Run ID: `052-json-replacer-array-20260428T083349Z`

Outcome: PROGRESS

Implementation commit: `2fa4ae2`

## Scope

- Implemented the narrow `JSON.stringify(value, ["key"])` property-list subset for object literals.
- Added Node/iwasm differential coverage for `JSON.stringify({ a: 1, b: 2 }, ["a"])`.
- Preserved diagnostics for function replacers and unsupported array replacer contents/forms.

## Evidence

- Pre-change reproduction: `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-replacer-array-unsupported.ts -o /tmp/ts2wasm-json-replacer-array.pre.wasm` rejected `["a"]` with `issue-052: JSON.stringify array replacer property lists are not supported yet`; Node printed `{"a":1}`.
- New fixture: `fixtures/builtins-and-io/json-stringify-replacer-array.ts`.
- Node/iwasm output for the new fixture:

```text
{"a":1}
```

## Validation

- `cargo fmt --all --check`: pass
- `cargo nextest run -E 'test(json)'`: pass, 17 passed
- `cargo nextest run -p ts2wasm-cli json`: pass, 14 passed
- `node fixtures/builtins-and-io/json-stringify-replacer-array.ts`: pass
- `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-replacer-array.ts -o /tmp/ts2wasm-json-replacer-array.wasm`: pass
- `iwasm /tmp/ts2wasm-json-replacer-array.wasm`: pass
- `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-replacer-array-unsupported.ts -o /tmp/ts2wasm-json-replacer-array-unsupported.wasm`: expected `UnsupportedSyntax`, status 1
- `scripts/manager check-issue-health`: pass
- `scripts/manager check-agent-state`: pass
- `cargo nextest run`: pass, 356 passed, 4 skipped

## Remaining Gaps

- Full replacer semantics beyond the single string-literal object-literal subset remain open.
- JSON number/string representation gaps and broader throw-compatible parse diagnostics remain open under issue 052.
