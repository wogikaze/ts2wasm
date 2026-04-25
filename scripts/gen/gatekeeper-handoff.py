#!/usr/bin/env python3
"""
Generate gatekeeper handoff from current_task.json.
"""

import json
import sys
from datetime import datetime
from pathlib import Path

PROJECT_ROOT = Path(__file__).parent.parent.parent
CURRENT_TASK = PROJECT_ROOT / ".agents" / "state" / "current_task.json"


def generate_handoff() -> dict:
    """Generate gatekeeper handoff from current_task.json."""
    with open(CURRENT_TASK) as f:
        task = json.load(f)
    
    if task.get("status") == "idle" or not task.get("id"):
        return {"status": "idle", "message": "No active task"}
    
    handoff = {
        "task_id": task.get("id"),
        "title": task.get("title"),
        "status": task.get("status"),
        "issue_path": task.get("issue_path"),
        "scope": task.get("scope"),
        "acceptance": task.get("acceptance"),
        "commands": task.get("commands"),
        "risk": task.get("risk"),
        "notes": task.get("notes"),
        "generated_at": datetime.utcnow().isoformat() + "Z"
    }
    
    return handoff


def main():
    handoff = generate_handoff()
    print(json.dumps(handoff, indent=2))


if __name__ == "__main__":
    main()
