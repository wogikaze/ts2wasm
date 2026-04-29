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
    
    content = """# Test Coverage

This section shows test coverage results across different test suites.

## Reference Test Suites

"""
    
    # List reference test suites
    if REFERENCE_DIR.exists():
        suites = sorted([d for d in REFERENCE_DIR.iterdir() if d.is_dir()])
        for suite in suites:
            suite_name = suite.name
            content += f"- [{suite_name}](./{suite_name}/)\n"
            
            # Generate suite page
            suite_output = coverage_output / suite_name
            ensure_dir(suite_output / "index.md")
            
            suite_content = f"""# {suite_name} Test Suite

"""
            
            # Check if test results exist
            test_dir = suite / "test"
            if test_dir.exists():
                test_count = len(list(test_dir.rglob("*.js")) + list(test_dir.rglob("*.ts")))
                suite_content += f"Total test files: {test_count}\n\n"
            
            # Check for coverage results
            results_dir = COVERAGE_DIR / "results"
            if results_dir.exists():
                # Try new format first (summary.json with jsonl_file)
                summary_file = results_dir / f"{suite_name}-summary.json"
                # Fall back to old format (test262.json)
                if not summary_file.exists():
                    summary_file = results_dir / f"{suite_name}.json"
                
                if summary_file.exists():
                    suite_content += f"## Latest Results\n\n"
                    try:
                        results = json.loads(summary_file.read_text(encoding="utf-8"))
                        suite_content += f"- Passed: {results.get('passed', 0)}\n"
                        suite_content += f"- Failed: {results.get('failed', 0)}\n"
                        suite_content += f"- Unsupported: {results.get('unsupported', 0)}\n"
                        suite_content += f"- Blocked: {results.get('blocked', 0)}\n"
                        suite_content += f"- Total: {results.get('total', 0)}\n"
                        if 'timestamp' in results:
                            suite_content += f"- Last run: {results['timestamp']}\n"
                        
                        # Process JSONL results if available
                        jsonl_file = results.get('jsonl_file')
                        if jsonl_file and Path(jsonl_file).exists():
                            jsonl_path = Path(jsonl_file)
                            suite_content += f"\n## Detailed Results\n\n"
                            
                            # Parse JSONL and categorize results
                            failed_tests = []
                            unsupported_tests = []
                            
                            with open(jsonl_path, 'r', encoding='utf-8') as f:
                                for line in f:
                                    try:
                                        record = json.loads(line.strip())
                                        if record.get('status') == 'fail':
                                            failed_tests.append(record)
                                        elif record.get('status') == 'unsupported':
                                            unsupported_tests.append(record)
                                    except:
                                        pass
                            
                            # Add failed tests section
                            if failed_tests:
                                suite_content += f"### Failed Tests ({len(failed_tests)})\n\n"
                                for i, test in enumerate(failed_tests[:50]):  # Limit to first 50
                                    case_path = test.get('case', 'unknown')
                                    reason = test.get('reason', 'No reason')
                                    if reason:
                                        # Unescape the reason
                                        reason = reason.replace('\\n', '\n').replace('\\t', '\t').replace('\\"', '"').replace('\\\\', '\\')
                                    actual = test.get('actual', '')
                                    if actual:
                                        # Unescape the actual
                                        actual = actual.replace('\\n', '\n').replace('\\t', '\t').replace('\\"', '"').replace('\\\\', '\\')
                                    expected = test.get('expected', '')
                                    if expected:
                                        # Unescape the expected
                                        expected = expected.replace('\\n', '\n').replace('\\t', '\t').replace('\\"', '"').replace('\\\\', '\\')
                                    error_line = test.get('error_line')
                                    stderr = test.get('stderr', '')
                                    if stderr:
                                        # Unescape the stderr
                                        stderr = stderr.replace('\\n', '\n').replace('\\t', '\t').replace('\\"', '"').replace('\\\\', '\\')
                                    
                                    # Get source code from JSONL or file
                                    test_source = test.get('source_code', '')
                                    if test_source:
                                        # Unescape the source code
                                        test_source = test_source.replace('\\n', '\n').replace('\\t', '\t').replace('\\"', '"').replace('\\\\', '\\')
                                    
                                    if not test_source:
                                        test_file_path = REFERENCE_DIR / suite_name / case_path
                                        if test_file_path.exists():
                                            try:
                                                test_source = test_file_path.read_text(encoding="utf-8")
                                            except:
                                                pass
                                    
                                    # Create individual test page
                                    test_id = case_path.replace('/', '-').replace('.', '-')
                                    test_page = suite_output / f"failed-{test_id}.md"
                                    
                                    # Add line number to source code if available
                                    source_with_lines = ""
                                    if test_source and error_line:
                                        lines = test_source.split('\n')
                                        for idx, line in enumerate(lines, 1):
                                            marker = ">>> " if idx == error_line else "    "
                                            source_with_lines += f"{marker}{idx}: {line}\n"
                                    else:
                                        source_with_lines = test_source
                                    
                                    test_content = f"""# Failed Test: {case_path}

**Status:** Failed  
**Reason:** {reason}
"""
                                    if error_line:
                                        test_content += f"**Error Line:** {error_line}\n"
                                    
                                    test_content += f"""
## Test Source Code

```javascript
{source_with_lines}
```

"""
                                    if stderr:
                                        test_content += f"""## Compiler/Runtime Error

```
{stderr}
```

"""
                                    
                                    if expected:
                                        test_content += f"""## Expected Output

```
{expected}
```

"""
                                    
                                    if actual:
                                        test_content += f"""## Actual Output

```
{actual}
```

"""
                                    
                                    test_content += f"""**Path:** `{case_path}`
"""
                                    test_page.parent.mkdir(parents=True, exist_ok=True)
                                    test_page.write_text(test_content, encoding="utf-8")
                                    
                                    line_info = f" (line {error_line})" if error_line else ""
                                    suite_content += f"- [{case_path}](./failed-{test_id}.html) - {reason}{line_info}\n"
                                
                                if len(failed_tests) > 50:
                                    suite_content += f"\n*... and {len(failed_tests) - 50} more failed tests*\n"
                                suite_content += "\n"
                            
                            # Add unsupported tests section
                            if unsupported_tests:
                                suite_content += f"### Unsupported Tests ({len(unsupported_tests)})\n\n"
                                for i, test in enumerate(unsupported_tests[:50]):  # Limit to first 50
                                    case_path = test.get('case', 'unknown')
                                    reason = test.get('reason', 'No reason')
                                    if reason:
                                        # Unescape the reason
                                        reason = reason.replace('\\n', '\n').replace('\\t', '\t').replace('\\"', '"').replace('\\\\', '\\')
                                    error_line = test.get('error_line')
                                    stderr = test.get('stderr', '')
                                    if stderr:
                                        # Unescape the stderr
                                        stderr = stderr.replace('\\n', '\n').replace('\\t', '\t').replace('\\"', '"').replace('\\\\', '\\')
                                    
                                    # Get source code from JSONL or file
                                    test_source = test.get('source_code', '')
                                    if test_source:
                                        # Unescape the source code
                                        test_source = test_source.replace('\\n', '\n').replace('\\t', '\t').replace('\\"', '"').replace('\\\\', '\\')
                                    
                                    if not test_source:
                                        test_file_path = REFERENCE_DIR / suite_name / case_path
                                        if test_file_path.exists():
                                            try:
                                                test_source = test_file_path.read_text(encoding="utf-8")
                                            except:
                                                pass
                                    
                                    # Create individual test page for unsupported tests
                                    test_id = case_path.replace('/', '-').replace('.', '-')
                                    test_page = suite_output / f"unsupported-{test_id}.md"
                                    
                                    # Add line number to source code if available
                                    source_with_lines = ""
                                    if test_source and error_line:
                                        lines = test_source.split('\n')
                                        for idx, line in enumerate(lines, 1):
                                            marker = ">>> " if idx == error_line else "    "
                                            source_with_lines += f"{marker}{idx}: {line}\n"
                                    else:
                                        source_with_lines = test_source
                                    
                                    test_content = f"""# Unsupported Test: {case_path}

**Status:** Unsupported  
**Reason:** {reason}
"""
                                    if error_line:
                                        test_content += f"**Error Line:** {error_line}\n"
                                    
                                    test_content += f"""
## Test Source Code

```javascript
{source_with_lines}
```

"""
                                    if stderr:
                                        test_content += f"""## Compiler Error

```
{stderr}
```

"""
                                    
                                    test_content += f"""**Path:** `{case_path}`
"""
                                    test_page.parent.mkdir(parents=True, exist_ok=True)
                                    test_page.write_text(test_content, encoding="utf-8")
                                    
                                    line_info = f" (line {error_line})" if error_line else ""
                                    suite_content += f"- [{case_path}](./unsupported-{test_id}.html) - {reason}{line_info}\n"
                                
                                if len(unsupported_tests) > 50:
                                    suite_content += f"\n*... and {len(unsupported_tests) - 50} more unsupported tests*\n"
                                suite_content += "\n"
                    
                    except Exception as e:
                        suite_content += f"Error loading results: {e}\n"
            
            (suite_output / "index.md").write_text(suite_content, encoding="utf-8")
    
    # Check for coverage matrix
    coverage_matrix = COVERAGE_DIR / "reference-coverage-matrix.md"
    if coverage_matrix.exists():
        content += "\n## Coverage Matrix\n\n"
        content += coverage_matrix.read_text(encoding="utf-8")
    
    # Add last updated timestamp
    content += f"\n---\n\n*Last updated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}*\n"
    
    (coverage_output / "index.md").write_text(content, encoding="utf-8")

def generate_home():
    """Generate home page with dashboard."""
    home_content = """# ts2wasm

TypeScript to WebAssembly compiler - Documentation and Test Explorer

## Quick Links

- [Documentation](./docs/) - Design documentation
- [Issues](./issues/) - Issue tracker and ready queue
- [Fixtures](./fixtures/) - Test fixtures browser
- [Coverage](./coverage/) - Test coverage results

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

def main():
    """Main generation function."""
    print("Generating site content...")
    
    # Clean output directory
    if SITE_DOCS.exists():
        shutil.rmtree(SITE_DOCS)
    SITE_DOCS.mkdir(parents=True, exist_ok=True)
    
    # Generate content
    generate_home()
    copy_docs()
    process_issues()
    process_fixtures()
    process_coverage()
    
    print(f"Site content generated in {SITE_DOCS}")
    print("Run 'cd site && npm run build' to build the site")

if __name__ == "__main__":
    main()