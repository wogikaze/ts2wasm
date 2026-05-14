#!/usr/bin/env python3
"""Performance smoke test: compilation time and wasm binary size tracker.

Measures compilation time and wasm binary size for a sample set of fixtures,
records results in a JSON file for historical comparison, and reports
regression alerts if metrics exceed configured thresholds.

Usage:
  python scripts/perf/benchmark-tracker.py --help
  python scripts/perf/benchmark-tracker.py
  python scripts/perf/benchmark-tracker.py --fixtures-dir fixtures/ --output results.json

Options:
  --help                      Show this help message and exit.
  --fixtures-dir DIR          Directory containing fixture subdirectories
                              (default: fixtures/).
  --output FILE               Output JSON file for historical results
                              (default: artifacts/benchmark-results.json).
  --sample N                  Number of fixtures to benchmark per
                              subdirectory (default: 2).
  --seed N                    Random seed for deterministic fixture sampling
                              (default: 0).
  --threshold-time PCT        Compilation time regression threshold as
                              percentage (default: 20). Alert if new time
                              exceeds previous by this %.
  --threshold-time-min-delta-ms N
                              Minimum absolute compile-time increase before
                              reporting a time regression (default: 25).
  --threshold-size PCT        Wasm size regression threshold as percentage
                              (default: 20). Alert if new size exceeds
                              previous by this %.
  --skip-build                Skip cargo build step (use existing binary).
  --json                      Print a stable machine-readable summary to stdout.
  -v, --verbose               Print detailed per-fixture output.

The script always exits 0 (info-only reporting). Regression alerts are
printed to stderr but do not change the exit code.
"""

import sys
import subprocess
import json
import tempfile
import time
import random
import os
from pathlib import Path
from datetime import datetime, timezone

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
DEFAULT_OUTPUT_FILE = "artifacts/benchmark-results.json"
DEFAULT_FIXTURES_DIR = "fixtures"
DEFAULT_SAMPLE = 2
DEFAULT_SEED = 0
DEFAULT_THRESHOLD_PCT = 20
DEFAULT_THRESHOLD_TIME_MIN_DELTA_MS = 25

# Subdirectories excluded from performance sampling.
EXCLUDED_DIRS = {
    "negative",        # expected to fail compilation
    "parser-errors",   # expected to fail compilation
    "html-comments",   # edge-case parsing
    "linker",          # module system, multi-file
    "module-system",   # module system, multi-file
    "atcoder",         # competitive programming, may have special deps
    "node-apis",       # may depend on host APIs not available
}


def eprint(*args, **kwargs):
    print(*args, file=sys.stderr, **kwargs)


def show_help(exit_code=0):
    print(__doc__)
    sys.exit(exit_code)


def collect_fixtures(fixtures_dir: Path, sample: int, seed: int) -> list:
    """Collect fixture .ts files from subdirectories of fixtures_dir.

    Returns a list of (label, absolute_path) tuples. Samples up to `sample`
    deterministic files per subdirectory. Skips excluded directories and empty dirs.
    """
    rng = random.Random(seed)
    entries = []
    if not fixtures_dir.is_dir():
        eprint(f"error: fixtures directory not found: {fixtures_dir}")
        return entries

    dirs = sorted(
        d for d in fixtures_dir.iterdir()
        if d.is_dir() and d.name not in EXCLUDED_DIRS
    )
    for d in dirs:
        ts_files = sorted(
            f for f in d.iterdir()
            if f.suffix == ".ts" and not f.name.endswith("-unsupported.ts")
            and not f.name.endswith("-invalid.ts")
        )
        if not ts_files:
            continue
        selected = rng.sample(ts_files, min(sample, len(ts_files)))
        for sf in selected:
            entries.append((f"{d.name}/{sf.name}", sf))

    rng.shuffle(entries)
    return entries


def build_ts2wasm(skip_build: bool) -> Path | None:
    """Ensure ts2wasm binary is built. Returns path to binary or None on fail."""
    bin_path = REPO_ROOT / "target/debug/ts2wasm"
    if skip_build:
        if bin_path.exists():
            return bin_path
        eprint("note: --skip-build specified but binary not found; building anyway")
    eprint("building ts2wasm-cli...")
    result = subprocess.run(
        ["cargo", "build", "-q", "-p", "ts2wasm-cli"],
        cwd=REPO_ROOT,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        eprint(f"error: cargo build failed:\n{result.stderr.strip()}")
        return None
    return bin_path


def benchmark_fixture(bin_path: Path, fixture_path: Path) -> dict | None:
    """Compile a single fixture and return metrics dict.

    Returns None if compilation fails (the fixture uses unsupported features).
    """
    with tempfile.NamedTemporaryFile(suffix=".wasm", delete=False) as tmp:
        wasm_path = Path(tmp.name)

    try:
        start_ns = time.time_ns()
        result = subprocess.run(
            [str(bin_path), "build", str(fixture_path), "-o", str(wasm_path)],
            capture_output=True, text=True,
        )
        elapsed_ns = time.time_ns() - start_ns

        if result.returncode != 0:
            stderr = result.stderr.strip() or result.stdout.strip()
            return {"status": "build_fail", "error": stderr[:200]}

        size = wasm_path.stat().st_size if wasm_path.exists() else 0
        compile_ms = elapsed_ns // 1_000_000
        return {
            "status": "ok",
            "compile_time_ms": compile_ms,
            "wasm_size_bytes": size,
        }
    finally:
        if wasm_path.exists():
            wasm_path.unlink()


def load_history(output_path: Path) -> list:
    """Load previous benchmark results from JSON file."""
    if not output_path.exists():
        return []
    try:
        with open(output_path) as f:
            data = json.load(f)
        if isinstance(data, list):
            return data
        return []
    except (json.JSONDecodeError, OSError):
        return []


def compute_threshold(previous_value: int, threshold_pct: int) -> int:
    """Compute the alert threshold value given a previous metric.

    Returns the value above which an alert should be raised.
    """
    return int(previous_value * (1 + threshold_pct / 100))


def check_regression(
    label: str,
    current: int,
    previous: int,
    metric_name: str,
    threshold_pct: int,
    regressions: list,
    min_delta: int = 0,
):
    """Compare current vs previous metric and record regression if exceeded."""
    if previous <= 0:
        return
    threshold = compute_threshold(previous, threshold_pct)
    delta = current - previous
    if current > threshold and delta >= min_delta:
        pct_increase = ((current - previous) / previous) * 100
        regressions.append({
            "fixture": label,
            "metric": metric_name,
            "current": current,
            "previous": previous,
            "threshold": threshold,
            "min_delta": min_delta,
            "pct_increase": round(pct_increase, 1),
            "threshold_pct": threshold_pct,
        })


def benchmark_config(
    fixtures_dir_str: str,
    sample: int,
    seed: int,
    threshold_time_pct: int,
    threshold_size_pct: int,
    threshold_time_min_delta_ms: int,
) -> dict:
    """Return the config fields that define a comparable benchmark run."""
    return {
        "fixtures_dir": fixtures_dir_str,
        "sample": sample,
        "seed": seed,
        "threshold_time_pct": threshold_time_pct,
        "threshold_size_pct": threshold_size_pct,
        "threshold_time_min_delta_ms": threshold_time_min_delta_ms,
    }


def comparable_previous_record(previous_record: dict | None, config: dict) -> bool:
    """Return true when aggregate regression comparison is meaningful."""
    if not previous_record:
        return False
    return previous_record.get("benchmark_config") == config


def main():
    raw_args = sys.argv[1:]

    # Parse options manually (avoid argparse for self-contained simplicity)
    fixtures_dir_str = DEFAULT_FIXTURES_DIR
    output_file_str = DEFAULT_OUTPUT_FILE
    sample = DEFAULT_SAMPLE
    seed = DEFAULT_SEED
    threshold_time_pct = DEFAULT_THRESHOLD_PCT
    threshold_size_pct = DEFAULT_THRESHOLD_PCT
    threshold_time_min_delta_ms = DEFAULT_THRESHOLD_TIME_MIN_DELTA_MS
    skip_build = False
    json_mode = False
    verbose = False

    i = 0
    while i < len(raw_args):
        a = raw_args[i]
        if a == "--help":
            show_help(0)
        elif a == "--fixtures-dir" and i + 1 < len(raw_args):
            fixtures_dir_str = raw_args[i + 1]
            i += 2
        elif a == "--output" and i + 1 < len(raw_args):
            output_file_str = raw_args[i + 1]
            i += 2
        elif a == "--sample" and i + 1 < len(raw_args):
            try:
                sample = int(raw_args[i + 1])
                i += 2
            except ValueError:
                eprint(f"error: --sample requires an integer, got '{raw_args[i + 1]}'")
                sys.exit(0)
        elif a == "--seed" and i + 1 < len(raw_args):
            try:
                seed = int(raw_args[i + 1])
                i += 2
            except ValueError:
                eprint(f"error: --seed requires an integer, got '{raw_args[i + 1]}'")
                sys.exit(0)
        elif a == "--threshold-time" and i + 1 < len(raw_args):
            try:
                threshold_time_pct = int(raw_args[i + 1])
                i += 2
            except ValueError:
                eprint("error: --threshold-time requires an integer")
                sys.exit(0)
        elif a == "--threshold-time-min-delta-ms" and i + 1 < len(raw_args):
            try:
                threshold_time_min_delta_ms = int(raw_args[i + 1])
                i += 2
            except ValueError:
                eprint("error: --threshold-time-min-delta-ms requires an integer")
                sys.exit(0)
        elif a == "--threshold-size" and i + 1 < len(raw_args):
            try:
                threshold_size_pct = int(raw_args[i + 1])
                i += 2
            except ValueError:
                eprint("error: --threshold-size requires an integer")
                sys.exit(0)
        elif a == "--skip-build":
            skip_build = True
            i += 1
        elif a == "--json":
            json_mode = True
            i += 1
        elif a in ("-v", "--verbose"):
            verbose = True
            i += 1
        elif a.startswith("--"):
            eprint(f"warning: unknown option '{a}', ignoring")
            i += 1
        else:
            eprint(f"warning: unexpected positional argument '{a}', ignoring")
            i += 1

    fixtures_dir = REPO_ROOT / fixtures_dir_str
    output_path = REPO_ROOT / output_file_str

    # Collect fixtures
    eprint(f"fixtures directory: {fixtures_dir}")
    eprint(f"output file: {output_path}")
    eprint(f"sample per directory: {sample}")
    eprint(f"sample seed: {seed}")
    eprint(f"time threshold: {threshold_time_pct}%")
    eprint(f"time min delta: {threshold_time_min_delta_ms}ms")
    eprint(f"size threshold: {threshold_size_pct}%")

    config = benchmark_config(
        fixtures_dir_str,
        sample,
        seed,
        threshold_time_pct,
        threshold_size_pct,
        threshold_time_min_delta_ms,
    )
    fixtures = collect_fixtures(fixtures_dir, sample, seed)
    if not fixtures:
        eprint("warning: no fixtures found for benchmarking")
        eprint("benchmark-tracker: OK (no fixtures, nothing to do)")
        sys.exit(0)

    eprint(f"collected {len(fixtures)} fixture(s) for benchmarking")

    # Build binary
    bin_path = build_ts2wasm(skip_build)
    if bin_path is None:
        eprint("error: ts2wasm binary not available, skipping benchmarks")
        eprint("benchmark-tracker: OK (compiler unavailable, skipped)")
        sys.exit(0)

    # Load history for regression comparison
    history = load_history(output_path)
    previous_record = history[-1] if history else None

    # Run benchmarks
    results = []
    ok_count = 0
    fail_count = 0
    total_compile_ms = 0
    total_size_bytes = 0

    eprint()
    eprint("running benchmarks...")

    for label, fixture_path in fixtures:
        if not fixture_path.exists():
            if verbose:
                eprint(f"  SKIP (not found): {label}")
            continue

        if verbose:
            eprint(f"  {label} ...", end="", flush=True)

        metric = benchmark_fixture(bin_path, fixture_path)

        if metric is None or metric.get("status") == "build_fail":
            fail_count += 1
            if verbose:
                err = metric.get("error", "unknown") if metric else "unknown"
                eprint(f"  SKIP (build failed): {label} -- {err}")
            else:
                eprint(f"  SKIP: {label} (build failed)")
            continue

        ok_count += 1
        compile_ms = metric["compile_time_ms"]
        wasm_bytes = metric["wasm_size_bytes"]
        total_compile_ms += compile_ms
        total_size_bytes += wasm_bytes

        entry = {
            "fixture": label,
            "compile_time_ms": compile_ms,
            "wasm_size_bytes": wasm_bytes,
        }
        results.append(entry)

        if verbose:
            eprint(f" {compile_ms}ms, {wasm_bytes}b")

    # Compute aggregates
    aggregates = {
        "total_fixtures_ok": ok_count,
        "total_fixtures_skipped": fail_count,
        "avg_compile_time_ms": total_compile_ms // ok_count if ok_count else 0,
        "total_compile_time_ms": total_compile_ms,
        "compiler_throughput_fixtures_per_sec": (
            round(ok_count / (total_compile_ms / 1000), 3)
            if total_compile_ms > 0
            else 0
        ),
        "total_wasm_size_bytes": total_size_bytes,
        "max_compile_time_ms": max(r["compile_time_ms"] for r in results) if results else 0,
        "min_compile_time_ms": min(r["compile_time_ms"] for r in results) if results else 0,
        "avg_wasm_size_bytes": total_size_bytes // ok_count if ok_count else 0,
        "max_wasm_size_bytes": max(r["wasm_size_bytes"] for r in results) if results else 0,
        "min_wasm_size_bytes": min(r["wasm_size_bytes"] for r in results) if results else 0,
    }

    # Compute commit info
    commit = "unknown"
    branch = "unknown"
    try:
        commit = subprocess.run(
            ["git", "rev-parse", "--short", "HEAD"],
            capture_output=True, text=True, cwd=REPO_ROOT,
        ).stdout.strip()
    except Exception:
        pass
    try:
        branch = subprocess.run(
            ["git", "rev-parse", "--abbrev-ref", "HEAD"],
            capture_output=True, text=True, cwd=REPO_ROOT,
        ).stdout.strip()
    except Exception:
        pass

    # Build record
    timestamp = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    record = {
        "timestamp": timestamp,
        "commit": commit,
        "branch": branch,
        "fixtures_sampled": len(fixtures),
        "fixtures_ok": ok_count,
        "fixtures_skipped": fail_count,
        "benchmark_config": config,
        "aggregates": aggregates,
        "fixtures": results,
    }

    # Regression detection against previous record
    regressions = []
    compared_previous = comparable_previous_record(previous_record, config)
    if previous_record and not compared_previous:
        eprint(
            "note: skipping regression comparison; previous benchmark config "
            "does not match current config"
        )
    if compared_previous:
        prev_agg = previous_record.get("aggregates", {})
        check_regression(
            "avg_compile_time", aggregates["avg_compile_time_ms"],
            prev_agg.get("avg_compile_time_ms", 0),
            "compile_time_ms", threshold_time_pct, regressions,
            min_delta=threshold_time_min_delta_ms,
        )
        check_regression(
            "avg_wasm_size", aggregates["avg_wasm_size_bytes"],
            prev_agg.get("avg_wasm_size_bytes", 0),
            "wasm_size_bytes", threshold_size_pct, regressions,
        )
        # Per-fixture regression check (match by fixture label)
        prev_fixtures = {f["fixture"]: f for f in previous_record.get("fixtures", [])}
        for r in results:
            prev = prev_fixtures.get(r["fixture"])
            if prev is None:
                continue
            check_regression(
                r["fixture"], r["compile_time_ms"],
                prev.get("compile_time_ms", 0),
                "compile_time_ms", threshold_time_pct, regressions,
                min_delta=threshold_time_min_delta_ms,
            )
            check_regression(
                r["fixture"], r["wasm_size_bytes"],
                prev.get("wasm_size_bytes", 0),
                "wasm_size_bytes", threshold_size_pct, regressions,
            )

    record["regressions"] = regressions
    record["perf_regression"] = bool(regressions)
    record["compared_previous"] = compared_previous

    # Save to output file
    output_path.parent.mkdir(parents=True, exist_ok=True)
    history.append(record)
    with open(output_path, "w") as f:
        json.dump(history, f, indent=2)

    # Print report to stderr
    eprint()
    eprint("=" * 60)
    eprint("BENCHMARK REPORT")
    eprint("=" * 60)
    eprint(f"  Timestamp:         {timestamp}")
    eprint(f"  Commit:            {commit}")
    eprint(f"  Branch:            {branch}")
    eprint(f"  Fixtures ok:       {ok_count}")
    eprint(f"  Fixtures skipped:  {fail_count}")
    eprint()
    eprint("  --- Averages ---")
    eprint(f"  Avg compile time:  {aggregates['avg_compile_time_ms']} ms")
    eprint(
        "  Compiler throughput:"
        f"  {aggregates['compiler_throughput_fixtures_per_sec']} fixtures/sec"
    )
    eprint(f"  Max compile time:  {aggregates['max_compile_time_ms']} ms")
    eprint(f"  Min compile time:  {aggregates['min_compile_time_ms']} ms")
    eprint(f"  Avg wasm size:     {aggregates['avg_wasm_size_bytes']} bytes")
    eprint(f"  Max wasm size:     {aggregates['max_wasm_size_bytes']} bytes")
    eprint(f"  Min wasm size:     {aggregates['min_wasm_size_bytes']} bytes")

    if regressions:
        eprint()
        eprint("  --- REGRESSION ALERTS ---")
        for rg in regressions:
            eprint(
                f"  {rg['fixture']} ({rg['metric']}): "
                f"{rg['current']} vs {rg['previous']} "
                f"(+{rg['pct_increase']}%, threshold={rg['threshold_pct']}%, "
                f"min_delta={rg['min_delta']})"
            )

    eprint()
    eprint(f"  Results saved to: {output_path}")

    # Print summary JSON to stdout for programmatic consumption
    summary = {
        "timestamp": timestamp,
        "commit": commit,
        "branch": branch,
        "fixtures_ok": ok_count,
        "fixtures_skipped": fail_count,
        "benchmark_config": config,
        "avg_compile_time_ms": aggregates["avg_compile_time_ms"],
        "compiler_throughput_fixtures_per_sec": aggregates[
            "compiler_throughput_fixtures_per_sec"
        ],
        "avg_wasm_size_bytes": aggregates["avg_wasm_size_bytes"],
        "total_wasm_size_bytes": aggregates["total_wasm_size_bytes"],
        "regression_count": len(regressions),
        "perf_regression": bool(regressions),
        "compared_previous": compared_previous,
    }
    if json_mode:
        print(json.dumps(summary, sort_keys=True))
    else:
        print(json.dumps(summary))

    eprint("benchmark-tracker: OK")
    sys.exit(0)


if __name__ == "__main__":
    main()
