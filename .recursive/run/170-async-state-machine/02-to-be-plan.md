# Phase 2: TO-BE Plan — Slice 2: charAt/at with code point indexing

## Goal

Make `"日本語".charAt(0)` return "日" (U+65E5, 3 bytes) instead of the first byte.

## Approach

1. Add `$utf8_cp_to_byte_index` WAT helper: given string + code point index, return byte index
2. Modify `emit_string_char_at` to call `$utf8_cp_to_byte_index` and copy multi-byte char
3. Modify `emit_string_at` similarly

## TDD Steps

1. **RED**: Extend fixture with charAt/at tests for multi-byte strings — confirm failure
2. **GREEN**: Add helper + modify charAt/at
3. **REFACTOR**: Clean up
