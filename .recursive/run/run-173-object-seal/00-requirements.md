# Run 173: Register Object.seal in builtin dispatch

## Source

TRACKING.yaml item 173: "Register Object.seal in builtin dispatch (program_builtins.rs)"

## Requirement

`Object.seal(obj)` currently produces `[UnresolvedName] unresolved name: 'Object'`
because `"seal"` is not listed in the `program_builtins.rs` Object method dispatch table.

Add `"seal" => Some("ObjectSeal".to_owned())` to the Object method table and create
a build_smoke fixture + test.

## Acceptance

```
cargo nextest run -p ts2wasm-cli --test m6_builtin_methods build_smoke_object_seal
```

## Non-goals

- No runtime enforcement of non-configurable semantics after seal.
- No new runtime helper RuntimFn variants.
