#!/usr/bin/env python3
"""Stream G: Differential Test Reporter

Usage:
  python scripts/manager.py test-differential-reporter [--html FILE] [--markdown FILE]

Reads JSONL test records from stdin and generates:
- HTML report with summary table and failure details
- Markdown report with grouped results
"""

import sys
import json
import re
import tempfile
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()

def main():
    args = sys.argv[1:]
    
    html_file = "/tmp/test262-report.html"
    md_file = "/tmp/test262-report.md"
    
    i = 0
    while i < len(args):
        if args[i] == "--html":
            if i + 1 >= len(args):
                print("ERROR: --html requires a value", file=sys.stderr)
                sys.exit(1)
            html_file = args[i + 1]
            i += 2
        elif args[i] == "--markdown":
            if i + 1 >= len(args):
                print("ERROR: --markdown requires a value", file=sys.stderr)
                sys.exit(1)
            md_file = args[i + 1]
            i += 2
        else:
            print(f"Unknown option: {args[i]}", file=sys.stderr)
            sys.exit(1)
    
    pass_count = 0
    fail_count = 0
    unsupported_count = 0
    blocked_count = 0
    
    pass_details = []
    fail_details = []
    unsupported_details = []
    blocked_details = []
    
    category_pass = {}
    category_fail = {}
    category_unsupported = {}
    
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        
        try:
            data = json.loads(line)
        except json.JSONDecodeError:
            continue
        
        case_path = data.get("case", "")
        status = data.get("status", "")
        expected = data.get("expected", "")
        actual = data.get("actual", "")
        reason = data.get("reason", "")
        
        # Extract category from case path
        match = re.search(r'test/language/([^/]+)/', case_path)
        category = match.group(1) if match else "unknown"
        
        if status == "pass":
            pass_count += 1
            category_pass[category] = category_pass.get(category, 0) + 1
            pass_details.append(case_path)
        elif status == "fail":
            fail_count += 1
            category_fail[category] = category_fail.get(category, 0) + 1
            fail_details.append(f"{case_path} | Expected: {expected} | Actual: {actual}")
        elif status == "unsupported":
            unsupported_count += 1
            category_unsupported[category] = category_unsupported.get(category, 0) + 1
            unsupported_details.append(f"{case_path} | Reason: {reason}")
        elif status == "blocked":
            blocked_count += 1
            blocked_details.append(f"{case_path} | Condition: {reason}")
    
    total = pass_count + fail_count + unsupported_count + blocked_count
    if total == 0:
        total = 1
    pass_rate = (pass_count * 100) // total
    
    # Generate HTML report
    html_content = f"""<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Test262 Differential Report</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; }}
        h1, h2 {{ color: #333; }}
        table {{ border-collapse: collapse; width: 100%; margin: 20px 0; }}
        th, td {{ border: 1px solid #ddd; padding: 8px; text-align: left; }}
        th {{ background-color: #4CAF50; color: white; }}
        .pass {{ background-color: #d4edda; }}
        .fail {{ background-color: #f8d7da; }}
        .unsupported {{ background-color: #fff3cd; }}
        .summary {{ font-size: 14px; margin: 10px 0; }}
        .metric {{ display: inline-block; margin-right: 20px; }}
        pre {{ background-color: #f5f5f5; padding: 10px; overflow-x: auto; }}
    </style>
</head>
<body>
    <h1>Test262 Differential Test Report</h1>
    <div class="summary">
        <h2>Summary</h2>
        <div class="metric"><strong>Pass:</strong> {pass_count} ({pass_rate}%)</div>
        <div class="metric"><strong>Fail:</strong> {fail_count}</div>
        <div class="metric"><strong>Unsupported:</strong> {unsupported_count}</div>
        <div class="metric"><strong>Blocked:</strong> {blocked_count}</div>
        <div class="metric"><strong>Total:</strong> {total}</div>
    </div>

    <h2>Results by Category</h2>
    <table>
        <thead>
            <tr>
                <th>Category</th>
                <th class="pass">Pass</th>
                <th class="fail">Fail</th>
                <th class="unsupported">Unsupported</th>
                <th>Pass Rate</th>
            </tr>
        </thead>
        <tbody>
"""
    
    all_categories = set(category_pass.keys()) | set(category_fail.keys()) | set(category_unsupported.keys())
    for category in sorted(all_categories):
        cat_pass = category_pass.get(category, 0)
        cat_fail = category_fail.get(category, 0)
        cat_unsupported = category_unsupported.get(category, 0)
        cat_total = cat_pass + cat_fail + cat_unsupported
        if cat_total == 0:
            cat_total = 1
        cat_rate = (cat_pass * 100) // cat_total
        
        html_content += f"""            <tr>
                <td>{category}</td>
                <td class="pass">{cat_pass}</td>
                <td class="fail">{cat_fail}</td>
                <td class="unsupported">{cat_unsupported}</td>
                <td>{cat_rate}%</td>
            </tr>
"""
    
    html_content += """        </tbody>
    </table>

    <h2>Failures</h2>
    <details>
        <summary>Failed Tests (click to expand)</summary>
        <pre>
"""
    
    if fail_details:
        html_content += "\n".join(fail_details) + "\n"
    else:
        html_content += "No failures\n"
    
    html_content += """        </pre>
    </details>

    <h2>Unsupported Features</h2>
    <details>
        <summary>Unsupported Tests (click to expand)</summary>
        <pre>
"""
    
    if unsupported_details:
        html_content += "\n".join(unsupported_details) + "\n"
    else:
        html_content += "No unsupported features\n"
    
    html_content += """        </pre>
    </details>

</body>
</html>
"""
    
    with open(html_file, 'w') as f:
        f.write(html_content)
    
    # Generate Markdown report
    md_content = f"""# Test262 Differential Test Report

## Summary

| Metric | Count |
|--------|-------|
| Pass | {pass_count} ({pass_rate}%) |
| Fail | {fail_count} |
| Unsupported | {unsupported_count} |
| Blocked | {blocked_count} |
| **Total** | **{total}** |

## Results by Category

| Category | Pass | Fail | Unsupported | Pass Rate |
|----------|------|------|-------------|-----------|
"""
    
    for category in sorted(all_categories):
        cat_pass = category_pass.get(category, 0)
        cat_fail = category_fail.get(category, 0)
        cat_unsupported = category_unsupported.get(category, 0)
        cat_total = cat_pass + cat_fail + cat_unsupported
        if cat_total == 0:
            cat_total = 1
        cat_rate = (cat_pass * 100) // cat_total
        
        md_content += f"| {category} | {cat_pass} | {cat_fail} | {cat_unsupported} | {cat_rate}% |\n"
    
    md_content += """
## Failures

```
"""
    
    if fail_details:
        md_content += "\n".join(fail_details) + "\n"
    else:
        md_content += "No failures\n"
    
    md_content += """```

## Unsupported Features

```
"""
    
    if unsupported_details:
        md_content += "\n".join(unsupported_details) + "\n"
    else:
        md_content += "No unsupported features\n"
    
    md_content += "```\n"
    
    with open(md_file, 'w') as f:
        f.write(md_content)
    
    print(f"HTML report: {html_file}", file=sys.stderr)
    print(f"Markdown report: {md_file}", file=sys.stderr)

if __name__ == "__main__":
    main()
