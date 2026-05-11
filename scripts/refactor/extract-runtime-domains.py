#!/usr/bin/env python3
"""Extract RuntimeFn domain files from runtime_fn_impl.rs.

Splits spec() and manifest_name() match arms into domain-specific files
under crates/backend-wasm/src/runtime/spec/.

Usage: python3 scripts/refactor/extract-runtime-domains.py
"""

import re
import sys
from pathlib import Path
from collections import OrderedDict

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
IMPL_PATH = REPO_ROOT / "crates" / "backend-wasm" / "src" / "runtime_fn_impl.rs"
SPEC_DIR = REPO_ROOT / "crates" / "backend-wasm" / "src" / "runtime" / "spec"
MANIFEST_DIR = REPO_ROOT / "crates" / "backend-wasm" / "src" / "runtime" / "manifest"

# Classify a RuntimeFn variant name into a domain
def classify(name):
    # Host variants (Fs, Process, Path, Crypto, Host prefixes)
    if name.startswith("Host") or name.startswith("Fs") or name.startswith("Process") \
        or name.startswith("Path") or name.startswith("Crypto"):
        return "Host"
    # BigInt prefixed
    if name.startswith("BigInt") or name == "MakeBigIntLiteral":
        return "BigInt"
    # Array prefixed
    if name.startswith("Array"):
        return "Array"
    # Object prefixed
    if name.startswith("Object"):
        return "Object"
    # Object property helpers
    if name in ("PropertyGet", "PropertySet", "PropertyDelete", "PropertyHas", "SpreadViaIterator"):
        return "Object"
    # String prefixed
    if name.startswith("String") or name in ("StringEqual",):
        return "String"
    # Date prefixed
    if name.startswith("Date"):
        return "Date"
    # Math prefixed
    if name.startswith("Math"):
        return "Math"
    # RegExp / Regexp prefixed
    if name.startswith("RegExp") or name.startswith("Regexp"):
        return "RegExp"
    # Promise prefixed
    if name.startswith("Promise"):
        return "Promise"
    # Task prefixed
    if name.startswith("Task"):
        return "Task"
    # Symbol prefixed
    if name.startswith("Symbol"):
        return "Symbol"
    # Map/Set/Weak prefixed
    if name.startswith("Map") or name.startswith("Set") or name.startswith("Weak"):
        return "MapSet"
    # TypedArray/ArrayBuffer/DataView prefixed
    if name.startswith("TypedArray") or name.startswith("ArrayBuffer") \
        or name.startswith("DataView"):
        return "TypedArray"
    # Module prefixed
    if name.startswith("Module"):
        return "Module"
    # Encoding (URI/escape-related)
    if name in ("EncodeURI", "DecodeURI", "Escape", "Unescape"):
        return "Encoding"
    # Concat is String
    if name == "Concat":
        return "String"
    # Iterator
    if name in ("GetIterator", "IteratorNext"):
        return "Iterator"
    # Operators
    if name in ("Add", "AddFast", "Sub", "SubFast", "Mul", "MulFast",
        "Div", "DivFast", "Mod", "ModFast", "Negate",
        "BitwiseToI32", "BitwiseAnd", "BitwiseXor", "BitwiseOr",
        "Less", "LessFast", "LessEqual", "LessEqualFast",
        "Greater", "GreaterFast", "GreaterEqual", "GreaterEqualFast",
        "StrictEqual", "EqualEqual", "BangEqual", "StrictNotEqual",
        "And", "Or"):
        return "Operator"
    # Type/Coercion
    if name in ("TruthyBool", "Not", "TypeOf", "IsString", "ValueOf",
        "InstanceOf", "BooleanCoerce", "NumberCoerce",
        "IsNaN", "ParseInt", "ParseFloat", "IsFinite"):
        return "TypeCoercion"
    # Number methods
    if name.startswith("Number"):
        return "Number"
    # JSON
    if name.startswith("Json"):
        return "Json"
    # Core (everything else)
    if name in ("ReadStdinBytes", "Write", "Copy", "ValueToStringInto",
        "ErrorMessage", "Log", "AllocHeap", "MemEqual", "Index",
        "GetLength", "PrivateBrandTypeError"):
        return "Core"
    return "Misc"


def find_brace_block(text, start):
    """Starting from text[start] that should be '{', find matching '}'."""
    assert text[start] == '{', f"Expected '{{' at {start}, got {repr(text[start])}"
    depth = 0
    i = start
    while i < len(text):
        if text[i] == '{':
            depth += 1
        elif text[i] == '}':
            depth -= 1
            if depth == 0:
                return i
        i += 1
    return -1


def extract_spec_arms(text):
    """Extract Self::Name => RuntimeSpec { ... }, arms."""
    # Find spec() function
    idx = text.find("pub(crate) const fn spec(self) -> RuntimeSpec {")
    if idx < 0:
        print("ERROR: spec() not found", file=sys.stderr)
        sys.exit(1)

    # Find match body
    midx = text.find("match self {", idx)
    if midx < 0:
        print("ERROR: match self not found in spec()", file=sys.stderr)
        sys.exit(1)

    brace = text.find("{", midx)
    close = find_brace_block(text, brace)

    body = text[brace + 1 : close]
    return _extract_arms(body, "RuntimeSpec")


def extract_manifest_arms(text):
    """Extract Self::Name => \"...\", arms."""
    idx = text.find("pub(crate) fn manifest_name(self)")
    if idx < 0:
        print("ERROR: manifest_name() not found", file=sys.stderr)
        sys.exit(1)

    midx = text.find("match self {", idx)
    if midx < 0:
        print("ERROR: match self not found in manifest_name()", file=sys.stderr)
        sys.exit(1)

    brace = text.find("{", midx)
    close = find_brace_block(text, brace)

    body = text[brace + 1 : close]
    return _extract_manifest_arms(body)


def _extract_arms(body, struct_name):
    """Extract Self::Name => StructName { ... }, match arms from body text."""
    arms = OrderedDict()
    i = 0
    while i < len(body):
        # Skip whitespace/newlines
        m = re.search(r"\S", body[i:])
        if not m:
            break
        i += m.start()

        # Try to match Self::Name
        m = re.match(r"Self::(\w+)", body[i:])
        if not m:
            i += 1
            continue

        name = m.group(1)
        arm_start = i

        # Find =>
        fat_arrow = body.find("=>", i)
        if fat_arrow < 0:
            break

        # Find struct name after =>
        struct_start = body.find(struct_name, fat_arrow)
        if struct_start < 0:
            i = fat_arrow + 2
            continue

        # Find opening brace of struct
        brace = body.find("{", struct_start)
        if brace < 0:
            break

        close = find_brace_block(body, brace)
        # Arm ends after the `,` following the closing brace
        arm_end = close + 1
        while arm_end < len(body) and body[arm_end] in ' ,\n\r':
            if body[arm_end] == ',':
                arm_end += 1
                break
            arm_end += 1

        arm_text = body[arm_start:arm_end]
        arms[name] = (classify(name), arm_text)
        i = arm_end

    return arms


def _extract_manifest_arms(body):
    """Extract Self::Name => \"value\", arms from manifest_name body."""
    arms = OrderedDict()
    i = 0
    while i < len(body):
        m = re.search(r"Self::(\w+)\s*=>\s*\"([^\"]+)\"\s*,?\s*", body[i:])
        if not m:
            break
        name = m.group(1)
        arm_text = m.group(0).strip()
        arms[name] = (classify(name), arm_text)
        i += m.end()
    return arms


def write_domain_files(by_domain):
    """Write one file per domain with spec *or* manifest arms."""
    SPEC_DIR.mkdir(parents=True, exist_ok=True)
    MANIFEST_DIR.mkdir(parents=True, exist_ok=True)

    # Collect all arms for combined files (index 1 = indentation level)
    all_spec_arms = []  # (domain, arm_text)
    all_manifest_arms = []  # (domain, arm_text)

    for domain in sorted(by_domain):
        data = by_domain[domain]
        spec_arms = data.get("spec", [])
        manifest_arms = data.get("manifest", [])

        for arm in spec_arms:
            all_spec_arms.append((domain, arm))
        for arm in manifest_arms:
            all_manifest_arms.append((domain, arm))

        # Write per-domain spec file
        spec_path = SPEC_DIR / f"{domain.lower()}.rs"
        with open(spec_path, "w") as f:
            f.write(f"// Domain: {domain} -- auto-generated.\n")
            f.write("// Regenerate with: python3 scripts/refactor/extract-runtime-domains.py\n\n")
            for arm in spec_arms:
                f.write(f"            {arm}\n")
        print(f"  spec/{domain.lower()}.rs: {len(spec_arms)} arms")

        # Write per-domain manifest file
        manifest_path = MANIFEST_DIR / f"{domain.lower()}.rs"
        with open(manifest_path, "w") as f:
            f.write(f"// Domain: {domain} -- auto-generated.\n")
            f.write("// Regenerate with: python3 scripts/refactor/extract-runtime-domains.py\n\n")
            for arm in manifest_arms:
                if not arm.rstrip().endswith(','):
                    arm += ','
                f.write(f"            {arm}\n")
        print(f"  manifest/{domain.lower()}.rs: {len(manifest_arms)} arms")

    # Write combined spec file (full match block for single include!() call)
    combined_spec_path = SPEC_DIR / "all.rs"
    with open(combined_spec_path, "w") as f:
        f.write("// Combined spec arms -- auto-generated.\n")
        f.write("// Regenerate with: python3 scripts/refactor/extract-runtime-domains.py\n\n")
        f.write("match self {\n")
        for domain, arm in all_spec_arms:
            f.write(f"            {arm}\n")
        f.write("        }\n")
    print(f"  spec/all.rs: {len(all_spec_arms)} arms combined")

    # Write combined manifest file (full match block)
    combined_manifest_path = MANIFEST_DIR / "all.rs"
    with open(combined_manifest_path, "w") as f:
        f.write("// Combined manifest arms -- auto-generated.\n")
        f.write("// Regenerate with: python3 scripts/refactor/extract-runtime-domains.py\n\n")
        f.write("match self {\n")
        for domain, arm in all_manifest_arms:
            f.write(f"            {arm}\n")
        f.write("        }\n")
    print(f"  manifest/all.rs: {len(all_manifest_arms)} arms combined")

    print(f"\nWrote {len(by_domain)} domain files to {SPEC_DIR} and {MANIFEST_DIR}")
    print(f"Also wrote combined all.rs files with include!()-compatible format")


def main():
    text = IMPL_PATH.read_text()

    spec_arms = extract_spec_arms(text)
    manifest_arms = extract_manifest_arms(text)

    print(f"Extracted {len(spec_arms)} spec arms, {len(manifest_arms)} manifest arms")

    # Group by domain
    by_domain = {}
    for name, (domain, arm_text) in spec_arms.items():
        by_domain.setdefault(domain, {"spec": [], "manifest": []})["spec"].append(arm_text)

    for name, (domain, arm_text) in manifest_arms.items():
        by_domain.setdefault(domain, {"spec": [], "manifest": []})["manifest"].append(arm_text)

    total_spec = sum(len(d.get("spec", [])) for d in by_domain.values())
    total_manifest = sum(len(d.get("manifest", [])) for d in by_domain.values())
    print(f"Grouped into {len(by_domain)} domains ({total_spec} spec, {total_manifest} manifest)")

    # Verify no lost arms
    assert total_spec == len(spec_arms), f"Lost {len(spec_arms) - total_spec} spec arms"
    assert total_manifest == len(manifest_arms), f"Lost {len(manifest_arms) - total_manifest} manifest arms"

    write_domain_files(by_domain)

    # Report unknowns
    unknowns = [d for d in by_domain if d in ("Misc", "Unknown")]
    if unknowns:
        print(f"WARNING: {len(unknowns)} unknown domains: {unknowns}", file=sys.stderr)


if __name__ == "__main__":
    main()
