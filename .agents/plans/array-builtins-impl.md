# Plan: Implement Array.prototype reduce/reduceRight/lastIndexOf/forEach/map

## Summary
Add 5 runtime builtins with ArrowFn callback expansion at IR level and identity WAT fallback.

## Implementation steps (completed)
1. Add LoweredExpr::Block variant for While-loop IR expansion
2. Add 5 RuntimeFn enum variants + dispatch
3. Add WAT identity implementations
4. Register in collection_method_runtime_fn
5. Add lower_array_callback_method for ArrowFn dispatch
6. Add LoweredExpr::Block handling in all match blocks
7. Rename map fixture (unsupported → supported)

## Verification (completed)
- cargo fmt --all --check: clean
- cargo nextest run: all array_map tests pass, 0 regressions
