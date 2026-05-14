#!/usr/bin/env python3
"""TypeScript boundary manifest checker.

Validates that the TypeScript syntax boundary manifest
(docs/language-reference/typescript-boundary.yaml) is consistent with
available fixtures, diagnostic codes, and tracking.

Usage:
  python scripts/check/typescript-boundary.py --self-test
  python scripts/check/typescript-boundary.py --check
  python scripts/check/typescript-boundary.py --verify-fixtures
"""

import os
import sys
import yaml
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
MANIFEST_PATH = REPO_ROOT / "docs" / "language-reference" / "typescript-boundary.yaml"

VALID_MODES = {"erase", "reject", "runtime", "declaration_emit_deferred", "deferred"}
VALID_OWNERS = {"frontend", "ir+runtime", "frontend+runtime"}
# Features in 'deferred' or 'declaration_emit_deferred' mode MUST have tracking.
TRACKING_REQUIRED_MODES = {"deferred", "declaration_emit_deferred"}
# Features in 'reject' mode MUST have a diagnostic_code.
DIAG_REQUIRED_MODES = {"reject"}


def load_manifest():
    if not MANIFEST_PATH.exists():
        print(f"ERROR: manifest not found at {MANIFEST_PATH}", file=sys.stderr)
        sys.exit(1)
    with open(MANIFEST_PATH) as f:
        return yaml.safe_load(f)


def check_manifest() -> list[str]:
    """Validate manifest structure and consistency."""
    errors = []
    manifest = load_manifest()

    if not isinstance(manifest, dict):
        errors.append("manifest must be a dict")
        return errors

    schema_version = manifest.get("schema_version")
    if schema_version != 1:
        errors.append(f"schema_version must be 1, got {schema_version}")

    features = manifest.get("features", [])
    if not isinstance(features, list):
        errors.append("features must be a list")
        return errors

    if len(features) == 0:
        errors.append("features list is empty")

    for i, feat in enumerate(features):
        if not isinstance(feat, dict):
            errors.append(f"features[{i}]: must be a dict")
            continue

        feature_name = feat.get("feature", f"<unnamed #{i}>")
        mode = feat.get("mode")
        owner = feat.get("owner")
        diagnostic_code = feat.get("diagnostic_code")
        tracking = feat.get("tracking")

        if not feature_name or not isinstance(feature_name, str):
            errors.append(f"features[{i}]: missing or invalid 'feature'")

        if mode not in VALID_MODES:
            errors.append(f"features[{i}]: '{feature_name}' has invalid mode '{mode}'")

        if owner not in VALID_OWNERS:
            errors.append(
                f"features[{i}]: '{feature_name}' has invalid owner '{owner}'"
            )

        if mode in TRACKING_REQUIRED_MODES and not tracking:
            errors.append(
                f"features[{i}]: '{feature_name}' in mode '{mode}' must have tracking"
            )

        if mode in DIAG_REQUIRED_MODES and not diagnostic_code:
            errors.append(
                f"features[{i}]: '{feature_name}' in mode '{mode}' must have diagnostic_code"
            )

    return errors


def verify_fixtures() -> list[str]:
    """Check that referenced fixture files exist."""
    errors = []
    manifest = load_manifest()
    features = manifest.get("features", [])

    for feat in features:
        fixture = feat.get("fixture")
        if fixture:
            fixture_path = REPO_ROOT / fixture
            if not fixture_path.exists():
                errors.append(
                    f"fixture not found: {fixture} (referenced by '{feat.get('feature')}')"
                )

    return errors


def self_test():
    """Run self-tests for the checker logic."""
    passed = 0
    failed = 0

    # Test: valid manifest loads
    try:
        manifest = load_manifest()
        assert isinstance(manifest, dict)
        assert manifest.get("schema_version") == 1
        passed += 1
    except Exception as e:
        print(f"FAIL: load_manifest: {e}", file=sys.stderr)
        failed += 1

    # Test: check_manifest produces no errors on current manifest
    errors = check_manifest()
    if errors:
        print("FAIL: check_manifest produced errors:", file=sys.stderr)
        for e in errors:
            print(f"  {e}", file=sys.stderr)
        failed += 1
    else:
        passed += 1

    # Test: verify_fixtures produces no errors
    errors = verify_fixtures()
    fixture_errors = [e for e in errors if "fixture not found" in e]
    if fixture_errors:
        print("FAIL: verify_fixtures produced errors:", file=sys.stderr)
        for e in fixture_errors:
            print(f"  {e}", file=sys.stderr)
        failed += 1
    else:
        passed += 1

    print(f"self-test: {passed} passed, {failed} failed")
    return failed == 0


def main():
    args = sys.argv[1:]

    if not args or "--help" in args or "-h" in args:
        print(__doc__)
        sys.exit(0)

    if "--self-test" in args:
        if self_test():
            sys.exit(0)
        sys.exit(1)

    if "--check" in args:
        errors = check_manifest()
        if errors:
            for e in errors:
                print(f"typescript-boundary: ERROR: {e}", file=sys.stderr)
            sys.exit(1)
        print("typescript-boundary: OK")
        sys.exit(0)

    if "--verify-fixtures" in args:
        errors = verify_fixtures()
        if errors:
            for e in errors:
                print(f"typescript-boundary: ERROR: {e}", file=sys.stderr)
            sys.exit(1)
        print("typescript-boundary: OK (all fixtures found)")
        sys.exit(0)

    print(f"unknown option: {args[0]}", file=sys.stderr)
    sys.exit(1)


if __name__ == "__main__":
    main()
