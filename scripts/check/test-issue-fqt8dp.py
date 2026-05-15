#!/usr/bin/env python3
"""Test: RUNTIME_ABI constants exist (I-20260515-FQT8DP).

RED test — expected to fail until RUNTIME_ABI_NAME and RUNTIME_ABI_VERSION
constants are defined in crates/runtime-abi/src/consts.rs.

Remove #[ignore] and this header when implementation is complete.
"""

import os, sys, subprocess, re

CRATE_SRC = "crates/runtime-abi/src/consts.rs"

if not os.path.exists(CRATE_SRC):
    print(f"SKIP: {CRATE_SRC} not found", file=sys.stderr)
    sys.exit(0)

with open(CRATE_SRC) as f:
    content = f.read()

errors = []

# Check RUNTIME_ABI_NAME
if "RUNTIME_ABI_NAME" not in content:
    errors.append("RUNTIME_ABI_NAME constant not found in consts.rs")
if "RUNTIME_ABI_VERSION" not in content and "ABI_VERSION" not in content:
    errors.append("RUNTIME_ABI_VERSION/ABI_VERSION constant not found in consts.rs")
if "pub const ABI_VERSION: u32" not in content:
    errors.append("ABI_VERSION: u32 not found (step 1 of FQT8DP)")

# Check manifest emission
manifest_files = [
    "crates/compiler/src/manifest.rs",
    "crates/shared/src/manifest.rs",
]
manifest_str = ""
for mf in manifest_files:
    if os.path.exists(mf):
        with open(mf) as f:
            manifest_str += f.read()

if "runtime_abi_name" not in manifest_str:
    errors.append("manifest field 'runtime_abi_name' not found")
if "target_id" not in manifest_str:
    errors.append("manifest field 'target_id' not found")

if errors:
    print(f"FAIL: {len(errors)} missing requirement(s)")
    for e in errors:
        print(f"  - {e}")
    sys.exit(1)
else:
    print("PASS: All RUNTIME_ABI requirements met")
    sys.exit(0)
