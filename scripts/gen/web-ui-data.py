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
DEFAULT_OUT_DIR = REPO_ROOT / "web-ui" / "public" / "data"

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
        with path.open("r", encoding="utf-8") as handle:
            data = json.load(handle)
        data["_source_path"] = path.relative_to(REPO_ROOT).as_posix()
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


def aggregate_test_records(artifacts):
    tests = []
    row_id = 1
    for artifact in artifacts:
        suite = artifact.get("suite") or artifact.get("suite_name") or "unknown"
        target = "reference-coverage"
        buckets = [
            ("semantic-pass", "semantic_pass", "pass", "Node/iwasm semantic match"),
            ("build-pass", "build_pass", "pass", "wasm build success"),
            ("unsupported", "unsupported", "unsupported", "unsupported by current compiler slice"),
            ("blocked", "blocked", "blocked", "external/runtime/toolchain blocker"),
            ("fail", "fail", "fail", "compiler failure"),
            ("skip-with-reason", "skip_with_reason", "skip", "explicitly skipped with reason"),
        ]
        for case_name, key, status, reason in buckets:
            count = int(artifact.get(key, 0) or 0)
            if count == 0:
                continue
            record = {
                "id": str(row_id),
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
            row_id += 1
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
    if jsonl_paths:
        all_tests = load_jsonl_test_records(jsonl_paths, 1)
        record_mode = "jsonl"
    else:
        all_tests = aggregate_test_records(artifacts)
        record_mode = "aggregate"

    shown_tests = all_tests[:row_limit]

    # Count by status
    passed = sum(1 for t in all_tests if t["status"] == "pass")
    mismatch = sum(1 for t in all_tests if t["status"] == "mismatch")
    runtime_error = sum(1 for t in all_tests if t["status"] == "runtime_error")
    build_error = sum(1 for t in all_tests if t["status"] == "fail")
    unsupported = sum(1 for t in all_tests if t["status"] == "unsupported")
    blocked = sum(1 for t in all_tests if t["status"] == "blocked")

    return {
        "tests": shown_tests,
        "summary": {
            "passed": passed,
            "mismatch": mismatch,
            "runtime_error": runtime_error,
            "build_error": build_error,
            "unsupported": unsupported,
            "blocked": blocked,
        },
        "metadata": {
            "schema_version": 2,
            "generated_at": generated_at,
            "generator": "scripts/gen/web-ui-data.py",
            "record_mode": record_mode,
            "total_records": len(all_tests),
            "shown_records": len(shown_tests),
            "row_limit": row_limit,
            "truncated": len(all_tests) > len(shown_tests),
            "sources": [item["_source_path"] for item in artifacts]
            + [path.relative_to(REPO_ROOT).as_posix() for path in jsonl_paths],
        },
    }

def priority_for_feature(feature):
    return FEATURE_PRIORITY.get(feature, "p2")


def build_coverage(artifacts):
    total = sum(int(item.get("denominator", 0) or 0) for item in artifacts)
    implemented = sum(int(item.get("build_pass", 0) or 0) for item in artifacts)
    unsupported = sum(int(item.get("unsupported", 0) or 0) for item in artifacts)
    failed = sum(int(item.get("fail", 0) or 0) for item in artifacts)
    blocked = sum(int(item.get("blocked", 0) or 0) for item in artifacts)
    executed = sum(int(item.get("executed", 0) or 0) for item in artifacts)
    future = max(total - executed, 0)

    by_priority = {"p0": failed + blocked, "p1": 0, "p2": 0, "p3": 0, "future": future}
    for artifact in artifacts:
        for feature, count in (artifact.get("unsupported_features") or {}).items():
            priority = priority_for_feature(feature)
            by_priority[priority] += int(count or 0)

    return {
        "total": total,
        "implemented": implemented,
        "unimplemented": unsupported + failed + blocked,
        "future": future,
        "byPriority": by_priority,
        "suites": [
            {
                "suite": item.get("suite") or item.get("suite_name") or "unknown",
                "denominator": int(item.get("denominator", 0) or 0),
                "executed": int(item.get("executed", 0) or 0),
                "build_pass": int(item.get("build_pass", 0) or 0),
                "semantic_pass": int(item.get("semantic_pass", 0) or 0),
                "unsupported": int(item.get("unsupported", 0) or 0),
                "blocked": int(item.get("blocked", 0) or 0),
                "fail": int(item.get("fail", 0) or 0),
                "source": item["_source_path"],
            }
            for item in artifacts
        ],
    }


def build_history(artifacts):
    history = []
    for item in artifacts:
        suite = item.get("suite") or item.get("suite_name") or "unknown"
        history.append({
            "run_id": f"{suite}-{item.get('executed', 0)}",
            "timestamp": item["_source_mtime"],
            "passed": int(item.get("build_pass", 0) or 0),
            "failed": int(item.get("fail", 0) or 0) + int(item.get("blocked", 0) or 0),
            "skipped": int(item.get("unsupported", 0) or 0) + int(item.get("skip_with_reason", 0) or 0),
            "compile_time": 0,
            "runtime": 0,
        })
    return history


def build_metadata(artifacts, jsonl_paths, generated_at):
    return {
        "schema_version": 1,
        "generated_at": generated_at,
        "generator": "scripts/gen/web-ui-data.py",
        "sources": [item["_source_path"] for item in artifacts]
        + [path.relative_to(REPO_ROOT).as_posix() for path in jsonl_paths],
        "output_files": [
            "web-ui/public/data/test-results.json",
            "web-ui/public/data/coverage.json",
            "web-ui/public/data/history.json",
            "web-ui/public/data/metadata.json",
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


def parse_args():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--coverage-dir", type=Path, default=DEFAULT_COVERAGE_DIR)
    parser.add_argument("--out-dir", type=Path, default=DEFAULT_OUT_DIR)
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
    out_dir = args.out_dir if args.out_dir.is_absolute() else REPO_ROOT / args.out_dir
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
    outputs = {
        "test-results.json": build_test_results(artifacts, jsonl_paths, generated_at),
        "coverage.json": build_coverage(artifacts),
        "history.json": build_history(artifacts),
        "metadata.json": build_metadata(artifacts, jsonl_paths, generated_at),
    }
    for filename, data in outputs.items():
        write_json(out_dir / filename, data)

    print(f"generated {len(outputs)} files under {out_dir.relative_to(REPO_ROOT)}")


if __name__ == "__main__":
    main()
