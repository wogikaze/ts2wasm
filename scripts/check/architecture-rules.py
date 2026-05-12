#!/usr/bin/env python3
"""Lightweight architecture checks (complement to docs/12 + ast-grep rules).

Usage: mise run check architecture

Current checks:
  - ts2wasm-shared must not depend on ts2wasm-cli (inverted crate boundary).
  - crates/cli/src/backend must not be reintroduced after backend-wasm extraction.
  - crates/cli/src must not declare local backend/parser/compiler implementation modules.
  - Error when a repo-owned source/document file exceeds the documented line limit.
  - Error when backend-wasm or ir directly depends on frontend via Cargo.toml.
  - Error when any Rust function exceeds 300 lines.
  - Error when any Rust file exceeds 2000 lines (known exceptions allowlisted).
  - Error when RuntimeCall { runtime_fn: String } found (migrate to typed enum).
  - Error when `use super::*` appears outside test modules.
  - Error when backend-wasm imports from ts2wasm_frontend.
  - Warn when `wat.push_str` in runtime helper files (prefer structured builders).
  - Error when `include!` used in src/ files outside tests (migrate to real modules).
  - Error when backend emit functions accept bare &LoweredProgram (must wrap in Validated<).
  - Error when RuntimeFn variant lacks spec/manifest_name/emission_order entry.
  - Warn when Diagnostic { span: None } appears outside validate.rs (source errors need spans).
  - Error when raw runtime symbol string used outside runtime catalog.
  - Error when LoweredExpr variant lacks validate_lowered coverage.
  - Error when hardcoded WASI/Node host import string used outside runtime catalog.
  - Error when RuntimeFn variant with host imports lacks explicit capability marker.
  - Error when HostImport variant is not covered by manifest/link-plan tests.
  - Warn when a module has more than 30 public API items.
  - Error when a match has more than 50 arms unless allowlisted.
"""

import os
import re
import sys
import subprocess
import shutil
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
DEFAULT_MAX_FILE_LINES = 2000
TARGET_MAX_FILE_LINES = 1200
MAX_RUST_FILE_LINES_HARD = 2000
MAX_FUNCTION_LINES_WARN = 200
MAX_FUNCTION_LINES_HARD = 300
MAX_CRATE_NORMAL_DEPS_WARN = 10
MAX_PUBLIC_API_ITEMS_WARN = 30
MAX_MATCH_ARMS_WARN = 30
MAX_MATCH_ARMS_HARD = 50

# Known oversized files that are exempt from the general line limit.
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
    # Runtime domain runtime files (P4: domain split)
    "crates/backend-wasm/src/runtime/core/bigint.rs": "P4: runtime domain split",
    # General files over 1500 lines (active hard gate since #337)
    "scripts/run/reference-coverage.py": "coverage script — naturally large",
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
    ".agent",
    ".claude",
    ".cache",
    ".commandcode",
    ".config",
    ".devin",
    ".git",
    ".mypy_cache",
    ".worktrees",
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

# Files exceeding 2000 lines (Rust) — explicit hard gate allowlist.
# Every entry must have a documented refactoring plan or be a test file.
FILE_SIZE_ALLOWLIST_2000 = {
    "crates/backend-wasm/src/runtime/core/bigint.rs": "P4: runtime domain split",
    "crates/backend-wasm/src/expr_emit.rs": "P4: expression emitter — pending domain split",
    "crates/ir/src/lowered/resolver/call.rs": "P7: resolver decomposition",
    "crates/ir/src/lowered/program.rs": "P7: resolver decomposition",
    "crates/backend-wasm/src/runtime/spec/all.rs": "auto-generated spec",
    "crates/runtime-catalog/src/runtime/spec/all.rs": "auto-generated spec",
    "crates/runtime-catalog/src/runtime_fn.rs": "runtime catalog registry",
    "crates/frontend/src/parser/expressions_main.rs": "P4: parser decomposition",
    "crates/frontend/src/parser/statements_general.rs": "P4: parser decomposition",
    "crates/ir/src/builtin_resolver.rs": "P7: resolver decomposition",
    # Test files (naturally large, not a concern)
    "crates/frontend/src/parser/tests.rs": "test file",
    "crates/cli/tests/common/m2_node_diff_fixture_tests.rs": "test file",
}



# Files that use `use super::*` outside test modules (known legacy pattern).
USE_SUPER_STAR_ALLOWLIST = {
    "crates/backend-wasm/src/expr_emit_helpers.rs",
    "crates/backend-wasm/src/runtime_core_comparison_alloc.rs",
    "crates/backend-wasm/src/runtime_fn_impl.rs",
    "crates/ir/src/builtin_resolver_bigint.rs",
    "crates/ir/src/builtin_resolver_bigint_ops.rs",
    "crates/ir/src/builtin_resolver_class_features.rs",
    "crates/ir/src/builtin_resolver_host.rs",
    "crates/ir/src/builtin_resolver_outer.rs",
}

# Runtime catalog paths (runtime_fn.rs includes impl and spec via include!).
RUNTIME_CATALOG_FILE_PREFIXES = (
    "crates/backend-wasm/src/runtime/",
    "crates/backend-wasm/src/runtime_fn",
    "crates/runtime-catalog/src/runtime/",
    "crates/runtime-catalog/src/runtime_fn",
)

# Files that use raw runtime symbol strings in test assertions (allowed).
RAW_SYMBOL_ALLOWLIST = {
    "crates/backend-wasm/src/lib.rs",   # Has test assertions checking WAT output
    "crates/compiler/src/tests.rs",     # Has test assertions checking WAT output
}

# Known large functions (over 300 lines) with documented refactoring plans.
# The 300-line hard gate ONLY permits entries here if they cannot be split yet.
# Key is (relative_file_path, function_name) tuple.
# #334: 14 functions allowlisted -- all >300 lines with P4/P7/P11 refactoring plans.
FUNCTION_LENGTH_ALLOWLIST = {
    ("crates/backend-wasm/src/runtime_link_plan.rs", "collect_required_runtime_expr"): "P11: link plan refactor -- 362 lines, iterates all RuntimeFn variants to build dependency graph",
    ("crates/ir/src/binding_pattern.rs", "parse_binding_pattern"): "P7: resolver decomposition -- 380 lines, complex destructuring pattern parser with many sub-pattern types",
    ("crates/resolve/src/binding_pattern.rs", "parse_binding_pattern"): "P7: resolver decomposition -- 380 lines, complex destructuring pattern parser with many sub-pattern types",
    ("crates/ir/src/builtin_resolver.rs", "fold_stmt"): "P7: resolver decomposition -- 337 lines, statement folding dispatches many statement types",
    ("crates/ir/src/builtin_resolver.rs", "fold_expr"): "P7: resolver decomposition -- 363 lines, expression folding dispatches many expression types",
    ("crates/ir/src/builtin_resolver.rs", "resolve_stmt_with_outer_bindings"): "P7: resolver decomposition -- 457 lines, statement resolution with nested scope tracking",
    ("crates/ir/src/builtin_resolver.rs", "resolve_expr"): "P7: resolver decomposition -- 577 lines, expression resolution with type dispatch",
    ("crates/ir/src/lowered/program.rs", "lower_program"): "P7: resolver decomposition -- 359 lines, program lowering orchestrates all resolve phases",
    ("crates/ir/src/lowered/validate.rs", "validate_expr"): "P7: resolver decomposition -- 430 lines, validate_expr covers all LoweredExpr variants",
    ("crates/ir/src/name_resolver.rs", "resolve_stmt"): "P7: resolver decomposition -- 498 lines, statement-level name resolution dispatches many AST types",
    ("crates/ir/src/name_resolver.rs", "resolve_expr"): "P7: resolver decomposition -- 498 lines, expression-level name resolution dispatches many AST types",
    ("crates/resolve/src/name_resolver.rs", "resolve_stmt"): "P7: resolver decomposition -- 498 lines, statement-level name resolution dispatches many AST types",
    ("crates/resolve/src/name_resolver.rs", "resolve_expr"): "P7: resolver decomposition -- 498 lines, expression-level name resolution dispatches many AST types",
    # #334: remaining oversized functions allowlisted
    ("crates/compiler/src/test262_preprocessor.rs", "build_feature_stubs"): "P4: preprocessor stub builder -- 362 lines, large match constructing feature stubs per test262 config",
    ("crates/frontend/src/parser/expressions_main.rs", "primary"): "P4: parser match dispatch -- 331 lines, primary expression parser with many sub-expression types",
    ("crates/frontend/src/parser/statements_class.rs", "class_decl_body"): "P4: parser class body -- 420 lines, class body parser with constructor/method/field/property handling",
    ("crates/ir/src/lowered/mir_dump.rs", "runtime_intrinsic_name"): "P4: dump intrinsic name match -- 334 lines, maps all RuntimeIntrinsic variants to display names",
}

# Known large functions (over 200 lines) — staged warning for 200-line reduction.
# These are distinct from the 300-line hard gate allowlist; entries here are
# between 200-300 lines and should be split as part of the staged reduction.
FUNCTION_WARN_ALLOWLIST_200 = {
    ("crates/backend-wasm/src/expr_emit.rs", "emit_binary_expr"): "P4: expression emitter",
    ("crates/backend-wasm/src/runtime_link_plan.rs", "bigint_runtime_arithmetic_selects_helper_deps"): "P11: link plan refactor",
    ("crates/backend-wasm/src/stmt_emit.rs", "emit_statement_with_label"): "P4: statement emitter",
    ("crates/compiler/src/stages/lower.rs", "rewrite_static_module_body_for_build"): "compiler pipeline",
    ("crates/frontend/src/lexer.rs", "tokenize_arithmetic_or_comparison_operator"): "P4: lexer decomposition",
    ("crates/frontend/src/lexer.rs", "tokenize_assignment_or_bitwise_operator"): "P4: lexer decomposition",
    ("crates/frontend/src/parser/expressions_main.rs", "assignment"): "P4: parser decomposition",
    ("crates/frontend/src/parser/expressions_main.rs", "unary"): "P4: parser decomposition",
    ("crates/frontend/src/parser/helpers.rs", "parse_template_parts"): "P4: parser decomposition",
    ("crates/frontend/src/parser/tokens.rs", "skip_type_annotation_until"): "P4: parser decomposition",
    ("crates/ir/src/lowered/resolver/array.rs", "lower_variable_array_callback_method"): "P7: resolver decomposition",
    ("crates/ir/src/lowered/resolver/call/method.rs", "lower_mcall_date_string"): "P7: resolver decomposition",
    ("crates/ir/src/lowered/resolver/call/method.rs", "lower_mcall_class_dispatch"): "P7: resolver decomposition",
    ("crates/runtime-catalog/src/runtime_fn.rs", "runtime_fn_from_name"): "runtime function registry",
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
    print("  - Error when RuntimeFn variant lacks spec/manifest_name/emission_order entry.")
    print("  - Warn when Diagnostic { span: None } appears outside validate.rs.")
    print("  - Error when raw runtime symbol string used outside runtime catalog.")
    print("  - Error when LoweredExpr variant lacks validate_lowered coverage.")
    print("  - Error when RuntimeFn variant with host imports lacks explicit capability marker.")
    print("  - Error when HostImport variant not covered by manifest/link-plan tests.")
    print("  - Warn when a module has more than 30 public API items.")
    print("  - Error when a match has more than 50 arms unless allowlisted.")


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
    if path.suffix not in LINE_COUNT_SUFFIXES:
        return False
    # .rs files are checked by dedicated rust-specific checks (check_rust_file_length),
    # so skip them here to avoid double-counting.
    if path.suffix == ".rs":
        return False
    return True


def line_count(path: Path) -> int:
    data = path.read_bytes()
    if not data:
        return 0
    return data.count(b"\n") + (0 if data.endswith(b"\n") else 1)


def iter_repo_files(suffix: str | None = None):
    for root, dirnames, filenames in os.walk(REPO_ROOT):
        rel_root = Path(root).relative_to(REPO_ROOT)
        dirnames[:] = [
            dirname
            for dirname in dirnames
            if dirname not in EXCLUDED_PATH_PARTS
            and dirname not in EXCLUDED_FILENAMES
            and (rel_root / dirname) != Path("TRACKING.yaml")
        ]
        for filename in filenames:
            path = Path(root) / filename
            if suffix is not None and not path.name.endswith(suffix):
                continue
            yield path


def check_oversized_files(max_file_lines: int) -> None:
    oversized: list[tuple[int, Path]] = []
    for path in iter_repo_files():
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
        f"{max_file_lines} lines (target {TARGET_MAX_FILE_LINES}); "
        "split ownership/modules or raise the documented limit",
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


# --- #262: Strengthen checks ---

def check_function_length() -> list[str]:
    """Check that no function exceeds 300 lines in .rs files."""
    violations = []
    fn_re = re.compile(r'^\s*(pub\s+)?(unsafe\s+)?(async\s+)?fn\s+(\w+)')
    max_fn_lines = MAX_FUNCTION_LINES_HARD

    for path in sorted(iter_repo_files(".rs")):
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
                allowlist_key = (str(rel), fn_name)
                if allowlist_key not in FUNCTION_LENGTH_ALLOWLIST:
                    violations.append(
                        f"check_architecture_rules: ERROR {rel}:{fn_start + 1}: "
                        f"function `{fn_name}` is {fn_length} lines (max {max_fn_lines})"
                    )

            i = j

    return violations


def check_no_new_string_runtime_call() -> list[str]:
    """Check that no RuntimeCall uses string-based runtime_fn.

    The typed RuntimeIntrinsic enum must be used instead.
    Scans all .rs files for any RuntimeCall construction with runtime_fn field.
    """
    violations = []
    runtime_call_re = re.compile(r'RuntimeCall\s*\{')

    for path in sorted(iter_repo_files(".rs")):
        rel = path.relative_to(REPO_ROOT)
        if any(part in EXCLUDED_PATH_PARTS for part in rel.parts):
            continue
        text = path.read_text()
        lines = text.split('\n')
        for i, line in enumerate(lines):
            if not runtime_call_re.search(line):
                continue
            stripped = line.strip()
            if stripped.startswith('//') or '//' in stripped and 'RuntimeCall' in stripped.split('//')[1]:
                continue
            # Look at the next few lines for the first field
            for offset in range(1, 5):
                if i + offset >= len(lines):
                    break
                nxt = lines[i + offset].strip()
                if nxt.startswith('//'):
                    continue
                if 'runtime_fn' in nxt and 'String' in nxt:
                    violations.append(
                        f"check_architecture_rules: ERROR {rel}:{i + 1}: "
                        f"RuntimeCall {{ runtime_fn: String }} — migrate to typed RuntimeIntrinsic"
                    )
                    break
                if 'intrinsic' in nxt:
                    break  # Already migrated, no issue
                if nxt == '}':
                    break  # Empty or single-line RuntimeCall

    return violations


# --- #265: Backend/frontend coupling ---

def check_backend_frontend_import() -> list[str]:
    """Check that backend-wasm doesn't import from ts2wasm_frontend.

    Permits imports inside #[cfg(test)] blocks (test-only dependencies).
    """
    violations = []
    backend_src = REPO_ROOT / "crates" / "backend-wasm" / "src"
    if not backend_src.exists():
        return violations

    for path in sorted(backend_src.rglob("*.rs")):
        rel = path.relative_to(REPO_ROOT)
        text = path.read_text()
        lines = text.split('\n')
        in_cfg_test = False
        cfg_test_brace_depth = 0
        for i, line in enumerate(lines, 1):
            stripped = line.strip()
            # Track #[cfg(test)] blocks
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
            if re.match(r'^\s*use\s+ts2wasm_frontend', line):
                violations.append(
                    f"check_architecture_rules: ERROR {rel}:{i}: "
                    f"backend module imports from ts2wasm_frontend"
                )

    return violations


def check_runtimefn_spec_gap() -> list[str]:
    """Check that every RuntimeFn variant has entries in spec/manifest/emission_order.

    Parses the RuntimeFn enum and checks against the spec table, manifest table,
    and emission_order list.
    """
    violations = []

    # 1. Parse RuntimeFn enum variants
    runtime_fn_path = REPO_ROOT / "crates" / "backend-wasm" / "src" / "runtime_fn.rs"
    if not runtime_fn_path.exists():
        return violations

    enum_text = runtime_fn_path.read_text()
    enum_match = re.search(r'enum RuntimeFn \{(.*?)^\}', enum_text, re.MULTILINE | re.DOTALL)
    if not enum_match:
        return violations

    enum_body = enum_match.group(1)
    variants = set()
    for m in re.finditer(r'^\s+([A-Z]\w+)\s*,?\s*$', enum_body, re.MULTILINE):
        variants.add(m.group(1))
    # Also catch variants after #[allow(dead_code)] attributes
    for m in re.finditer(r'#\[.*?\]\s*\n\s+([A-Z]\w+)\s*,?\s*', enum_body):
        variants.add(m.group(1))

    # 2. Parse spec entries from runtime/spec/all.rs
    spec_path = REPO_ROOT / "crates" / "backend-wasm" / "src" / "runtime" / "spec" / "all.rs"
    spec_variants = set()
    if spec_path.exists():
        spec_text = spec_path.read_text()
        for m in re.finditer(r'Self::(\w+)\s*=>\s*RuntimeSpec', spec_text):
            spec_variants.add(m.group(1))

    # 3. Parse manifest entries from runtime/manifest/all.rs
    manifest_path = REPO_ROOT / "crates" / "backend-wasm" / "src" / "runtime" / "manifest" / "all.rs"
    manifest_variants = set()
    if manifest_path.exists():
        manifest_text = manifest_path.read_text()
        for m in re.finditer(r'Self::(\w+)\s*=>', manifest_text):
            manifest_variants.add(m.group(1))

    # 4. Parse emission_order entries from runtime_fn_impl.rs
    emission_order_path = REPO_ROOT / "crates" / "backend-wasm" / "src" / "runtime_fn_impl.rs"
    emission_variants = set()
    if emission_order_path.exists():
        emission_text = emission_order_path.read_text()
        for m in re.finditer(r'Self::(\w+)', emission_text):
            emission_variants.add(m.group(1))

    # Check for gaps
    if not spec_variants:
        violations.append(
            "check_architecture_rules: ERROR cannot parse runtime/spec/all.rs -- "
            "spec table might have changed format"
        )
        return violations

    missing_spec = variants - spec_variants
    missing_manifest = variants - manifest_variants
    missing_emission = variants - emission_variants

    for v in sorted(missing_spec):
        violations.append(
            f"check_architecture_rules: ERROR RuntimeFn::{v} missing from runtime/spec/all.rs"
        )
    for v in sorted(missing_manifest):
        violations.append(
            f"check_architecture_rules: ERROR RuntimeFn::{v} missing from runtime/manifest/all.rs"
        )
    for v in sorted(missing_emission):
        violations.append(
            f"check_architecture_rules: ERROR RuntimeFn::{v} missing from emission_order"
        )

    return violations


# --- #269 P2/P3: File size limits ---

def check_rust_file_length(max_lines: int = MAX_RUST_FILE_LINES_HARD) -> list[str]:
    """Check that no .rs file exceeds 2000 lines (with allowlist)."""
    violations = []
    for path in sorted(iter_repo_files(".rs")):
        rel = path.relative_to(REPO_ROOT)
        if any(part in EXCLUDED_PATH_PARTS for part in rel.parts):
            continue
        if str(rel) in FILE_SIZE_ALLOWLIST_2000:
            continue
        count = line_count(path)
        if count > max_lines:
            violations.append(
                f"check_architecture_rules: ERROR {rel}: {count} lines "
                f"(max {max_lines}, target {TARGET_MAX_FILE_LINES})"
            )
    return violations


# --- #277: Span/catalog/validate coverage ---

def check_diagnostic_span_none() -> list[str]:
    """Check for Diagnostic { span: None } in non-validate, non-test files.

    Internal invariant diagnostics (validate.rs) are expected to have span: None.
    Source-origin errors should include source location spans.
    """
    violations = []
    diag_re = re.compile(
        r'Diagnostic\s*\{[^}]*?span:\s*None[^}]*?\}',
        re.DOTALL,
    )
    invariant_code_re = re.compile(r'code:\s*DiagCode::InvariantViolation')
    backend_io_code_re = re.compile(r'code:\s*DiagCode::BackendIo')

    for path in sorted(iter_repo_files(".rs")):
        rel = path.relative_to(REPO_ROOT)
        if any(part in EXCLUDED_PATH_PARTS for part in rel.parts):
            continue
        # Skip validate.rs (internal invariants)
        if str(rel) == "crates/ir/src/lowered/validate.rs":
            continue
        # Skip test files
        if "tests" in rel.parts:
            continue
        text = path.read_text()
        # Find each Diagnostic block with span: None
        for m in diag_re.finditer(text):
            block = m.group(0)
            # InvariantViolation is expected to have no span
            if invariant_code_re.search(block):
                continue
            # BackendIo is an I/O error, not source-locatable
            if backend_io_code_re.search(block):
                continue
            violations.append(
                f"check_architecture_rules: WARN {rel}: span: None in "
                f"non-invariant Diagnostic — source errors need source location spans"
            )

    return violations


def check_raw_runtime_symbol_outside_catalog() -> list[str]:
    """Check that raw $runtime_symbol strings are not used outside the runtime catalog.

    The runtime spec/symbol catalog in runtime/spec/all.rs defines all runtime symbols.
    Other code must use RuntimeFn::symbol() or similar typed access.
    """
    violations = []

    # Parse all runtime symbols from the spec file
    spec_path = REPO_ROOT / "crates" / "backend-wasm" / "src" / "runtime" / "spec" / "all.rs"
    if not spec_path.exists():
        return violations

    spec_text = spec_path.read_text()
    symbols = set()
    for m in re.finditer(r'symbol:\s*"(\$\w+)"', spec_text):
        symbols.add(m.group(1))

    # Check all .rs files for raw symbol usage
    for path in sorted(iter_repo_files(".rs")):
        rel = path.relative_to(REPO_ROOT)
        if any(part in EXCLUDED_PATH_PARTS for part in rel.parts):
            continue
        # Skip the catalog files themselves
        skip = False
        for prefix in RUNTIME_CATALOG_FILE_PREFIXES:
            if str(rel).startswith(prefix):
                skip = True
                break
        if skip:
            continue
        # Skip test files
        if "tests" in rel.parts:
            continue
        # Skip files with known test assertions using runtime symbols
        if str(rel) in RAW_SYMBOL_ALLOWLIST:
            continue
        text = path.read_text()
        for sym in symbols:
            # Check for the symbol as a string literal
            if f'"{sym}"' in text:
                violations.append(
                    f"check_architecture_rules: ERROR {rel}: "
                    f"raw runtime symbol `{sym}` used outside runtime catalog"
                )
                break  # One violation per file is enough

    return violations


def check_lowered_expr_validate_coverage() -> list[str]:
    """Check that every LoweredExpr variant is covered by validate_lowered's validate_expr.

    Parses the LoweredExpr enum from types.rs and checks validate_expr match arms.
    """
    violations = []

    types_path = REPO_ROOT / "crates" / "ir" / "src" / "lowered" / "types.rs"
    validate_path = REPO_ROOT / "crates" / "ir" / "src" / "lowered" / "validate.rs"

    if not types_path.exists() or not validate_path.exists():
        return violations

    # Parse LoweredExpr enum variants
    types_text = types_path.read_text()
    enum_match = re.search(r'pub enum LoweredExpr \{(.*?)\n\}', types_text, re.MULTILINE | re.DOTALL)
    if not enum_match:
        return violations

    enum_body = enum_match.group(1)
    enum_variants = set()
    for m in re.finditer(r'^\s+(\w+)\s*(?:\(|\{|,|$)', enum_body, re.MULTILINE):
        name = m.group(1)
        if name[0].isupper():
            enum_variants.add(name)

    # Parse LoweredExpr variants referenced in validate_lowered (validate_expr match arms)
    validate_text = validate_path.read_text()
    covered = set()
    for m in re.finditer(r'LoweredExpr::(\w+)', validate_text):
        covered.add(m.group(1))

    # Report missing variants
    missing = sorted(enum_variants - covered)
    for v in missing:
        violations.append(
            f"check_architecture_rules: ERROR LoweredExpr::{v} is not covered by "
            f"validate_lowered's validate_expr"
        )

    return violations


def check_host_import_string_outside_catalog() -> list[str]:
    """Check that hardcoded WASI host import module/name strings are not used
    outside the runtime catalog in crates/backend-wasm/src/.

    The WASI module/name strings (e.g., "wasi_snapshot_preview1", "fd_write", "proc_exit")
    are defined in crates/runtime-catalog/src/host_import.rs as the single source of truth.
    Other code must use HostImport::spec() or RuntimeFn's spec.imports instead.

    Only checks crates/backend-wasm/src/ for WASI-specific strings. Node shim import
    names (e.g., "escape", "path.join") are excluded because they are also JS global
    function names and would produce false positives.
    """
    violations = []

    # WASI-specific import module/name strings (unlikely to appear outside WASM import context)
    wasi_import_strings = {
        "wasi_snapshot_preview1",
        "fd_read",
        "fd_write",
        "path_open",
        "fd_close",
        "proc_exit",
        "clock_time_get",
        "clock_res_get",
        "random_get",
        "args_sizes_get",
        "args_get",
        "environ_sizes_get",
        "environ_get",
    }

    backend_src = REPO_ROOT / "crates" / "backend-wasm" / "src"
    if not backend_src.exists():
        return violations

    for path in sorted(backend_src.rglob("*.rs")):
        rel = path.relative_to(REPO_ROOT)
        # Skip files inside the runtime catalog and spec directories
        catalog_paths = (
            "crates/backend-wasm/src/runtime/spec/",
            "crates/backend-wasm/src/runtime/catalog/",
            "crates/backend-wasm/src/runtime/manifest/",
        )
        skip = False
        for cp in catalog_paths:
            if str(rel).startswith(cp):
                skip = True
                break
        if skip:
            continue
        # Skip test files
        if "tests" in rel.parts:
            continue
        # Skip lib.rs (has WASM binary output test assertions)
        if str(rel) == "crates/backend-wasm/src/lib.rs":
            continue
        # Skip capability_manifest.rs (metadata mapping, not code generation)
        if str(rel) == "crates/backend-wasm/src/capability_manifest.rs":
            continue
        # Skip wat_writer.rs (WASM import section emission uses Import struct)
        if str(rel) == "crates/backend-wasm/src/wat_writer.rs":
            continue
        # Skip wasm_ir.rs (conversion tests use canonical HostImportSpec literals).
        if str(rel) == "crates/backend-wasm/src/wasm_ir.rs":
            continue
        text = path.read_text()
        for s in wasi_import_strings:
            if f'"{s}"' in text:
                violations.append(
                    f"check_architecture_rules: ERROR {rel}: "
                    f"hardcoded WASI host import string `{s}` used outside runtime catalog"
                )
                break  # One per file is enough

    return violations


# --- Existing utility/helper checks ---

def check_use_super_star() -> list[str]:
    """Check that `use super::*` appears only in test modules."""
    violations = []
    use_super_re = re.compile(r'^\s*use\s+super::\*;?\s*$')

    for path in sorted(iter_repo_files(".rs")):
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
                if str(rel) in USE_SUPER_STAR_ALLOWLIST:
                    violations.append(
                        f"check_architecture_rules: WARN {rel}:{i + 1}: "
                        f"`use super::*` outside test module (known legacy)"
                    )
                else:
                    violations.append(
                        f"check_architecture_rules: ERROR {rel}:{i + 1}: "
                        f"`use super::*` outside test module"
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


# --- #292: 200-line function warning ---

def check_smaller_function_warning() -> list[str]:
    """Warn when functions exceed 200 lines (staged reduction toward 200-line ideal)."""
    violations = []
    fn_re = re.compile(r'^\s*(pub\s+)?(unsafe\s+)?(async\s+)?fn\s+(\w+)')
    max_fn_lines = MAX_FUNCTION_LINES_WARN

    for path in sorted(iter_repo_files(".rs")):
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
                allowlist_key = (str(rel), fn_name)
                is_documented_exception = (
                    allowlist_key in FUNCTION_WARN_ALLOWLIST_200
                    or allowlist_key in FUNCTION_LENGTH_ALLOWLIST
                )
                if not is_documented_exception:
                    violations.append(
                        f"check_architecture_rules: WARN {rel}:{fn_start + 1}: "
                        f"function `{fn_name}` is {fn_length} lines (max {max_fn_lines})"
                    )

            i = j

    return violations


# --- #299: Fan-out check ---

# Allowlist for files with high public API count (> 50 pub declarations).
HIGH_PUBLIC_API_COUNT_ALLOWLIST = {
    "crates/backend-wasm/src/wasm_binary.rs": "wasm binary encoder exports many instruction helpers",
    "crates/backend-wasm/src/wat_writer.rs": "WatWriter has many public wrapper methods",
    "crates/runtime-abi/src/layout.rs": "type layout definitions are inherently public",
    "crates/runtime-abi/src/value.rs": "value type definitions are inherently public",
}

# Allowlist for files with oversized match expressions (> 30 arms).
# Start as known legacy -- these match expressions dispatch many variants
# and should be refactored by domain dispatch.
LARGE_MATCH_ALLOWLIST = {
    "crates/backend-wasm/src/emitter/strings.rs": "string escape dispatch",
    "crates/backend-wasm/src/expr_emit.rs": "expression emitter dispatches many runtime calls",
    "crates/backend-wasm/src/runtime/manifest/all.rs": "manifest table covers all RuntimeFn variants",
    "crates/backend-wasm/src/runtime/spec/all.rs": "spec table covers all RuntimeFn variants",
    "crates/backend-wasm/src/runtime/spec/manifest_map.rs": "manifest mapping for all RuntimeFn variants",
    "crates/backend-wasm/src/runtime_dispatch_array.rs": "array domain dispatch",
    "crates/backend-wasm/src/runtime_dispatch_collections.rs": "collections domain dispatch",
    "crates/backend-wasm/src/runtime_dispatch_core.rs": "core domain dispatch",
    "crates/backend-wasm/src/runtime_dispatch_date.rs": "date domain dispatch",
    "crates/backend-wasm/src/runtime_dispatch_string.rs": "string domain dispatch",
    "crates/backend-wasm/src/runtime_link_plan.rs": "link plan covers many runtime fn deps",
    "crates/backend-wasm/src/wasm_encoder_backend.rs": "WASM instruction encoding dispatch",
    "crates/backend-wasm/src/wat_writer.rs": "WAT instruction rendering dispatch",
    "crates/compiler/src/dump.rs": "AST/IR dump dispatches many node types",
    "crates/frontend/src/parser/tokens.rs": "token keyword/keyword context matching",
    "crates/ir/src/builtin_resolver.rs": "builtin resolution dispatches many expression/statement types",
    "crates/ir/src/builtin_resolver_bigint.rs": "bigint builtin resolution dispatches many types",
    "crates/ir/src/lowered/program_builtins.rs": "program builtins maps many RuntimeIntrinsic variants",
    "crates/ir/src/lowered/mir_dump.rs": "MIR dump maps many intrinsic names",
    "crates/ir/src/lowered/resolver/expr.rs": "lower_expr dispatches many expression types",
    "crates/ir/src/lowered/runtime_intrinsic.rs": "RuntimeIntrinsic::all covers all variants",
    "crates/ir/src/lowered/resolver/array.rs": "array method dispatch",
    "crates/ir/src/lowered/resolver/call.rs": "call lowering dispatches many call kinds",
    "crates/ir/src/lowered/resolver/class.rs": "class member dispatch",
    "crates/ir/src/lowered/resolver/object.rs": "object property dispatch",
    "crates/ir/src/lowered/validate.rs": "validate_expr covers all LoweredExpr variants",
    "crates/ir/src/name_resolver.rs": "name resolver dispatches many statement/expression types",
    "crates/ir/src/resolved/mod.rs": "resolved expression dispatch",
    "crates/frontend/src/lexer_identifiers.rs": "lexer keyword/identifier matching",
    "crates/runtime-catalog/src/runtime/manifest/all.rs": "runtime catalog manifest dispatch",
    "crates/runtime-catalog/src/runtime/spec/all.rs": "runtime catalog spec dispatch",
    "crates/runtime-catalog/src/runtime_fn.rs": "RuntimeFn dispatch covers all variants",
}


def check_module_fan_out() -> list[str]:
    """Check that no crate has excessive dependency fan-out (> 10 normal [dependencies]).

    Counts entries under [dependencies] in Cargo.toml, excluding dev-dependencies
    and build-dependencies. High fan-out increases coupling.
    """
    violations = []
    max_deps = MAX_CRATE_NORMAL_DEPS_WARN
    deps_section_re = re.compile(r'^\[dependencies\]\s*$', re.MULTILINE)
    dep_entry_re = re.compile(r'^\s+([a-zA-Z][a-zA-Z0-9_-]*)\s*=\s*{?\s*$', re.MULTILINE)
    # Exclude workspace/path-only entries that reuse crate name as dep name
    # (these are self-referencing workspace crates)

    for path in sorted(iter_repo_files("Cargo.toml")):
        rel = path.relative_to(REPO_ROOT)
        if any(part in EXCLUDED_PATH_PARTS for part in rel.parts):
            continue
        if rel.name in EXCLUDED_FILENAMES:
            continue
        if not str(rel).startswith("crates/"):
            continue

        text = path.read_text()
        deps_match = deps_section_re.search(text)
        if not deps_match:
            continue

        deps_start = deps_match.end()
        rest = text[deps_start:]
        next_section = re.search(r'^\s*\[', rest, re.MULTILINE)
        if next_section:
            deps_body = rest[:next_section.start()]
        else:
            deps_body = rest

        dep_names = dep_entry_re.findall(deps_body)
        count = len(dep_names)

        if count > max_deps:
            violations.append(
                f"check_architecture_rules: WARN {rel}: {count} dependencies "
                f"(max {max_deps} recommended)"
            )

    return violations


def check_public_api_count() -> list[str]:
    """Warn when a module exports more than the documented danger threshold."""
    violations = []
    max_pub_items = MAX_PUBLIC_API_ITEMS_WARN
    public_item_re = re.compile(
        r'pub(?:\([^)]*\))?\s+(?:(?:async|unsafe|const)\s+)*'
        r'(fn|struct|enum|trait|type|const|mod|use)\s'
    )

    for path in sorted(iter_repo_files(".rs")):
        rel = path.relative_to(REPO_ROOT)
        if any(part in EXCLUDED_PATH_PARTS for part in rel.parts):
            continue
        if "tests" in rel.parts:
            continue
        if str(rel) in HIGH_PUBLIC_API_COUNT_ALLOWLIST:
            continue
        text = path.read_text()
        lines = text.split('\n')
        pub_count = 0
        for line in lines:
            stripped = line.strip()
            if public_item_re.match(stripped):
                pub_count += 1
        if pub_count > max_pub_items:
            violations.append(
                f"check_architecture_rules: WARN {rel}: "
                f"{pub_count} public API items (danger > {max_pub_items})"
            )

    return violations


def check_oversized_match_arms() -> list[str]:
    """Check for match expressions above the warning or hard danger thresholds."""
    violations = []
    warn_arms = MAX_MATCH_ARMS_WARN
    hard_arms = MAX_MATCH_ARMS_HARD

    for path in sorted(iter_repo_files(".rs")):
        rel = path.relative_to(REPO_ROOT)
        if any(part in EXCLUDED_PATH_PARTS for part in rel.parts):
            continue
        if "tests" in rel.parts:
            continue
        text = path.read_text()
        lines = text.split('\n')
        for i, line in enumerate(lines):
            match_m = re.match(r'^(\s*)match\s+\S+\s*\{', line)
            if not match_m:
                continue
            arm_count = 0
            j = i + 1
            brace_depth = 1
            while j < len(lines) and brace_depth > 0:
                brace_depth += lines[j].count('{') - lines[j].count('}')
                if '=>' in lines[j] and not lines[j].strip().startswith('//'):
                    arm_count += 1
                j += 1
            if arm_count > warn_arms:
                if str(rel) in LARGE_MATCH_ALLOWLIST:
                    violations.append(
                        f"check_architecture_rules: WARN {rel}:{i + 1}: "
                        f"~{arm_count} match arms (allowlisted)"
                    )
                elif arm_count > hard_arms:
                    violations.append(
                        f"check_architecture_rules: ERROR {rel}:{i + 1}: "
                        f"match expression has {arm_count} arms (danger > {hard_arms}); "
                        "split domain dispatch or add a documented allowlist reason"
                    )
                else:
                    violations.append(
                        f"check_architecture_rules: WARN {rel}:{i + 1}: "
                        f"match expression has {arm_count} arms (warn > {warn_arms})"
                    )

    return violations


def check_validated_backend_contract() -> list[str]:
    """Check that public emit functions in backend-wasm use Validated<LoweredProgram>."""
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
        if not fn_name.startswith('emit'):
            continue
        # Metadata/diagnostic functions are exempt: they report program structure
        # rather than emitting code, and callers need them before or without validation.
        if fn_name in ('emit_canonical_manifest_json', 'emit_link_plan_snapshot_json'):
            continue
        end = min(i + 4, len(lines))
        fn_window = ' '.join(lines[i - 1:end])
        if 'LoweredProgram' in fn_window and 'Validated<' not in fn_window:
            violations.append(
                f"check_architecture_rules: ERROR crates/backend-wasm/src/lib.rs:{i}: "
                f"`pub fn {fn_name}` uses `LoweredProgram` without `Validated<` wrapper"
            )
    return violations


# --- #309: Capability fitness checks ---


def check_runtimefn_capability() -> list[str]:
    """Check that every RuntimeFn variant with host imports has explicit capability marker.

    Parses backend-wasm's runtime/spec/all.rs and ensures:
    - Any variant with imports != NO_IMPORTS also has capability != NO_CAPS.
    - Any variant with capability != NO_CAPS also has imports != NO_IMPORTS.
    """
    violations = []
    spec_path = REPO_ROOT / "crates" / "backend-wasm" / "src" / "runtime" / "spec" / "all.rs"
    if not spec_path.exists():
        return violations

    text = spec_path.read_text()
    blocks = re.findall(
        r'Self::(\w+)\s*=>\s*RuntimeSpec\s*\{(.*?)\}',
        text,
        re.DOTALL,
    )

    if not blocks:
        violations.append(
            "check_architecture_rules: ERROR cannot parse RuntimeSpec blocks from "
            "crates/backend-wasm/src/runtime/spec/all.rs"
        )
        return violations

    for name, block in blocks:
        has_imports = 'imports: NO_IMPORTS' not in block
        has_cap = 'capability: NO_CAPS' not in block
        if has_imports and not has_cap:
            violations.append(
                f"check_architecture_rules: ERROR RuntimeFn::{name} has host imports "
                f"but capability: NO_CAPS -- must declare explicit capability"
            )
        if has_cap and not has_imports:
            violations.append(
                f"check_architecture_rules: ERROR RuntimeFn::{name} has capability "
                f"but imports: NO_IMPORTS -- capability without host import is misleading"
            )

    return violations


def check_host_import_manifest() -> list[str]:
    """Check that host import variants are covered by manifest/link-plan tests.

    Parses HostImport enum and its spec() match arms from runtime-catalog to
    derive each variant's manifest_name (e.g. "wasi_snapshot_preview1.proc_exit").
    Then verifies that each manifest_name appears in:
    - crates/backend-wasm/tests/runtime_link_plan.rs
    - crates/compiler/tests/manifest_snapshot.rs
    - crates/backend-wasm/src/runtime/manifest/all.rs (manifest mapping)

    Missing entries indicate untested host import bindings.
    """
    violations = []
    src_path = REPO_ROOT / "crates" / "runtime-catalog" / "src" / "host_import.rs"
    if not src_path.exists():
        return violations

    text = src_path.read_text()

    # Determine which variants are intentionally dead_code
    # by parsing the enum definition for #[allow(dead_code)] annotations.
    enum_match = re.search(r'pub enum HostImport \{(.*?)^\}', text, re.MULTILINE | re.DOTALL)
    dead_code_variants = set()
    if enum_match:
        enum_body = enum_match.group(1)
        lines = enum_body.split('\n')
        in_dead = False
        for line in lines:
            stripped = line.strip()
            if stripped == '#[allow(dead_code)]':
                in_dead = True
            elif stripped.startswith('#['):
                continue
            elif stripped and not stripped.startswith('//'):
                variant_match = re.match(r'(\w+)', stripped)
                if variant_match and in_dead:
                    dead_code_variants.add(variant_match.group(1))
                    in_dead = False
                elif variant_match:
                    in_dead = False

    # Parse HostImport::spec() match arms for module+name per variant
    spec_match = re.search(
        r'Self::(\w+)\s*=>\s*HostImportSpec\s*\{[^}]*module:\s*"([^"]+)"[^}]*name:\s*"([^"]+)"[^}]*\}',
        text,
    )
    # Fall back to iterating all match arms
    if not spec_match:
        return violations

    manifest_names = {}
    for m in re.finditer(
        r'Self::(\w+)\s*=>\s*HostImportSpec\s*\{[^}]*module:\s*"([^"]+)"[^}]*name:\s*"([^"]+)"[^}]*\}',
        text,
    ):
        variant = m.group(1)
        manifest = f"{m.group(2)}.{m.group(3)}"
        manifest_names[variant] = manifest

    # Files to check for manifest_name coverage
    # Spec files are included since they define which RuntimeFn imports each HostImport.
    # Manifest files map RuntimeFn variants to their manifest names.
    # Test files verify link plan output includes the correct imports.
    check_files = [
        REPO_ROOT / "crates" / "backend-wasm" / "tests" / "runtime_link_plan.rs",
        REPO_ROOT / "crates" / "compiler" / "tests" / "manifest_snapshot.rs",
        REPO_ROOT / "crates" / "backend-wasm" / "src" / "runtime" / "manifest" / "all.rs",
        REPO_ROOT / "crates" / "runtime-catalog" / "src" / "runtime" / "manifest" / "all.rs",
        REPO_ROOT / "crates" / "backend-wasm" / "src" / "runtime" / "spec" / "all.rs",
        REPO_ROOT / "crates" / "runtime-catalog" / "src" / "runtime" / "spec" / "all.rs",
        REPO_ROOT / "crates" / "runtime-catalog" / "src" / "runtime_fn.rs",
    ]

    for variant, manifest in sorted(manifest_names.items()):
        if variant in dead_code_variants:
            continue  # Intentionally dead code -- no coverage needed
        found = False
        for cf in check_files:
            if not cf.exists():
                continue
            cf_text = cf.read_text()
            if manifest in cf_text or variant in cf_text:
                found = True
                break
        if not found:
            violations.append(
                f"check_architecture_rules: ERROR HostImport::{variant} "
                f"(manifest: {manifest}) not covered in test, spec, or manifest files"
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
    # #262 checks
    violations.extend(check_function_length())
    violations.extend(check_no_new_string_runtime_call())
    # #265 checks
    violations.extend(check_backend_frontend_import())
    violations.extend(check_runtimefn_spec_gap())
    # #269 P2/P3 checks (hard gate at 2000 lines)
    violations.extend(check_rust_file_length())
    # #277 checks
    violations.extend(check_diagnostic_span_none())
    violations.extend(check_raw_runtime_symbol_outside_catalog())
    violations.extend(check_lowered_expr_validate_coverage())
    # #292 checks
    violations.extend(check_smaller_function_warning())
    # #295 checks
    violations.extend(check_host_import_string_outside_catalog())
    # #299 checks
    violations.extend(check_module_fan_out())
    violations.extend(check_public_api_count())
    violations.extend(check_oversized_match_arms())
    # #309 checks
    violations.extend(check_runtimefn_capability())
    violations.extend(check_host_import_manifest())
    # Existing checks
    violations.extend(check_use_super_star())
    violations.extend(check_runtime_push_str())
    violations.extend(check_include_in_src())
    violations.extend(check_validated_backend_contract())

    for v in violations:
        print(v, file=sys.stderr)
    if any(": ERROR " in v for v in violations):
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
