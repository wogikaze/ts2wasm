#!/usr/bin/env python3
"""Measure how actionable open issues are.

The score is intentionally mechanical. It does not decide whether an issue is
important; it measures whether another engineer can pick it up without doing
scope design first.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
import shlex
from dataclasses import asdict, dataclass
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent / "lib"))
from issue_common import Issue, load_issues

REPO = Path(__file__).resolve().parents[2]

GENERIC_ACCEPTANCE_PHRASES = (
    "passes for basic cases",
    "related diagnostics reduced",
    "regression test added",
    "docs updated if semantics change",
    "works correctly",
    "no regression",
)

PLACEHOLDER_PHRASES = (
    "add exact commands here",
    "fill only when moving",
    "short imperative title",
    "describe the work",
    "describe the concrete problem",
    "none",
)

SIZE_BAND_RANK = {"S": 0, "M": 1, "L": 2, "XL": 3}

EXACT_COMMAND_RE = re.compile(
    r"\b(cargo|mise run|node|iwasm|wasm-tools|python|TS2WASM_REFERENCE_ROOT=)"
)
TEST_FILE_RE = re.compile(r"\b[^\s`\"']+\.(?:ts|js|jsx|mjs|rs|wat|jsonl|json|toml)\b")
METHOD_LIKE_RE = re.compile(r"\b(method|opcode|instruction|builtin|runtime helper|runtime-?|emit|lower)\b", re.I)
METHOD_NAME_RE = re.compile(r"`[^`]*(?:\.\w+|\([^`]*\)|::\w+)`")
DATA_MODEL_LIKE_RE = re.compile(r"\b(data model|データ構造|schema|ast|ir|manifest|representation|type|struct|enum)\b", re.I)
ABI_LIKE_RE = re.compile(r"\b(abi|component|runtime-abi|imports?|exports?|table|memory|global|func type)\b", re.I)
CASE_COUNT_RE = re.compile(
    r"\b(?P<count>[0-9][0-9_,]*)\s+(?:test262|tsc)\s+cases?\b",
    re.I,
)
CASES_RE = re.compile(r"\b(?P<count>[0-9][0-9_,]*)\s+cases?\b", re.I)
LARGE_LITERAL_RE = re.compile(r"\b(?P<number>[0-9][0-9_,.]*)\b")


def normalize_size_band(value: str) -> str:
    band = value.strip().upper()
    if band not in SIZE_BAND_RANK:
        raise argparse.ArgumentTypeError(f"invalid size band: {value}")
    return band


def to_size_band(score: int) -> str:
    if score >= 32:
        return "XL"
    if score >= 24:
        return "L"
    if score >= 16:
        return "M"
    return "S"


def _tokenize_command(command: str) -> list[str]:
    try:
        return shlex.split(command)
    except ValueError:
        return command.split()


def _iter_command_lines(text: str) -> list[str]:
    blocks = command_blocks(text)
    lines: list[str] = []
    for block in blocks:
        for raw in block.splitlines():
            line = raw.strip()
            if not line or line.startswith("#"):
                continue
            if line.endswith("\\"):
                line = line[:-1].strip()
            lines.append(line)
    return lines


def _count_tests_from_command(command: str) -> int:
    tokens = _tokenize_command(command)
    if not tokens:
        return 0
    head = tokens[0]

    if head == "node":
        if any(tok.endswith((".ts", ".js", ".mjs")) for tok in tokens[1:]):
            return 1
        return 0

    if head == "cargo":
        if len(tokens) < 2:
            return 0

        if tokens[1] == "nextest":
            if len(tokens) < 3 or tokens[2] != "run":
                return 0
            args = tokens[3:]
            if "--" in args:
                args = args[: args.index("--")]
            test_like = [tok for tok in args if not tok.startswith("-")]
            if test_like:
                return 1 + min(3, len(set(test_like)))
            return 6

        if tokens[1] == "test":
            args = tokens[2:]
            if "--" in args:
                args = args[: args.index("--")]
            test_like = [tok for tok in args if not tok.startswith("-")]
            if test_like:
                return 1 + min(3, len(set(test_like)))
            return 6

    joined = " ".join(tokens)
    if "reference-coverage" in joined:
        sample = re.search(r"--sample\s+(\d+)", joined)
        if sample:
            return max(1, (int(sample.group(1)) + 29) // 30)
        if "--path-filter" in joined:
            return 4
        return 3

    if "reference-triage" in joined:
        return 1

    if head in {"mise", "bunx", "python", "pytest"}:
        if "test262" in joined:
            return 2
        return 1 if "test" in joined else 0

    return 0


def estimate_issue_test_load(issue: Issue) -> tuple[int, list[str]]:
    total = 0
    reasons: list[str] = []
    sections = [section(issue.text, "Validation"), section(issue.text, "Current failure"), section(issue.text, "Acceptance criteria")]
    for section_text in sections:
        for command in _iter_command_lines(section_text):
            estimate = _count_tests_from_command(command)
            if estimate:
                total += estimate
                reasons.append(f"{estimate}:{command[:40]}")

    file_refs = set(TEST_FILE_RE.findall(issue.text))
    file_bonus = min(len(file_refs), 5)
    if file_bonus:
        total += file_bonus
        reasons.append(f"+{file_bonus} fixture/test refs")

    return max(1, total), reasons


def estimate_reference_case_count(text: str) -> int:
    max_count = 0
    for match in CASE_COUNT_RE.finditer(text):
        raw = match.group("count").replace("_", "").replace(",", "")
        try:
            max_count = max(max_count, int(raw))
        except ValueError:
            continue

    for match in CASES_RE.finditer(text):
        raw = match.group("count").replace("_", "").replace(",", "")
        try:
            max_count = max(max_count, int(raw))
        except ValueError:
            continue

    return max_count


def estimate_reference_case_bonus(case_count: int) -> int:
    if case_count >= 3000:
        return 12
    if case_count >= 1500:
        return 10
    if case_count >= 500:
        return 8
    if case_count >= 200:
        return 6
    if case_count >= 80:
        return 1
    return 0


def has_large_numeric_literal(text: str) -> bool:
    for match in LARGE_LITERAL_RE.finditer(text):
        raw = match.group("number").replace("_", "").replace(",", "")
        try:
            value = int(raw)
        except ValueError:
            continue
        if value >= 1_000_000:
            return True
    return False


def estimate_issue_size_score(issue: Issue, findings: list[str]) -> tuple[int, int]:
    test_estimate, _ = estimate_issue_test_load(issue)
    work_units, _method_units, _abi_units, _data_model_units, _unmatched_units = estimate_issue_work_units(issue)

    if issue.orch_class == "triage-needed":
        base_size = min(test_estimate, 14) + min(work_units, 3)
    else:
        base_size = test_estimate + min(work_units, 8)

    if issue.orch_class == "blocked":
        base_size += 8
    elif issue.orch_class == "triage-needed":
        base_size += 2

    if issue.depends:
        base_size += len(issue.depends) * 3

    case_count = estimate_reference_case_count(issue.text)
    base_size += estimate_reference_case_bonus(case_count)
    if case_count >= 80:
        findings.append(f"reference-case scale indicates larger implementation slice ({case_count} cases)")

    if issue.orch_class in {"blocked", "triage-needed"} and has_large_numeric_literal(issue.text):
        base_size += 2

    return base_size, case_count


def estimate_issue_work_units(issue: Issue) -> tuple[int, int, int, int, int]:
    scope = section(issue.text, "Scope")
    acceptance = section(issue.text, "Acceptance criteria")
    items = unchecked_items(scope) + unchecked_items(acceptance)
    if not items:
        return 0, 0, 0, 0, 0

    method_units = 0
    abi_units = 0
    data_model_units = 0
    unmatched_units = 0

    for item in items:
        lowered = item.lower()
        if METHOD_NAME_RE.search(item) or METHOD_LIKE_RE.search(lowered):
            method_units += 1
            continue
        if DATA_MODEL_LIKE_RE.search(lowered):
            data_model_units += 1
            continue
        if ABI_LIKE_RE.search(lowered):
            abi_units += 1
            continue
        unmatched_units += 1

    return len(items), method_units, abi_units, data_model_units, unmatched_units


@dataclass(frozen=True)
class IssueReadiness:
    issue_id: str
    title: str
    path: str
    issue_class: str
    priority: str
    score: int
    band: str
    metadata: int
    problem: int
    scope: int
    acceptance: int
    validation: int
    size: int
    size_score: int
    size_band: str
    work_units: int
    method_units: int
    abi_units: int
    data_model_units: int
    test_estimate: int
    findings: list[str]


def section(text: str, heading: str) -> str:
    pattern = re.compile(rf"^## {re.escape(heading)}\s*$", re.M)
    match = pattern.search(text)
    if not match:
        return ""
    start = match.end()
    next_match = re.search(r"^##\s+", text[start:], re.M)
    end = start + next_match.start() if next_match else len(text)
    return text[start:end].strip()


def has_line_problem(text: str) -> bool:
    match = re.search(r"^Problem:\s*(.+)$", text, re.M)
    return bool(match and len(match.group(1).strip()) >= 20)


def unchecked_items(text: str) -> list[str]:
    return [line.strip()[6:].strip() for line in text.splitlines() if line.strip().startswith("- [ ] ")]


def command_blocks(text: str) -> list[str]:
    blocks = []
    for match in re.finditer(r"```(?:sh|bash|text)?\n(.*?)```", text, re.S):
        blocks.append(match.group(1).strip())
    return blocks


def score_metadata(issue: Issue, findings: list[str]) -> int:
    required = {
        "id": issue.body_id,
        "title": issue.title,
        "type": issue.type_val,
        "area": issue.area,
        "class": issue.orch_class,
        "priority": issue.priority,
    }
    missing = [name for name, value in required.items() if not value]
    if missing:
        findings.append(f"missing metadata: {', '.join(missing)}")
    return round(20 * (len(required) - len(missing)) / len(required))


def score_problem(issue: Issue, findings: list[str]) -> int:
    text = issue.text
    score = 0
    if has_line_problem(text):
        score += 10
    else:
        findings.append("missing one-line Problem: summary for index/reporting")

    problem = section(text, "Problem")
    current_failure_markers = (
        "command:",
        "stderr:",
        "stdout:",
        "pre-change",
        "currently",
        "Reference test results",
        "fails",
        "unsupported",
    )
    if problem and any(marker in problem for marker in current_failure_markers):
        score += 10
    else:
        findings.append("problem statement lacks concrete current failure or evidence")
    return score


def score_scope(issue: Issue, findings: list[str]) -> int:
    text = issue.text
    scope_text = section(text, "Scope")
    paths_text = section(text, "Affected paths")
    score = 0
    if "In scope:" in scope_text and "- [ ]" in scope_text:
        score += 6
    else:
        findings.append("scope lacks checked-list in-scope items")
    if "Out of scope:" in scope_text:
        score += 4
    else:
        findings.append("scope lacks out-of-scope boundary")
    if "Expected:" in paths_text and re.search(r"`[^`]+/[^`]*`", paths_text):
        score += 6
    else:
        findings.append("affected paths lacks concrete expected paths")
    if "Do not touch:" in paths_text:
        score += 4
    else:
        findings.append("affected paths lacks do-not-touch boundary")
    return score


def score_acceptance(issue: Issue, findings: list[str]) -> int:
    acceptance = section(issue.text, "Acceptance criteria")
    items = unchecked_items(acceptance)
    if not items:
        findings.append("acceptance criteria has no unchecked measurable items")
        return 0

    score = 0
    if 2 <= len(items) <= 7:
        score += 6
    elif len(items) == 1:
        score += 3
        findings.append("acceptance criteria has only one item")
    else:
        score += 2
        findings.append("acceptance criteria has too many items for a small slice")

    generic = [
        item
        for item in items
        if any(phrase.lower() in item.lower() for phrase in GENERIC_ACCEPTANCE_PHRASES)
    ]
    if not generic:
        score += 8
    else:
        findings.append("acceptance criteria contains generic non-measurable items")

    observable_markers = (
        "fixture",
        "diagnostic",
        "stdout",
        "stderr",
        "pass",
        "reject",
        "reports",
        "emits",
        "runs",
        "matches",
        "issue",
    )
    if any(any(marker in item.lower() for marker in observable_markers) for item in items):
        score += 6
    else:
        findings.append("acceptance criteria lacks observable behavior/evidence")
    return score


def score_validation(issue: Issue, findings: list[str]) -> int:
    validation = section(issue.text, "Validation")
    blocks = command_blocks(validation)
    commands = "\n".join(blocks)
    score = 0
    if "Required commands:" in validation and commands:
        score += 6
    else:
        findings.append("validation lacks required command block")
    if EXACT_COMMAND_RE.search(commands):
        score += 8
    else:
        findings.append("validation lacks exact runnable commands")
    if "Impacted commands:" in validation and len(blocks) >= 2:
        score += 4
    else:
        findings.append("validation lacks impacted command block")
    if not any(phrase in validation.lower() for phrase in PLACEHOLDER_PHRASES[:1]):
        score += 2
    else:
        findings.append("validation contains placeholder text")
    return score


def score_size(issue: Issue, findings: list[str]) -> int:
    lines = issue.text.splitlines()
    checks = unchecked_items(issue.text)
    score = 20
    if len(lines) > 260:
        score -= 8
        findings.append("issue is long enough to hide current work")
    elif len(lines) > 180:
        score -= 4
        findings.append("issue is large; consider splitting or summarizing progress")
    if len(checks) > 22:
        score -= 6
        findings.append("issue has too many unchecked items for one slice")
    if "Progress evidence" in issue.text and len(lines) > 180:
        score -= 4
        findings.append("progress log should be summarized or moved to done evidence")
    if "Related diagnostics reduced in reference tests" in issue.text:
        score -= 6
        findings.append("generated reference issue needs triage before implementation")
    return max(score, 0)


def band(score: int) -> str:
    if score >= 80:
        return "ready"
    if score >= 60:
        return "needs-refinement"
    return "not-ready"


def measure(issue: Issue) -> IssueReadiness:
    findings: list[str] = []
    metadata = score_metadata(issue, findings)
    problem = score_problem(issue, findings)
    scope = score_scope(issue, findings)
    acceptance = score_acceptance(issue, findings)
    validation = score_validation(issue, findings)
    size = score_size(issue, findings)
    size_score, case_count = estimate_issue_size_score(issue, findings)
    test_estimate, _test_reasons = estimate_issue_test_load(issue)
    work_units, method_units, abi_units, data_model_units, unmatched_units = estimate_issue_work_units(issue)
    if work_units == 0:
        findings.append("work-units could not be derived from scope/acceptance checklists")
    if unmatched_units == work_units and work_units > 0:
        findings.append("work-units are not typed by method/ABI/data-model")
    if work_units >= 10:
        findings.append(
            f"work units ({work_units}) suggests splitting this issue across data-model/ABI/method slices"
        )
    if case_count >= 5000:
        findings.append("estimated reference-case count suggests oversized implementation slice")
    if to_size_band(size_score) in {"L", "XL"}:
        findings.append(f"estimated implementation size is {to_size_band(size_score)}")
    if test_estimate >= 40:
        findings.append("estimated test load suggests oversized implementation slice")
    total = round((metadata * 0.5) + problem + scope + acceptance + validation + (size * 0.5))
    generated_reference_bucket = (
        issue.orch_class == "triage-needed"
        or "Reference test results show" in issue.text
        and (
            "Related diagnostics reduced in reference tests" in issue.text
            or "needs smart-triage evidence before implementation starts" in issue.text
        )
        and "## Affected test files" in issue.text
    )
    if generated_reference_bucket:
        total = min(total, 55)
        findings.append("generated reference bucket must be split or triaged before selection")
    return IssueReadiness(
        issue_id=issue.name_id,
        title=issue.title,
        path=str(issue.path.relative_to(REPO)),
        issue_class=issue.orch_class,
        priority=issue.priority,
        score=total,
        band=band(total),
        metadata=metadata,
        problem=problem,
        scope=scope,
        acceptance=acceptance,
        validation=validation,
        size=size,
        size_score=size_score,
        size_band=to_size_band(size_score),
        work_units=work_units,
        method_units=method_units,
        abi_units=abi_units,
        data_model_units=data_model_units,
        findings=findings,
        test_estimate=test_estimate,
    )


def render_text(rows: list[IssueReadiness], limit: int) -> str:
    by_band: dict[str, int] = {"ready": 0, "needs-refinement": 0, "not-ready": 0}
    by_class: dict[str, int] = {}
    by_size_band: dict[str, int] = {"S": 0, "M": 0, "L": 0, "XL": 0}
    for row in rows:
        by_band[row.band] += 1
        by_class[row.issue_class] = by_class.get(row.issue_class, 0) + 1
        by_size_band[row.size_band] = by_size_band.get(row.size_band, 0) + 1

    lines = [
        f"open issues: {len(rows)}",
        "readiness bands: " + ", ".join(f"{k}={v}" for k, v in by_band.items()),
        "size bands: " + ", ".join(f"{k}={v}" for k, v in by_size_band.items()),
        "classes: " + ", ".join(f"{k or 'missing'}={v}" for k, v in sorted(by_class.items())),
        "",
        "lowest scoring issues:",
    ]
    for row in sorted(rows, key=lambda r: (r.score, r.issue_id))[:limit]:
        first = row.findings[0] if row.findings else "no findings"
        lines.append(
            f"- {row.issue_id} [readiness={row.score:03d} {row.band}] "
            f"[size={row.size_score} {row.size_band}] "
            f"[test={row.test_estimate}] {row.title} ({row.issue_class}): {first}"
        )
    return "\n".join(lines)


def render_markdown(rows: list[IssueReadiness], limit: int) -> str:
    lines = [
        "| ID | Score | Band | Size score | Size band | Class | Priority | Test estimate | Work units (M/A/D) | Title | Top finding |",
        "|---:|---:|---|---:|---|---|---|---:|---:|---|---|",
    ]
    for row in sorted(rows, key=lambda r: (r.score, r.issue_id))[:limit]:
        top = row.findings[0] if row.findings else ""
        title = row.title.replace("|", "\\|")
        top = top.replace("|", "\\|")
        lines.append(
            f"| {row.issue_id} | {row.score} | {row.band} | {row.size_score} | {row.size_band} | "
            f"{row.issue_class} | "
            f"{row.priority} | {row.test_estimate} | "
            f"{row.method_units}/{row.abi_units}/{row.data_model_units} ({row.work_units}) | "
            f"{title} | {top} |"
        )
    return "\n".join(lines)


def parse_args(argv: list[str]) -> argparse.Namespace:
    if argv and argv[0] == "--":
        argv = argv[1:]

    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--format", choices=["text", "json", "markdown"], default="text")
    parser.add_argument("--limit", type=int, default=20, help="number of lowest-scoring rows to show")
    parser.add_argument(
        "--fail-ready-below",
        type=int,
        default=0,
        help="fail if any non-blocked open issue scores below this threshold",
    )
    parser.add_argument(
        "--fail-test-estimate-above",
        type=int,
        default=0,
        help="fail if any non-blocked open issue estimate exceeds this test-load threshold",
    )
    parser.add_argument(
        "--fail-work-units-above",
        type=int,
        default=0,
        help="fail if any non-blocked open issue has too many scope/acceptance work units",
    )
    parser.add_argument(
        "--warn-size-band",
        type=normalize_size_band,
        default="M",
        help="warn if any non-blocked issue reaches or exceeds this implementation-size band",
    )
    parser.add_argument(
        "--fail-size-band",
        type=normalize_size_band,
        default="L",
        help="fail if any non-blocked issue reaches or exceeds this implementation-size band",
    )
    return parser.parse_args(argv)


def filter_offenders_by_size_band(rows: list[IssueReadiness], min_band: str) -> list[IssueReadiness]:
    min_rank = SIZE_BAND_RANK[min_band]
    return [
        row
        for row in rows
        if row.issue_class not in {"blocked", "triage-needed"}
        and SIZE_BAND_RANK.get(row.size_band, 0) >= min_rank
    ]


def main(argv: list[str]) -> int:
    args = parse_args(argv)
    rows = [measure(issue) for issue in load_issues(REPO) if issue.state == "open"]

    if args.format == "json":
        print(json.dumps([asdict(row) for row in rows], indent=2))
    elif args.format == "markdown":
        print(render_markdown(rows, args.limit))
    else:
        print(render_text(rows, args.limit))

    if args.fail_ready_below:
        offenders = [
            row
            for row in rows
            if row.issue_class not in {"blocked", "triage-needed"} and row.score < args.fail_ready_below
        ]
        if offenders:
            print(
                f"issue-readiness: {len(offenders)} non-blocked issue(s) below {args.fail_ready_below}",
                file=sys.stderr,
            )
            return 1

    if args.fail_test_estimate_above:
        offenders = [
            row
            for row in rows
            if row.issue_class not in {"blocked", "triage-needed"} and row.test_estimate > args.fail_test_estimate_above
        ]
        if offenders:
            print(
                f"issue-readiness: {len(offenders)} non-blocked issue(s) above test-estimate threshold {args.fail_test_estimate_above}",
                file=sys.stderr,
            )
            return 1

    if args.fail_work_units_above:
        offenders = [
            row
            for row in rows
            if row.issue_class not in {"blocked", "triage-needed"} and row.work_units > args.fail_work_units_above
        ]
        if offenders:
            print(
                f"issue-readiness: {len(offenders)} non-blocked issue(s) above work-unit threshold {args.fail_work_units_above}",
                file=sys.stderr,
            )
            return 1

    warn_offenders = filter_offenders_by_size_band(rows, args.warn_size_band)
    fail_offenders = filter_offenders_by_size_band(rows, args.fail_size_band)

    warn_only = [row for row in warn_offenders if row not in fail_offenders]
    if warn_only:
        warn_only = sorted(
            warn_only,
            key=lambda row: (SIZE_BAND_RANK.get(row.size_band, 0), row.size_score, row.issue_id),
            reverse=True,
        )
        print(
            f"issue-readiness: {len(warn_only)} non-blocked issue(s) at-or-above {args.warn_size_band}: "
            + ", ".join(f"{row.issue_id}({row.size_band})" for row in warn_only),
            file=sys.stderr,
        )

    if fail_offenders:
        fail_offenders = sorted(
            fail_offenders,
            key=lambda row: (SIZE_BAND_RANK.get(row.size_band, 0), row.size_score, row.issue_id),
            reverse=True,
        )
        print(
            f"issue-readiness: {len(fail_offenders)} non-blocked issue(s) at-or-above {args.fail_size_band} (fail): "
            + ", ".join(f"{row.issue_id}({row.size_band})" for row in fail_offenders),
            file=sys.stderr,
        )
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
