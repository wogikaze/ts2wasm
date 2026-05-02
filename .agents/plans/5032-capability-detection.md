# Issue 5032: Add deterministic external tool capability detection

## Plan

1. Create `crates/cli/tests/common/capability.rs` with:
   - `pub fn has_tool(name: &str) -> bool` — checks `which`/`where`
   - `pub fn require_tool(name: &str)` — panics with clear message if missing
   - `pub fn node_command() -> Command` — returns `Command::new("node")` with capability check
   - `pub fn iwasm_command() -> Command` — returns `Command::new("iwasm")` with capability check

2. Register the module in the test harness (re-exported via a shim)

3. Update `html_comments.rs` and `m2_node_diff.rs` to use `node_command()` instead of raw `Command::new("node")`

## Files

- Create: `crates/cli/tests/common/capability.rs`
- Modify: `crates/cli/tests/html_comments.rs`
- Modify: `crates/cli/tests/m2_node_diff.rs`

## Verification

```bash
cargo fmt --all --check
cargo check -p ts2wasm-cli --tests
cargo nextest run -p ts2wasm-cli
```
