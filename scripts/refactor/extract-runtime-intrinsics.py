#!/usr/bin/env python3
"""Generate RuntimeIntrinsic enum from RuntimeFn spec files.

Outputs:
1. RuntimeIntrinsic enum definition (for crates/ir/src/lowered/)
2. name() method mapping variant -> runtime_fn string

Deduplicates variants since the RuntimeFn spec may have duplicate entries.
"""

import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
SPEC_ALL = REPO_ROOT / "crates" / "backend-wasm" / "src" / "runtime" / "spec" / "all.rs"

def extract_spec_arm_names(text):
    """Extract Self::Name from match arms, preserving order but deduplicating."""
    seen = set()
    names = []
    for m in re.finditer(r'Self::(\w+)', text):
        name = m.group(1)
        if name not in seen:
            seen.add(name)
            names.append(name)
    return names


def main():
    text = SPEC_ALL.read_text()
    all_variants = extract_spec_arm_names(text)
    print(f"Total unique RuntimeFn variants: {len(all_variants)}", file=sys.stderr)

    # Generate enum
    lines = []
    lines.append("/// Strongly-typed intrinsic identifier for runtime function calls.")
    lines.append("///")
    lines.append("/// This replaces the previous `RuntimeCall { runtime_fn: String }` pattern.")
    lines.append("/// Each variant corresponds to a RuntimeFn in the backend-wasm catalog.")
    lines.append("/// Auto-generated from backend-wasm spec files.")
    lines.append("///")
    lines.append("/// Pseudo-intrinsics (no corresponding RuntimeFn):")
    lines.append("/// - ArrayPushMany: Expanded during IR lowering into ArrayPushGrow + ArrayPush")
    lines.append("/// - HeapClosureCall: Direct heap closure calling convention")
    lines.append("/// - PrivateFieldGet/Set/BrandCheck: Class private field operations")
    lines.append("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]")
    lines.append("pub enum RuntimeIntrinsic {")
    for v in all_variants:
        lines.append(f"    {v},")
    lines.append("    /// Pseudo: expanded into ArrayPushGrow + ArrayPush during IR lowering")
    lines.append("    ArrayPushMany,")
    lines.append("    /// Pseudo: direct heap closure calling convention")
    lines.append("    HeapClosureCall,")
    lines.append("    /// Pseudo: class private field get")
    lines.append("    PrivateFieldGet,")
    lines.append("    /// Pseudo: class private field set")
    lines.append("    PrivateFieldSet,")
    lines.append("    /// Pseudo: class private brand check")
    lines.append("    PrivateBrandCheck,")
    lines.append("}")
    lines.append("")

    # Generate name() method
    lines.append("impl RuntimeIntrinsic {")
    lines.append("    /// Returns the RuntimeFn name string for backend dispatch.")
    lines.append("    pub const fn name(self) -> &'static str {")
    lines.append("        match self {")
    for v in all_variants:
        lines.append(f"            RuntimeIntrinsic::{v} => \"{v}\",")
    lines.append(f"            RuntimeIntrinsic::ArrayPushMany => \"ArrayPushMany\",")
    lines.append(f"            RuntimeIntrinsic::HeapClosureCall => \"HeapClosureCall\",")
    lines.append(f"            RuntimeIntrinsic::PrivateFieldGet => \"PrivateFieldGet\",")
    lines.append(f"            RuntimeIntrinsic::PrivateFieldSet => \"PrivateFieldSet\",")
    lines.append(f"            RuntimeIntrinsic::PrivateBrandCheck => \"PrivateBrandCheck\",")
    lines.append("        }")
    lines.append("    }")
    lines.append("}")

    print("\n".join(lines))


if __name__ == "__main__":
    main()
