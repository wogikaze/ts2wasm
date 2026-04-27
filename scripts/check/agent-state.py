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


def validate_json_file(errors: list[str], path: Path) -> Any | None:
    if not path.exists():
        errors.append(f"{path.relative_to(PROJECT_ROOT)} is missing")
        return None
    try:
        return load_json(path)
    except json.JSONDecodeError as e:
        errors.append(f"{path.relative_to(PROJECT_ROOT)} invalid JSON: {e}")
        return None


def validate_test_report(errors: list[str], report_path: Path, schema_path: Path) -> None:
    if not report_path.exists():
        errors.append(f"{report_path.relative_to(PROJECT_ROOT)} referenced by last_run.json does not exist")
        return
    if schema_path.exists() and not validate_json_schema(report_path, schema_path):
        errors.append(f"{report_path.relative_to(PROJECT_ROOT)} schema validation failed")


def main():
    errors = []

    # Validate current_task.json
    current_task = STATE_DIR / "current_task.json"
    current_task_schema = SCHEMAS_DIR / "current_task.schema.json"
    current_task_data = validate_json_file(errors, current_task)
    if current_task.exists() and current_task_schema.exists():
        if not validate_json_schema(current_task, current_task_schema):
            errors.append(f"current_task.json schema validation failed")

    project_state = STATE_DIR / "project_state.json"
    project_state_data = validate_json_file(errors, project_state)

    last_run = STATE_DIR / "last_run.json"
    last_run_data = validate_json_file(errors, last_run)

    if isinstance(current_task_data, dict):
        issue_path = current_task_data.get("issue_path")
        if issue_path is not None and not (PROJECT_ROOT / issue_path).exists():
            errors.append(f"current_task.json issue_path does not exist: {issue_path}")

        status = current_task_data.get("status")
        if status == "idle":
            for field in ["issue_path", "scope", "acceptance", "commands", "risk"]:
                if current_task_data.get(field) is not None:
                    errors.append(f"current_task.json status idle requires {field} to be null")

    if isinstance(project_state_data, dict):
        for field in ["fsm", "active_task_id", "updated_at"]:
            if field not in project_state_data:
                errors.append(f"project_state.json missing required field: {field}")

        active_task_id = project_state_data.get("active_task_id")
        current_task_id = current_task_data.get("id") if isinstance(current_task_data, dict) else None
        if active_task_id is not None and active_task_id != current_task_id:
            errors.append("project_state.json active_task_id does not match current_task.json id")

    test_report_schema = SCHEMAS_DIR / "test_report.schema.json"
    if isinstance(last_run_data, dict):
        for field in ["run_id", "ended_at", "task_id", "summary", "test_report_path"]:
            if field not in last_run_data:
                errors.append(f"last_run.json missing required field: {field}")
        report_ref = last_run_data.get("test_report_path")
        if isinstance(report_ref, str):
            validate_test_report(errors, PROJECT_ROOT / report_ref, test_report_schema)

    # Validate test_report.jsonl if present
    test_report = STATE_DIR / "test_report.jsonl"
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
