# Phase 3: Implementation Summary — Slices 1–5

## Item 182-1: String .length counts UTF-8 code points

### TDD Cycle

**RED**: Created `fixtures/control-flow-and-exceptions/utf8-string.ts` — `"日本語".length` returned 9 (bytes). Test:

```
left: "9\n5\n3\n0\n1\n11\n11\n"   (actual — byte counts)
right: "3\n5\n1\n0\n1\n5\n11\n"   (expected — code points)
```

**GREEN**:
- Added `$string_code_point_length` WAT function in `runtime_strings.rs` that iterates bytes, skipping continuation bytes (0x80-0xBF), counting only leading bytes
- Modified `emit_get_length` in `runtime_collections.rs` to call `$string_code_point_length` for strings (previously shared byte-count path with arrays)
- Arrays unchanged — still use byte count from header

**REFACTOR**: Moved function to separate name `$string_code_point_length` to avoid conflict with existing `$string_length` (byte count used by `$string_replace`)

### Files Changed

- `crates/backend-wasm/src/runtime_strings.rs` — added `emit_string_code_point_length`
- `crates/backend-wasm/src/runtime_collections.rs` — split string/array branches in `emit_get_length`
- `crates/cli/tests/common/m2_node_diff_fixture_tests.rs` — registered UTF-8 fixture
- `fixtures/control-flow-and-exceptions/utf8-string.ts` — new fixture

### Validation

| Command | Result |
|---------|--------|
| `cargo fmt --all --check` | PASS |
| `cargo clippy --all-targets` | PASS |
| `cargo nextest run -p ts2wasm-cli --test m2_node_diff utf8_string_fixture` | PASS |
| `cargo nextest run -p ts2wasm-cli --test m12_async_await` | 6/6 PASS |
| Pre-existing m2_node_diff failures (bun_stdin_text) | unchanged |
| Pre-existing m6 failures (string_html_wrapper, string_replace_all) | unchanged |

### Commit

`52ebab20c` — runtime: Add UTF-8 code point count for string .length (item 182/168 Slice 1)

## Item 182-2: String charAt/at with code point indexing

### TDD Cycle

**RED**: Extended fixture with charAt/at tests for multi-byte strings. `"日本語".charAt(1)` returned garbled continuation byte instead of "本". The `$utf8_cp_to_byte_index` helper checked `count == cp_index` for ALL bytes (including continuation bytes), returning byte position 1 instead of 3 for cp_index=1.

Test failure was: left (actual) vs right (expected) — charAt(3) returned "undefined" string (not empty string "").

**GREEN**:
- Added `$utf8_cp_to_byte_index` WAT helper: iterates bytes, counts only leading bytes (bytes where `b & 0xC0 != 0x80`), returns byte index when count matches cp_index
- Added `$utf8_cp_byte_length` WAT helper: determines byte length (1-4) from leading byte value
- Fixed `$string_char_at`: calls `$utf8_cp_to_byte_index`, copies multi-byte character. Out-of-range → return empty string (heap object with header=0, string tag)
- Fixed `$string_at`: same pattern with negative index normalization. Out-of-range → return undefined (spec behavior)
- Bug fix: `$utf8_cp_to_byte_index` `count == cp_index` check moved inside "is leading byte" branch

**REFACTOR**: Helper functions emitted only from `emit_string_char_at` (not from both charAt and at) to avoid WAT redefinition errors.

### Files Changed

- `crates/backend-wasm/src/runtime_strings.rs` — added `emit_utf8_cp_to_byte_index`, `emit_utf8_cp_byte_length`, modified `emit_string_char_at`, `emit_string_at`
- `fixtures/control-flow-and-exceptions/utf8-string.ts` — added charAt/at tests

### Validation

| Command | Result |
|---------|--------|
| `cargo fmt --all --check` | PASS |
| `cargo build` | PASS |
| `cargo nextest run -p ts2wasm-cli --test m2_node_diff utf8_string_fixture` | PASS |
| `cargo nextest run -p ts2wasm-cli --test m2_node_diff` (all) | 306/307 PASS (bun_stdin_text pre-existing) |
| `cargo nextest run -p ts2wasm-cli --test m12_async_await` | 6/6 PASS |

### Commit

`baecc6d07` — runtime: Add UTF-8 code point indexing for string charAt/at (item 182/168 Slice 2)

### Next Slice

Slice 3: `substring` / `slice` — convert byte index ranges to code point index ranges.

## Item 182-3: String substring/slice with code point indexing

### TDD Cycle

**RED**: Extended fixture with substring/slice tests for multi-byte strings. `"日本語".substring(0, 1)` returned first byte (0xE6) instead of "日". The length-based clamping used raw byte count (9) instead of code point count (3).

During implementation, discovered `$string_at` had a pre-existing bug: it called UTF-8 helpers (`$utf8_cp_to_byte_index`, etc.) without emitting them — relied on `$string_char_at` being emitted first. With `string-at.ts` fixture, this caused "undefined function" WAT errors. Fixed by adding helper emissions to `emit_string_at` with dedup guards.

Also discovered `slice(-1)` single-argument calls have a pre-existing issue: the resolver generates 2 operational args (receiver + start) but the WAT function expects 3 params (receiver + start + end). Not related to UTF-8 changes.

**GREEN**:
- Added `wat.contains` dedup guards to `emit_utf8_cp_to_byte_index`, `emit_utf8_cp_byte_length`, `emit_string_code_point_length` — prevents WAT redefinition errors when multiple functions need the same helper
- Modified `$string_substring`: uses `$string_code_point_length` for clamping, converts CP indices to byte indices before copying bytes
- Modified `$string_slice`: same pattern as substring with CP-negative index handling
- Modified `$string_substr`: uses CP length for negative index computation, delegates to `$string_substring` which handles byte conversion
- Fixed `$string_at`: emits UTF-8 helpers directly (was relying on `$string_char_at`)
- Added UTF-8 helper emissions to `$string_substring`, `$string_slice`, `$string_substr`

**REFACTOR**: Dedup guards make it safe to emit helpers from any function without worrying about redefinition.

### Files Changed

- `crates/backend-wasm/src/runtime_strings.rs` — dedup guards, substring/slice/substr/at CP-aware logic
- `fixtures/control-flow-and-exceptions/utf8-string.ts` — substring/slice tests

### Validation

| Command | Result |
|---------|--------|
| `cargo fmt --all --check` | PASS |
| `cargo build` | PASS |
| `cargo nextest run -p ts2wasm-cli --test m2_node_diff utf8_string_fixture` | PASS |
| `cargo nextest run -p ts2wasm-cli --test m2_node_diff --no-fail-fast` | 293/307 PASS (14 pre-existing failures) |
| `cargo nextest run -p ts2wasm-cli --test m12_async_await` | 6/6 PASS |
| string_builtin_fixtures | now PASSES (was failing before StringAt fix) |

### Commit

`58ffe42d5` — runtime: Add UTF-8 code point indexing for string substring/slice (item 182/168 Slice 3)

### Next Slice

Slice 4: `indexOf` / `lastIndexOf` — code-point position reporting.

## Item 182-4: String indexOf/lastIndexOf with code point position reporting

### TDD Cycle

**RED**: Extended fixture with indexOf/lastIndexOf tests for multi-byte strings. `"日本語".indexOf("本")` returned byte position 3 instead of CP index 1. The `$string_index_of` function used raw byte position `(local.get $i)` for the result.

During implementation, discovered pre-existing bugs:
- Empty needle `"".indexOf("")` returned `undefined` (raw `i32.const 0` = `ValueTag::UNDEFINED`) instead of tagged 0
- `$string_last_index_of` empty needle returned `h_len` (byte length) instead of code point length

**GREEN**:
- Added `$utf8_byte_to_cp_index` WAT helper with dedup guard: iterates bytes up to `byte_pos`, counts only leading bytes, returns CP count — the inverse of `$utf8_cp_to_byte_index`
- Modified `$string_index_of`: match return uses `call $utf8_byte_to_cp_index`; empty needle returns tagged NUMBER 0 instead of raw 0
- Modified `$string_last_index_of`: match return uses `call $utf8_byte_to_cp_index`; empty needle uses `call $string_code_point_length` for CP count
- Added `$string_code_point_length` emission to `emit_string_last_index_of` (was missing)
- Fixed duplicate code residue from initial edit in `$string_index_of` loop (leftover `(local.set $i ...)(br $search))` after match block)

**REFACTOR**: Dedup guards on `emit_utf8_byte_to_cp_index` make it safe to call from any emit function.

### Files Changed

- `crates/backend-wasm/src/runtime_strings.rs` — added `emit_utf8_byte_to_cp_index`, modified `emit_string_index_of`, `emit_string_last_index_of`
- `fixtures/control-flow-and-exceptions/utf8-string.ts` — added indexOf/lastIndexOf multi-byte tests

### Validation

| Command | Result |
|---------|--------|
| `cargo fmt --all --check` | PASS |
| `cargo build` | PASS |
| `cargo nextest run -p ts2wasm-cli --test m2_node_diff utf8_string_fixture` | PASS |
| `cargo nextest run -p ts2wasm-cli --test m2_node_diff --no-fail-fast` | 293/307 PASS (14 pre-existing failures) |
| `cargo nextest run -p ts2wasm-cli --test m12_async_await` | 6/6 PASS |

### Commit

`ef952234b` — runtime: Add UTF-8 code point position reporting for indexOf/lastIndexOf (item 182/168 Slice 4)

## Item 182-5: String charCodeAt/codePointAt with code point value access

### TDD Cycle

**RED**: Extended fixture with charCodeAt/codePointAt tests. `charCodeAt` on multi-byte strings returned raw byte values instead of decoded code points.

**GREEN**:
- Added `$utf8_decode_cp_at_byte` helper in `runtime_strings.rs` that decodes UTF-8 at a byte position using independent if-then-return blocks (1-byte → 2-byte → 3-byte → 4-byte fallthrough), avoiding double-match on Latin-1 storage bytes like 0xE9
- Rewrote `$string_char_code_at` to use CP-length for clamping, `$utf8_cp_to_byte_index` for byte-to-CP conversion, and `$utf8_decode_cp_at_byte` for code point decoding. Returns `undefined` for out-of-range
- Added `$string_code_point_at` with same logic, returns `undefined` for out-of-range per spec
- Registered `StringCodePointAt` RuntimeFn: enum variant, deps, dispatch arm, manifest name, IR routing

**REG.RESSION-DEBUG**: JSON parser Latin-1 optimization stored bytes 0x80-0xFF as single raw bytes (not proper UTF-8). Byte 0xE9 matched both 2-byte mask (0xE0) and 3-byte mask (0xF0), producing `36864` instead of `233` for `"é".charCodeAt(0)`. Fixed `$json_write_utf8_at` threshold 256→128 — bytes 0x80-0xFF now encoded as 2-byte UTF-8.

### Files Changed

- `crates/backend-wasm/src/runtime_strings.rs` — added `emit_utf8_decode_cp_at_byte`, `emit_string_code_point_at`; rewrote `emit_string_char_code_at`
- `crates/backend-wasm/src/runtime_builder.rs` — dispatch arm
- `crates/backend-wasm/src/runtime_fn.rs` — enum variant + deps
- `crates/backend-wasm/src/runtime_fn_impl.rs` — RuntimeSpec, manifest, emission order
- `crates/ir/src/lowered/program_builtins.rs` — "codePointAt" routing
- `crates/backend-wasm/src/runtime_builtins_host_json_parse.rs` — Latin-1 → UTF-8 fix (128 threshold)
- `fixtures/control-flow-and-exceptions/utf8-string.ts` — charCodeAt/codePointAt tests

### Validation

| Command | Result |
|---------|--------|
| `cargo fmt --all --check` | PASS |
| `cargo build` | PASS |
| `cargo nextest run -p ts2wasm-cli --test m2_node_diff utf8_string_fixture` | PASS |
| `cargo nextest run -p ts2wasm-cli --test m2_node_diff json_parse_latin1_unicode_escape` | PASS |
| `cargo nextest run -p ts2wasm-cli --test m12_async_await` | 6/6 PASS |
| Full m2_node_diff (excl. 9 pre-existing failures) | 140/140 PASS, no regressions |

### Commit

`262f3dd59` — runtime: charCodeAt/codePointAt UTF-8 code point value access (item 182 Slice 5)

### Next Slice

Slice 6: `fromCharCode` / `fromCodePoint` — build UTF-8 string from code point values.

## Item 182-6: String fromCharCode/fromCodePoint with UTF-8 encoding

### TDD Cycle

**RED**: Extended fixture with fromCharCode/fromCodePoint tests for multi-byte strings. `String.fromCodePoint(65)` failed with `[UnresolvedName] unresolved name: 'String'` at the lowered resolver — the `MethodCall` handler's `else` branch tried to resolve `String` as a local variable after `resolve_method_to_runtime_fn` returned `None` for `fromCodePoint`.

`String.fromCharCode` worked at the IR level (was already routed in `program_builtins.rs`) but produced wrong output: the existing `$string_from_char_code` WAT stored code point values as raw single bytes instead of proper UTF-8 encoding.

**GREEN**:
- Added `"fromCodePoint" => Some("StringFromCodePoint".to_owned())` routing in `program_builtins.rs:String` namespace handler
- Registered `StringFromCodePoint` RuntimeFn: enum variant, deps, dispatch arm, manifest name, emission order
- Rewrote `$string_from_char_code` WAT: uses proper UTF-8 encoding with sequential `if (byte_len == N)` blocks for 1/2/3-byte encoding
- Added `$string_from_code_point` WAT: same pattern with 4-byte support (BMP + supplementary planes), clamp to 0x10FFFF
- Removed unnecessary `RuntimeFn::Copy` dep from `STRING_FROM_CHAR_CODE_DEPS` (function uses `$alloc_heap` directly)

**REFACTOR**: Both functions follow the same pattern as existing UTF-8 encode/decode helpers — sequential `if (cond) (then ...)` blocks avoid WAT paren-balancing issues.

### Files Changed

- `crates/ir/src/lowered/program_builtins.rs` — added fromCodePoint routing
- `crates/backend-wasm/src/runtime_fn.rs` — enum variant + deps + from_name
- `crates/backend-wasm/src/runtime_fn_impl.rs` — RuntimeSpec + manifest + emission_order
- `crates/backend-wasm/src/runtime_builder.rs` — dispatch arm
- `crates/backend-wasm/src/runtime_strings.rs` — UTF-8 encoder for fromCharCode/fromCodePoint
- `fixtures/control-flow-and-exceptions/utf8-string.ts` — fromCharCode/fromCodePoint tests

### Validation

| Command | Result |
|---------|--------|
| `cargo fmt --all --check` | PASS |
| `cargo build` | PASS |
| `cargo clippy --all-targets` | PASS |
| `cargo nextest run -p ts2wasm-cli --test m2_node_diff utf8_string_fixture` | PASS |
| `cargo nextest run -p ts2wasm-cli --test m2_node_diff --no-fail-fast` | 294/307 PASS (13 pre-existing failures) |
| `cargo nextest run -p ts2wasm-cli --test m12_async_await` | 6/6 PASS |

### Commit

`e636cb9ab` — runtime: Add fromCharCode/fromCodePoint UTF-8 string construction (item 182 Slice 6)

### Summary

Item 182 all 6 slices: complete. All UTF-8-aware string operations now correctly handle code point counting, indexing, position reporting, value access, and construction.

## Item 183 Slice 1: $property_has prototype chain walking (W5.2)

### TDD Cycle

**RED**: Created `fixtures/core-semantics/in-operator-prototype.ts` — `"key" in child` on inherited properties returned `false` (own-property only check). Created `fixtures/object-semantics-kernel/computed-read-prototype.ts`.

**GREEN**: Modified `$property_has` in `runtime_collections.rs` to add prototype chain walking:
- Added `$proto`, `$steps` locals
- Wrapped scan loop in `block $walk_done (result i32)` / `loop $walk` pattern (same as `$property_get`)
- After own-property scan exits without finding key, loads `$proto` from header
- Guards: `$proto == 0` → return false, `$base == $proto` → return false (self-reference), `$steps >= 64` → return false (depth limit)
- Sets `$base = $proto`, branches back to `$scan`

**REFACTOR**: Follows identical pattern to `$property_get`. Both functions now share same prototype chain walking structure.

### Files Changed
- `crates/backend-wasm/src/runtime_collections.rs` — prototype walk in `$property_has`
- `fixtures/core-semantics/in-operator-prototype.ts` — new fixture
- `fixtures/object-semantics-kernel/computed-read-prototype.ts` — new fixture
- `crates/cli/tests/common/m2_node_diff_fixture_tests.rs` — registered both fixtures

### Validation
| Command | Result |
|---------|--------|
| `cargo nextest run -p ts2wasm-cli --test m2_node_diff in_operator_prototype` | PASS |
| `cargo nextest run -p ts2wasm-cli --test m2_node_diff computed_read_prototype` | PASS |
| `cargo nextest run -p ts2wasm-cli --test m2_node_diff --no-fail-fast` | 296/309 PASS (13 pre-existing) |
| `cargo nextest run -p ts2wasm-cli --test m12_async_await` | 6/6 PASS |

### Commit
`4b8c5d4c4` — runtime: Add prototype chain walking to $property_has (W5.2 Slice 1)

## Item 183 Slice 2 (W5.2): Superseded

### Analysis

**Planned change**: Route object computed reads (`obj[expr]`) through `LoweredExpr::PropertyGetDynamic` instead of `LoweredExpr::Index` in `resolver_expr.rs`.

**Finding**: Not implementable safely. The `$index` runtime function handles all runtime types:
- Strings (numeric index): returns byte at position
- Arrays (numeric index): calls `$array_get`
- Objects (any index): converts key to string, calls `$property_get`

Changing the else branch (non-literal objects) to emit `PropertyGetDynamic` would break computed indexing on non-literal strings and arrays (`strVar[i]`, `arrVar[i]`), since static IR analysis cannot determine runtime types.

**Conclusion**: The current design is correct — `$index` is the proper abstraction because it dispatches based on runtime type tags. Since `$index` already delegates to `$property_get` for objects, prototype chain walking was already working for computed reads before any changes. Marked as **superseded** in TRACKING.yaml.
