# Phase 4: Validation

## Acceptance Test
```bash
cargo nextest run -p ts2wasm-cli --test m2_node_diff promise_basic_matches_node_output
```
→ PASS [0.201s]

## Related Tests
```bash
cargo nextest run -p ts2wasm-cli --test m6_promise
```
→ PASS: build_smoke_promise_basic

```bash
cargo fmt --all --check
```
→ clean

```bash
cargo build
```
→ clean

## Pre-existing Failures (unrelated)
- json_parse_latin1_unicode_escape_matches_node_output
- descriptor_combinations_matches_node_output
- enumerable_filtering_matches_node_output
