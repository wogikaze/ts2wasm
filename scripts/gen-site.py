#!/usr/bin/env python3
"""
Generate static site content for ts2wasm documentation and test explorer.
Extracts data from docs/, issues/, fixtures/, and coverage artifacts.
"""

# /// script
# requires-python = ">=3.8"
# dependencies = []
# ///

import os
import re
import json
import shutil
import subprocess
import sys
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Any

# Paths
PROJECT_ROOT = Path(__file__).parent.parent
SITE_DOCS = PROJECT_ROOT / "site" / "docs"
DOCS_DIR = PROJECT_ROOT / "docs"
ISSUES_DIR = PROJECT_ROOT / "issues"
FIXTURES_DIR = PROJECT_ROOT / "fixtures"
COVERAGE_DIR = PROJECT_ROOT / "artifacts" / "coverage"
REFERENCE_DIR = PROJECT_ROOT / "reference"
COVERAGE_WEB_UI_DIR = SITE_DOCS / "coverage" / "web-ui"
WEB_UI_DATA_DIR = COVERAGE_WEB_UI_DIR / "public" / "data"
SITE_DASHBOARD_DIST_INDEX = SITE_DOCS / "public" / "dashboard" / "index.html"
WEB_UI_DATA_SCRIPT = PROJECT_ROOT / "scripts" / "gen" / "web-ui-data.py"


def read_json_safe(path: Path) -> Dict[str, Any]:
    if not path.exists():
        return {}

    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return {}


def dashboard_available() -> bool:
    return SITE_DASHBOARD_DIST_INDEX.exists()


def detect_dashboard_url() -> str:
    if dashboard_available():
        return "/dashboard/"
    return ""


def to_int(value: Any, default: int = 0) -> int:
    try:
        return int(value)
    except (TypeError, ValueError):
        return default

def ensure_dir(path: Path):
    path.parent.mkdir(parents=True, exist_ok=True)

def copy_docs():
    """Copy and process documentation files."""
    docs_output = SITE_DOCS / "docs"
    ensure_dir(docs_output / "index.md")
    
    # Generate docs index
    index_content = """# Documentation

This section contains all project design documentation.

"""
    
    for doc_file in sorted(DOCS_DIR.glob("*.md")):
        if doc_file.name.startswith("00"):
            continue  # Skip index
            
        # Copy doc file
        shutil.copy(doc_file, docs_output / doc_file.name)
        
        # Add to index
        title = extract_title(doc_file)
        index_content += f"- [{title}](./{doc_file.name})\n"
    
    (docs_output / "index.md").write_text(index_content, encoding="utf-8")

def extract_title(file_path: Path) -> str:
    """Extract title from markdown file."""
    content = file_path.read_text(encoding="utf-8")
    match = re.search(r'^#\s+(.+)$', content, re.MULTILINE)
    if match:
        return match.group(1).strip()
    return file_path.stem

def process_issues():
    """Process issues and generate issue listing pages."""
    issues_output = SITE_DOCS / "issues"
    ensure_dir(issues_output / "index.md")
    
    # Parse issue index
    index_file = ISSUES_DIR / "index.md"
    index_content = index_file.read_text(encoding="utf-8")
    
    # Generate main issues page
    main_content = """# Issues

This section tracks all project issues and their status.

## Summary

"""
    
    # Extract summary table
    summary_match = re.search(r'<!-- generated:summary:start -->(.*?)<!-- generated:summary:end -->', 
                             index_content, re.DOTALL)
    if summary_match:
        main_content += summary_match.group(1) + "\n"
    
    # Extract ready queue
    ready_match = re.search(r'<!-- generated:ready:start -->(.*?)<!-- generated:ready:end -->', 
                           index_content, re.DOTALL)
    if ready_match:
        main_content += "\n## Ready Queue\n\n" + ready_match.group(1) + "\n"
    
    (issues_output / "index.md").write_text(main_content, encoding="utf-8")
    
    # Generate ready queue page
    if ready_match:
        ready_content = """# Ready Queue

Issues ready for implementation.

"""
        ready_content += ready_match.group(1) + "\n"
        (issues_output / "ready.md").write_text(ready_content, encoding="utf-8")
    
    # Generate done page
    done_content = """# Done Issues

Completed issues.

See [issues/index.md](./index.md) for the full list.
"""
    (issues_output / "done.md").write_text(done_content, encoding="utf-8")

def process_fixtures():
    """Process fixtures and generate test case listing pages."""
    fixtures_output = SITE_DOCS / "fixtures"
    ensure_dir(fixtures_output / "index.md")
    
    # Generate fixtures index
    index_content = """# Test Fixtures

This section contains all test fixtures organized by category.

## Categories

"""
    
    categories = sorted([d for d in FIXTURES_DIR.iterdir() if d.is_dir()])
    
    for category in categories:
        category_name = category.name.replace("-", " ").title()
        index_content += f"- [{category_name}](./{category.name}/)\n"
        
        # Generate category page
        category_output = fixtures_output / category.name
        ensure_dir(category_output / "index.md")
        
        category_content = f"""# {category_name}

Test fixtures in this category.

"""
        
        # List all test files in category
        test_files = sorted(category.glob("*.ts")) + sorted(category.glob("*.js"))
        
        for test_file in test_files:
            test_name = test_file.stem
            category_content += f"- [{test_name}](./{test_file}.html)\n"
            
            # Generate individual test page with code
            test_content = f"""# {test_name}

```typescript
{test_file.read_text(encoding="utf-8")}
```

**Path:** `{test_file.relative_to(PROJECT_ROOT)}`
"""
            ensure_dir(category_output / f"{test_name}.md")
            (category_output / f"{test_name}.md").write_text(test_content, encoding="utf-8")
        
        (category_output / "index.md").write_text(category_content, encoding="utf-8")
    
    (fixtures_output / "index.md").write_text(index_content, encoding="utf-8")

def process_coverage():
    """Process coverage data and generate coverage pages."""
    coverage_output = SITE_DOCS / "coverage"
    ensure_dir(coverage_output / "index.md")
    web_ui_coverage = read_json_safe(WEB_UI_DATA_DIR / "coverage.json")
    web_ui_tests = read_json_safe(WEB_UI_DATA_DIR / "test-results.json")
    web_ui_suites = web_ui_coverage.get("suites", [])
    suite_summary = {suite.get("suite"): suite for suite in web_ui_suites if suite.get("suite")}
    dashboard_url = detect_dashboard_url()
    has_dashboard = dashboard_available()
    summary = web_ui_tests.get("summary", {})
    shown_records = to_int(web_ui_tests.get("metadata", {}).get("shown_records", 0))

    total = to_int(web_ui_coverage.get("total", 0))
    implemented = to_int(web_ui_coverage.get("implemented", 0))
    unimplemented = to_int(web_ui_coverage.get("unimplemented", 0))
    future = to_int(web_ui_coverage.get("future", 0))
    supported_total = implemented + unimplemented + future
    priority = web_ui_coverage.get("byPriority", {})

    implemented_pct = (implemented / supported_total * 100) if supported_total else 0
    by_web_ui = bool(web_ui_coverage)

    content = "# Test Coverage\n\n"
    content += "This section mirrors the ts2wasm web UI dashboard data.\n\n"
    content += "## Coverage Data\n\n"
    content += "- Source files: `coverage/web-ui/public/data/coverage.json` and `coverage/web-ui/public/data/test-results.json`\n"
    content += "- Generated by `mise run coverage-dashboard-data`\n"

    if by_web_ui:
        if has_dashboard:
            content += "\n## Coverage Dashboard\n\n"
            content += f"- Dashboard: [{dashboard_url}]({dashboard_url})\n"
            content += "\nOpen this page directly (no iframe) for the interactive dashboard.\n"
        else:
            content += "\nNo coverage dashboard bundle is shipped under `site/docs/coverage/web-ui/`. JSON-based summary is still available in the generated output above.\n"
        content += "## Coverage Snapshot\n\n"
        content += f"- Total: {total}\n"
        content += f"- Implemented: {implemented} ({implemented_pct:.2f}%)\n"
        content += f"- Unimplemented: {unimplemented}\n"
        content += f"- Future: {future}\n"
        content += f"- P0: {to_int(priority.get('p0', 0))}\n"
        content += f"- P1: {to_int(priority.get('p1', 0))}\n"
        content += f"- P2: {to_int(priority.get('p2', 0))}\n"
        content += f"- P3: {to_int(priority.get('p3', 0))}\n"
        content += f"- Future (priority): {to_int(priority.get('future', 0))}\n\n"
        content += "### Latest test summary\n\n"
        content += f"- Passed: {to_int(summary.get('passed', 0))}\n"
        content += f"- Unsupported: {to_int(summary.get('unsupported', 0))}\n"
        content += f"- Build error: {to_int(summary.get('build_error', 0))}\n"
        content += f"- Runtime error: {to_int(summary.get('runtime_error', 0))}\n"
        content += f"- Mismatch: {to_int(summary.get('mismatch', 0))}\n"
        content += f"- Blocked: {to_int(summary.get('blocked', 0))}\n"
        content += f"- Shown records: {shown_records}\n"
    else:
        content += "\nNo dashboard data found. Run `mise run coverage-dashboard-data` first.\n\n"

    content += "\n## Reference Test Suites\n\n"
    if REFERENCE_DIR.exists():
        suites = sorted([d for d in REFERENCE_DIR.iterdir() if d.is_dir()])
        for suite in suites:
            suite_name = suite.name
            content += f"- [{suite_name}](./{suite_name}/)\n"

            suite_output = coverage_output / suite_name
            ensure_dir(suite_output / "index.md")

            suite_data = suite_summary.get(suite_name, {})
            suite_content = f"# {suite_name} Coverage\n\n"
            if has_dashboard:
                suite_content += f"Web UI dashboard path: [/dashboard/](/dashboard/)\n\n"
            else:
                suite_content += "Coverage dashboard: not bundled in this checkout.\n\n"
            if suite_data:
                suite_content += "## Latest suite snapshot\n\n"
                suite_content += f"- Executed: {to_int(suite_data.get('executed', 0))}\n"
                suite_content += f"- Denominator: {to_int(suite_data.get('denominator', 0))}\n"
                suite_content += f"- Build pass: {to_int(suite_data.get('build_pass', 0))}\n"
                suite_content += f"- Semantic pass: {to_int(suite_data.get('semantic_pass', 0))}\n"
                suite_content += f"- Unsupported: {to_int(suite_data.get('unsupported', 0))}\n"
                suite_content += f"- Failed: {to_int(suite_data.get('fail', 0))}\n"
                suite_content += f"- Blocked: {to_int(suite_data.get('blocked', 0))}\n"
                suite_content += f"- Source: {suite_data.get('source', 'n/a')}\n"
            else:
                suite_content += "\nNo coverage suite snapshot available.\n"

            suite_content += "\nSee details in the interactive dashboard table.\n"
            suite_output.joinpath("index.md").write_text(suite_content, encoding="utf-8")

    # Check for coverage matrix
    coverage_matrix = COVERAGE_DIR / "reference-coverage-matrix.md"
    if coverage_matrix.exists():
        content += "\n## Coverage Matrix\n\n"
        content += coverage_matrix.read_text(encoding="utf-8")
    
    # Add last updated timestamp
    content += f"\n---\n\n*Last updated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}*\n"
    
    (coverage_output / "index.md").write_text(content, encoding="utf-8")

def refresh_coverage_dashboard_data():
    """Refresh dashboard JSON inside site/docs after the output tree is reset."""
    subprocess.check_call([sys.executable, str(WEB_UI_DATA_SCRIPT)], cwd=PROJECT_ROOT)

def generate_home():
    """Generate home page with dashboard."""
    home_content = """# ts2wasm

TypeScript to WebAssembly compiler - Documentation and Test Explorer

## Quick Links

- [Documentation](./docs/) - Design documentation
- [Issues](./issues/) - Issue tracker and ready queue
- [Fixtures](./fixtures/) - Test fixtures browser
- [Coverage](./coverage/) - Test coverage results
- [Coverage dashboard](/dashboard/) - Interactive dashboard UI

## Project Statistics

"""
    
    # Add statistics
    doc_count = len(list(DOCS_DIR.glob("*.md")))
    issue_count = len(list(ISSUES_DIR.glob("open/*.md"))) + len(list(ISSUES_DIR.glob("done/*.md")))
    fixture_count = len(list(FIXTURES_DIR.rglob("*.ts"))) + len(list(FIXTURES_DIR.rglob("*.js")))
    
    home_content += f"- **Documentation Files:** {doc_count}\n"
    home_content += f"- **Total Issues:** {issue_count}\n"
    home_content += f"- **Test Fixtures:** {fixture_count}\n"
    
    # Add recent activity
    home_content += "\n## Recent Activity\n\n"
    home_content += "Run `mise run gen-site` to update this site after running tests.\n"
    
    (SITE_DOCS / "index.md").write_text(home_content, encoding="utf-8")

def generate_dashboard_redirect():
    """Generate a docs route that redirects to the static dashboard bundle."""
    dashboard_dir = SITE_DOCS / "dashboard"
    ensure_dir(dashboard_dir / "index.md")
    content = """# Coverage Dashboard

<script>
(function() {
  if (typeof window === 'undefined') return;
  const pathname = window.location.pathname;
  const dashboardStart = pathname.indexOf('/dashboard');
  const rootPrefix = dashboardStart === -1 ? '' : pathname.slice(0, dashboardStart);
  const normalizedPrefix = rootPrefix.endsWith('/') ? rootPrefix.slice(0, -1) : rootPrefix;
  const target = `${normalizedPrefix}/dashboard/index.html`;
  if (window.location.pathname !== target) {
    window.location.replace(target);
  }
})();
</script>

<noscript>
  Redirect failed because JavaScript is disabled.
  <a href="/dashboard/index.html">Open Coverage Dashboard</a>.
</noscript>
"""
    (dashboard_dir / "index.md").write_text(content, encoding="utf-8")

def main():
    """Main generation function."""
    print("Generating site content...")
    
    # Clean output directory (preserve public/ for static assets like dashboard bundle)
    if SITE_DOCS.exists():
        for item in list(SITE_DOCS.iterdir()):
            if item.name == "public":
                continue
            if item.is_dir():
                shutil.rmtree(item)
            else:
                item.unlink()
    else:
        SITE_DOCS.mkdir(parents=True, exist_ok=True)

    refresh_coverage_dashboard_data()
    
    # Generate content
    generate_home()
    generate_dashboard_redirect()
    copy_docs()
    process_issues()
    process_fixtures()
    process_coverage()
    
    print(f"Site content generated in {SITE_DOCS}")
    print("Run 'cd site && npm run build' to build the site")

if __name__ == "__main__":
    main()
