#!/usr/bin/env python3
"""
Validate .agents/state/ files against their schemas and invariants.
"""

import json
import sys
from pathlib import Path
from typing import Any

PROJECT_ROOT = Path(__file__).parent.parent.parent
STATE_DIR = PROJECT_ROOT / ".agents" / "state"
SCHEMAS_DIR = STATE_DIR / "schemas"

try:
    import jsonschema
    JsonSchemaValidationError = jsonschema.ValidationError
except ImportError:  # pragma: no cover - exercised in minimal environments
    print("ERROR: jsonschema is required for check-agent-state", file=sys.stderr)
    print("Install with: pip install jsonschema", file=sys.stderr)
    sys.exit(1)


def load_json(path: Path) -> Any:
    with open(path) as f:
        return json.load(f)


def validate_json_schema(filepath: Path, schema_path: Path) -> bool:
    """Validate a JSON file against its schema."""
    data = load_json(filepath)
    schema = load_json(schema_path)

    try:
        jsonschema.validate(instance=data, schema=schema)
        return True
    except JsonSchemaValidationError as e:
        print(f"ERROR: {filepath} does not match schema: {e.message}", file=sys.stderr)
        return False


def main():
    errors = []
    
    # Validate current_task.json
    current_task = STATE_DIR / "current_task.json"
    current_task_schema = SCHEMAS_DIR / "current_task.schema.json"
    if current_task.exists() and current_task_schema.exists():
        if not validate_json_schema(current_task, current_task_schema):
            errors.append(f"current_task.json schema validation failed")
    
    # Validate test_report.jsonl if present
    test_report = STATE_DIR / "test_report.jsonl"
    test_report_schema = SCHEMAS_DIR / "test_report.schema.json"
    if test_report.exists() and test_report_schema.exists():
        schema = load_json(test_report_schema)
        with open(test_report) as f:
            for line_num, line in enumerate(f, 1):
                if not line.strip():
                    continue
                try:
                    data = json.loads(line)
                    jsonschema.validate(instance=data, schema=schema)
                except json.JSONDecodeError as e:
                    errors.append(f"test_report.jsonl:{line_num} invalid JSON: {e}")
                except JsonSchemaValidationError as e:
                    errors.append(f"test_report.jsonl:{line_num} schema validation failed: {e.message}")
    
    if errors:
        for error in errors:
            print(f"ERROR: {error}", file=sys.stderr)
        sys.exit(1)
    
    print("OK: agent state files validated")
    sys.exit(0)


if __name__ == "__main__":
    main()
