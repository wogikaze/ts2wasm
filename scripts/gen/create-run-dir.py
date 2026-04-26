#!/usr/bin/env python3
"""Create a new run directory for cycle reports.

Usage: python scripts/manager.py create-run-dir [run_id]

Creates reports/runs/<run_id>/ directory with cycle_report.md template.
If run_id is not provided, uses current timestamp (YYYYMMDD-HHMMSS).
"""

import sys
from datetime import datetime
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
REPORTS_DIR = REPO_ROOT / "reports" / "runs"


def create_run_dir(run_id: str | None = None) -> Path:
    """Create a new run directory and return its path."""
    if run_id is None:
        run_id = datetime.now().strftime("%Y%m%d-%H%M%S")
    
    run_dir = REPORTS_DIR / run_id
    run_dir.mkdir(parents=True, exist_ok=True)
    
    # Create cycle report template
    cycle_report = run_dir / "cycle_report.md"
    if not cycle_report.exists():
        cycle_report.write_text(f"""# Cycle Report: {run_id}

## Summary

- Start time: {datetime.now().isoformat()}
- End time: TBD
- Issue: TBD
- Status: In progress

## Tasks Completed

- [ ] Task 1
- [ ] Task 2

## Issues Encountered

None

## Next Steps

- [ ] Next step 1
- [ ] Next step 2

## Notes

Add any additional notes here.
""")
    
    return run_dir


def main():
    args = sys.argv[1:]
    run_id = args[0] if args else None
    
    run_dir = create_run_dir(run_id)
    print(f"Created run directory: {run_dir.relative_to(REPO_ROOT)}")
    print(f"Cycle report: {run_dir / 'cycle_report.md'}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
