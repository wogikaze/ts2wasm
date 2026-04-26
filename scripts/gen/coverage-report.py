#!/usr/bin/env python3
"""Generate coverage report from language-reference markdown tables
Usage: python scripts/manager.py coverage-report [--format text|markdown]
"""

import sys
import re
from pathlib import Path
from datetime import datetime

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()

def extract_table(file_path, section):
    """Extract table data from markdown for a given section."""
    with open(file_path) as f:
        lines = f.readlines()
    
    in_section = False
    in_table = False
    table_lines = []
    
    for line in lines:
        if line.strip() == f"## {section}":
            in_section = True
            continue
        if in_section and line.startswith("## "):
            in_section = False
            continue
        if in_section and line.startswith("|"):
            in_table = True
            table_lines.append(line)
        elif in_table and line.strip() == "":
            in_table = False
    
    return table_lines

def count_status(file_path, section):
    """Count implementation status for a section."""
    table_lines = extract_table(file_path, section)
    
    total = 0
    implemented = 0
    unimplemented = 0
    future = 0
    
    for line in table_lines[1:]:  # Skip header
        if not line.strip():
            continue
        parts = [p.strip() for p in line.split("|")]
        if len(parts) < 5:
            continue
        
        status = parts[4]
        if status == "実装済み" or "実装済み" in status:
            implemented += 1
        elif status == "未実装" or "未実装" in status:
            unimplemented += 1
        elif status == "将来対応" or status == "将来検討" or "将来" in status:
            future += 1
        
        total += 1
    
    return total, implemented, unimplemented, future

def count_priority(file_path, section):
    """Count by priority for a section."""
    table_lines = extract_table(file_path, section)
    
    total = 0
    p0 = 0
    p1 = 0
    p2 = 0
    p3 = 0
    future = 0
    
    for line in table_lines[1:]:  # Skip header
        if not line.strip():
            continue
        parts = [p.strip() for p in line.split("|")]
        if len(parts) < 6:
            continue
        
        priority = parts[5]
        if priority == "P0":
            p0 += 1
        elif priority == "P1":
            p1 += 1
        elif priority == "P2":
            p2 += 1
        elif priority == "P3":
            p3 += 1
        elif priority == "将来検討":
            future += 1
        elif priority == "-":
            pass  # ignore
        else:
            pass  # unknown priority
        
        total += 1
    
    return total, p0, p1, p2, p3, future

def generate_markdown_report():
    """Generate report in markdown format."""
    print("# Language Coverage Report")
    print()
    print(f"Generated: {datetime.utcnow().strftime('%Y-%m-%d %H:%M:%S UTC')}")
    print()
    
    lang_ref_dir = REPO_ROOT / "docs/language-reference"
    for file_path in sorted(lang_ref_dir.glob("*.md")):
        basename = file_path.stem
        print(f"## {basename}")
        print()
        
        # Extract sections
        with open(file_path) as f:
            content = f.read()
        
        sections = re.findall(r'^## (.+)$', content, re.MULTILINE)
        
        skip_sections = {"仕様リファレンス", "仕様詳細", "実装方針の原則", "Capability Mapping"}
        
        for section in sections:
            if section in skip_sections:
                continue
            
            total, implemented, unimplemented, future = count_status(file_path, section)
            if total == 0:
                continue
            
            print(f"### {section}")
            print()
            print("| Total | Implemented | Unimplemented | Future |")
            print("|-------|-------------|---------------|--------|")
            print(f"| {total} | {implemented} | {unimplemented} | {future} |")
            print()
            
            total, p0, p1, p2, p3, future = count_priority(file_path, section)
            print("| Priority | P0 | P1 | P2 | P3 | Future |")
            print("|----------|----|----|----|----|--------|")
            print(f"| Count | {p0} | {p1} | {p2} | {p3} | {future} |")
            print()

def generate_text_report():
    """Generate report in text format."""
    print(f"{'File':<30} {'Section':<20} {'Total':<12} {'Impl':<12} {'Unimpl':<12} {'Future':<8}")
    print("-" * 100)
    
    lang_ref_dir = REPO_ROOT / "docs/language-reference"
    for file_path in sorted(lang_ref_dir.glob("*.md")):
        basename = file_path.stem
        
        with open(file_path) as f:
            content = f.read()
        
        sections = re.findall(r'^## (.+)$', content, re.MULTILINE)
        
        skip_sections = {"仕様リファレンス", "仕様詳細", "実装方針の原則", "Capability Mapping"}
        
        for section in sections:
            if section in skip_sections:
                continue
            
            total, implemented, unimplemented, future = count_status(file_path, section)
            if total == 0:
                continue
            
            print(f"{basename:<30} {section:<20} {total:<12} {implemented:<12} {unimplemented:<12} {future:<8}")
    
    print()
    print("Priority breakdown (unimplemented only):")
    print(f"{'File':<30} {'Section':<20} {'P0':<4} {'P1':<4} {'P2':<4} {'P3':<4} {'Future':<8}")
    print("-" * 100)
    
    for file_path in sorted(lang_ref_dir.glob("*.md")):
        basename = file_path.stem
        
        with open(file_path) as f:
            content = f.read()
        
        sections = re.findall(r'^## (.+)$', content, re.MULTILINE)
        
        skip_sections = {"仕様リファレンス", "仕様詳細", "実装方針の原則", "Capability Mapping"}
        
        for section in sections:
            if section in skip_sections:
                continue
            
            total, p0, p1, p2, p3, future = count_priority(file_path, section)
            if (p0 + p1 + p2 + p3 + future) == 0:
                continue
            
            print(f"{basename:<30} {section:<20} {p0:<4} {p1:<4} {p2:<4} {p3:<4} {future:<8}")

def main():
    args = sys.argv[1:]
    
    format_type = "text"
    i = 0
    while i < len(args):
        if args[i] == "--":
            i += 1
            break
        elif args[i] == "--format":
            if i + 1 >= len(args):
                print("error: --format requires text or markdown", file=sys.stderr)
                sys.exit(1)
            format_type = args[i + 1]
            i += 2
        elif args[i] in ("text", "markdown"):
            format_type = args[i]
            i += 1
        elif args[i] in ("-h", "--help"):
            print("Generate coverage report from language-reference markdown tables")
            print("Usage: python scripts/manager.py coverage-report [--format text|markdown]")
            sys.exit(0)
        else:
            print(f"unknown option: {args[i]}", file=sys.stderr)
            print("Usage: python scripts/manager.py coverage-report [--format text|markdown]", file=sys.stderr)
            sys.exit(1)
    
    if format_type not in ("text", "markdown"):
        print(f"unknown format: {format_type}", file=sys.stderr)
        print("Usage: python scripts/manager.py coverage-report [--format text|markdown]", file=sys.stderr)
        sys.exit(1)
    
    if format_type == "markdown":
        generate_markdown_report()
    else:
        generate_text_report()

if __name__ == "__main__":
    main()
