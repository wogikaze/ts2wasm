# Phase 1: AS-IS Analysis — UTF-8 Runtime String Representation

## Item 182: Implement UTF-8-aware runtime string representation

## Current State

The runtime stores each byte as a single "character". Strings are stored as:
- Header: 4 bytes (length = byte count)
- Body: N bytes of raw data

All `runtime_strings.rs` WAT functions operate on **byte-level** indices:
- `$string_length`: returns byte count from header
- `$string_char_at`: loads byte at index, wraps in 1-char string
- `$string_substring`: byte-level substring
- `$string_index_of`: byte-level search
- `$string_char_code_at`: returns byte value as i32
- `$string_from_char_code`: stores byte value

## Affected Functions (33 total, 1714 lines)

Byte-level functions that need UTF-8 awareness:
1. `emit_string_length` (line 607) — byte count → code point count
2. `emit_string_char_at` (line 5) — byte index → code point index
3. `emit_string_at` (line 40) — byte index → code point index
4. `emit_string_substring` (line 79) — byte indices → code point indices
5. `emit_string_substr` (line 123) — byte indices → code point indices
6. `emit_string_slice` (line 183) — byte indices → code point indices
7. `emit_string_index_of` (line 233) — byte-level search → needs UTF-8 alignment
8. `emit_string_char_code_at` (line 1371) — byte index → code point at index

Shared helpers (`runtime_collections.rs`):
- `emit_get_length` — used by `.length` for all types, needs `$utf8_cp_count` for strings

## Required WAT Helpers

Need to add to runtime_strings.rs:
1. `$utf8_cp_count` — count UTF-8 code points from byte buffer
2. `$utf8_cp_to_byte_index` — convert code point index to byte index
3. `$utf8_cp_byte_length` — byte length of code point at position
4. `$utf8_decode_cp_at_byte` — decode full code point at byte position

## UTF-8 Encoding Rules

| Byte 1 | Continuation | Code points |
|--------|-------------|-------------|
| 0xxxxxxx | — | U+0000-U+007F (1 byte) |
| 110xxxxx | 10xxxxxx | U+0080-U+07FF (2 bytes) |
| 1110xxxx | 10xxxxxx ×2 | U+0800-U+FFFF (3 bytes) |
| 11110xxx | 10xxxxxx ×3 | U+10000-U+10FFFF (4 bytes) |

Continuation bytes: 0x80-0xBF (bits 6-7 = 10)
Leading byte high bit count = total byte length of code point

## pre-existing m2_node_diff Failure

`bun_stdin_text_fixture_matches_node_baseline_under_iwasm`:
- Expected: "hello\n", Got: "undefined\n"
- Root cause: WASI stdin not provided when running under iwasm
- NOT related to UTF-8 — this is a test infrastructure issue

## Plan for TDD Slices

1. **Slice 1**: Add `$utf8_cp_count` helper + fix `$string_length` (and `emit_get_length`)
2. **Slice 2**: Add `$utf8_cp_to_byte_index` + fix `$string_char_at` / `$string_at`
3. **Slice 3**: Fix `$string_substring` / `$string_substr` / `$string_slice`
4. **Slice 4**: Fix `$string_index_of` / `$string_last_index_of`
5. **Slice 5**: Fix `$string_char_code_at`
6. **Slice 6**: Fix `$string_pad_start` / `$string_pad_end` / `$string_repeat`
7. **Slice 7**: Fix remaining string functions (trim, startsWith, endsWith, case conversion)

Each slice: RED (failing fixture with multi-byte chars) → GREEN (WAT change) → REFACTOR.
