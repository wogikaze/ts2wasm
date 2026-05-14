"""Coverage Outcome Taxonomy and Schema v2.

Defines CoverageOutcome and CoveragePhase enums used by the coverage runner,
schema checker, and dashboard generators. Provides make_record() for emitting
schema v2 JSONL records that preserve legacy ``status`` while adding precise
outcome and phase fields.

Usage:
    from coverage_outcome import CoverageOutcome, CoveragePhase, make_record
    record = make_record(
        suite="test262", case="path/to/test.js",
        outcome=CoverageOutcome.SEMANTIC_PASS,
        phase=CoveragePhase.ORACLE,
    )

Self-test:
    python scripts/lib/coverage_outcome.py --self-test
"""

from __future__ import annotations

import enum
import json
import sys
from typing import Any


class CoverageOutcome(str, enum.Enum):
    """Precise outcome classification for a single test case.

    These replace ad hoc top-level ``status`` values like ``mismatch``,
    ``runtime_error``, and ``oracle_skipped`` while still mapping to a
    legacy ``status`` for downstream consumers.

    Enum values are the canonical outcome string used in schema v2 JSONL.
    """

    BUILD_PASS = "build_pass"
    """Compiler produced a wasm module; no semantic check attempted or needed."""

    SEMANTIC_PASS = "semantic_pass"
    """Wasm output matched Node.js reference output (or negative test verified)."""

    SEMANTIC_MISMATCH = "semantic_mismatch"
    """Wasm output differed from Node.js reference output."""

    RUNTIME_ERROR = "runtime_error"
    """Wasm module ran but iwasm exited with a non-zero code or trap."""

    UNSUPPORTED = "unsupported"
    """Compiler diagnostic indicated the source uses unsupported syntax."""

    BLOCKED = "blocked"
    """Runner infrastructure failure (I/O error, timeout, missing binary)."""

    INTERNAL_FAILURE = "internal_failure"
    """Compiler panicked or hit an invariant violation (``[InvariantViolation]``)."""

    VERIFIED_NEGATIVE_COMPILE = "verified_negative_compile"
    """Compile-phase negative test was verified: compiler correctly rejected input."""

    UNVERIFIED_NEGATIVE_COMPILE = "unverified_negative_compile"
    """Negative compile test rejected by compiler but error type was not verified."""

    ORACLE_SKIPPED = "oracle_skipped"
    """Node.js oracle was unavailable or skipped (e.g. timeout)."""

    SKIP_WITH_REASON = "skip_with_reason"
    """Test was skipped due to runner configuration or known blocker."""


class CoveragePhase(str, enum.Enum):
    """Pipeline phase where the outcome was determined."""

    METADATA = "metadata"
    """YAML front-matter parsing / filtering (test262 metadata check)."""

    PREPARE = "prepare"
    """Source preparation (harness injection, stubs)."""

    PARSE = "parse"
    """Lexing and parsing."""

    COMPILE = "compile"
    """IR lowering, type resolution, and wasm emission."""

    LINK = "link"
    """Module linking (import resolution, ABI validation)."""

    RUNTIME = "runtime"
    """iwasm execution of the compiled module."""

    ORACLE = "oracle"
    """Node.js reference output comparison."""

    TRIAGE = "triage"
    """Post-hoc classification / triage aggregation."""


# ---------------------------------------------------------------------------
# Legacy status mapping
# ---------------------------------------------------------------------------

_OUTCOME_TO_LEGACY_STATUS: dict[CoverageOutcome, str] = {
    CoverageOutcome.BUILD_PASS: "build_pass",
    CoverageOutcome.SEMANTIC_PASS: "pass",
    CoverageOutcome.SEMANTIC_MISMATCH: "fail",
    CoverageOutcome.RUNTIME_ERROR: "runtime_error",
    CoverageOutcome.UNSUPPORTED: "unsupported",
    CoverageOutcome.BLOCKED: "blocked",
    CoverageOutcome.INTERNAL_FAILURE: "fail",
    CoverageOutcome.VERIFIED_NEGATIVE_COMPILE: "pass",
    CoverageOutcome.UNVERIFIED_NEGATIVE_COMPILE: "unsupported",
    CoverageOutcome.ORACLE_SKIPPED: "oracle_skipped",
    CoverageOutcome.SKIP_WITH_REASON: "skip-with-reason",
}

_OUTCOME_TO_BUILD_PASS: dict[CoverageOutcome, bool] = {
    CoverageOutcome.BUILD_PASS: True,
    CoverageOutcome.SEMANTIC_PASS: True,
    CoverageOutcome.SEMANTIC_MISMATCH: True,
    CoverageOutcome.RUNTIME_ERROR: True,
    CoverageOutcome.UNSUPPORTED: False,
    CoverageOutcome.BLOCKED: False,
    CoverageOutcome.INTERNAL_FAILURE: False,
    CoverageOutcome.VERIFIED_NEGATIVE_COMPILE: True,
    CoverageOutcome.UNVERIFIED_NEGATIVE_COMPILE: True,
    CoverageOutcome.ORACLE_SKIPPED: True,
    CoverageOutcome.SKIP_WITH_REASON: False,
}

_OUTCOME_TO_SEMANTIC_CHECKED: dict[CoverageOutcome, bool] = {
    CoverageOutcome.BUILD_PASS: False,
    CoverageOutcome.SEMANTIC_PASS: True,
    CoverageOutcome.SEMANTIC_MISMATCH: True,
    CoverageOutcome.RUNTIME_ERROR: False,
    CoverageOutcome.UNSUPPORTED: False,
    CoverageOutcome.BLOCKED: False,
    CoverageOutcome.INTERNAL_FAILURE: False,
    CoverageOutcome.VERIFIED_NEGATIVE_COMPILE: False,
    CoverageOutcome.UNVERIFIED_NEGATIVE_COMPILE: False,
    CoverageOutcome.ORACLE_SKIPPED: False,
    CoverageOutcome.SKIP_WITH_REASON: False,
}


def legacy_status(outcome: CoverageOutcome) -> str:
    """Map a ``CoverageOutcome`` to its legacy coarse ``status`` string."""
    return _OUTCOME_TO_LEGACY_STATUS[outcome]


def is_build_pass(outcome: CoverageOutcome) -> bool:
    """Return ``True`` if the outcome represents a successful build."""
    return _OUTCOME_TO_BUILD_PASS[outcome]


def is_semantic_checked(outcome: CoverageOutcome) -> bool:
    """Return ``True`` if the outcome was determined by semantic comparison."""
    return _OUTCOME_TO_SEMANTIC_CHECKED[outcome]


# ---------------------------------------------------------------------------
# Oracle policy
# ---------------------------------------------------------------------------

class OraclePolicy(str, enum.Enum):
    """Policy controlling when the Node.js oracle is invoked."""

    AUTO = "auto"
    """Oracle invoked when the Node comparison completes within timeout."""

    ALWAYS = "always"
    """Oracle always invoked; ``make_record()`` returns ``oracle_skipped`` on failure."""

    NEVER = "never"
    """Oracle never invoked; outcome defaults to ``build_pass`` when build succeeds."""


# ---------------------------------------------------------------------------
# make_record
# ---------------------------------------------------------------------------

def make_record(
    *,
    suite: str,
    case: str,
    outcome: CoverageOutcome | str,
    phase: CoveragePhase | str | None = None,
    target: str = "wasm-iwasm",
    expected: str | None = None,
    actual: str | None = None,
    reason: str | None = None,
    tracking: str | None = None,
    diagnostic_code: str | None = None,
    feature_label: str | None = None,
    unresolved_symbol: str | None = None,
    harness_includes: list[str] | None = None,
    source_code: str | None = None,
    error_line: int | None = None,
    stderr: str | None = None,
    node_exit_status: int | None = None,
    iwasm_exit_status: int | None = None,
    duration_ms: int | None = None,
    oracle_policy: OraclePolicy | str | None = None,
    selection_hash: str | None = None,
    abi_version: str | None = None,
    target_id: str | None = None,
    ts_boundary: str | None = None,
    executable_source: bool | None = None,
    declaration_only: bool | None = None,
    **extra: Any,
) -> str:
    """Create a schema v2 JSONL record string.

    Every returned record includes ``schema_version`` (integer 2), ``status``
    (legacy coarse status), ``outcome`` (canonical outcome string),
    ``build_pass`` (boolean), and ``semantic_checked`` (boolean). All other
    fields are passed through when provided.

    Parameters
    ----------
    suite : str
        Test suite identifier (e.g. ``"test262"``, ``"tsc"``, ``"tsgo"``).
    case : str
        Case path relative to the reference root.
    outcome : CoverageOutcome | str
        The precise coverage outcome.
    phase : CoveragePhase | str | None
        Pipeline phase where the outcome was determined.
    target : str
        Target runtime (default ``"wasm-iwasm"``).

    Returns
    -------
    str
        JSON-encoded record string, one line suitable for JSONL output.
    """
    if isinstance(outcome, str):
        outcome = CoverageOutcome(outcome)
    if isinstance(phase, str):
        phase = CoveragePhase(phase) if phase else None
    if isinstance(oracle_policy, str):
        oracle_policy = OraclePolicy(oracle_policy) if oracle_policy else None

    record: dict[str, Any] = {
        "schema_version": 2,
        "suite": suite,
        "case": case,
        "target": target,
        "status": legacy_status(outcome),
        "outcome": outcome.value,
        "build_pass": is_build_pass(outcome),
        "semantic_checked": is_semantic_checked(outcome),
    }

    if phase is not None:
        record["phase"] = phase.value

    # Pass-through optional fields
    for key, value in [
        ("expected", expected),
        ("actual", actual),
        ("reason", reason),
        ("tracking", tracking),
        ("diagnostic_code", diagnostic_code),
        ("feature_label", feature_label),
        ("unresolved_symbol", unresolved_symbol),
        ("harness_includes", harness_includes),
        ("source_code", source_code),
        ("error_line", error_line),
        ("stderr", stderr),
        ("node_exit_status", node_exit_status),
        ("iwasm_exit_status", iwasm_exit_status),
        ("duration_ms", duration_ms),
        ("oracle_policy", oracle_policy.value if oracle_policy else None),
        ("selection_hash", selection_hash),
        ("abi_version", abi_version),
        ("target_id", target_id),
        ("ts_boundary", ts_boundary),
        ("executable_source", executable_source),
        ("declaration_only", declaration_only),
    ]:
        if value is not None:
            record[key] = value

    # Pass through any extra kwargs
    record.update(extra)

    return json.dumps(record, ensure_ascii=False, sort_keys=True)


# ---------------------------------------------------------------------------
# Oracle-policy-aware make_record
# ---------------------------------------------------------------------------

def make_record_with_policy(
    *,
    suite: str,
    case: str,
    outcome: CoverageOutcome | str,
    phase: CoveragePhase | str | None = None,
    oracle_policy: OraclePolicy | str | None = None,
    node_ok: bool | None = None,
    **kwargs: Any,
) -> str:
    """Create a schema v2 record respecting the oracle policy.

    When ``oracle_policy`` is ``NEVER`` and the outcome would be
    ``SEMANTIC_PASS``, the record is downgraded to ``BUILD_PASS`` because
    no oracle comparison was performed.

    When ``oracle_policy`` is ``ALWAYS`` and the oracle is unavailable
    (``node_ok`` is ``False``), the record is downgraded to ``ORACLE_SKIPPED``.
    """
    if isinstance(outcome, str):
        outcome = CoverageOutcome(outcome)
    if isinstance(oracle_policy, str):
        oracle_policy = OraclePolicy(oracle_policy) if oracle_policy else None

    if oracle_policy == OraclePolicy.NEVER and outcome in (
        CoverageOutcome.SEMANTIC_PASS,
        CoverageOutcome.SEMANTIC_MISMATCH,
    ):
        outcome = CoverageOutcome.BUILD_PASS
        phase = phase or CoveragePhase.COMPILE

    if oracle_policy == OraclePolicy.ALWAYS and node_ok is False and outcome in (
        CoverageOutcome.SEMANTIC_PASS,
        CoverageOutcome.SEMANTIC_MISMATCH,
    ):
        outcome = CoverageOutcome.ORACLE_SKIPPED
        phase = phase or CoveragePhase.ORACLE

    return make_record(
        suite=suite,
        case=case,
        outcome=outcome,
        phase=phase,
        oracle_policy=oracle_policy,
        **kwargs,
    )


# ---------------------------------------------------------------------------
# Self-test
# ---------------------------------------------------------------------------

def _self_test() -> int:
    """Run self-test to verify schema v2 record structure."""
    errors: list[str] = []

    # Test 1: basic semantic_pass record
    record_str = make_record(
        suite="test262",
        case="test/language/foo.js",
        outcome=CoverageOutcome.SEMANTIC_PASS,
        phase=CoveragePhase.ORACLE,
        expected="ok",
        actual="ok",
        duration_ms=42,
    )
    record = json.loads(record_str)
    for key in ("schema_version", "suite", "case", "target", "status",
                "outcome", "build_pass", "semantic_checked"):
        if key not in record:
            errors.append(f"Test 1: missing key {key}")
    if record.get("schema_version") != 2:
        errors.append(f"Test 1: schema_version != 2: got {record.get('schema_version')}")
    if record.get("status") != "pass":
        errors.append(f"Test 1: status != 'pass': got {record.get('status')}")
    if record.get("outcome") != "semantic_pass":
        errors.append(f"Test 1: outcome != 'semantic_pass': got {record.get('outcome')}")
    if record.get("build_pass") is not True:
        errors.append(f"Test 1: build_pass != True: got {record.get('build_pass')}")
    if record.get("semantic_checked") is not True:
        errors.append(f"Test 1: semantic_checked != True: got {record.get('semantic_checked')}")

    # Test 2: unsupported record
    record_str = make_record(
        suite="test262",
        case="test/built-ins/foo.js",
        outcome=CoverageOutcome.UNSUPPORTED,
        phase=CoveragePhase.PARSE,
        reason="Unsupported syntax",
        tracking="feature:async",
        diagnostic_code="UnsupportedSyntax",
    )
    record = json.loads(record_str)
    if record.get("status") != "unsupported":
        errors.append(f"Test 2: status != 'unsupported': got {record.get('status')}")
    if record.get("build_pass") is not False:
        errors.append(f"Test 2: build_pass != False: got {record.get('build_pass')}")
    if record.get("semantic_checked") is not False:
        errors.append(f"Test 2: semantic_checked != False: got {record.get('semantic_checked')}")
    if not record.get("reason"):
        errors.append("Test 2: missing reason")
    if not record.get("tracking"):
        errors.append("Test 2: missing tracking")

    # Test 3: mismatch outcome
    record_str = make_record(
        suite="test262",
        case="test/language/bar.js",
        outcome=CoverageOutcome.SEMANTIC_MISMATCH,
        phase=CoveragePhase.ORACLE,
        expected="expected\n",
        actual="got\n",
        reason="output mismatch",
    )
    record = json.loads(record_str)
    if record.get("status") != "fail":
        errors.append(f"Test 3: status != 'fail': got {record.get('status')}")
    if record.get("outcome") != "semantic_mismatch":
        errors.append(f"Test 3: outcome != 'semantic_mismatch': got {record.get('outcome')}")
    if record.get("build_pass") is not True:
        errors.append(f"Test 3: build_pass != True: got {record.get('build_pass')}")
    if record.get("semantic_checked") is not True:
        errors.append(f"Test 3: semantic_checked != True: got {record.get('semantic_checked')}")

    # Test 4: runtime_error outcome
    record_str = make_record(
        suite="test262",
        case="test/language/baz.js",
        outcome=CoverageOutcome.RUNTIME_ERROR,
        phase=CoveragePhase.RUNTIME,
        reason="iwasm trap: unreachable",
        iwasm_exit_status=1,
    )
    record = json.loads(record_str)
    if record.get("status") != "runtime_error":
        errors.append(f"Test 4: status != 'runtime_error': got {record.get('status')}")
    if record.get("outcome") != "runtime_error":
        errors.append(f"Test 4: outcome != 'runtime_error': got {record.get('outcome')}")
    if record.get("build_pass") is not True:
        errors.append(f"Test 4: build_pass != True: got {record.get('build_pass')}")
    if record.get("semantic_checked") is not False:
        errors.append(f"Test 4: semantic_checked != False: got {record.get('semantic_checked')}")

    # Test 5: oracle_skipped outcome
    record_str = make_record(
        suite="test262",
        case="test/language/qux.js",
        outcome=CoverageOutcome.ORACLE_SKIPPED,
        phase=CoveragePhase.ORACLE,
        reason="node oracle unavailable",
    )
    record = json.loads(record_str)
    if record.get("status") != "oracle_skipped":
        errors.append(f"Test 5: status != 'oracle_skipped': got {record.get('status')}")
    if record.get("outcome") != "oracle_skipped":
        errors.append(f"Test 5: outcome != 'oracle_skipped': got {record.get('outcome')}")

    # Test 6: verified negative compile
    record_str = make_record(
        suite="test262",
        case="test/language/negative.js",
        outcome=CoverageOutcome.VERIFIED_NEGATIVE_COMPILE,
        phase=CoveragePhase.PARSE,
        reason="negative parse/SyntaxError rejected during compilation",
    )
    record = json.loads(record_str)
    if record.get("status") != "pass":
        errors.append(f"Test 6: status != 'pass': got {record.get('status')}")
    if record.get("outcome") != "verified_negative_compile":
        errors.append(f"Test 6: outcome != 'verified_negative_compile': got {record.get('outcome')}")

    # Test 7: schema v1 fallback (no schema_version should still be valid as reader)
    v1_record = json.dumps({
        "suite": "test262",
        "case": "test/language/v1.js",
        "target": "wasm-iwasm",
        "status": "pass",
        "expected": "ok",
        "actual": "ok",
    }, sort_keys=True)
    # Just ensure it's valid JSON
    json.loads(v1_record)

    # Test 8: oracle_policy=NEVER downgrade
    record_str = make_record_with_policy(
        suite="test262",
        case="test/language/no-oracle.js",
        outcome=CoverageOutcome.SEMANTIC_PASS,
        phase=CoveragePhase.ORACLE,
        oracle_policy=OraclePolicy.NEVER,
    )
    record = json.loads(record_str)
    if record.get("outcome") != "build_pass":
        errors.append(f"Test 8: outcome != 'build_pass': got {record.get('outcome')}")

    # Test 9: oracle_policy=ALWAYS with node_ok=False
    record_str = make_record_with_policy(
        suite="test262",
        case="test/language/no-node.js",
        outcome=CoverageOutcome.SEMANTIC_PASS,
        phase=CoveragePhase.ORACLE,
        oracle_policy=OraclePolicy.ALWAYS,
        node_ok=False,
    )
    record = json.loads(record_str)
    if record.get("outcome") != "oracle_skipped":
        errors.append(f"Test 9: outcome != 'oracle_skipped': got {record.get('outcome')}")

    # Summary
    if errors:
        print("coverage_outcome self-test FAILED:", file=sys.stderr)
        for error in errors:
            print(f"  - {error}", file=sys.stderr)
        return 1

    print("coverage_outcome: self-test OK (9 checks)", file=sys.stderr)
    return 0


if __name__ == "__main__":
    if "--self-test" in sys.argv:
        sys.exit(_self_test())
    print(__doc__)
