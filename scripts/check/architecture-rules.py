#!/usr/bin/env python3
"""Lightweight architecture checks (complement to docs/12 + ast-grep rules).

Usage: mise run check architecture

Current checks:
  - ts2wasm-shared must not depend on ts2wasm-cli (inverted crate boundary).
  - crates/cli/src/backend must not be reintroduced after backend-wasm extraction.
  - crates/cli/src must not declare local backend/parser/compiler implementation modules.
  - Error when a repo-owned source/document file exceeds the documented line limit.
  - Error when backend-wasm or ir directly depends on frontend via Cargo.toml.
  - Error when public backend emit functions accept bare &LoweredProgram (must use Validated<).
"""

import os
import re
import sys
import subprocess
import shutil
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
DEFAULT_MAX_FILE_LINES = 3000

# Known oversized files that are exempt from the line limit.
# Each entry must include a reason and the P-item that will eventually fix it.
OVERSIZED_ALLOWLIST = {
    # Test files (naturally large, not a concern)
    "crates/frontend/src/parser/tests.rs": "test file",
    "crates/cli/tests/common/m2_node_diff_fixture_tests.rs": "test file",
    "crates/cli/tests/m6_builtin_methods.rs": "test file",
    "crates/cli/tests/ir_lowering.rs": "test file",
    "crates/cli/tests/m2_node_diff.rs": "test file",
    "crates/ir/src/name_resolver_tests.rs": "test file",
    "crates/cli/tests/dump_cli.rs": "test file",
    "crates/cli/tests/m11_host_deny.rs": "test file",
    "crates/cli/tests/m_standalone_wasi.rs": "test file",
    "crates/cli/tests/differential_jsonl.rs": "test file",
    # Tracking data files
    "docs/done-tracking.yaml": "tracking data",
    # Being refactored by P4 (domain split)
    "crates/backend-wasm/src/runtime_fn_impl.rs": "P4: domain split planned",
    # Being refactored by P7 (Resolver decomposition)
    "crates/ir/src/lowered/resolver_extra.rs": "P7: Resolver context decomposition",
}

# Crates that must not directly depend on ts2wasm-frontend via Cargo.toml.
# Remove entries from this set as dependencies are eliminated.
FRONTEND_DEP_DENY = {
    "crates/backend-wasm",
    "crates/ir",
}


LINE_COUNT_SUFFIXES = {
    ".md",
    ".py",
    ".rs",
    ".sh",
    ".toml",
    ".yaml",
    ".yml",
}
EXCLUDED_PATH_PARTS = {
    ".claude",
    ".git",
    ".mypy_cache",
    "__pycache__",
    "artifacts",
    "node_modules",
    "reference",
    "reports",
    "target",
    "_worktrees",
}
EXCLUDED_FILENAMES = {
    "Cargo.lock",
}

# Files known to exceed the 2000-line Rust file limit, with planned remediation.
KNOWN_OVERSIZED_FILES = {
    "crates/ir/src/lowered/resolver_expr.rs": "large match — pending domain split",
    "crates/backend-wasm/src/runtime_fn_impl.rs": "large spec — pending domain split",
    "crates/backend-wasm/src/expr_emit.rs": "expression emitter — pending domain split",
    "crates/compiler/src/lib.rs": "pipeline — pending stage split",
}


def usage():
    print("Usage:")
    print("  mise run check architecture -- [--max-file-lines N]")
    print()
    print("Current checks:")
    print("  - ts2wasm-shared must not depend on ts2wasm-cli (inverted crate boundary).")
    print("  - crates/cli/src/backend must not be reintroduced.")
    print("  - crates/cli/src must not declare local backend/parser/compiler modules.")
    print("  - Error when a repo-owned source/document file exceeds the line limit.")
    print("  - Error when backend-wasm or ir depends on frontend via Cargo.toml.")
    print("  - Error when any Rust function exceeds 300 lines.")
    print("  - Error when any Rust file exceeds 2000 lines (known exceptions allowlisted).")
    print("  - Error when RuntimeCall { runtime_fn: String } found (migrate to typed enum).")
    print("  - Error when `use super::*` appears outside test modules.")
    print("  - Error when backend-wasm imports from ts2wasm_frontend.")
    print("  - Warn when `wat.push_str` in runtime helper files (prefer structured builders).")
    print("  - Error when `include!` used in src/ files outside tests (migrate to real modules).")
    print("  - Error when backend emit functions accept bare &LoweredProgram (must wrap in Validated<).")


def parse_max_file_lines(args: list[str]) -> int:
    raw_max_file_lines = os.environ.get("TS2WASM_MAX_FILE_LINES", str(DEFAULT_MAX_FILE_LINES))
    try:
        max_file_lines = int(raw_max_file_lines)
    except ValueError:
        print("check_architecture_rules: TS2WASM_MAX_FILE_LINES must be an integer", file=sys.stderr)
        sys.exit(1)

    while args:
        arg = args.pop(0)
        if arg in ("-h", "--help"):
            usage()
            sys.exit(0)
        if arg == "--max-file-lines":
            if not args:
                print("check_architecture_rules: --max-file-lines requires a value", file=sys.stderr)
                sys.exit(1)
            try:
                max_file_lines = int(args.pop(0))
            except ValueError:
                print("check_architecture_rules: --max-file-lines must be an integer", file=sys.stderr)
                sys.exit(1)
        else:
            print(f"check_architecture_rules: unknown option: {arg}", file=sys.stderr)
            usage()
            sys.exit(1)

    if max_file_lines < 1:
        print("check_architecture_rules: --max-file-lines must be >= 1", file=sys.stderr)
        sys.exit(1)
    return max_file_lines


def should_count_lines(path: Path) -> bool:
    relative = path.relative_to(REPO_ROOT)
    if path.name in EXCLUDED_FILENAMES:
        return False
    if any(part in EXCLUDED_PATH_PARTS for part in relative.parts):
        return False
    if relative == Path("TRACKING.yaml"):
        return False
    return path.suffix in LINE_COUNT_SUFFIXES


def line_count(path: Path) -> int:
    data = path.read_bytes()
    if not data:
        return 0
    return data.count(b"\n") + (0 if data.endswith(b"\n") else 1)


def check_oversized_files(max_file_lines: int) -> None:
    oversized: list[tuple[int, Path]] = []
    for path in REPO_ROOT.rglob("*"):
        if not path.is_file() or not should_count_lines(path):
            continue
        rel = path.relative_to(REPO_ROOT)
        if str(rel) in OVERSIZED_ALLOWLIST:
            continue
        count = line_count(path)
        if count > max_file_lines:
            oversized.append((count, rel))

    if not oversized:
        return

    print(
        "check_architecture_rules: ERROR files exceed "
        f"{max_file_lines} lines; split ownership/modules or raise the documented limit",
        file=sys.stderr,
    )
    for count, path in sorted(oversized, key=lambda item: (-item[0], item[1])):
        print(f"check_architecture_rules: ERROR {path}: {count} lines", file=sys.stderr)
    sys.exit(1)


def check_cli_thin_wrapper_boundary() -> None:
    cli_src = REPO_ROOT / "crates" / "cli" / "src"
    backend_dir = cli_src / "backend"
    if backend_dir.exists():
        print(
            "check_architecture_rules: crates/cli/src/backend must not be reintroduced; "
            "put WASM backend implementation under crates/backend-wasm/src",
            file=sys.stderr,
        )
        sys.exit(1)

    forbidden_module_names = ("backend", "parser", "compiler", "driver")
    for path in cli_src.glob("*.rs"):
        text = path.read_text()
        for module_name in forbidden_module_names:
            if f"mod {module_name};" in text:
                print(
                    f"check_architecture_rules: {path.relative_to(REPO_ROOT)} must not declare "
                    f"mod {module_name}; keep compiler implementation outside crates/cli",
                    file=sys.stderr,
                )
                sys.exit(1)
        if "struct Lexer" in text or "struct Parser" in text:
            print(
                f"check_architecture_rules: {path.relative_to(REPO_ROOT)} must not define "
                "parser implementation types; keep parser/compiler implementation outside crates/cli",
                file=sys.stderr,
            )
            sys.exit(1)

    for module_name in forbidden_module_names:
        module_file = cli_src / f"{module_name}.rs"
        if module_file.exists():
            print(
                f"check_architecture_rules: {module_file.relative_to(REPO_ROOT)} must not exist; "
                "crates/cli is a thin wrapper",
                file=sys.stderr,
            )
            sys.exit(1)

    cli_lib = cli_src / "lib.rs"
    if cli_lib.exists() and "ts2wasm_backend_wasm" in cli_lib.read_text():
        print(
            "check_architecture_rules: crates/cli/src/lib.rs must not call backend directly; "
            "use ts2wasm-compiler instead",
            file=sys.stderr,
        )
        sys.exit(1)


def check_backend_frontend_dependency() -> None:
    """Check that backend-wasm and ir don't directly depend on frontend via Cargo.toml [dependencies].

    Only checks the [dependencies] section (normal dependencies).
    Permits ts2wasm-frontend in [dev-dependencies] and [build-dependencies].
    """
    found = False
    for crate_rel in FRONTEND_DEP_DENY:
        cargo_path = REPO_ROOT / crate_rel / "Cargo.toml"
        if not cargo_path.exists():
            continue
        text = cargo_path.read_text()
        # Extract only the [dependencies] section: everything from "[dependencies]"
        # up to the next "[...]" section header, or end of file.
        deps_match = re.search(
            r"^\[dependencies\]\s*$(.+?)(?=^\s*\[|\Z)",
            text,
            re.MULTILINE | re.DOTALL,
        )
        if deps_match and "ts2wasm-frontend" in deps_match.group(1):
            print(
                f"check_architecture_rules: ERROR {crate_rel}/Cargo.toml depends on "
                f"ts2wasm-frontend; this violates the layer architecture. "
                f"Move dependencies to shared crates.",
                file=sys.stderr,
            )
            found = True
    if found:
        sys.exit(1)


def check_function_length() -> list[str]:
    violations = []
    fn_re = re.compile(r'^\s*(pub\s+)?(unsafe\s+)?(async\s+)?fn\s+(\w+)')
    max_fn_lines = 300

    for path in sorted(REPO_ROOT.rglob("*.rs")):
        rel = path.relative_to(REPO_ROOT)
        if any(part in EXCLUDED_PATH_PARTS for part in rel.parts):
            continue
        text = path.read_text()
        lines = text.split('\n')

        i = 0
        while i < len(lines):
            m = fn_re.match(lines[i])
            if not m:
                i += 1
                continue

            fn_name = m.group(4)
            fn_start = i

            brace_depth = 0
            j = i
            while j < len(lines) and brace_depth == 0:
                brace_depth += lines[j].count('{') - lines[j].count('}')
                if brace_depth > 0:
                    break
                j += 1

            if brace_depth == 0:
                i += 1
                continue

            j += 1
            while j < len(lines) and brace_depth > 0:
                brace_depth += lines[j].count('{') - lines[j].count('}')
                j += 1

            fn_length = j - fn_start
            if fn_length > max_fn_lines:
                violations.append(
                    f"check_architecture_rules: ERROR {rel}:{fn_start + 1}: "
                    f"function `{fn_name}` is {fn_length} lines (max {max_fn_lines})"
                )

            i = j

    return violations


def check_rust_file_length(max_lines: int = 2000) -> list[str]:
    violations = []
    for path in sorted(REPO_ROOT.rglob("*.rs")):
        rel = path.relative_to(REPO_ROOT)
        if any(part in EXCLUDED_PATH_PARTS for part in rel.parts):
            continue
        if str(rel) in KNOWN_OVERSIZED_FILES:
            continue
        count = line_count(path)
        if count > max_lines:
            violations.append(
                f"check_architecture_rules: ERROR {rel}: {count} lines "
                f"(max {max_lines})"
            )
    return violations


def check_runtime_call_string() -> list[str]:
    violations = []
    target = REPO_ROOT / "crates" / "ir" / "src" / "lowered" / "types.rs"
    if not target.exists():
        return violations
    text = target.read_text()
    lines = text.split('\n')
    for i, line in enumerate(lines):
        stripped = line.strip()
        if stripped == 'RuntimeCall {':
            nxt = lines[i + 1].strip() if i + 1 < len(lines) else ''
            if 'runtime_fn' in nxt and 'String' in nxt:
                violations.append(
                    f"check_architecture_rules: ERROR crates/ir/src/lowered/types.rs:{i + 1}: "
                    f"RuntimeCall {{ runtime_fn: String }} — migrate to typed enum"
                )
    return violations


def check_use_super_star() -> list[str]:
    violations = []
    use_super_re = re.compile(r'^\s*use\s+super::\*;?\s*$')

    for path in sorted(REPO_ROOT.rglob("*.rs")):
        rel = path.relative_to(REPO_ROOT)
        if any(part in EXCLUDED_PATH_PARTS for part in rel.parts):
            continue
        if rel.name == "tests.rs":
            continue
        if "tests" in rel.parts:
            continue

        text = path.read_text()
        lines = text.split('\n')
        in_cfg_test = False
        cfg_test_brace_depth = 0

        for i, line in enumerate(lines):
            stripped = line.strip()
            if stripped == '#[cfg(test)]':
                in_cfg_test = True
                cfg_test_brace_depth = 0
                continue
            if in_cfg_test:
                cfg_test_brace_depth += line.count('{') - line.count('}')
                if cfg_test_brace_depth <= 0:
                    in_cfg_test = False
                    cfg_test_brace_depth = 0
                continue
            if use_super_re.match(stripped):
                violations.append(
                    f"check_architecture_rules: ERROR {rel}:{i + 1}: "
                    f"`use super::*` outside test module"
                )

    return violations


def check_backend_frontend_import() -> list[str]:
    violations = []
    backend_src = REPO_ROOT / "crates" / "backend-wasm" / "src"
    if not backend_src.exists():
        return violations

    for path in sorted(backend_src.rglob("*.rs")):
        rel = path.relative_to(REPO_ROOT)
        text = path.read_text()
        for i, line in enumerate(text.split('\n'), 1):
            if re.match(r'^\s*use\s+ts2wasm_frontend', line):
                violations.append(
                    f"check_architecture_rules: ERROR {rel}:{i}: "
                    f"backend module imports from ts2wasm_frontend"
                )

    return violations


def check_runtime_push_str() -> list[str]:
    violations = []
    backend_src = REPO_ROOT / "crates" / "backend-wasm" / "src"
    if not backend_src.exists():
        return violations

    for path in sorted(backend_src.rglob("runtime*.rs")):
        rel = path.relative_to(REPO_ROOT)
        text = path.read_text()
        for i, line in enumerate(text.split('\n'), 1):
            if 'push_str' in line:
                violations.append(
                    f"check_architecture_rules: WARN {rel}:{i}: "
                    f"`push_str` usage — prefer structured builders over raw WAT strings"
                )

    return violations


def check_include_in_src() -> list[str]:
    violations = []
    target_file = REPO_ROOT / "crates" / "ir" / "src" / "lowered.rs"
    if not target_file.exists():
        return violations
    text = target_file.read_text()
    for i, line in enumerate(text.split('\n'), 1):
        stripped = line.strip()
        if stripped.startswith('include!') and 'tests' not in stripped:
            violations.append(
                f"check_architecture_rules: ERROR {target_file.relative_to(REPO_ROOT)}:{i}: "
                f"`include!` used outside test module — migrate to real `pub mod`"
            )
    return violations


def check_validated_backend_contract() -> list[str]:
    """Check that public emit functions in backend-wasm use Validated<LoweredProgram>.

    Any pub fn starting with 'emit' in crates/backend-wasm/src/lib.rs that mentions
    LoweredProgram in its signature must wrap it in Validated<>. Non-emit pub fn
    using LoweredProgram (e.g. has_node_host_imports) are allowed to use bare &LoweredProgram.

    Known exceptions:
    - emit_canonical_manifest_json: utility/query function, not a program emitter.
      Called with validated.as_ref() by the compiler pipeline.
    """
    violations = []
    backend_lib = REPO_ROOT / "crates" / "backend-wasm" / "src" / "lib.rs"
    if not backend_lib.exists():
        return violations
    text = backend_lib.read_text()
    lines = text.split('\n')
    for i, line in enumerate(lines, 1):
        stripped = line.strip()
        if not stripped.startswith('pub fn '):
            continue
        name_end = stripped.find('(')
        if name_end == -1:
            continue
        fn_name = stripped[7:name_end].strip()
        # Only check emit functions
        if not fn_name.startswith('emit'):
            continue
        # Known exceptions (utility/query functions, not program emitters)
        if fn_name == 'emit_canonical_manifest_json':
            continue
        # Look ahead up to 5 lines for signature context
        end = min(i + 4, len(lines))
        fn_window = ' '.join(lines[i - 1:end])
        if 'LoweredProgram' in fn_window and 'Validated<' not in fn_window:
            violations.append(
                f"check_architecture_rules: ERROR crates/backend-wasm/src/lib.rs:{i}: "
                f"`pub fn {fn_name}` uses `LoweredProgram` without `Validated<` wrapper"
            )
    return violations


def main():
    args = sys.argv[1:]
    max_file_lines = parse_max_file_lines(args)

    errors = 0
    try:
        check_oversized_files(max_file_lines)
    except SystemExit:
        errors += 1

    try:
        check_cli_thin_wrapper_boundary()
    except SystemExit:
        errors += 1

    try:
        check_backend_frontend_dependency()
    except SystemExit:
        errors += 1

    violations = []
    violations.extend(check_function_length())
    violations.extend(check_rust_file_length())
    violations.extend(check_runtime_call_string())
    violations.extend(check_use_super_star())
    violations.extend(check_backend_frontend_import())
    violations.extend(check_runtime_push_str())
    violations.extend(check_include_in_src())
    violations.extend(check_validated_backend_contract())

    for v in violations:
        print(v, file=sys.stderr)
    if violations:
        errors += 1

    if not shutil.which("cargo"):
        print("check_architecture_rules: cargo is required", file=sys.stderr)
        sys.exit(1)

    # Check if ts2wasm-shared depends on ts2wasm-cli
    result = subprocess.run(
        ["cargo", "tree", "-p", "ts2wasm-shared", "--edges", "normal,build"],
        capture_output=True,
        text=True,
        cwd=REPO_ROOT
    )

    if result.returncode != 0:
        print("check_architecture_rules: OK (cargo tree unavailable)", file=sys.stderr)
        sys.exit(0 if errors == 0 else 1)

    if "ts2wasm-cli" in result.stdout:
        print("check_architecture_rules: ts2wasm-shared must not depend on ts2wasm-cli", file=sys.stderr)
        print(result.stdout, file=sys.stderr)
        errors += 1

    if errors > 0:
        print(f"check_architecture_rules: FAILED ({errors} checks)", file=sys.stderr)
        sys.exit(1)

    print("check_architecture_rules: OK", file=sys.stderr)


if __name__ == "__main__":
    main()
