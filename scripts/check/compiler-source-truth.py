#!/usr/bin/env python3
"""Check that compiler defaults do not silently make legacy backend source-of-truth.

P0/P0a coverage work may keep a legacy compatibility path, but the library
default must route through SpecKernel/correctness rather than treating
LoweredProgram/backend-wasm as the default owning path.

Usage:
  python scripts/check/compiler-source-truth.py
"""

import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
PIPELINE = REPO_ROOT / "crates/compiler/src/pipeline.rs"


def main() -> None:
    text = PIPELINE.read_text()
    violations: list[str] = []

    default_match = re.search(
        r"impl Default for BuildPipelineOptions \{(.*?)^\}",
        text,
        re.MULTILINE | re.DOTALL,
    )
    if not default_match:
        violations.append("cannot find BuildPipelineOptions default")
    elif "spec_kernel_mode: SpecKernelMode::Strict" not in default_match.group(1):
        violations.append(
            "BuildPipelineOptions::default must use SpecKernelMode::Strict; "
            "legacy backend cannot be the compiler source-of-truth default"
        )

    host_deny_match = re.search(
        r"pub fn build_file_with_host_deny\(.*?build_file_impl\((.*?)\n\s*\)",
        text,
        re.MULTILINE | re.DOTALL,
    )
    if not host_deny_match:
        violations.append("cannot find build_file_with_host_deny implementation")
    elif "BuildPipelineOptions::default()" not in host_deny_match.group(1):
        violations.append(
            "build_file_with_host_deny must inherit BuildPipelineOptions::default; "
            "do not reintroduce implicit legacy defaults"
        )

    if "if options.spec_kernel_mode != SpecKernelMode::Disabled" not in text:
        violations.append("spec_kernel_mode is not connected to build_file_impl dispatch")

    for violation in violations:
        print(f"compiler_source_truth: ERROR {violation}", file=sys.stderr)

    if violations:
        print(f"compiler_source_truth: FAILED ({len(violations)} errors)", file=sys.stderr)
        sys.exit(1)
    print("compiler_source_truth: OK", file=sys.stderr)


if __name__ == "__main__":
    main()
