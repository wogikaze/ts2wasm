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

EXACT_COMMAND_RE = re.compile(
    r"\b(cargo|mise run|node|iwasm|wasm-tools|python|TS2WASM_REFERENCE_ROOT=)"
)


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
        findings=findings,
    )


def render_text(rows: list[IssueReadiness], limit: int) -> str:
    by_band: dict[str, int] = {"ready": 0, "needs-refinement": 0, "not-ready": 0}
    by_class: dict[str, int] = {}
    for row in rows:
        by_band[row.band] += 1
        by_class[row.issue_class] = by_class.get(row.issue_class, 0) + 1

    lines = [
        f"open issues: {len(rows)}",
        "readiness bands: " + ", ".join(f"{k}={v}" for k, v in by_band.items()),
        "classes: " + ", ".join(f"{k or 'missing'}={v}" for k, v in sorted(by_class.items())),
        "",
        "lowest scoring issues:",
    ]
    for row in sorted(rows, key=lambda r: (r.score, r.issue_id))[:limit]:
        first = row.findings[0] if row.findings else "no findings"
        lines.append(f"- {row.issue_id} [{row.score:03d} {row.band}] {row.title} ({row.issue_class}): {first}")
    return "\n".join(lines)


def render_markdown(rows: list[IssueReadiness], limit: int) -> str:
    lines = [
        "| ID | Score | Band | Class | Priority | Title | Top finding |",
        "|---:|---:|---|---|---|---|---|",
    ]
    for row in sorted(rows, key=lambda r: (r.score, r.issue_id))[:limit]:
        top = row.findings[0] if row.findings else ""
        title = row.title.replace("|", "\\|")
        top = top.replace("|", "\\|")
        lines.append(f"| {row.issue_id} | {row.score} | {row.band} | {row.issue_class} | {row.priority} | {title} | {top} |")
    return "\n".join(lines)


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--format", choices=["text", "json", "markdown"], default="text")
    parser.add_argument("--limit", type=int, default=20, help="number of lowest-scoring rows to show")
    parser.add_argument(
        "--fail-ready-below",
        type=int,
        default=0,
        help="fail if any non-blocked open issue scores below this threshold",
    )
    return parser.parse_args(argv)


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
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
