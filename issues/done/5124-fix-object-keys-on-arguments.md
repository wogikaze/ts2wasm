---
id: 5124
title: "Fix Object.keys on arguments exotic object"
type: bug
area: runtime
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-04
updated: 2026-05-04
---

## Summary

Object.keys works correctly on regular objects in WASM but produces wrong
results on the `arguments` exotic object. The result array has incorrect
length and undefined elements.

## Current failure

```sh
cat > /tmp/test-arguments-keys.js <<'EOF'
function test(x, y, z) {
  var a = Object.keys(arguments);
  console.log(a.length);
  console.log(a[0]);
  console.log(a[1]);
  return a.length === 3;
}
var result = test(1, 2, 3);
console.log(result);
EOF
cargo run -q -- build /tmp/test-arguments-keys.js -o /tmp/arguments-keys.wasm --host-deny && iwasm /tmp/arguments-keys.wasm
```

Current result:
```text
4
undefined
undefined
false
```

Expected result (Node):
```text
3
0
1
true
```

## Desired final state

Object.keys on the `arguments` object returns correct string indices.

## Scope

In scope:

- [ ] Fix Object.keys WAT runtime function to correctly enumerate `arguments`
      exotic object properties
- [ ] Or fix `arguments` object property enumeration in WASM backend

Out of scope:

- Other Object built-in methods
- Object.keys on regular objects (already working)

## Affected paths

Expected:

- `crates/backend-wasm/src/runtime_arrays_objects.rs`
- `crates/backend-wasm/src/runtime_fn.rs`
- `crates/backend-wasm/src/runtime_fn_impl.rs`
- `crates/backend-wasm/src/runtime_builder.rs`

Do not touch:

- crates/ir/ (unless arguments lowering needs changes)
- crates/compiler/ (harness/preprocessor not involved)

## Acceptance criteria

- [ ] `Object.keys(arguments)` returns correct string indices
- [ ] Regression test in fixtures/ for Object.keys on arguments
- [ ] test262 case `built-ins/Object/keys/15.2.3.14-3-4.js` passes through
      harness

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -q -- build /tmp/test-arguments-keys.js -o /tmp/arguments-keys.wasm --host-deny && iwasm /tmp/arguments-keys.wasm
PYTHONPATH=scripts/lib python3 -c "import sys; sys.path.insert(0,'scripts/lib'); from test262_harness import *; import tempfile,json; r,s=process_one_test(Path('reference/test262/test/built-ins/Object/keys/15.2.3.14-3-4.js'),Path(tempfile.mkdtemp())); print('status:',s)"
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [ ] none

## Notes

Observations from initial investigation:

- `Object.keys(arguments)` returns `[length=4]` with `undefined` elements for
  a 3-parameter function called with 3 args. Expected: `["0", "1", "2"]`.
- `Object.keys({a:1, b:2})` works correctly (returns `["a", "b"]`).
- The bug is in how the `arguments` exotic object exposes its indexed
  properties to Object.keys enumeration.

## Completion evidence

Commits:

- (pending commit)

Validation result:

```text
cargo run -q -- build /tmp/test-arguments-keys2.js -o /tmp/args-keys2.wasm --host-deny && iwasm /tmp/args-keys2.wasm
result:
3
["0","1","2"]
3
true
```

Remaining risks:

- arguments exotic object still does not fully match ES spec (length is the only non-enumerable property tracked)
