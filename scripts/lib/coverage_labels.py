"""Feature labels and owner map for coverage triage.

Provides a single source of truth for feature labels used by the coverage
runner, triage reporter, and dashboard. Labels map to ownership areas and
are stable across runs for deterministic triage output.

Usage:
    from coverage_labels import FEATURE_LABELS, LABEL_OWNERS, classify_diagnostic
    label = classify_diagnostic("UnsupportedSyntax", "stderr content", "/path/to/file.ts")
    owner = LABEL_OWNERS.get(label, "reference")
"""

from __future__ import annotations

import sys
from typing import Any

# ---------------------------------------------------------------------------
# Feature label definitions
# ---------------------------------------------------------------------------

FEATURE_LABELS: dict[str, dict[str, Any]] = {
    "parser-syntax": {
        "description": "Parser does not accept this syntax construct",
        "owner": "frontend",
        "diag_codes": ["UnsupportedSyntax"],
        "legacy_labels": ["feature-unsupported"],
    },
    "typescript-erase": {
        "description": "TypeScript-specific syntax that erases to nothing",
        "owner": "frontend",
        "diag_codes": ["UnsupportedTypeScriptSyntax"],
        "legacy_labels": ["parser-syntax"],
    },
    "unsupported-builtin": {
        "description": "Unsupported built-in API or global object",
        "owner": "runtime",
        "diag_codes": ["UnsupportedBuiltin"],
        "legacy_labels": ["builtin-api"],
    },
    "object-kernel": {
        "description": "Object kernel operations (get/set/delete property, prototype)",
        "owner": "runtime",
        "diag_codes": [],
        "legacy_labels": ["object-builtin"],
    },
    "array-exotic": {
        "description": "Array exotic object behaviors (length, prototype chain)",
        "owner": "runtime",
        "diag_codes": [],
        "legacy_labels": ["array-builtin"],
    },
    "gc-runtime": {
        "description": "GC-related runtime gaps (typed arrays, WeakMap, finalization)",
        "owner": "runtime",
        "diag_codes": [],
        "legacy_labels": [],
    },
    "node-host": {
        "description": "Node.js host environment API (fs, process, crypto)",
        "owner": "host",
        "diag_codes": [],
        "legacy_labels": ["host-env"],
    },
    "test262-harness": {
        "description": "test262 harness feature or include not supported",
        "owner": "harness",
        "diag_codes": ["UnsupportedTest262Metadata"],
        "legacy_labels": ["test262-metadata"],
    },
    "unknown-unsupported": {
        "description": "Unsupported outcome without a visible feature label",
        "owner": "reference",
        "diag_codes": [],
        "legacy_labels": [],
    },
    "name-resolution": {
        "description": "Name or function resolution failure",
        "owner": "ir",
        "diag_codes": ["UnresolvedName", "UnresolvedFunction"],
        "legacy_labels": ["feature-resolution"],
    },
    "builtin-date": {
        "description": "Date built-in not fully supported",
        "owner": "runtime",
        "diag_codes": ["UnsupportedDate"],
        "legacy_labels": ["date"],
    },
    "regexp-literal": {
        "description": "RegExp literal not fully supported",
        "owner": "runtime",
        "diag_codes": ["UnsupportedRegExp"],
        "legacy_labels": ["regexp-literal"],
    },
    "module-resolution": {
        "description": "Module import/export resolution failure",
        "owner": "ir",
        "diag_codes": ["UnsupportedModule"],
        "legacy_labels": ["import-export"],
    },
    "eval-unsupported": {
        "description": "eval/Function constructor not supported",
        "owner": "runtime",
        "diag_codes": ["UnsupportedEval"],
        "legacy_labels": ["eval"],
    },
    "runtime-subset": {
        "description": "Syntax and lowering succeeded but runtime/link-plan support absent",
        "owner": "runtime",
        "diag_codes": ["UnsupportedRuntimeSubset"],
        "legacy_labels": ["runtime-subset"],
    },
    "compiler-invariant": {
        "description": "Compiler internal invariant violation",
        "owner": "reference",
        "diag_codes": ["InvariantViolation"],
        "legacy_labels": ["compiler-invariant"],
    },
    "negative-compile": {
        "description": "Negative compile test verification outcome",
        "owner": "reference",
        "diag_codes": ["NegativeCompileUnverified", "NegativeCompileMismatch",
                       "NegativeRuntimeUnverified", "ExpectedNegativeSyntax"],
        "legacy_labels": ["negative-compile-unverified", "negative-compile-mismatch",
                         "negative-runtime-unverified", "negative-parse-syntaxerror"],
    },
    "build-pass": {
        "description": "Build succeeded without semantic check",
        "owner": "reference",
        "diag_codes": [],
        "legacy_labels": [],
    },
}

# ---------------------------------------------------------------------------
# Owner map
# ---------------------------------------------------------------------------

LABEL_OWNERS: dict[str, str] = {
    "parser-syntax": "frontend",
    "typescript-erase": "frontend",
    "unsupported-builtin": "runtime",
    "object-kernel": "runtime",
    "array-exotic": "runtime",
    "gc-runtime": "runtime",
    "node-host": "host",
    "test262-harness": "harness",
    "unknown-unsupported": "reference",
    "name-resolution": "ir",
    "builtin-date": "runtime",
    "regexp-literal": "runtime",
    "module-resolution": "ir",
    "eval-unsupported": "runtime",
    "runtime-subset": "runtime",
    "compiler-invariant": "reference",
    "negative-compile": "reference",
    "build-pass": "reference",
}

# ---------------------------------------------------------------------------
# Diagnostic to label mapping
# ---------------------------------------------------------------------------

_DIAG_CODE_TO_LABEL: dict[str, str] = {}
for label, info in FEATURE_LABELS.items():
    for code in info.get("diag_codes", []):
        _DIAG_CODE_TO_LABEL[code] = label

# Legacy label map for migration (old labels -> new labels)
_LEGACY_LABEL_MAP: dict[str, str] = {}
for label, info in FEATURE_LABELS.items():
    for legacy in info.get("legacy_labels", []):
        _LEGACY_LABEL_MAP[legacy] = label


def classify_diagnostic(
    diag_code: str,
    stderr: str = "",
    file_path: str = "",
    phase: str | None = None,
) -> str:
    """Classify a diagnostic code into a feature label.

    Returns the canonical feature label, or ``"unknown-unsupported"`` if
    no label matches. Phase-aware classification is applied for parser
    diagnostics when ``phase`` is provided.
    """
    # Direct diagnostic code lookup
    if diag_code in _DIAG_CODE_TO_LABEL:
        return _DIAG_CODE_TO_LABEL[diag_code]

    # Legacy label support (for old-format diagnostic references)
    label = _classify_legacy(diag_code, stderr, file_path)
    if label:
        return label

    # Phase-aware parser classification
    if diag_code in ("UnsupportedSyntax", "SyntaxError") and phase is not None:
        parser_phases = {"lexer", "parser", "ast-validator"}
        if phase in parser_phases:
            return "parser-syntax"

    return "unknown-unsupported"


def _classify_legacy(
    diag_code: str,
    stderr: str,
    file_path: str,
) -> str | None:
    """Legacy classification for old diagnostic codes."""
    code_label_map = {
        "ExpectedNegativeSyntax": "negative-compile",
        "SyntaxError": "parser-syntax",
        "TypeError": "unknown-unsupported",
        "RuntimeError": "runtime-subset",
        "CompilationError": "unknown-unsupported",
        "BackendIo": "unknown-unsupported",
        "Test262AssertionFailure": "unknown-unsupported",
        "HarnessError": "test262-harness",
    }
    if diag_code in code_label_map:
        return code_label_map[diag_code]
    return None


def migrate_label(old_label: str) -> str:
    """Migrate a legacy label name to the current canonical label."""
    return _LEGACY_LABEL_MAP.get(old_label, old_label)


# ---------------------------------------------------------------------------
# Top failure bucketing
# ---------------------------------------------------------------------------

def build_top_failures(
    records: list[dict],
    top_n: int = 10,
) -> list[dict]:
    """Build a deterministic top-N failure bucket list from records.

    Buckets are grouped by ``(outcome, phase, diagnostic_code, feature_label,
    unresolved_symbol)`` and sorted by count then by tuple for determinism.

    Returns a list of dicts with keys:
        outcome_kind, phase, diag_code, feature, symbol, count, examples
    """
    from collections import Counter

    buckets: Counter[tuple] = Counter()
    example_map: dict[tuple, list[str]] = {}

    for rec in records:
        outcome = rec.get("outcome") or rec.get("status", "")
        code = rec.get("diagnostic_code") or ""
        label = rec.get("feature_label") or ""
        symbol = (rec.get("unresolved_symbol") or
                  _extract_unresolved_name(rec.get("stderr", "")) or "")
        phase = rec.get("phase") or ""
        key = (outcome, phase, code, label, symbol)

        buckets[key] += 1
        if key not in example_map:
            example_map[key] = []
        if len(example_map[key]) < 3:
            example_map[key].append(rec.get("case", ""))

    sorted_buckets = sorted(
        buckets.items(),
        key=lambda item: (-item[1], item[0]),
    )[:top_n]

    result = []
    for (outcome, phase, code, label, symbol), count in sorted_buckets:
        result.append({
            "outcome_kind": outcome,
            "phase": phase,
            "diag_code": code,
            "feature": label,
            "symbol": symbol,
            "count": count,
            "examples": example_map.get((outcome, phase, code, label, symbol), []),
        })

    return result


def _extract_unresolved_name(stderr: str) -> str:
    """Extract unresolved symbol name from stderr."""
    if not stderr:
        return ""
    import re
    match = re.search(r"unresolved name[`'\"]([^`'\"]+)[`'\"]", stderr, re.IGNORECASE)
    return match.group(1).strip() if match else ""


# ---------------------------------------------------------------------------
# Triage report generation
# ---------------------------------------------------------------------------

def format_triage_markdown(
    suite: str,
    top_buckets: list[dict],
) -> str:
    """Format a Markdown triage report from top failure buckets."""
    lines = [
        f"# Triage Report: {suite}",
        "",
        f"Top failure buckets: {len(top_buckets)}",
        "",
    ]
    for i, bucket in enumerate(top_buckets, 1):
        lines.extend([
            f"## {i}. {bucket.get('feature', 'unknown')}: {bucket.get('diag_code', '')}",
            "",
            f"- **Outcome**: `{bucket.get('outcome_kind', '')}`",
            f"- **Phase**: `{bucket.get('phase', '')}`",
            f"- **Count**: {bucket.get('count', 0)}",
        ])
        if bucket.get("symbol"):
            lines.append(f"- **Symbol**: `{bucket['symbol']}`")
        examples = bucket.get("examples", [])
        if examples:
            lines.append("- **Examples**:")
            for ex in examples:
                lines.append(f"  - `{ex}`")
        lines.append("")

    return "\n".join(lines)


# ---------------------------------------------------------------------------
# Self-test
# ---------------------------------------------------------------------------

def _self_test() -> int:
    """Run self-test for coverage_labels module."""
    errors: list[str] = []

    # Test 1: classify known diagnostic code
    label = classify_diagnostic("UnsupportedSyntax")
    if label != "parser-syntax":
        errors.append(f"Test 1: expected 'parser-syntax', got '{label}'")

    # Test 2: classify unknown diagnostic code
    label = classify_diagnostic("UnknownCode")
    if label != "unknown-unsupported":
        errors.append(f"Test 2: expected 'unknown-unsupported', got '{label}'")

    # Test 3: classify with phase
    label = classify_diagnostic("UnsupportedSyntax", phase="parser")
    if label != "parser-syntax":
        errors.append(f"Test 3: expected 'parser-syntax', got '{label}'")

    # Test 4: classify UnsupportedBuiltin
    label = classify_diagnostic("UnsupportedBuiltin")
    if label != "unsupported-builtin":
        errors.append(f"Test 4: expected 'unsupported-builtin', got '{label}'")

    # Test 5: classify UnresolvedName
    label = classify_diagnostic("UnresolvedName")
    if label != "name-resolution":
        errors.append(f"Test 5: expected 'name-resolution', got '{label}'")

    # Test 6: classify UnsupportedDate
    label = classify_diagnostic("UnsupportedDate")
    if label != "builtin-date":
        errors.append(f"Test 6: expected 'builtin-date', got '{label}'")

    # Test 7: build_top_failures with sample data
    records = [
        {"outcome": "unsupported", "phase": "parse", "diagnostic_code": "UnsupportedSyntax",
         "feature_label": "parser-syntax", "case": "test/foo.js", "stderr": ""},
        {"outcome": "unsupported", "phase": "parse", "diagnostic_code": "UnsupportedSyntax",
         "feature_label": "parser-syntax", "case": "test/bar.js", "stderr": ""},
        {"outcome": "unsupported", "phase": "compile", "diagnostic_code": "UnsupportedBuiltin",
         "feature_label": "unsupported-builtin", "case": "test/baz.js", "stderr": ""},
    ]
    top = build_top_failures(records, top_n=5)
    if len(top) != 2:
        errors.append(f"Test 7: expected 2 buckets, got {len(top)}")
    elif top[0]["count"] == 2 and top[0]["feature"] == "parser-syntax":
        pass  # expected
    else:
        errors.append(f"Test 7: unexpected sort order: {top}")

    # Test 8: format_triage_markdown
    md = format_triage_markdown("test262", top)
    if "# Triage Report: test262" not in md:
        errors.append("Test 8: missing report header")
    if "parser-syntax" not in md:
        errors.append("Test 8: missing parser-syntax bucket")

    # Test 9: LABEL_OWNERS consistency
    for label in FEATURE_LABELS:
        if label not in LABEL_OWNERS:
            errors.append(f"Test 9: label '{label}' missing from LABEL_OWNERS")

    # Summary
    if errors:
        print("coverage_labels self-test FAILED:", file=sys.stderr)
        for error in errors:
            print(f"  - {error}", file=sys.stderr)
        return 1

    print("coverage_labels: self-test OK (9 checks)", file=sys.stderr)
    return 0


if __name__ == "__main__":
    if "--self-test" in sys.argv:
        sys.exit(_self_test())
    print(__doc__)
