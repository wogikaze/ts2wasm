#!/usr/bin/env python3
"""Host import baseline checker.

Compares host imports declared in the runtime catalog (HostImport::spec())
against the checked-in baseline at artifacts/abi/host-imports-baseline.json.

Usage:
  python scripts/check/host-import-baseline.py              # check baseline matches catalog
  python scripts/check/host-import-baseline.py --update      # update baseline from catalog
  python scripts/check/host-import-baseline.py --diff        # show diff only

Exits with 0 on match, 1 on mismatch (unless --update).
"""

import sys
import json
import re
import argparse
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
BASELINE_PATH = REPO_ROOT / "artifacts" / "abi" / "host-imports-baseline.json"


# Mapping from HostImport variant to capability
VARIANT_TO_CAPABILITY = {
    # WASI imports
    "FdRead": "wasi.fd",
    "FdWrite": "wasi.fd",
    "PathOpen": "wasi.filesystem",
    "FdClose": "wasi.fd",
    "FdSeek": "wasi.fd",
    "FdPrestatGet": "wasi.fd",
    "FdPrestatDirName": "wasi.fd",
    "WasiProcExit": "wasi.proc_exit",
    "ClockTimeGet": "wasi.clock",
    "ClockResGet": "wasi.clock",
    "RandomGet": "wasi.random",
    "ArgsSizesGet": "wasi.args",
    "ArgsGet": "wasi.args",
    "EnvironSizesGet": "wasi.env",
    "EnvironGet": "wasi.env",
    "PathCreateDirectory": "wasi.filesystem",
    "PathFilestatGet": "wasi.filesystem",
    "PathReadlink": "wasi.filesystem",
    "PathRemoveDirectory": "wasi.filesystem",
    "PathRename": "wasi.filesystem",
    "PathSymlink": "wasi.filesystem",
    "PathUnlinkFile": "wasi.filesystem",
    # Node shim imports
    "FsReadFileSync": "host.fs.readFileSync",
    "FsWriteFileSync": "host.fs.writeFileSync",
    "FsAppendFileSync": "host.fs.appendFileSync",
    "ProcessExit": "host.process.exit",
    "PathJoin": "host.path.join",
    "PathResolve": "host.path.resolve",
    "PathBasename": "host.path.basename",
    "PathDirname": "host.path.dirname",
    "CryptoRandomBytes": "host.crypto.randomBytes",
    "EncodeURI": "host.encodeURI",
    "DecodeURI": "host.decodeURI",
    "Escape": "host.escape",
    "Unescape": "host.unescape",
    "DateToString": "host.dateToString",
    "DateGetLocalTimeField": "host.dateGetLocalTimeField",
    "DateToISOString": "host.dateToISOString",
    "DateGetTimezoneOffset": "host.dateGetTimezoneOffset",
    "DateToDateString": "host.dateToDateString",
    "DateToTimeString": "host.dateToTimeString",
    "DateParse": "host.dateParse",
    "DateUTC": "host.dateUTC",
}


def collect_catalog_imports():
    """Collect HostImport spec entries by parsing the Rust host_import.rs."""
    host_import_path = REPO_ROOT / "crates" / "runtime-catalog" / "src" / "host_import.rs"
    if not host_import_path.exists():
        print(f"ERROR: host_import.rs not found at {host_import_path}", file=sys.stderr)
        sys.exit(1)

    content = host_import_path.read_text()
    imports = []

    spec_pattern = re.compile(
        r'Self::(\w+)\s*=>\s*HostImportSpec\s*\{'
        r'[^}]*?module:\s*"([^"]+)"'
        r'[^}]*?name:\s*"([^"]+)"'
        r'[^}]*?wat_symbol:\s*"([^"]+)"'
        r'[^}]*?abi:\s*HostAbi::(\w+)',
        re.DOTALL,
    )

    for m in spec_pattern.finditer(content):
        variant = m.group(1)
        module = m.group(2)
        name = m.group(3)
        wat_symbol = m.group(4)
        abi = m.group(5)
        capability = VARIANT_TO_CAPABILITY.get(variant, f"unknown.{variant}")
        imports.append({
            "module": module,
            "name": name,
            "wat_symbol": wat_symbol,
            "abi": abi,
            "capability": capability,
            "reason_owner": "runtime-catalog",
        })

    imports.sort(key=lambda x: (x["module"], x["name"]))
    return imports


def load_baseline():
    """Load the checked-in baseline JSON."""
    if not BASELINE_PATH.exists():
        return {"schema_version": 1, "imports": []}
    with open(BASELINE_PATH) as f:
        return json.load(f)


def save_baseline(imports):
    """Save baseline JSON."""
    baseline = {
        "schema_version": 1,
        "comment": "Host import baseline. Do not edit by hand. Sorted by module then name.",
        "imports": imports,
    }
    BASELINE_PATH.write_text(json.dumps(baseline, indent=2) + "\n")
    print(f"Updated baseline: {BASELINE_PATH}")


def diff_imports(catalog_imports, baseline_imports):
    """Compare two sorted import lists. Returns (added, removed, changed)."""
    catalog_map = {(imp["module"], imp["name"]): imp for imp in catalog_imports}
    baseline_map = {(imp["module"], imp["name"]): imp for imp in baseline_imports}

    catalog_keys = set(catalog_map.keys())
    baseline_keys = set(baseline_map.keys())

    added = sorted(catalog_keys - baseline_keys)
    removed = sorted(baseline_keys - catalog_keys)

    changed = []
    common = catalog_keys & baseline_keys
    for key in sorted(common):
        if catalog_map[key] != baseline_map[key]:
            changed.append(key)

    return added, removed, changed


def main():
    parser = argparse.ArgumentParser(description="Host import baseline checker")
    parser.add_argument("--update", action="store_true", help="Update baseline from catalog")
    parser.add_argument("--diff", action="store_true", help="Show diff only")
    args = parser.parse_args()

    catalog_imports = collect_catalog_imports()
    baseline = load_baseline()
    baseline_imports = baseline.get("imports", [])

    added, removed, changed = diff_imports(catalog_imports, baseline_imports)

    if args.update:
        save_baseline(catalog_imports)
        print(f"  {len(catalog_imports)} entries written")
        return

    has_diff = bool(added or removed or changed)

    if args.diff:
        for key in added:
            imp = catalog_map if False else None
            cat = next(x for x in catalog_imports if x["module"] == key[0] and x["name"] == key[1])
            print(f"+ {key[0]}.{key[1]}: {cat}")
        for key in removed:
            imp = next(x for x in baseline_imports if x["module"] == key[0] and x["name"] == key[1])
            print(f"- {key[0]}.{key[1]}: {imp}")
        for key in changed:
            print(f"~ {key[0]}.{key[1]}: changed")
        if not has_diff:
            print("No diff: baseline matches catalog.")
        return

    if has_diff:
        print(f"FAIL: host import baseline mismatch")
        print(f"  Catalog imports: {len(catalog_imports)}")
        print(f"  Baseline imports: {len(baseline_imports)}")
        print(f"  Added: {len(added)}, Removed: {len(removed)}, Changed: {len(changed)}")
        for key in added:
            print(f"    + {key[0]}.{key[1]}")
        for key in removed:
            print(f"    - {key[0]}.{key[1]}")
        for key in changed:
            print(f"    ~ {key[0]}.{key[1]}")
        print()
        print("Run with --update to refresh baseline, or --diff for details.")
        sys.exit(1)

    print(f"OK: host import baseline matches catalog ({len(catalog_imports)} entries)")
    sys.exit(0)


if __name__ == "__main__":
    main()
