#!/usr/bin/env python3
"""Break down test262 unsupported categories into per-method/per-feature counts.

Usage:
  # From cached detail output
  mise run reference-coverage -- test262 --limit 2000 --detail 2>&1 | tee /tmp/t262-detail.txt
  python3 scripts/analyze/test262-method-breakdown.py /tmp/t262-detail.txt

  # Direct from reference-coverage (slower)
  mise run reference-coverage -- test262 --limit 5000 --detail \\
    | python3 scripts/analyze/test262-method-breakdown.py
"""

import re
import sys
from collections import defaultdict

def extract_feature(path: str) -> str:
    """Extract the feature name from a test262 path."""
    m = re.search(r'(?:reference/)?test262/test/', path)
    if not m:
        return path.split("/")[-1]
    
    rest = path[m.end():]
    parts = rest.split("/")
    
    # built-ins/Array/prototype/map/something.js -> Array.prototype.map
    if parts[0] == "built-ins":
        if len(parts) >= 4 and parts[2] == "prototype":
            return f"{parts[1]}.prototype.{parts[3]}"
        base = parts[1]
        if len(parts) >= 3 and parts[2] not in ("prototype",):
            base += "." + parts[2]
        return base
    
    # language/statements/for-of -> statements/for-of
    if parts[0] == "language":
        return "/".join(parts[1:3]) if len(parts) >= 3 else (parts[1] if len(parts) > 1 else parts[0])
    
    # annexB/... -> strip annexB prefix for cleaner grouping
    if parts[0] in ("annexB",):
        rest_annex = "/".join(parts[1:])
        # annexB/built-ins/String/prototype/anchor -> String.prototype.anchor
        if len(parts) >= 4 and parts[2] == "built-ins":
            inner = parts[3:]
            if len(inner) >= 3 and inner[1] == "prototype":
                return f"A.{inner[0]}.prototype.{inner[2]}"
            return "A." + ".".join(inner[:2]) if len(inner) >= 2 else "A." + inner[0]
        # annexB/language/eval-code/direct/... -> eval-code
        if len(parts) >= 3:
            return "/".join(parts[1:3])
        return rest_annex
    
    # intl402/..., harness/... -> first component
    if parts[0] in ("intl402", "harness"):
        return parts[0]
    
    return "/".join(parts[:min(len(parts), 2)])


def main():
    lines = sys.stdin.read().splitlines() if not sys.argv[1:] else open(sys.argv[1]).read().splitlines()
    
    # Parse detail output
    categories = defaultdict(lambda: defaultdict(int))
    total_by_cat = defaultdict(int)
    
    for line in lines:
        # Format: reference/test262/test/.../file.js: <status>: <category>
        m = re.match(r'^(.*?\.(?:js|mjs)):\s+(\S+):\s+(.*)$', line)
        if not m:
            continue
        
        path = m.group(1)
        status = m.group(2)
        category = m.group(3).strip()
        
        feature = extract_feature(path)
        categories[category][feature] += 1
        total_by_cat[category] += 1
    
    if not categories:
        print("No data parsed. Check input format.")
        sys.exit(1)
    
    total = sum(total_by_cat.values())
    print(f"Total classified: {total}\n")
    
    # Sort categories by count descending
    for cat in sorted(total_by_cat, key=lambda c: total_by_cat[c], reverse=True):
        items = categories[cat]
        cat_total = total_by_cat[cat]
        pct = cat_total / total * 100
        
        print(f"{'─'*66}")
        print(f"  {cat}  ({cat_total}, {pct:.1f}%)")
        print(f"{'─'*66}")
        
        # Top features in this category
        for feat, cnt in sorted(items.items(), key=lambda x: -x[1])[:20]:
            bar = "█" * min(cnt // 50, 40)
            print(f"    {feat:<40} {cnt:>6} {bar}")
        
        remaining = cat_total - sum(c for _, c in sorted(items.items(), key=lambda x: -x[1])[:20])
        if remaining > 0:
            print(f"    ... and {remaining} more")
        
        # Per-item breakdown for small categories
        if len(items) <= 20:
            for feat, cnt in sorted(items.items(), key=lambda x: -x[1]):
                print(f"    {feat:<40} {cnt:>6}")
        
        print()
    
    # ROI analysis: top quick-win opportunities
    print(f"{'='*66}")
    print("  ROI Analysis: Quick Wins (high count, narrow scope)")
    print(f"{'='*66}")
    candidates = []
    for cat in categories:
        for feat, cnt in categories[cat].items():
            # Focus on specific builtin methods (not broad categories)
            if ".prototype." in feat or feat.startswith("Array.") or feat.startswith("String.") or feat.startswith("Math.") or feat.startswith("Number.") or feat.startswith("Date.") or feat.startswith("RegExp.") or feat.startswith("JSON.") or feat.startswith("Map") or feat.startswith("Set"):
                candidates.append((cnt, cat, feat))
    
    for cnt, cat, feat in sorted(candidates, reverse=True)[:30]:
        print(f"  +{cnt:<6} {feat:<40} ({cat})")


if __name__ == "__main__":
    main()
