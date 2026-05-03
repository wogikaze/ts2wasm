#!/usr/bin/env python3
"""Generate static web UI JSON data from repository artifacts.

The generator prefers existing coverage artifacts in artifacts/coverage/results.
Per-case JSONL test records are used by default when present so web-ui reloads
do not silently fall back to aggregate rows like semantic-pass/build-pass.
"""

import argparse
import json
import os
from datetime import datetime, timezone
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
DEFAULT_COVERAGE_DIR = REPO_ROOT / "artifacts" / "coverage" / "results"
DEFAULT_SITE_DOCS_ROOT = REPO_ROOT / "site" / "docs"
DEFAULT_HISTORY_FILE = REPO_ROOT / "artifacts" / "coverage" / "history" / "runs.jsonl"


def resolve_output_dir() -> Path:
    """Resolve output directory for dashboard JSON artifacts.

    Priority:
    1. TS2WASM_WEB_UI_DATA_DIR if set
    2. TS2WASM_DOCS_REPO_PATH + coverage/web-ui/public/data
    3. site/docs/coverage/web-ui/public/data (default)
    """
    explicit = os.environ.get("TS2WASM_WEB_UI_DATA_DIR")
    if explicit:
        explicit_path = Path(explicit)
        return explicit_path if explicit_path.is_absolute() else REPO_ROOT / explicit_path

    docs_repo = Path(os.environ.get("TS2WASM_DOCS_REPO_PATH", str(DEFAULT_SITE_DOCS_ROOT)))
    if not docs_repo.is_absolute():
        docs_repo = REPO_ROOT / docs_repo
    return docs_repo / "coverage" / "web-ui" / "public" / "data"

STATUS_MAP = {
    "pass": "pass",
    "build_pass": "pass",
    "semantic_pass": "pass",
    "fail": "fail",
    "failed": "fail",
    "mismatch": "mismatch",
    "runtime_error": "runtime_error",
    "blocked": "blocked",
    "error": "error",
    "unsupported": "unsupported",
    "skip-with-reason": "skip",
    "skip_with_reason": "skip",
    "skipped": "skip",
}

FEATURE_PRIORITY = {
    "name-resolution": "p1",
    "function-resolution": "p1",
    "parser-syntax": "p1",
    "array-builtin": "p1",
    "object-builtin": "p1",
    "string-builtin": "p1",
    "builtin-api": "p1",
    "function": "p1",
    "class": "p1",
    "import-export": "p1",
    "module-resolution": "p1",
    "destructuring": "p1",
    "switch": "p1",
    "object-literal": "p1",
    "arguments-object": "p1",
    "date": "p2",
    "regexp-literal": "p2",
    "type-annotation": "p2",
    "type-system": "p2",
    "type-alias": "p2",
    "type-assertion": "p2",
    "async": "p2",
    "async-iteration": "p2",
    "decorator": "p3",
    "declaration-emit": "p3",
    "module-system-amd": "p3",
    "ambient-declaration": "p3",
    "legacy-global-builtin": "p3",
    "annexb-ishtmldda": "p3",
    "eval": "p3",
    "jsx": "p3",
}


def format_utc(timestamp):
    return timestamp.replace(microsecond=0).isoformat().replace("+00:00", "Z")


def generated_at_iso(artifacts):
    source_date_epoch = os.environ.get("SOURCE_DATE_EPOCH")
    if source_date_epoch:
        timestamp = datetime.fromtimestamp(int(source_date_epoch), timezone.utc)
        return format_utc(timestamp)

    latest_mtime = max(item["_source_mtime_epoch"] for item in artifacts)
    return format_utc(datetime.fromtimestamp(latest_mtime, timezone.utc))


def load_coverage_artifacts(coverage_dir):
    artifacts = []
    if not coverage_dir.is_dir():
        return artifacts

    for path in sorted(coverage_dir.glob("*.json")):
        if path.name.endswith("-summary.json"):
            continue
        with path.open("r", encoding="utf-8") as handle:
            data = json.load(handle)
        try:
            data["_source_path"] = path.relative_to(REPO_ROOT).as_posix()
        except ValueError:
            data["_source_path"] = path.as_posix()
        data["_source_mtime_epoch"] = path.stat().st_mtime
        data["_source_mtime"] = format_utc(
            datetime.fromtimestamp(data["_source_mtime_epoch"], timezone.utc)
        )
        artifacts.append(data)
    return artifacts


def normalize_status(status):
    return STATUS_MAP.get(str(status or "").strip(), "error")


def count_summary(tests):
    summary = {"total": 0, "passed": 0, "failed": 0, "skipped": 0}
    for test in tests:
        weight = int(test.get("count", 1))
        summary["total"] += weight
        status = test["status"]
        if status == "pass":
            summary["passed"] += weight
        elif status in ("fail", "error"):
            summary["failed"] += weight
        else:
            summary["skipped"] += weight
    return summary


def empty_test_summary():
    return {
        "passed": 0,
        "mismatch": 0,
        "runtime_error": 0,
        "build_error": 0,
        "unsupported": 0,
        "blocked": 0,
    }


def record_weight(record):
    return int(record.get("count", 1) or 1)


def add_record_to_test_summary(summary, record):
    weight = record_weight(record)
    status = record["status"]
    if status == "pass":
        summary["passed"] += weight
    elif status == "mismatch":
        summary["mismatch"] += weight
    elif status == "runtime_error":
        summary["runtime_error"] += weight
    elif status == "fail":
        summary["build_error"] += weight
    elif status == "unsupported":
        summary["unsupported"] += weight
    elif status == "blocked":
        summary["blocked"] += weight


def summarize_test_records(records):
    summary = empty_test_summary()
    for record in records:
        add_record_to_test_summary(summary, record)
    return summary


def summary_total(summary):
    return sum(int(summary.get(key, 0) or 0) for key in empty_test_summary())


def aggregate_test_records(artifacts):
    tests = []
    for artifact in artifacts:
        suite = artifact.get("suite") or artifact.get("suite_name") or "unknown"
        target = "reference-coverage"
        if int(artifact.get("semantic_pass", 0) or 0) > 0:
            pass_bucket = ("semantic-pass", "semantic_pass", "pass", "Node/iwasm semantic match")
        elif int(artifact.get("passed", 0) or 0) > 0:
            pass_bucket = ("passed", "passed", "pass", "passed")
        else:
            pass_bucket = ("build-pass", "build_pass", "pass", "wasm build success")
        fail_bucket = ("fail", "fail", "fail", "compiler failure")
        if int(artifact.get("fail", 0) or 0) == 0 and int(artifact.get("failed", 0) or 0) > 0:
            fail_bucket = ("failed", "failed", "fail", "failed")
        buckets = [
            pass_bucket,
            ("mismatch", "mismatch", "mismatch", "Node/iwasm semantic mismatch"),
            ("runtime-error", "runtime_error", "runtime_error", "runtime failure"),
            ("unsupported", "unsupported", "unsupported", "unsupported by current compiler slice"),
            ("blocked", "blocked", "blocked", "external/runtime/toolchain blocker"),
            fail_bucket,
            ("skip-with-reason", "skip_with_reason", "skip", "explicitly skipped with reason"),
        ]
        for case_name, key, status, reason in buckets:
            count = int(artifact.get(key, 0) or 0)
            if count == 0:
                continue
            record = {
                "id": f"{suite}:{case_name}",
                "suite": suite,
                "case": case_name,
                "name": case_name,
                "target": target,
                "status": status,
                "count": count,
                "reason": reason,
            }
            if status in ("fail", "error"):
                record["error"] = reason
            tests.append(record)
    return tests


def normalize_detail_text(value):
    if value is None:
        return None
    text = str(value)
    if "\\n" in text and "\n" not in text:
        text = text.replace("\\r\\n", "\n").replace("\\n", "\n").replace("\\t", "\t")
    return text


def default_jsonl_test_records(coverage_dir):
    if not coverage_dir.is_dir():
        return []
    return sorted(coverage_dir.glob("*-results.jsonl"))


def load_jsonl_test_records(paths, start_id):
    tests = []
    row_id = start_id
    for path in paths:
        with path.open("r", encoding="utf-8") as handle:
            for line_number, raw_line in enumerate(handle, start=1):
                raw_line = raw_line.strip()
                if not raw_line:
                    continue
                data = json.loads(raw_line)
                suite = data.get("suite") or path.name.removesuffix("-results.jsonl") or "unknown"
                case_name = data.get("case") or data.get("name") or data.get("path") or f"line-{line_number}"
                clean_name = case_name.split("/")[-1] if "/" in case_name else case_name
                status = normalize_status(data.get("status"))
                reason = normalize_detail_text(data.get("reason"))
                stderr = normalize_detail_text(data.get("stderr"))
                actual = normalize_detail_text(data.get("actual"))
                error = normalize_detail_text(data.get("error") or stderr or (actual if status in ("fail", "error") else None))

                record = {
                    "id": str(row_id),
                    "suite": suite,
                    "case": case_name,
                    "name": clean_name,
                    "target": data.get("target") or "wasm",
                    "status": status,
                }
                duration = data.get("duration") or data.get("duration_ms")
                if isinstance(duration, (int, float)):
                    record["duration"] = duration
                if reason:
                    record["reason"] = reason
                if error:
                    record["error"] = error

                for detail_key in ("expected", "actual", "stderr", "source_code", "error_line"):
                    value = data.get(detail_key)
                    if value is not None:
                        record[detail_key] = normalize_detail_text(value) if isinstance(value, str) else value

                tests.append(record)
                row_id += 1
    return tests


def build_test_results(artifacts, jsonl_paths, generated_at, row_limit=1000):
    jsonl_tests = load_jsonl_test_records(jsonl_paths, 1) if jsonl_paths else []
    jsonl_suites = {record["suite"] for record in jsonl_tests}
    aggregate_tests = aggregate_test_records(
        [artifact for artifact in artifacts if (artifact.get("suite") or artifact.get("suite_name") or "unknown") not in jsonl_suites]
    )
    all_tests = jsonl_tests + aggregate_tests

    if jsonl_tests and aggregate_tests:
        record_mode = "mixed"
    elif jsonl_tests:
        record_mode = "jsonl"
    else:
        record_mode = "aggregate"

    tests_by_suite = {}
    for record in all_tests:
        tests_by_suite.setdefault(record["suite"], []).append(record)

    shown_tests = []
    shown_by_suite = {}
    total_by_suite = {}
    summary_by_suite = {}
    for suite in sorted(tests_by_suite):
        records = tests_by_suite[suite]
        shown = records[:row_limit]
        shown_tests.extend(shown)
        total_by_suite[suite] = sum(record_weight(record) for record in records)
        shown_by_suite[suite] = sum(record_weight(record) for record in shown)
        summary_by_suite[suite] = summarize_test_records(records)

    global_summary = summarize_test_records(all_tests)
    total_records = sum(record_weight(record) for record in all_tests)
    shown_records = sum(record_weight(record) for record in shown_tests)

    return {
        "tests": shown_tests,
        "summary": global_summary,
        "metadata": {
            "schema_version": 2,
            "generated_at": generated_at,
            "generator": "scripts/gen/web-ui-data.py",
            "record_mode": record_mode,
            "total_records": total_records,
            "shown_records": shown_records,
            "row_limit": row_limit,
            "row_limit_per_suite": row_limit,
            "truncated": shown_records < total_records,
            "total_by_suite": total_by_suite,
            "shown_by_suite": shown_by_suite,
            "summary_by_suite": summary_by_suite,
            "sources": [item["_source_path"] for item in artifacts]
            + [path.relative_to(REPO_ROOT).as_posix() for path in jsonl_paths],
        },
    }

def priority_for_feature(feature):
    return FEATURE_PRIORITY.get(feature, "p2")


def normalized_suite_metrics(item):
    suite = item.get("suite") or item.get("suite_name") or "unknown"
    total = int(item.get("total", 0) or 0)
    denominator = int(item.get("denominator", 0) or total or 0)
    executed = int(item.get("executed", 0) or total or 0)
    build_pass = int(item.get("build_pass", item.get("passed", 0)) or 0)
    semantic_pass = int(item.get("semantic_pass", item.get("passed", 0)) or 0)
    fail = int(item.get("fail", item.get("failed", 0)) or 0)
    unsupported = int(item.get("unsupported", 0) or 0)
    blocked = int(item.get("blocked", 0) or 0)
    skip_with_reason = int(item.get("skip_with_reason", item.get("skipped", 0)) or 0)
    return {
        "suite": suite,
        "denominator": denominator,
        "executed": executed,
        "build_pass": build_pass,
        "semantic_pass": semantic_pass,
        "unsupported": unsupported,
        "blocked": blocked,
        "fail": fail,
        "skip_with_reason": skip_with_reason,
        "source": item["_source_path"],
    }


def build_coverage(artifacts):
    suites = [normalized_suite_metrics(item) for item in artifacts]
    total = sum(item["denominator"] for item in suites)
    implemented = sum(item["build_pass"] for item in suites)
    unsupported = sum(item["unsupported"] for item in suites)
    failed = sum(item["fail"] for item in suites)
    blocked = sum(item["blocked"] for item in suites)
    executed = sum(item["executed"] for item in suites)
    future = max(total - executed, 0)

    by_priority = {"p0": failed + blocked, "p1": 0, "p2": 0, "p3": 0, "future": future}
    for artifact in artifacts:
        unsupported_features = artifact.get("unsupported_features") or {}
        if unsupported_features:
            accounted = 0
            for feature, count in unsupported_features.items():
                priority = priority_for_feature(feature)
                feature_count = int(count or 0)
                by_priority[priority] += feature_count
                accounted += feature_count
            remaining = max(int(artifact.get("unsupported", 0) or 0) - accounted, 0)
            by_priority["p2"] += remaining
            continue
        by_priority["p2"] += int(artifact.get("unsupported", 0) or 0)

    unimplemented = unsupported + failed + blocked
    if total > 0:
        unimplemented = min(unimplemented, max(total - implemented - future, 0))

    return {
        "total": total,
        "implemented": implemented,
        "unimplemented": unimplemented,
        "future": future,
        "byPriority": by_priority,
        "suites": suites,
    }


def jsonl_durations_by_suite(coverage_dir):
    durations = {}
    for path in default_jsonl_test_records(coverage_dir):
        total_duration_ms = 0.0
        timed_records = 0
        with path.open("r", encoding="utf-8") as handle:
            for raw_line in handle:
                raw_line = raw_line.strip()
                if not raw_line:
                    continue
                data = json.loads(raw_line)
                duration = data.get("duration_ms", data.get("duration"))
                if isinstance(duration, (int, float)):
                    total_duration_ms += float(duration)
                    timed_records += 1
        if timed_records:
            durations[path.name.removesuffix("-results.jsonl")] = total_duration_ms
    return durations


def history_snapshot(item, coverage_dir):
    metrics = normalized_suite_metrics(item)
    durations = jsonl_durations_by_suite(coverage_dir)
    suite = metrics["suite"]
    duration_ms = item.get("duration_ms")
    if not isinstance(duration_ms, (int, float)):
        duration_ms = durations.get(suite)
    executed = metrics["executed"]
    return {
        "run_id": f"{suite}-{executed}",
        "suite": suite,
        "executed": executed,
        "denominator": metrics["denominator"],
        "timestamp": item["_source_mtime"],
        "passed": metrics["build_pass"],
        "failed": metrics["fail"] + metrics["blocked"],
        "skipped": metrics["unsupported"] + metrics["skip_with_reason"],
        "duration_ms": duration_ms,
    }


def history_key(row):
    return "|".join([
        str(row.get("suite", "unknown")),
        str(row.get("timestamp", "")),
        str(row.get("executed", "")),
        str(row.get("denominator", "")),
    ])


def load_persisted_history(history_file):
    rows = []
    if not history_file.is_file():
        return rows
    with history_file.open("r", encoding="utf-8") as handle:
        for raw_line in handle:
            raw_line = raw_line.strip()
            if not raw_line:
                continue
            rows.append(json.loads(raw_line))
    return rows


def append_history_snapshots(artifacts, coverage_dir, history_file=DEFAULT_HISTORY_FILE):
    existing = load_persisted_history(history_file)
    seen = {history_key(row) for row in existing}
    new_rows = []
    for item in artifacts:
        row = history_snapshot(item, coverage_dir)
        key = history_key(row)
        if key not in seen:
            new_rows.append(row)
            seen.add(key)
    if not new_rows:
        return
    history_file.parent.mkdir(parents=True, exist_ok=True)
    with history_file.open("a", encoding="utf-8") as handle:
        for row in new_rows:
            handle.write(json.dumps(row, sort_keys=True))
            handle.write("\n")


def build_history(artifacts, coverage_dir):
    rows_by_key = {history_key(row): row for row in load_persisted_history(DEFAULT_HISTORY_FILE)}
    for item in artifacts:
        row = history_snapshot(item, coverage_dir)
        rows_by_key[history_key(row)] = row
    return sorted(rows_by_key.values(), key=lambda row: (row["timestamp"], row["run_id"]))


def build_metadata(artifacts, jsonl_paths, generated_at, out_dir):
    return {
        "schema_version": 1,
        "generated_at": generated_at,
        "generator": "scripts/gen/web-ui-data.py",
        "sources": [item["_source_path"] for item in artifacts]
        + [path.relative_to(REPO_ROOT).as_posix() for path in jsonl_paths],
        "output_files": [
            str((out_dir / "test-results.json").as_posix()),
            str((out_dir / "coverage.json").as_posix()),
            str((out_dir / "history.json").as_posix()),
            str((out_dir / "metadata.json").as_posix()),
        ],
        "notes": [
            "Coverage totals are derived from artifacts/coverage/results/*.json.",
            "Per-case rows use artifacts/coverage/results/*-results.jsonl by default when available.",
        ],
    }


def write_json(path, data):
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8") as handle:
        json.dump(data, handle, indent=2, sort_keys=True)
        handle.write("\n")


def display_path(path):
    try:
        return path.relative_to(REPO_ROOT).as_posix()
    except ValueError:
        return path.as_posix()


def parse_args():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--coverage-dir", type=Path, default=DEFAULT_COVERAGE_DIR)
    parser.add_argument(
        "--out-dir",
        type=Path,
        default=None,
        help=(
            "Output directory for JSON artifacts. If omitted, writes to "
            "the docs web-ui data directory."
        ),
    )
    parser.add_argument(
        "--test-jsonl",
        type=Path,
        action="append",
        default=[],
        help="Optional JSONL TestRecord file to include as per-case test rows.",
    )
    return parser.parse_args()


def main():
    args = parse_args()
    coverage_dir = args.coverage_dir if args.coverage_dir.is_absolute() else REPO_ROOT / args.coverage_dir
    if args.out_dir is None:
        out_dirs = [resolve_output_dir()]
    else:
        out_dir = args.out_dir if args.out_dir.is_absolute() else REPO_ROOT / args.out_dir
        out_dirs = [out_dir]
    explicit_jsonl_paths = [
        path if path.is_absolute() else REPO_ROOT / path
        for path in args.test_jsonl
    ]
    jsonl_paths = explicit_jsonl_paths or default_jsonl_test_records(coverage_dir)

    artifacts = load_coverage_artifacts(coverage_dir)
    if not artifacts:
        raise SystemExit(f"no coverage artifacts found in {coverage_dir}")
    for path in jsonl_paths:
        if not path.is_file():
            raise SystemExit(f"--test-jsonl not found: {path}")

    generated_at = generated_at_iso(artifacts)
    append_history_snapshots(artifacts, coverage_dir)
    outputs = {
        "test-results.json": build_test_results(artifacts, jsonl_paths, generated_at),
        "coverage.json": build_coverage(artifacts),
        "history.json": build_history(artifacts, coverage_dir),
    }
    for filename, data in outputs.items():
        for out_dir in out_dirs:
            write_json(out_dir / filename, data)
    for out_dir in out_dirs:
        write_json(out_dir / "metadata.json", build_metadata(artifacts, jsonl_paths, generated_at, out_dir))

    print("generated 4 files under:")
    for out_dir in out_dirs:
        print(f"  - {display_path(out_dir)}")


if __name__ == "__main__":
    main()
