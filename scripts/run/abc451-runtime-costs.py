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


def insert_before_function_end(text: str, func_name: str, insertion: str) -> str:
    start, end = find_balanced_span(text, f"(func {func_name}")
    return text[: end - 1] + insertion + text[end - 1 :]


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
    first_data = wat.find("  (data ")
    if first_data < 0:
        raise RuntimeError("missing data segment insertion point")
    wat = wat[:first_data] + globals_wat + wat[first_data:]

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
    wat = replace_once(
        wat,
        "  (func $copy (param $src i32) (param $dst i32) (param $len i32)\n    (local $i i32)\n",
        "  (func $copy (param $src i32) (param $dst i32) (param $len i32)\n    (local $i i32)\n"
        + copy_probe,
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
    (call $abc451_diag_emit_counter (global.get $abc451_diag_free_list_scan_visits)))

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
    wat = wat.replace(start_marker, report_fn + start_marker, 1)
    wat = insert_before_function_end(wat, "$_start", "\n    (call $abc451_diag_report)\n")
    return wat


def parse_counter_lines(stdout: str) -> dict[str, int]:
    numeric_lines: list[int] = []
    for line in stdout.splitlines():
        stripped = line.strip()
        if stripped.lstrip("-").isdigit():
            numeric_lines.append(int(stripped))
    if len(numeric_lines) < len(COUNTERS):
        raise RuntimeError(
            f"expected at least {len(COUNTERS)} numeric counter lines, got {len(numeric_lines)}"
        )
    values = numeric_lines[-len(COUNTERS) :]
    return dict(zip(COUNTERS, values))


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
    return {
        "fixture": str(fixture.relative_to(REPO_ROOT) if fixture.is_relative_to(REPO_ROOT) else fixture),
        "diagnostic": "abc451-runtime-costs",
        "default_off": True,
        "event_budget": args.event_budget,
        "diagnostic_stop": iwasm.returncode != 0 and not timed_out,
        "timed_out": timed_out,
        "counters": counters,
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
