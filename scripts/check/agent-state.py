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
    jsonschema = None

    class JsonSchemaValidationError(Exception):
        pass


def load_json(path: Path) -> Any:
    with open(path) as f:
        return json.load(f)


def json_type_name(value: Any) -> str:
    if value is None:
        return "null"
    if isinstance(value, bool):
        return "boolean"
    if isinstance(value, dict):
        return "object"
    if isinstance(value, list):
        return "array"
    if isinstance(value, str):
        return "string"
    if isinstance(value, int):
        return "integer"
    if isinstance(value, float):
        return "number"
    return type(value).__name__


def matches_type(value: Any, expected: str) -> bool:
    if expected == "null":
        return value is None
    if expected == "boolean":
        return isinstance(value, bool)
    if expected == "object":
        return isinstance(value, dict)
    if expected == "array":
        return isinstance(value, list)
    if expected == "string":
        return isinstance(value, str)
    if expected == "integer":
        return isinstance(value, int) and not isinstance(value, bool)
    if expected == "number":
        return (isinstance(value, int) or isinstance(value, float)) and not isinstance(value, bool)
    return True


def basic_schema_errors(data: Any, schema: dict[str, Any], path: str = "$") -> list[str]:
    """Validate the JSON Schema subset used by .agents/state/schemas."""
    errors: list[str] = []

    expected_type = schema.get("type")
    if expected_type is not None:
        expected_types = expected_type if isinstance(expected_type, list) else [expected_type]
        if not any(matches_type(data, t) for t in expected_types):
            expected = " or ".join(expected_types)
            return [f"{path}: expected {expected}, got {json_type_name(data)}"]

    if "enum" in schema and data not in schema["enum"]:
        errors.append(f"{path}: value {data!r} is not one of {schema['enum']!r}")

    if isinstance(data, str) and "minLength" in schema and len(data) < int(schema["minLength"]):
        errors.append(f"{path}: string is shorter than minLength {schema['minLength']}")

    if (isinstance(data, int) or isinstance(data, float)) and not isinstance(data, bool) and "minimum" in schema:
        if data < schema["minimum"]:
            errors.append(f"{path}: value {data!r} is below minimum {schema['minimum']}")

    if isinstance(data, dict):
        required = schema.get("required", [])
        for key in required:
            if key not in data:
                errors.append(f"{path}: missing required property {key!r}")

        properties = schema.get("properties", {})
        for key, value in data.items():
            child_path = f"{path}.{key}"
            if key in properties:
                errors.extend(basic_schema_errors(value, properties[key], child_path))
            elif schema.get("additionalProperties") is False:
                errors.append(f"{child_path}: additional property is not allowed")

    if isinstance(data, list) and "items" in schema:
        for idx, item in enumerate(data):
            errors.extend(basic_schema_errors(item, schema["items"], f"{path}[{idx}]"))

    return errors


def validate_json_schema(filepath: Path, schema_path: Path) -> bool:
    """Validate a JSON file against its schema."""
    data = load_json(filepath)
    schema = load_json(schema_path)

    if jsonschema is not None:
        try:
            jsonschema.validate(instance=data, schema=schema)
            return True
        except JsonSchemaValidationError as e:
            print(f"ERROR: {filepath} does not match schema: {e.message}", file=sys.stderr)
            return False

    errors = basic_schema_errors(data, schema)
    if errors:
        for error in errors:
            print(f"ERROR: {filepath} does not match schema: {error}", file=sys.stderr)
        return False
    return True


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
                    if jsonschema is not None:
                        jsonschema.validate(instance=data, schema=schema)
                    else:
                        schema_errors = basic_schema_errors(data, schema)
                        for error in schema_errors:
                            errors.append(f"test_report.jsonl:{line_num} schema validation failed: {error}")
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
