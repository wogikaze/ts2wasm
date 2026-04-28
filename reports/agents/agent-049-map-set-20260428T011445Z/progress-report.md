# Progress Report: issue 049 Map and Set

Child id: agent-049-map-set-20260428T011445Z
Branch: agent/049-map-set-20260428T011445Z
Date: 2026-04-28
Status: PROGRESS

## Implemented slice

- `new Map()` lowers to a runtime collection object.
- `Map.prototype.set`, `get`, and `has` work for the current basic key path.
- `new Set()` lowers to a runtime collection object.
- `Set.prototype.add` and `has` work for the current basic key path.
- Runtime helpers for `MapDelete` and `SetDelete` are wired, but the parser currently blocks `.delete()` member syntax before these helpers can be reached.
- Added Node/iwasm differential fixture: `fixtures/builtins-and-io/map-set.ts`.

## Validation

Passed:

```text
cargo fmt --all --check
cargo nextest run -E 'test(map) or test(set)'
  4 tests run: 4 passed
cargo nextest run -p ts2wasm-cli map
  1 test run: 1 passed
cargo nextest run -p ts2wasm-cli set
  2 tests run: 2 passed
cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/map-set.ts -o /tmp/ts2wasm-049-map-set.wasm && iwasm /tmp/ts2wasm-049-map-set.wasm
  stdout matched Node fixture output
scripts/manager check-issue-health
  check_issue_health: OK
```

Not run:

```text
cargo nextest run
```

Reason: parent requested the smallest validated progress slice now; full Map/Set acceptance is not closable in this child scope.

## Reproduction and remaining work

Initial reproduction of `m.delete("a")` failed before runtime:

```text
error: [UnsupportedSyntax] expected identifier, got Some(SpannedToken { kind: Delete, ... })
```

Remaining issue 049 criteria:

- Enable parser/frontend member access for keyword property names such as `.delete`, or otherwise route `Map.prototype.delete` / `Set.prototype.delete` through supported syntax.
- Validate `map.delete` and `set.delete` with Node/iwasm differential coverage.
- Tighten collection key semantics beyond the current `value_to_string_into` normalization so number and string keys do not collide.
- Run full `cargo nextest run` before moving issue 049 to `done/`.
