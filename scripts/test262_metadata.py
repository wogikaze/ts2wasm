#!/usr/bin/env python3
"""Parse test262 YAML frontmatter to extract includes list.

Usage:
  python scripts/test262_metadata.py <filepath>
  python scripts/test262_metadata.py --self-test
"""

import json
import re
import sys


def _parse_yaml_list(value):
    value = value.split("#", 1)[0].strip()
    if not value:
        return []
    if value.startswith("[") and value.endswith("]"):
        inner = value[1:-1].strip()
        if not inner:
            return []
        return [part.strip().strip("'\"") for part in inner.split(",") if part.strip()]
    return [value.strip().strip("'\"")]


def parse_test262_metadata(filepath):
    """Parse test262 YAML frontmatter from a file.

    Args:
        filepath: Path to a test262 test file.

    Returns:
        dict with keys:
            - includes (list of str): harness file names to inject
            - negative (dict or None): {"phase": str, "type": str} if negative test
    """
    with open(filepath, "r", encoding="utf-8") as f:
        source_code = f.read()

    match = re.search(r'/\*---(.*?)---\*/', source_code, re.DOTALL)
    if not match:
        return {"includes": [], "negative": None}

    includes = []
    negative = None
    in_negative = False

    for raw_line in match.group(1).splitlines():
        stripped = raw_line.strip()
        if not stripped or stripped.startswith("#"):
            continue

        if not raw_line.startswith((" ", "\t")):
            in_negative = False

        if ":" not in stripped:
            continue

        key, value = stripped.split(":", 1)
        key = key.strip()
        value = value.strip()

        if key == "includes":
            includes = _parse_yaml_list(value)
        elif key == "negative":
            in_negative = True
            negative = {}
        elif in_negative and key == "phase":
            if negative is not None:
                negative["phase"] = value.strip("'\"")
        elif in_negative and key == "type":
            if negative is not None:
                negative["type"] = value.strip("'\"")
        elif key == "include":
            includes.extend(_parse_yaml_list(value))

    return {"includes": includes, "negative": negative}


def main():
    if len(sys.argv) < 2 or sys.argv[1] in ("-h", "--help"):
        print("Usage: python scripts/test262_metadata.py <filepath>")
        print("       python scripts/test262_metadata.py --self-test")
        sys.exit(0)

    if sys.argv[1] == "--self-test":
        result = parse_test262_metadata(__file__)
        print(json.dumps(result, indent=2))
        sys.exit(0)

    result = parse_test262_metadata(sys.argv[1])
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
