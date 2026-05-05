#!/usr/bin/env python3
"""Default-off ABC451 runtime cost attribution diagnostic.

This script leaves normal ts2wasm output untouched. It asks the CLI for the
ordinary WAT, instruments a temporary copy with counters, compiles that copy,
and runs it under iwasm. The diagnostic intentionally stops after an event
budget so the depth-8 timeout path still yields baseline counters.
"""

from __future__ import annotations

import argparse
import json
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[2]
DEFAULT_FIXTURE = "fixtures/core-semantics/abc451-depth8-live-set.ts"
HEAP_START = 2048

COUNTERS = [
    "array_copy_calls",
    "array_copy_bytes",
    "array_copy_elements",
    "all_copy_calls",
    "all_copy_bytes",
    "allocation_attempts",
    "allocation_requested_bytes",
    "gc_collections",
    "sweep_visits",
    "free_list_scan_visits",
]

COPY_ATTRIBUTION_COUNTERS = [
    "copy_array_growth_calls",
    "copy_array_growth_bytes",
    "copy_concat_left_calls",
    "copy_concat_left_bytes",
    "copy_concat_right_calls",
    "copy_concat_right_bytes",
    "copy_value_to_string_calls",
    "copy_value_to_string_bytes",
    "copy_array_map_string_calls",
    "copy_array_map_string_bytes",
]

ALLOC_ATTRIBUTION_COUNTERS = [
    "alloc_array_growth_calls",
    "alloc_array_growth_bytes",
    "alloc_concat_calls",
    "alloc_concat_bytes",
    "alloc_array_map_result_calls",
    "alloc_array_map_result_bytes",
    "alloc_array_map_string_calls",
    "alloc_array_map_string_bytes",
    "alloc_number_to_string_calls",
    "alloc_number_to_string_bytes",
    "alloc_scratch_array_calls",
    "alloc_scratch_array_bytes",
    "alloc_gc_roots_calls",
    "alloc_gc_roots_bytes",
]

ARRAY_PUSH_GROW_COUNTERS = [
    "array_push_capacity_hits",
    "array_push_capacity_misses",
    "array_push_top_heap_hits",
    "array_push_top_heap_miss_non_top",
    "array_push_top_heap_miss_memory",
    "array_push_non_top_after_array_block",
    "array_push_non_top_after_non_array_block",
    "array_push_non_top_after_unknown_block",
    "array_push_non_top_unknown_separation",
    "array_push_growth_double_capacity",
    "array_push_growth_linear_capacity",
    "array_push_growth_min_capacity",
    "array_push_growth_required_capacity",
]

ALL_COUNTERS = COUNTERS + COPY_ATTRIBUTION_COUNTERS + ALLOC_ATTRIBUTION_COUNTERS + ARRAY_PUSH_GROW_COUNTERS


def run_checked(args: list[str], *, cwd: Path, timeout: int | None = None) -> subprocess.CompletedProcess[str]:
    result = subprocess.run(
        args,
        cwd=cwd,
        text=True,
        capture_output=True,
        timeout=timeout,
    )
    if result.returncode != 0:
        raise RuntimeError(
            f"command failed ({result.returncode}): {' '.join(args)}\n"
            f"stdout:\n{result.stdout}\n"
            f"stderr:\n{result.stderr}"
        )
    return result


def extract_wat(stdout: str) -> str:
    start = stdout.find("(module")
    if start < 0:
        raise RuntimeError("ts2wasm dump did not emit a WAT module")
    return stdout[start:]


def find_balanced_span(text: str, needle: str) -> tuple[int, int]:
    start = text.find(needle)
    if start < 0:
        raise RuntimeError(f"missing WAT fragment: {needle}")
    open_at = text.find("(", start)
    if open_at < 0:
        raise RuntimeError(f"missing opening paren for WAT fragment: {needle}")
    depth = 0
    for index in range(open_at, len(text)):
        char = text[index]
        if char == "(":
            depth += 1
        elif char == ")":
            depth -= 1
            if depth == 0:
                return open_at, index + 1
    raise RuntimeError(f"unterminated WAT fragment: {needle}")


def replace_once(text: str, old: str, new: str) -> str:
    count = text.count(old)
    if count != 1:
        raise RuntimeError(f"expected exactly one occurrence of {old!r}, found {count}")
    return text.replace(old, new, 1)


def replace_expected(text: str, old: str, new: str, *, expected: int, label: str) -> str:
    count = text.count(old)
    if count != expected:
        raise RuntimeError(f"expected {expected} occurrences for {label}, found {count}")
    return text.replace(old, new)


def replace_optional_once(text: str, old: str, new: str, *, label: str) -> str:
    count = text.count(old)
    if count > 1:
        raise RuntimeError(f"expected at most 1 occurrence for {label}, found {count}")
    return text.replace(old, new)


def replace_at_most(text: str, old: str, new: str, *, max_expected: int, label: str) -> str:
    count = text.count(old)
    if count > max_expected:
        raise RuntimeError(f"expected at most {max_expected} occurrences for {label}, found {count}")
    return text.replace(old, new)


def insert_before_function_end(text: str, func_name: str, insertion: str) -> str:
    start, end = find_balanced_span(text, f"(func {func_name}")
    return text[: end - 1] + insertion + text[end - 1 :]


def counter_globals(names: list[str]) -> str:
    return "".join(f"  (global $abc451_diag_{name} (mut i32) (i32.const 0))\n" for name in names)


def emit_counter_calls(names: list[str]) -> str:
    return "".join(
        f"    (call $abc451_diag_emit_counter (global.get $abc451_diag_{name}))\n" for name in names
    )


def copy_attribution_wrapper(name: str) -> str:
    return f"""
  (func $abc451_diag_{name} (param $src i32) (param $dst i32) (param $len i32)
    (if (i32.eqz (global.get $abc451_diag_reported))
      (then
        (global.set $abc451_diag_{name}_calls
          (i32.add (global.get $abc451_diag_{name}_calls) (i32.const 1)))
        (global.set $abc451_diag_{name}_bytes
          (i32.add (global.get $abc451_diag_{name}_bytes) (local.get $len)))))
    (call $copy (local.get $src) (local.get $dst) (local.get $len)))
"""


def alloc_attribution_wrapper(name: str) -> str:
    return f"""
  (func $abc451_diag_{name} (param $size i32) (result i32)
    (if (i32.eqz (global.get $abc451_diag_reported))
      (then
        (global.set $abc451_diag_{name}_calls
          (i32.add (global.get $abc451_diag_{name}_calls) (i32.const 1)))
        (global.set $abc451_diag_{name}_bytes
          (i32.add (global.get $abc451_diag_{name}_bytes) (local.get $size)))))
    (call $alloc_heap (local.get $size)))
"""


def attribution_wrappers() -> str:
    return (
        copy_attribution_wrapper("copy_array_growth")
        + copy_attribution_wrapper("copy_concat_left")
        + copy_attribution_wrapper("copy_concat_right")
        + copy_attribution_wrapper("copy_value_to_string")
        + copy_attribution_wrapper("copy_array_map_string")
        + alloc_attribution_wrapper("alloc_array_growth")
        + alloc_attribution_wrapper("alloc_concat")
        + alloc_attribution_wrapper("alloc_array_map_result")
        + alloc_attribution_wrapper("alloc_array_map_string")
        + alloc_attribution_wrapper("alloc_number_to_string")
        + alloc_attribution_wrapper("alloc_scratch_array")
        + alloc_attribution_wrapper("alloc_gc_roots")
    )


def array_push_grow_helpers() -> str:
    return """
  (func $abc451_diag_array_push_capacity_check (param $len i32) (param $cap i32) (result i32)
    (if (i32.lt_u (local.get $len) (local.get $cap))
      (then
        (if (i32.eqz (global.get $abc451_diag_reported))
          (then
            (global.set $abc451_diag_array_push_capacity_hits
              (i32.add (global.get $abc451_diag_array_push_capacity_hits) (i32.const 1))))))
      (else
        (if (i32.eqz (global.get $abc451_diag_reported))
          (then
            (global.set $abc451_diag_array_push_capacity_misses
              (i32.add (global.get $abc451_diag_array_push_capacity_misses) (i32.const 1)))))))
    (i32.lt_u (local.get $len) (local.get $cap)))

  (func $abc451_diag_array_push_growth_capacity (param $len i32) (param $old_cap i32) (param $new_cap i32)
    (if (i32.eqz (global.get $abc451_diag_reported))
      (then
        (if (i32.gt_u (local.get $old_cap) (i32.const 3072))
          (then
            (global.set $abc451_diag_array_push_growth_linear_capacity
              (i32.add (global.get $abc451_diag_array_push_growth_linear_capacity) (i32.const 1))))
          (else
            (global.set $abc451_diag_array_push_growth_double_capacity
              (i32.add (global.get $abc451_diag_array_push_growth_double_capacity) (i32.const 1)))))
        (if (i32.eq (local.get $new_cap) (i32.const 4))
          (then
            (global.set $abc451_diag_array_push_growth_min_capacity
              (i32.add (global.get $abc451_diag_array_push_growth_min_capacity) (i32.const 1)))))
        (if (i32.eq (local.get $new_cap) (i32.add (local.get $len) (i32.const 1)))
          (then
            (global.set $abc451_diag_array_push_growth_required_capacity
              (i32.add (global.get $abc451_diag_array_push_growth_required_capacity) (i32.const 1))))))))

  (func $abc451_diag_array_push_top_check (param $array_ptr i32) (param $new_body_size i32) (result i32)
    (local $old_body_size i32)
    (local $current_end i32)
    (local $requested_end i32)
    (local $memory_bytes i32)
    (local $is_top i32)
    (local $fits_memory i32)
    (local $next_kind i32)
    (local.set $old_body_size
      (i32.load
        (i32.add
          (i32.sub (local.get $array_ptr) (i32.const 16))
          (i32.const 4))))
    (local.set $current_end (i32.add (local.get $array_ptr) (local.get $old_body_size)))
    (local.set $requested_end
      (i32.add
        (local.get $array_ptr)
        (i32.and
          (i32.add (local.get $new_body_size) (i32.const 7))
          (i32.const -8))))
    (local.set $memory_bytes (i32.mul (memory.size) (i32.const 65536)))
    (local.set $is_top (i32.eq (global.get $heap) (local.get $current_end)))
    (local.set $fits_memory (i32.le_u (local.get $requested_end) (local.get $memory_bytes)))
    (if (i32.eqz (global.get $abc451_diag_reported))
      (then
        (if (i32.and (local.get $is_top) (local.get $fits_memory))
          (then
            (global.set $abc451_diag_array_push_top_heap_hits
              (i32.add (global.get $abc451_diag_array_push_top_heap_hits) (i32.const 1)))))
        (if (i32.eqz (local.get $is_top))
          (then
            (global.set $abc451_diag_array_push_top_heap_miss_non_top
              (i32.add (global.get $abc451_diag_array_push_top_heap_miss_non_top) (i32.const 1)))
            (if (i32.gt_u (global.get $heap) (local.get $current_end))
              (then
                (local.set $next_kind
                  (i32.and
                    (i32.load (local.get $current_end))
                    (i32.const 28)))
                (if (i32.eq (local.get $next_kind) (i32.const 8))
                  (then
                    (global.set $abc451_diag_array_push_non_top_after_array_block
                      (i32.add (global.get $abc451_diag_array_push_non_top_after_array_block) (i32.const 1))))
                  (else
                    (if (i32.eqz (local.get $next_kind))
                      (then
                        (global.set $abc451_diag_array_push_non_top_after_unknown_block
                          (i32.add (global.get $abc451_diag_array_push_non_top_after_unknown_block) (i32.const 1))))
                      (else
                        (global.set $abc451_diag_array_push_non_top_after_non_array_block
                          (i32.add (global.get $abc451_diag_array_push_non_top_after_non_array_block) (i32.const 1))))))))
              (else
                (global.set $abc451_diag_array_push_non_top_unknown_separation
                  (i32.add (global.get $abc451_diag_array_push_non_top_unknown_separation) (i32.const 1)))))))
        (if (i32.and (local.get $is_top) (i32.eqz (local.get $fits_memory)))
          (then
            (global.set $abc451_diag_array_push_top_heap_miss_memory
              (i32.add (global.get $abc451_diag_array_push_top_heap_miss_memory) (i32.const 1))))))))
    (i32.and (local.get $is_top) (local.get $fits_memory)))
"""


def instrument_array_push_grow_attribution(wat: str) -> str:
    wat = replace_expected(
        wat,
        "      (i32.lt_u (local.get $old_len) (local.get $old_capacity))",
        "      (call $abc451_diag_array_push_capacity_check (local.get $old_len) (local.get $old_capacity))",
        expected=1,
        label="array push capacity helper",
    )
    wat = replace_expected(
        wat,
        """        (if (i32.lt_u (local.get $new_capacity) (i32.add (local.get $old_len) (i32.const 1)))
          (then (local.set $new_capacity (i32.add (local.get $old_len) (i32.const 1)))))
        (if (result i32)""",
        """        (if (i32.lt_u (local.get $new_capacity) (i32.add (local.get $old_len) (i32.const 1)))
          (then (local.set $new_capacity (i32.add (local.get $old_len) (i32.const 1)))))
        (call $abc451_diag_array_push_growth_capacity (local.get $old_len) (local.get $old_capacity) (local.get $new_capacity))
        (if (result i32)""",
        expected=1,
        label="array push growth capacity helper",
    )
    wat = replace_expected(
        wat,
        """          (i32.and
            (i32.eq
              (global.get $heap)
              (i32.add
                (i32.and (local.get $arr) (i32.const -8))
                (i32.load
                  (i32.add
                    (i32.sub (i32.and (local.get $arr) (i32.const -8)) (i32.const 16))
                    (i32.const 4)))))
            (i32.le_u
              (i32.add
                (i32.and (local.get $arr) (i32.const -8))
                (i32.and
                  (i32.add
                    (i32.add
                      (i32.const 20)
                      (i32.shl (local.get $new_capacity) (i32.const 2)))
                    (i32.const 7))
                  (i32.const -8)))
              (i32.mul (memory.size) (i32.const 65536))))""",
        """          (call $abc451_diag_array_push_top_check
            (i32.and (local.get $arr) (i32.const -8))
            (i32.add
              (i32.const 20)
              (i32.shl (local.get $new_capacity) (i32.const 2))))""",
        expected=1,
        label="array push top check helper",
    )
    return wat


def instrument_callsite_attribution(wat: str) -> str:
    wat = replace_optional_once(
        wat,
        "        (call $copy (i32.const 300) (local.get $ptr) (i32.const 9))",
        "        (call $abc451_diag_copy_value_to_string (i32.const 300) (local.get $ptr) (i32.const 9))",
        label="value_to_string undefined copy",
    )
    wat = replace_optional_once(
        wat,
        "        (call $copy (i32.const 284) (local.get $ptr) (i32.const 4))",
        "        (call $abc451_diag_copy_value_to_string (i32.const 284) (local.get $ptr) (i32.const 4))",
        label="value_to_string true copy",
    )
    wat = replace_optional_once(
        wat,
        "        (call $copy (i32.const 268) (local.get $ptr) (i32.const 5))",
        "        (call $abc451_diag_copy_value_to_string (i32.const 268) (local.get $ptr) (i32.const 5))",
        label="value_to_string false copy",
    )
    wat = replace_optional_once(
        wat,
        "        (call $copy (i32.const 292) (local.get $ptr) (i32.const 4))",
        "        (call $abc451_diag_copy_value_to_string (i32.const 292) (local.get $ptr) (i32.const 4))",
        label="value_to_string null copy",
    )
    wat = replace_expected(
        wat,
        "        (call $copy (i32.add (local.get $obj) (i32.const 4)) (local.get $ptr) (local.get $len))",
        "        (call $abc451_diag_copy_value_to_string (i32.add (local.get $obj) (i32.const 4)) (local.get $ptr) (local.get $len))",
        expected=1,
        label="value_to_string inline string copy",
    )
    wat = replace_expected(
        wat,
        """            (call $copy
              (i32.add (local.get $obj) (i32.const 20))
              (local.get $ptr)
              (local.get $len))""",
        """            (call $abc451_diag_copy_value_to_string
              (i32.add (local.get $obj) (i32.const 20))
              (local.get $ptr)
              (local.get $len))""",
        expected=1,
        label="value_to_string bigint copy",
    )
    wat = replace_expected(
        wat,
        """            (call $copy
              (i32.add (local.get $obj) (i32.const 12))
              (local.get $ptr)
              (local.get $len))""",
        """            (call $abc451_diag_copy_value_to_string
              (i32.add (local.get $obj) (i32.const 12))
              (local.get $ptr)
              (local.get $len))""",
        expected=1,
        label="value_to_string heap string copy",
    )

    wat = replace_expected(
        wat,
        "    (call $copy (local.get $src_a) (local.get $data_ptr) (local.get $len_a))",
        "    (call $abc451_diag_copy_concat_left (local.get $src_a) (local.get $data_ptr) (local.get $len_a))",
        expected=1,
        label="concat left copy",
    )
    wat = replace_expected(
        wat,
        """    (call $copy
      (local.get $src_b)
      (i32.add (local.get $data_ptr) (local.get $len_a))
      (local.get $len_b))""",
        """    (call $abc451_diag_copy_concat_right
      (local.get $src_b)
      (i32.add (local.get $data_ptr) (local.get $len_a))
      (local.get $len_b))""",
        expected=1,
        label="concat right copy",
    )
    wat = replace_expected(
        wat,
        """        (call $copy
          (i32.const 1500)
          (i32.add (local.get $mapped_ptr) (i32.const 4))
          (local.get $mapped_len))""",
        """        (call $abc451_diag_copy_array_map_string
          (i32.const 1500)
          (i32.add (local.get $mapped_ptr) (i32.const 4))
          (local.get $mapped_len))""",
        expected=1,
        label="array_map string copy",
    )
    wat = replace_expected(
        wat,
        """            (call $copy
              (i32.add (i32.and (local.get 8) (i32.const -8)) (i32.const 4))
              (i32.add (local.get 10) (i32.const 4))
              (i32.shl (local.get 11) (i32.const 2)))""",
        """            (call $abc451_diag_copy_array_growth
              (i32.add (i32.and (local.get 8) (i32.const -8)) (i32.const 4))
              (i32.add (local.get 10) (i32.const 4))
              (i32.shl (local.get 11) (i32.const 2)))""",
        expected=0,
        label="array growth copy local 8/10/11 outer",
    )
    wat = replace_expected(
        wat,
        """                (call $copy
                  (i32.add (i32.and (local.get 8) (i32.const -8)) (i32.const 4))
                  (i32.add (local.get 10) (i32.const 4))
                  (i32.shl (local.get 11) (i32.const 2)))""",
        """                (call $abc451_diag_copy_array_growth
                  (i32.add (i32.and (local.get 8) (i32.const -8)) (i32.const 4))
                  (i32.add (local.get 10) (i32.const 4))
                  (i32.shl (local.get 11) (i32.const 2)))""",
        expected=0,
        label="array growth copy local 8/10/11 inner",
    )
    wat = replace_expected(
        wat,
        """            (call $copy
              (i32.add (i32.and (local.get 4) (i32.const -8)) (i32.const 4))
              (i32.add (local.get 6) (i32.const 4))
              (i32.shl (local.get 7) (i32.const 2)))""",
        """            (call $abc451_diag_copy_array_growth
              (i32.add (i32.and (local.get 4) (i32.const -8)) (i32.const 4))
              (i32.add (local.get 6) (i32.const 4))
              (i32.shl (local.get 7) (i32.const 2)))""",
        expected=0,
        label="array growth copy local 4/6/7",
    )
    wat = replace_expected(
        wat,
        """            (call $copy
              (i32.add (i32.and (local.get $arr) (i32.const -8)) (i32.const 20))
              (i32.add (local.get $new_array) (i32.const 20))
              (i32.shl (local.get $old_len) (i32.const 2)))""",
        """            (call $abc451_diag_copy_array_growth
              (i32.add (i32.and (local.get $arr) (i32.const -8)) (i32.const 20))
              (i32.add (local.get $new_array) (i32.const 20))
              (i32.shl (local.get $old_len) (i32.const 2)))""",
        expected=1,
        label="array growth copy helper",
    )

    wat = replace_expected(
        wat,
        """      (call $alloc_heap
        (i32.add (i32.const 12) (local.get $str_len)))""",
        """      (call $abc451_diag_alloc_number_to_string
        (i32.add (i32.const 12) (local.get $str_len)))""",
        expected=1,
        label="number_to_string allocation",
    )
    wat = replace_expected(
        wat,
        """      (call $alloc_heap
        (i32.add
          (i32.const 4)
          (i32.add (local.get $len_a) (local.get $len_b))))""",
        """      (call $abc451_diag_alloc_concat
        (i32.add
          (i32.const 4)
          (i32.add (local.get $len_a) (local.get $len_b))))""",
        expected=1,
        label="concat allocation",
    )
    wat = replace_expected(
        wat,
        """      (call $alloc_heap
        (i32.add
          (i32.const 20)
          (i32.shl (local.get $len) (i32.const 2))))""",
        """      (call $abc451_diag_alloc_array_map_result
        (i32.add
          (i32.const 20)
          (i32.shl (local.get $len) (i32.const 2))))""",
        expected=1,
        label="array_map result allocation",
    )
    wat = replace_expected(
        wat,
        """          (call $alloc_heap
            (i32.add (i32.const 4) (local.get $mapped_len)))""",
        """          (call $abc451_diag_alloc_array_map_string
            (i32.add (i32.const 4) (local.get $mapped_len)))""",
        expected=1,
        label="array_map string allocation",
    )
    wat = replace_at_most(
        wat,
        "(call $alloc_heap (i32.const 4))",
        "(call $abc451_diag_alloc_scratch_array (i32.const 4))",
        max_expected=2,
        label="scratch array allocation",
    )
    wat = replace_expected(
        wat,
        "(call $alloc_heap (i32.const 16452))",
        "(call $abc451_diag_alloc_gc_roots (i32.const 16452))",
        expected=1,
        label="gc roots allocation",
    )
    wat = replace_expected(
        wat,
        """                  (call $alloc_heap
                    (i32.add
                      (i32.const 4)
                      (i32.shl (local.get 13) (i32.const 2))))""",
        """                  (call $abc451_diag_alloc_array_growth
                    (i32.add
                      (i32.const 4)
                      (i32.shl (local.get 13) (i32.const 2))))""",
        expected=0,
        label="array growth allocation local 13 outer",
    )
    wat = replace_expected(
        wat,
        """                      (call $alloc_heap
                        (i32.add
                          (i32.const 4)
                          (i32.shl (local.get 13) (i32.const 2))))""",
        """                      (call $abc451_diag_alloc_array_growth
                        (i32.add
                        (i32.const 4)
                        (i32.shl (local.get 13) (i32.const 2))))""",
        expected=0,
        label="array growth allocation local 13 inner",
    )
    wat = replace_expected(
        wat,
        """                  (call $alloc_heap
                    (i32.add
                      (i32.const 4)
                      (i32.shl (local.get 9) (i32.const 2))))""",
        """                  (call $abc451_diag_alloc_array_growth
                    (i32.add
                      (i32.const 4)
                      (i32.shl (local.get 9) (i32.const 2))))""",
        expected=0,
        label="array growth allocation local 9",
    )
    wat = replace_expected(
        wat,
        """              (call $alloc_heap
                (i32.add
                  (i32.const 20)
                  (i32.shl (local.get $new_capacity) (i32.const 2))))""",
        """              (call $abc451_diag_alloc_array_growth
                (i32.add
                  (i32.const 20)
                  (i32.shl (local.get $new_capacity) (i32.const 2))))""",
        expected=1,
        label="array growth allocation helper",
    )
    return wat


def instrument_wat(wat: str, event_budget: int) -> str:
    globals_wat = """
  (global $abc451_diag_array_copy_calls (mut i32) (i32.const 0))
  (global $abc451_diag_array_copy_bytes (mut i32) (i32.const 0))
  (global $abc451_diag_all_copy_calls (mut i32) (i32.const 0))
  (global $abc451_diag_all_copy_bytes (mut i32) (i32.const 0))
  (global $abc451_diag_alloc_attempts (mut i32) (i32.const 0))
  (global $abc451_diag_alloc_requested_bytes (mut i32) (i32.const 0))
  (global $abc451_diag_gc_collections (mut i32) (i32.const 0))
  (global $abc451_diag_sweep_visits (mut i32) (i32.const 0))
  (global $abc451_diag_free_list_scan_visits (mut i32) (i32.const 0))
  (global $abc451_diag_events (mut i32) (i32.const 0))
  (global $abc451_diag_reported (mut i32) (i32.const 0))
"""
    globals_wat += counter_globals(COPY_ATTRIBUTION_COUNTERS + ALLOC_ATTRIBUTION_COUNTERS)
    globals_wat += counter_globals(ARRAY_PUSH_GROW_COUNTERS)
    first_data = wat.find("  (data ")
    if first_data < 0:
        raise RuntimeError("missing data segment insertion point")
    wat = wat[:first_data] + globals_wat + wat[first_data:]
    wat = instrument_callsite_attribution(wat)
    wat = instrument_array_push_grow_attribution(wat)

    copy_probe = f"""
    (if (i32.eqz (global.get $abc451_diag_reported))
      (then
        (global.set $abc451_diag_all_copy_calls
          (i32.add (global.get $abc451_diag_all_copy_calls) (i32.const 1)))
        (global.set $abc451_diag_all_copy_bytes
          (i32.add (global.get $abc451_diag_all_copy_bytes) (local.get $len)))
        (if
          (i32.and
            (i32.and
              (i32.eqz (i32.rem_u (local.get $len) (i32.const 4)))
              (i32.ge_u (local.get $src) (i32.const {HEAP_START})))
            (i32.ge_u (local.get $dst) (i32.const {HEAP_START})))
          (then
            (global.set $abc451_diag_array_copy_calls
              (i32.add (global.get $abc451_diag_array_copy_calls) (i32.const 1)))
            (global.set $abc451_diag_array_copy_bytes
              (i32.add (global.get $abc451_diag_array_copy_bytes) (local.get $len)))))
        (call $abc451_diag_tick)))
"""
    old_copy_header = "  (func $copy (param $src i32) (param $dst i32) (param $len i32)\n    (local $i i32)\n"
    bulk_copy_header = "  (func $copy (param $src i32) (param $dst i32) (param $len i32)\n"
    if old_copy_header in wat:
        wat = replace_once(
            wat,
            old_copy_header,
            old_copy_header + copy_probe,
        )
    else:
        wat = replace_once(
            wat,
            bulk_copy_header,
            bulk_copy_header + copy_probe,
        )

    alloc_probe = """
    (if (i32.eqz (global.get $abc451_diag_reported))
      (then
        (global.set $abc451_diag_alloc_attempts
          (i32.add (global.get $abc451_diag_alloc_attempts) (i32.const 1)))
        (global.set $abc451_diag_alloc_requested_bytes
          (i32.add (global.get $abc451_diag_alloc_requested_bytes) (local.get $size)))
        (call $abc451_diag_tick)))
"""
    wat = replace_once(
        wat,
        "    (local $alloc_pressure i32)\n",
        "    (local $alloc_pressure i32)\n" + alloc_probe,
    )

    gc_probe = """
    (if (i32.eqz (global.get $abc451_diag_reported))
      (then
        (global.set $abc451_diag_gc_collections
          (i32.add (global.get $abc451_diag_gc_collections) (i32.const 1)))
        (call $abc451_diag_tick)))
"""
    wat = replace_once(
        wat,
        "  (func $gc_collect\n    ;;",
        "  (func $gc_collect\n" + gc_probe + "    ;;",
    )

    sweep_probe = """
        (if (i32.eqz (global.get $abc451_diag_reported))
          (then
            (global.set $abc451_diag_sweep_visits
              (i32.add (global.get $abc451_diag_sweep_visits) (i32.const 1)))
            (call $abc451_diag_tick)))
"""
    wat = replace_once(
        wat,
        "      (loop $scan\n        (br_if $done (i32.ge_u (local.get $cursor) (local.get $heap_end)))\n",
        "      (loop $scan\n        (br_if $done (i32.ge_u (local.get $cursor) (local.get $heap_end)))\n"
        + sweep_probe,
    )

    free_list_probe = """
            (if (i32.eqz (global.get $abc451_diag_reported))
              (then
                (global.set $abc451_diag_free_list_scan_visits
                  (i32.add (global.get $abc451_diag_free_list_scan_visits) (i32.const 1)))
                (call $abc451_diag_tick)))
"""
    wat = replace_once(
        wat,
        "          (loop $free_scan\n            (br_if $free_not_found (i32.eqz (local.get $free_header)))\n",
        "          (loop $free_scan\n            (br_if $free_not_found (i32.eqz (local.get $free_header)))\n"
        + free_list_probe,
    )

    report_fn = f"""
  (func $abc451_diag_emit_counter (param $value i32)
    (call $log
      (i32.or
        (i32.shl (local.get $value) (i32.const 3))
        (i32.const 4))))

  (func $abc451_diag_report
    (if (global.get $abc451_diag_reported)
      (then (return)))
    (global.set $abc451_diag_reported (i32.const 1))
    (call $abc451_diag_emit_counter (global.get $abc451_diag_array_copy_calls))
    (call $abc451_diag_emit_counter (global.get $abc451_diag_array_copy_bytes))
    (call $abc451_diag_emit_counter
      (i32.div_u (global.get $abc451_diag_array_copy_bytes) (i32.const 4)))
    (call $abc451_diag_emit_counter (global.get $abc451_diag_all_copy_calls))
    (call $abc451_diag_emit_counter (global.get $abc451_diag_all_copy_bytes))
    (call $abc451_diag_emit_counter (global.get $abc451_diag_alloc_attempts))
    (call $abc451_diag_emit_counter (global.get $abc451_diag_alloc_requested_bytes))
    (call $abc451_diag_emit_counter (global.get $abc451_diag_gc_collections))
    (call $abc451_diag_emit_counter (global.get $abc451_diag_sweep_visits))
    (call $abc451_diag_emit_counter (global.get $abc451_diag_free_list_scan_visits))
{emit_counter_calls(COPY_ATTRIBUTION_COUNTERS + ALLOC_ATTRIBUTION_COUNTERS + ARRAY_PUSH_GROW_COUNTERS).rstrip()})

  (func $abc451_diag_tick
    (global.set $abc451_diag_events
      (i32.add (global.get $abc451_diag_events) (i32.const 1)))
    (if
      (i32.ge_u
        (global.get $abc451_diag_events)
        (i32.const {event_budget}))
      (then
        (call $abc451_diag_report)
        (unreachable))))
"""
    start_marker = "  (func $_start"
    if start_marker not in wat:
        raise RuntimeError("missing $_start insertion point")
    wat = wat.replace(start_marker, attribution_wrappers() + report_fn + start_marker, 1)
    wat = wat.replace(attribution_wrappers(), attribution_wrappers() + array_push_grow_helpers(), 1)
    wat = insert_before_function_end(wat, "$_start", "\n    (call $abc451_diag_report)\n")
    return wat


def parse_counter_lines(stdout: str) -> dict[str, int]:
    numeric_lines: list[int] = []
    for line in stdout.splitlines():
        stripped = line.strip()
        if stripped.lstrip("-").isdigit():
            numeric_lines.append(int(stripped))
    if len(numeric_lines) < len(ALL_COUNTERS):
        raise RuntimeError(
            f"expected at least {len(ALL_COUNTERS)} numeric counter lines, got {len(numeric_lines)}"
        )
    values = numeric_lines[-len(ALL_COUNTERS) :]
    return dict(zip(ALL_COUNTERS, values))


def pair_entries(counters: dict[str, int], prefix: str) -> list[dict[str, int | str]]:
    entries: list[dict[str, int | str]] = []
    suffix = "_calls"
    for name, calls in counters.items():
        if not name.startswith(prefix) or not name.endswith(suffix):
            continue
        category = name[len(prefix) : -len(suffix)]
        bytes_name = f"{prefix}{category}_bytes"
        entries.append(
            {
                "category": category,
                "calls": calls,
                "bytes": counters.get(bytes_name, 0),
            }
        )
    return sorted(entries, key=lambda entry: (int(entry["bytes"]), int(entry["calls"])), reverse=True)


def build_attribution(counters: dict[str, int]) -> dict[str, Any]:
    copy_categories = pair_entries(counters, "copy_")
    alloc_categories = pair_entries(counters, "alloc_")
    copy_attributed_calls = sum(int(entry["calls"]) for entry in copy_categories)
    copy_attributed_bytes = sum(int(entry["bytes"]) for entry in copy_categories)
    alloc_attributed_calls = sum(int(entry["calls"]) for entry in alloc_categories)
    alloc_attributed_bytes = sum(int(entry["bytes"]) for entry in alloc_categories)
    copy_unattributed = {
        "category": "unattributed",
        "calls": counters["all_copy_calls"] - copy_attributed_calls,
        "bytes": counters["all_copy_bytes"] - copy_attributed_bytes,
    }
    alloc_unattributed = {
        "category": "unattributed",
        "calls": counters["allocation_attempts"] - alloc_attributed_calls,
        "bytes": counters["allocation_requested_bytes"] - alloc_attributed_bytes,
    }
    array_push_miss_reasons = [
        {
            "reason": "non_top_heap",
            "calls": counters["array_push_top_heap_miss_non_top"],
        },
        {
            "reason": "committed_memory",
            "calls": counters["array_push_top_heap_miss_memory"],
        },
    ]
    array_push_miss_reasons = sorted(array_push_miss_reasons, key=lambda entry: int(entry["calls"]), reverse=True)
    array_push_separation_reasons = [
        {
            "reason": "recursive_or_result_array_after_current_array",
            "calls": counters["array_push_non_top_after_array_block"],
        },
        {
            "reason": "retained_live_non_array_after_current_array",
            "calls": counters["array_push_non_top_after_non_array_block"],
        },
        {
            "reason": "intervening_unknown_heap_block_after_current_array",
            "calls": counters["array_push_non_top_after_unknown_block"],
        },
        {
            "reason": "unknown_or_non_forward_heap_separation",
            "calls": counters["array_push_non_top_unknown_separation"],
        },
    ]
    array_push_separation_reasons = sorted(
        array_push_separation_reasons,
        key=lambda entry: int(entry["calls"]),
        reverse=True,
    )
    return {
        "copy": {
            "top": copy_categories[:5],
            "unattributed": copy_unattributed,
        },
        "allocation": {
            "top": alloc_categories[:5],
            "unattributed": alloc_unattributed,
        },
        "top_targets": sorted(
            [
                {
                    "kind": "copy",
                    "category": str(entry["category"]),
                    "calls": int(entry["calls"]),
                    "bytes": int(entry["bytes"]),
                }
                for entry in copy_categories
            ]
            + [
                {
                    "kind": "allocation",
                    "category": str(entry["category"]),
                    "calls": int(entry["calls"]),
                    "bytes": int(entry["bytes"]),
                }
                for entry in alloc_categories
            ],
            key=lambda entry: (entry["bytes"], entry["calls"]),
            reverse=True,
        )[:8],
        "array_push_grow": {
            "capacity_hits": counters["array_push_capacity_hits"],
            "capacity_misses": counters["array_push_capacity_misses"],
            "top_heap_hits": counters["array_push_top_heap_hits"],
            "top_heap_misses": counters["array_push_top_heap_miss_non_top"]
            + counters["array_push_top_heap_miss_memory"],
            "miss_reasons": array_push_miss_reasons,
            "top_miss_reason": array_push_miss_reasons[0]["reason"] if array_push_miss_reasons else "none",
            "non_top_separation_reasons": array_push_separation_reasons,
            "intervening_allocation_after_array": {
                "calls": counters["array_push_non_top_after_array_block"]
                + counters["array_push_non_top_after_non_array_block"]
                + counters["array_push_non_top_after_unknown_block"],
            },
            "growth_capacity_paths": {
                "double_capacity": counters["array_push_growth_double_capacity"],
                "linear_capacity": counters["array_push_growth_linear_capacity"],
                "min_capacity": counters["array_push_growth_min_capacity"],
                "required_capacity": counters["array_push_growth_required_capacity"],
            },
            "fallback_allocation": {
                "calls": counters["alloc_array_growth_calls"],
                "bytes": counters["alloc_array_growth_bytes"],
            },
            "fallback_copy": {
                "calls": counters["copy_array_growth_calls"],
                "bytes": counters["copy_array_growth_bytes"],
            },
        },
    }


def run_iwasm(args: list[str], *, cwd: Path, timeout: int) -> subprocess.CompletedProcess[str]:
    return subprocess.run(args, cwd=cwd, text=True, capture_output=True, timeout=timeout)


def build_payload(args: argparse.Namespace) -> dict[str, Any]:
    if not shutil.which("wat2wasm"):
        raise RuntimeError("wat2wasm is required")
    if not shutil.which("iwasm"):
        raise RuntimeError("iwasm is required")

    fixture = Path(args.fixture)
    if not fixture.is_absolute():
        fixture = REPO_ROOT / fixture

    dump = run_checked(
        ["cargo", "run", "-q", "-p", "ts2wasm-cli", "--", "dump", "--wat", str(fixture)],
        cwd=REPO_ROOT,
        timeout=args.build_timeout,
    )
    wat = extract_wat(dump.stdout)
    instrumented_wat = instrument_wat(wat, args.event_budget)

    temp_root = Path(args.out_dir) if args.out_dir else Path(tempfile.mkdtemp(prefix="abc451-runtime-costs-"))
    temp_root.mkdir(parents=True, exist_ok=True)
    wat_path = temp_root / "abc451-runtime-costs.instrumented.wat"
    wasm_path = temp_root / "abc451-runtime-costs.instrumented.wasm"
    wat_path.write_text(instrumented_wat, encoding="utf-8")

    run_checked(["wat2wasm", str(wat_path), "-o", str(wasm_path)], cwd=REPO_ROOT)
    try:
        iwasm = run_iwasm(["iwasm", str(wasm_path)], cwd=REPO_ROOT, timeout=args.timeout)
        timed_out = False
    except subprocess.TimeoutExpired as exc:
        stdout = exc.stdout if isinstance(exc.stdout, str) else (exc.stdout or b"").decode()
        stderr = exc.stderr if isinstance(exc.stderr, str) else (exc.stderr or b"").decode()
        iwasm = subprocess.CompletedProcess(["iwasm", str(wasm_path)], 124, stdout, stderr)
        timed_out = True

    counters = parse_counter_lines(iwasm.stdout)
    attribution = build_attribution(counters)
    return {
        "fixture": str(fixture.relative_to(REPO_ROOT) if fixture.is_relative_to(REPO_ROOT) else fixture),
        "diagnostic": "abc451-runtime-costs",
        "default_off": True,
        "event_budget": args.event_budget,
        "diagnostic_stop": iwasm.returncode != 0 and not timed_out,
        "timed_out": timed_out,
        "counters": counters,
        "attribution": attribution,
        "runtime_exit": {
            "code": iwasm.returncode,
            "stderr_tail": iwasm.stderr[-500:],
        },
        "artifacts": {
            "wat": str(wat_path),
            "wasm": str(wasm_path),
        },
    }


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--fixture", default=DEFAULT_FIXTURE)
    parser.add_argument("--event-budget", type=int, default=100_000)
    parser.add_argument("--timeout", type=int, default=30)
    parser.add_argument("--build-timeout", type=int, default=60)
    parser.add_argument("--out-dir")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        payload = build_payload(args)
    except Exception as exc:  # noqa: BLE001 - command-line diagnostic needs a concise failure.
        print(f"abc451-runtime-costs: {exc}", file=sys.stderr)
        return 1
    print(json.dumps(payload, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
