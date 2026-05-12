#!/usr/bin/env python3
"""Update program_builtins.rs helper functions to return RuntimeIntrinsic."""

import re
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
PATH = REPO_ROOT / "crates/ir/src/lowered/program_builtins.rs"

text = PATH.read_text()
original = text

# 1. Change resolve_method_to_runtime_fn return type
text = text.replace(
    "pub(super) fn resolve_method_to_runtime_fn(object: &ResolvedExpr, method: &str) -> Option<String> {",
    "pub(super) fn resolve_method_to_runtime_fn(object: &ResolvedExpr, method: &str) -> Option<RuntimeIntrinsic> {"
)

# Replace all Some("Xxx") with Some(RuntimeIntrinsic::Xxx) in this function
# Match any Some("Word") pattern
text = re.sub(
    r'Some\("(\w+)"\)',
    r'Some(RuntimeIntrinsic::\1)',
    text
)

# 2. Change collection_method_runtime_fn return type
text = text.replace(
    "pub(super) fn collection_method_runtime_fn(class_name: &str, method: &str) -> Option<&'static str> {",
    "pub(super) fn collection_method_runtime_fn(class_name: &str, method: &str) -> Option<RuntimeIntrinsic> {"
)

# Replace Some("Xxx"), with Some(RuntimeIntrinsic::Xxx),
# Being careful to avoid the &'static str case
text = re.sub(
    r'Some\("(\w+)"\)',
    r'Some(RuntimeIntrinsic::\1)',
    text
)

# 3. Change collection_method_runtime_fn_arg return type
text = text.replace(
    "pub(super) fn collection_method_runtime_fn_arg(method: &str) -> Option<&'static str> {",
    "pub(super) fn collection_method_runtime_fn_arg(method: &str) -> Option<RuntimeIntrinsic> {"
)

# Fix .to_owned() calls on RuntimeIntrinsic values at the calling sites — actually let's
# also fix up the calling conventions. When these return RuntimeIntrinsic, callers
# don't need .to_owned() anymore.

if text != original:
    PATH.write_text(text)
    print(f"Updated {PATH.relative_to(REPO_ROOT)}")
else:
    print("No changes needed")
