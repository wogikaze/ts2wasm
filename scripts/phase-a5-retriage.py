#!/usr/bin/env python3
"""Phase A.5: Enhanced re-triage for issue #5000 child issues.

Phase A (phase-a-retriage.py) reclassified ~543 issues by title keywords.
The remaining ~630 issues have generic "Implement {fixture_name}" titles
not caught by Phase A keyword rules.

This Phase A.5 script searches issue BODIES for diagnostic patterns
(DiagCode::*, diagnostic code references) and also catches title
keywords missed by Phase A (API, async/await, generator/yield, sourcemap).

Classification rules (first match wins):
  BODY diagnostic patterns:
    - UnsupportedModule        → area: frontend/resolver, depends_on: [5005]
    - UnresolvedName           → area: frontend/resolver, depends_on: [5005]
    - UnresolvedFunction       → area: frontend/resolver, depends_on: [5005]
    - DuplicateFunction        → area: frontend/semantics, depends_on: [5001]
    - DuplicateParameter       → area: frontend/semantics, depends_on: [5001]
    - ArityMismatch            → area: frontend/semantics, depends_on: [5001]
    - UnsupportedEval          → area: runtime/builtins, depends_on: [5004]
    - UnsupportedDate          → area: runtime/builtins, depends_on: [5004]
    - UnsupportedRegExp        → area: runtime/builtins, depends_on: [5004]
    - UnsupportedRuntimeSubset → area: runtime/builtins, depends_on: [5004]
  TITLE keywords:
    - API                      → area: runtime/builtins, depends_on: [5004]
    - async / await            → area: runtime/builtins, depends_on: [5004]
    - generator / yield        → area: runtime/builtins, depends_on: [5004]
    - sourcemap / source.?map  → stays as frontend/syntax, depends_on: [5000]
    - regexp / regex           → area: runtime/builtins, depends_on: [5004]
    - arguments                → area: runtime/builtins, depends_on: [5004]

Also marks confirmed parser issues (sourcemap, etc.) as "confirmed"
without changing frontmatter.
"""

import os
import re
import sys

ISSUES_DIR = "TRACKING.yaml"  # migrated from issues/open/

# Body diagnostic pattern -> (new_area, new_depends_on, label)
BODY_RULES = [
    # Resolver issues
    (r"\bUnsupportedModule\b",       "frontend/resolver", [5005], "module-namespace"),
    (r"\bUnresolvedName\b",          "frontend/resolver", [5005], "unresolved-name"),
    (r"\bUnresolvedFunction\b",      "frontend/resolver", [5005], "unresolved-function"),
    (r"\bDiagCode::UnsupportedModule\b",  "frontend/resolver", [5005], "module-namespace"),
    (r"\bDiagCode::UnresolvedName\b",     "frontend/resolver", [5005], "unresolved-name"),
    (r"\bDiagCode::UnresolvedFunction\b", "frontend/resolver", [5005], "unresolved-function"),
    # Semantic issues
    (r"\bDuplicateFunction\b",       "frontend/semantics", [5001], "duplicate-function"),
    (r"\bDuplicateParameter\b",      "frontend/semantics", [5001], "duplicate-parameter"),
    (r"\bArityMismatch\b",           "frontend/semantics", [5001], "arity-mismatch"),
    (r"\bDiagCode::DuplicateFunction\b",  "frontend/semantics", [5001], "duplicate-function"),
    (r"\bDiagCode::ArityMismatch\b",      "frontend/semantics", [5001], "arity-mismatch"),
    # Runtime/builtins issues
    (r"\bUnsupportedEval\b",         "runtime/builtins", [5004], "eval"),
    (r"\bUnsupportedDate\b",         "runtime/builtins", [5004], "date"),
    (r"\bUnsupportedRegExp\b",       "runtime/builtins", [5004], "regexp"),
    (r"\bUnsupportedRuntimeSubset\b","runtime/builtins", [5004], "runtime-subset"),
    (r"\bDiagCode::UnsupportedEval\b",    "runtime/builtins", [5004], "eval"),
    (r"\bDiagCode::UnsupportedRuntimeSubset\b", "runtime/builtins", [5004], "runtime-subset"),
]

# Title keyword patterns -> (new_area, new_depends_on, label)
# Note: sourcemap stays in parser syntax [5000] (confirmed, not changed)
TITLE_RULES = [
    # These stay as parser issues (confirmed, no frontmatter change)
    (r"(?i)(source.?map)",          "frontend/syntax", [5000], "source-map"),
    # These get reclassified to runtime/builtins
    (r"(?i)(async|await)",          "runtime/builtins", [5004], "async-await"),
    (r"(?i)(generator|yield)",      "runtime/builtins", [5004], "generator-yield"),
    (r"(?i)(regexp|regex)",         "runtime/builtins", [5004], "regexp"),
    (r"(?i)(arguments)",            "runtime/builtins", [5004], "arguments"),
    (r"(?i)(api)",                  "runtime/builtins", [5004], "tsc-api"),
]


def depends_on_5000(content: str) -> bool:
    """Check if issue depends on [5000] (simple or compound)."""
    # Match depends_on: [5000] or depends_on: [5000, ...]
    return bool(re.search(r'depends_on:\s*\[.*\b5000\b.*\]', content))


def is_meta(content: str) -> bool:
    """Check if issue is a meta milestone."""
    return bool(re.search(r'^type:\s*meta', content, re.MULTILINE))


def classify_by_body(body: str) -> tuple:
    """Returns (new_area, new_depends_on, label) based on body diagnostic patterns."""
    for pattern, new_area, new_depends, label in BODY_RULES:
        if re.search(pattern, body):
            return (new_area, new_depends, label)
    return None


def classify_by_title(title: str) -> tuple:
    """Returns (new_area, new_depends_on, label) based on title keywords."""
    for pattern, new_area, new_depends, label in TITLE_RULES:
        if re.search(pattern, title):
            return (new_area, new_depends, label)
    return None


def update_issue(filepath: str, new_area: str, new_depends: list) -> bool:
    """Update area and depends_on in issue frontmatter."""
    with open(filepath) as f:
        content = f.read()

    original = content

    # Update area
    content = re.sub(r'^area: .*$', f'area: {new_area}', content, flags=re.MULTILINE)
    # Update depends_on (replace the entire depends_on line)
    depends_str = "[" + ", ".join(str(d) for d in new_depends) + "]"
    content = re.sub(r'^depends_on: \[.*\]$', f'depends_on: {depends_str}', content, flags=re.MULTILINE)

    if content == original:
        return False  # No change

    with open(filepath, 'w') as f:
        f.write(content)
    return True


def main():
    # Counters
    body_reclassified = []
    title_reclassified = []
    confirmed_parser = []
    unchanged_unsorted = []

    errors = []

    for fname in sorted(os.listdir(ISSUES_DIR)):
        if not fname.endswith(".md"):
            continue
        fpath = os.path.join(ISSUES_DIR, fname)

        try:
            with open(fpath) as f:
                content = f.read()
        except Exception as e:
            errors.append((fname, f"read error: {e}"))
            continue

        # Only process issues that depend on [5000]
        if not depends_on_5000(content):
            continue

        # Skip meta milestones
        if is_meta(content):
            continue

        # Extract title
        title_match = re.search(r'^title: "(.*)"', content, re.MULTILINE)
        title = title_match.group(1) if title_match else fname

        body = content  # Full content including frontmatter

        # Step 1: Check body for diagnostic patterns (higher priority)
        body_result = classify_by_body(body)
        if body_result is not None:
            new_area, new_depends, label = body_result
            if new_depends == [5000]:
                # Stays as parser issue
                confirmed_parser.append((fname, title, label))
                continue
            try:
                changed = update_issue(fpath, new_area, new_depends)
                if changed:
                    body_reclassified.append((fname, title, new_area, new_depends, label))
                    print(f"BODY: {fname}")
                    print(f"  Title: {title}")
                    print(f"  -> area={new_area}, depends_on={new_depends} ({label})")
                else:
                    unchanged_unsorted.append((fname, title, new_area, new_depends, label, "body-no-change"))
            except Exception as e:
                errors.append((fname, f"update error (body): {e}"))
            continue

        # Step 2: Check title for keywords
        title_result = classify_by_title(title)
        if title_result is not None:
            new_area, new_depends, label = title_result
            if new_depends == [5000]:
                # Stays as parser issue
                confirmed_parser.append((fname, title, label))
                continue
            try:
                changed = update_issue(fpath, new_area, new_depends)
                if changed:
                    title_reclassified.append((fname, title, new_area, new_depends, label))
                    print(f"TITLE: {fname}")
                    print(f"  Title: {title}")
                    print(f"  -> area={new_area}, depends_on={new_depends} ({label})")
                else:
                    unchanged_unsorted.append((fname, title, new_area, new_depends, label, "title-no-change"))
            except Exception as e:
                errors.append((fname, f"update error (title): {e}"))
            continue

        # No match — still depends on [5000]
        unchanged_unsorted.append((fname, title, None, None, None, "no-match"))

    # Report
    total_processed = len(body_reclassified) + len(title_reclassified) + len(confirmed_parser) + len(unchanged_unsorted)

    print(f"\n{'='*60}")
    print(f"PHASE A.5 RETRIAGE RESULTS")
    print(f"{'='*60}")
    print(f"Total [5000] non-meta issues processed: {total_processed}")
    print()
    print(f"Body-diagnostic reclassified:    {len(body_reclassified)}")
    print(f"Title-keyword reclassified:       {len(title_reclassified)}")
    print(f"Confirmed as parser issue (kept): {len(confirmed_parser)}")
    print(f"Unchanged (no match):             {len(unchanged_unsorted)}")
    print()

    if body_reclassified:
        print(f"--- Body diagnostic reclassifications ({len(body_reclassified)}) ---")
        for fname, title, area, dep, label in body_reclassified:
            print(f"  {fname}")
            print(f"    Title: {title}")
            print(f"    -> area={area}, depends_on={dep} ({label})")
        print()

    if title_reclassified:
        print(f"--- Title keyword reclassifications ({len(title_reclassified)}) ---")
        for fname, title, area, dep, label in title_reclassified:
            print(f"  {fname}")
            print(f"    Title: {title}")
            print(f"    -> area={area}, depends_on={dep} ({label})")
        print()

    if confirmed_parser:
        print(f"--- Confirmed parser issues (no change needed) ({len(confirmed_parser)}) ---")
        for fname, title, label in confirmed_parser:
            print(f"  {fname}: {title} ({label})")
        print()

    if unchanged_unsorted:
        no_match_count = sum(1 for x in unchanged_unsorted if x[5] == "no-match")
        print(f"--- Unchanged ({len(unchanged_unsorted)}: {no_match_count} no-match, {len(unchanged_unsorted) - no_match_count} no-change) ---")

    if errors:
        print(f"--- ERRORS ({len(errors)}) ---")
        for fname, err in errors:
            print(f"  {fname}: {err}")

    # Summary for machine parsing
    print(f"\n{'='*60}")
    print(f"SUMMARY")
    print(f"{'='*60}")
    print(f"reclassified:            {len(body_reclassified) + len(title_reclassified)}")
    print(f"  body-diagnostic:       {len(body_reclassified)}")
    print(f"  title-keyword:         {len(title_reclassified)}")
    print(f"confirmed-parser:        {len(confirmed_parser)}")
    print(f"still-unclassified:      {no_match_count}")
    print(f"errors:                  {len(errors)}")

    if no_match_count > 0:
        print(f"\nRemaining unclassified [5000] issues: {no_match_count}")
        print("These need manual triage or Phase B automated triage.")


if __name__ == "__main__":
    main()
