# Issue 050 Date Runtime Blocker

Date: 2026-04-28
Branch: `agent/050-date-runtime-20260428T102000Z`
Issue: `issues/open/050-implement-date.md`

## Outcome

`BLOCKED`.

The assigned Date runtime slice cannot be completed safely within the allowed files. The frontend/IR currently prevents both acceptance-entry forms from reaching backend runtime emission:

- `new Date(0)` is treated as a class constructor and fails because `Date` is not a supported class constructor.
- `Date.now()` fails as an unresolved name before a backend runtime helper could be linked.

The required changes are in `crates/ir/src/name_resolver.rs` and `crates/ir/src/lowered.rs`, which are not in the assignment's allowed file list. In addition, `Date.now()` and zero-argument `new Date()` need an auditable time capability policy before any host import or nondeterministic clock access is added.

## Reproduction

Temporary `new Date(0)` fixture:

```ts
const d = new Date(0);
console.log(d.getTime());
console.log(d.toString());
console.log(Date.now());
```

Node output under the local timezone:

```text
0
Thu Jan 01 1970 09:00:00 GMT+0900 (Japan Standard Time)
1777336481803
```

Compiler result:

```text
command: cargo run -q -p ts2wasm-cli -- build /tmp/ts2wasm-date-ZvvJxH.ts -o /tmp/ts2wasm-date-test.wasm
result: exit 1
stderr: error: [UnsupportedSyntax] issue-207: instanceof right-hand side must be a supported class constructor `Date`
```

Temporary `Date.now()` fixture:

```ts
console.log(Date.now());
```

Compiler result:

```text
command: cargo run -q -p ts2wasm-cli -- build /tmp/ts2wasm-date-now-jjxJat.ts -o /tmp/ts2wasm-date-now-test.wasm
result: exit 1
stderr: error: [UnresolvedName] unresolved name: `Date`
```

## Required Follow-up

Open or update a follow-up that permits `crates/ir/src/**` changes and defines the Date time policy:

- How `Date.now()` and `new Date()` obtain time, and which manifest capability records that access.
- Whether the first deterministic slice is limited to `new Date(<small-int-ms>)`, `getTime()`, and a deterministic string form.
- How `Date.prototype.toString()` handles timezone and formatting relative to Node differential evidence.

No runtime/backend code was changed in this child run because it would be unreachable from fixtures without the forbidden IR lowering changes.
