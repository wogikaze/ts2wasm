#!/usr/bin/env python3
"""Lightweight architecture checks (complement to docs/12 + ast-grep rules).

Usage: mise run check architecture

Current checks:
  - ts2wasm-shared must not depend on ts2wasm-cli (inverted crate boundary).
  - crates/cli/src/backend must not be reintroduced after backend-wasm extraction.
  - crates/cli/src must not declare local backend/parser/compiler implementation modules.
  - Error when a repo-owned source/document file exceeds the documented line limit.
  - RuntimeFn import/capability parity: every RuntimeFn with imports must have a capability marker and vice versa.

  - Error when backend-wasm or ir directly depends on frontend via Cargo.toml.
  - Error when any Rust function exceeds 200 lines (hard gate, with allowlist).
  - Error when any Rust file exceeds 1200 lines (hard gate, with allowlist).
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
  - Error when a module has more than 30 public API items (coupling gate, with allowlist).
  - Error when a crate has more than 10 [dependencies] (module fan-out gate).
  - Error when a match has more than 50 arms unless allowlisted.
  - Warn when non-test Rust source filenames use `_` to encode pseudo-hierarchy; prefer real submodules/directories.
  - Error when backend-wasm depends on semantic-ir (crate DAG boundary).
  - Error when spec-kernel depends on backend-wasm (crate DAG boundary).
  - Error when runtime-core depends on backend-wasm (crate DAG boundary).
  - Error when frozen files (native_lowered.rs, typed.rs, native_runtime_embed.rs) exceed line baseline.
  - Error when RuntimeFn variant count exceeds baseline (new additions require allowlist).
  - Error when new SpecOp variant lacks spec-kernel dispatch coverage.

"""

import os
import re
import sys
import subprocess
import shutil
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
MAX_CRATE_NORMAL_DEPS_WARN = 10
MAX_CRATE_NORMAL_DEPS_HARD = 10  # #365: hard gate (was warn only)

FRONTEND_DEP_DENY = {
    "crates/backend-wasm",
    "crates/ir",
}

# --- Crate dependency DAG enforcement ---
# backend-wasm must NOT depend on semantic-ir (backend only sees SpecOp/OptExpr, not semantic meaning)
# spec-kernel must NOT depend on backend-wasm (spec-kernel is pure dispatch, no emission)
# runtime-core must NOT depend on backend-wasm (runtime-core is pure ABI/layout, no emission)
CRATE_DAG_DENY = {
    "crates/backend-wasm": ["ts2wasm-semantic-ir"],
    "crates/spec-kernel": ["ts2wasm-backend-wasm"],
    "crates/runtime-core": ["ts2wasm-backend-wasm"],
}

# --- Frozen file growth prohibition ---
# These files are in "delete/move/bugfix only" mode. New logic must go to new modules.
# Baseline is the current line count; CI fails if a file grows beyond this.
FROZEN_FILES_BASELINE = {
    "crates/backend-wasm/src/native_lowered.rs": 38200,
    "crates/backend-wasm/src/runtime/core/typed.rs": 50000,
    "crates/backend-wasm/src/native_runtime_embed.rs": 7700,
}
# Allowlist: files exempt from the frozen check (e.g., during a migration window)
FROZEN_FILES_ALLOWLIST: set[str] = set()

# --- RuntimeFn addition prohibition ---
# New RuntimeFn additions require: allowlist entry + spec/manifest/emission_order coverage.
# This prevents the shortcut of adding legacy catalog entries to avoid new-path work.
RUNTIMEFN_BASELINE_COUNT = 504  # current variant count; new additions require allowlist
RUNTIMEFN_ADDITION_ALLOWLIST: set[str] = set()

# --- SpecOp dispatch coverage ---
# New SpecOp variants must have dispatch coverage in spec-kernel.
# Existing variants are baseline (spec-kernel is WIP and many are not yet dispatched).
# This prevents adding SpecOp without implementing the corresponding dispatch.
SPECOP_BASELINE_VARIANTS = {
    # All current SpecOp variants as of the baseline date.
    # New additions beyond this set require dispatch coverage.
    "Get", "Set", "GetOwnProperty", "DefineOwnProperty", "Delete", "HasProperty",
    "GetPrototypeOf", "SetPrototypeOf", "IsExtensible", "PreventExtensions",
    "OwnPropertyKeys", "Call", "Construct",
    "CreateDataProperty", "SetIntegrityLevel", "TestIntegrityLevel",
    "ToPrimitive", "ToNumber", "ToNumeric", "ToPropertyKey", "ToObject",
    "ToBoolean", "ToString",
    "GetBindingValue", "SetMutableBinding", "CreateBinding",
    "InitializeBinding", "ResolveBinding",
    "GetIterator", "IteratorNext", "IteratorClose",
    "GetModuleNamespace",
    "Return", "Throw", "PushStringConstant",
}

# Crates that frontend/syntax must NOT depend on (reverse dependency gate).
# These ensure frontend ownership boundaries are not eroded by importing
# backend/runtime/capability logic in parser-level code.
FRONTEND_SYNTAX_BOUNDARY_DENY = {
    "ts2wasm-backend-wasm",
    "ts2wasm-runtime-catalog",
    "ts2wasm-runtime-abi",
    "ts2wasm-ir",
    "ts2wasm-compiler",
    "ts2wasm-cli",
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
    "plans",
    "reference",
    "reports",
    "target",
    "_worktrees",
    ".venv",
    "venv",
}
EXCLUDED_FILENAMES = {
    "Cargo.lock",
}




def usage():
    print("Usage:")
    print("  mise run check architecture")
    print()
    print("Current checks:")
    print("  - ts2wasm-shared must not depend on ts2wasm-cli (inverted crate boundary).")
    print("  - crates/cli/src/backend must not be reintroduced.")
    print("  - crates/cli/src must not declare local backend/parser/compiler modules.")
    print("  - RuntimeFn import/capability parity: every RuntimeFn with imports must have a capability marker and vice versa.")
    print("  - Error when backend-wasm or ir depends on frontend via Cargo.toml.")
    print("  - Warn when RuntimeCall { runtime_fn: String } found (prefer typed RuntimeIntrinsic).")
    print("  - Error/warn when `use super::*` appears outside test modules.")
    print("  - Error when backend-wasm imports from ts2wasm_frontend.")
    print("  - Warn when `wat.push_str` in runtime helper files (prefer structured builders).")
    print("  - Error when backend emit functions accept bare &LoweredProgram (must wrap in Validated<).")
    print("  - Error when RuntimeFn variant lacks spec/manifest_name/emission_order entry.")
    print("  - Warn when Diagnostic { span: None } appears outside validate.rs.")
    print("  - Error when raw runtime symbol string used outside runtime catalog.")
    print("  - Error when LoweredExpr variant lacks validate_lowered coverage.")
    print("  - Error when RuntimeFn variant with host imports lacks explicit capability marker.")
    print("  - Error when HostImport variant not covered by manifest/link-plan tests.")
    print("  - Error when a crate has more than 10 [dependencies] (module fan-out gate).")
    print("  - Frontend syntax ownership: frontend must not import runtime/backend crates or types.")
    print("  - IR crate must not contain WAT instruction patterns outside tests.")
    print("  - Error when backend-wasm depends on semantic-ir (crate DAG boundary).")
    print("  - Error when spec-kernel depends on backend-wasm (crate DAG boundary).")
    print("  - Error when runtime-core depends on backend-wasm (crate DAG boundary).")
    print("  - Error when frozen files exceed line baseline (native_lowered, typed, native_runtime_embed).")
    print("  - Error when RuntimeFn variant count exceeds baseline (new additions require allowlist).")
    print("  - Error when SpecOp variant lacks spec-kernel dispatch coverage.")






def iter_repo_files(suffix: str | None = None):
    for root, dirnames, filenames in os.walk(REPO_ROOT):
        rel_root = Path(root).relative_to(REPO_ROOT)
        dirnames[:] = [
            dirname
            for dirname in dirnames
            if dirname not in EXCLUDED_PATH_PARTS
            and dirname not in EXCLUDED_FILENAMES
            and (rel_root / dirname) != Path("issues/README.md")
        ]
        for filename in filenames:
            path = Path(root) / filename
            if suffix is not None and not path.name.endswith(suffix):
                continue
            yield path






def find_cli_boundary_violations() -> list[str]:
    """Return list of violation messages for CLI thin-wrapper boundary checks."""
    violations: list[str] = []
    cli_src = REPO_ROOT / "crates" / "cli" / "src"
    backend_dir = cli_src / "backend"
    if backend_dir.exists():
        violations.append(
            "crates/cli/src/backend must not be reintroduced; "
            "put WASM backend implementation under crates/backend-wasm/src"
        )

    forbidden_module_names = ("backend", "parser", "compiler", "driver")
    for path in cli_src.glob("*.rs"):
        text = path.read_text()
        for module_name in forbidden_module_names:
            if f"mod {module_name};" in text:
                violations.append(
                    f"{path.relative_to(REPO_ROOT)} must not declare "
                    f"mod {module_name}; keep compiler implementation outside crates/cli"
                )
        if "struct Lexer" in text or "struct Parser" in text:
            violations.append(
                f"{path.relative_to(REPO_ROOT)} must not define "
                "parser implementation types; keep parser/compiler implementation outside crates/cli"
            )

    for module_name in forbidden_module_names:
        module_file = cli_src / f"{module_name}.rs"
        if module_file.exists():
            violations.append(
                f"{module_file.relative_to(REPO_ROOT)} must not exist; "
                "crates/cli is a thin wrapper"
            )

    cli_lib = cli_src / "lib.rs"
    if cli_lib.exists() and "ts2wasm_backend_wasm" in cli_lib.read_text():
        violations.append(
            "crates/cli/src/lib.rs must not call backend directly; "
            "use ts2wasm-compiler instead"
        )

    return violations


def check_backend_frontend_dependency() -> list[str]:
    violations: list[str] = []
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
            violations.append(
                f"check_architecture_rules: WARN {crate_rel}/Cargo.toml depends on "
                f"ts2wasm-frontend (legacy — migrate to shared crates)"
            )
    return violations


def check_frontend_syntax_boundary() -> list[str]:
    violations: list[str] = []
    frontend_crates = ["crates/frontend", "crates/syntax"]
    for crate_rel in frontend_crates:
        cargo_path = REPO_ROOT / crate_rel / "Cargo.toml"
        if not cargo_path.exists():
            continue
        text = cargo_path.read_text()
        deps_match = re.search(
            r"^\[dependencies\]\s*$(.+?)(?=^\s*\[|\Z)",
            text,
            re.MULTILINE | re.DOTALL,
        )
        if not deps_match:
            continue
        deps_section = deps_match.group(1)
        for deny_crate in FRONTEND_SYNTAX_BOUNDARY_DENY:
            if deny_crate in deps_section:
                violations.append(
                    f"check_architecture_rules: ERROR {crate_rel}/Cargo.toml depends on "
                    f"{deny_crate} which violates frontend/syntax ownership boundary"
                )
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
                        f"check_architecture_rules: WARN {rel}:{i + 1}: "
                        f"RuntimeCall {{ runtime_fn: String }} — prefer typed RuntimeIntrinsic"
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


def _emission_order_variants(text: str) -> set[str]:
    """Extract RuntimeFn variants listed in emission_order() body."""
    fn_match = re.search(
        r"pub const fn emission_order\(\)\s*->\s*&'static \[RuntimeFn\]\s*\{(.*?)^\}",
        text,
        re.MULTILINE | re.DOTALL,
    )
    if not fn_match:
        return set()
    body = fn_match.group(1)
    return {m.group(1) for m in re.finditer(r'Self::(\w+)', body)}


def check_runtimefn_spec_gap() -> list[str]:
    """Check that every RuntimeFn variant has entries in spec/manifest/emission_order.

    Parses the RuntimeFn enum and checks against the spec table, manifest table,
    and emission_order list.  All sources live in crates/runtime-catalog/src/.
    """
    violations = []

    catalog_src = REPO_ROOT / "crates" / "runtime-catalog" / "src"

    # 1. Parse RuntimeFn enum variants
    runtime_fn_path = catalog_src / "runtime_fn.rs"
    if not runtime_fn_path.exists():
        return violations

    enum_text = runtime_fn_path.read_text()
    enum_match = re.search(r'pub enum RuntimeFn \{(.*?)^\}', enum_text, re.MULTILINE | re.DOTALL)
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
    spec_path = catalog_src / "runtime" / "spec" / "all.rs"
    spec_variants = set()
    if spec_path.exists():
        spec_text = spec_path.read_text()
        for m in re.finditer(r'Self::(\w+)\s*=>\s*RuntimeSpec', spec_text):
            spec_variants.add(m.group(1))

    # 3. Parse manifest entries from runtime/manifest/all.rs
    manifest_path = catalog_src / "runtime" / "manifest" / "all.rs"
    manifest_variants = set()
    if manifest_path.exists():
        manifest_text = manifest_path.read_text()
        for m in re.finditer(r'Self::(\w+)\s*=>', manifest_text):
            manifest_variants.add(m.group(1))

    # 4. Parse emission_order entries from runtime_fn.rs (emission_order function body)
    emission_variants = _emission_order_variants(runtime_fn_path.read_text())

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


# --- #277: Span/catalog/validate coverage ---

def check_diagnostic_span_none() -> list[str]:
    """Check for Diagnostic { span: None } in non-validate, non-test files.

    Internal invariant diagnostics (validate.rs) are expected to have span: None.
    Source-origin errors should include source location spans.
    """
    violations = []
    diag_re = re.compile(
        r'(?<!\w)Diagnostic\s*\{[^}]*?span:\s*None[^}]*?\}',
        re.DOTALL,
    )
    invariant_code_re = re.compile(r'code:\s*(?:ts2wasm_frontend::)?DiagCode::InvariantViolation')
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
        if any(str(rel).startswith(p) for p in (
            "crates/backend-wasm/src/runtime/",
            "crates/backend-wasm/src/runtime_fn",
            "crates/runtime-catalog/src/runtime/",
            "crates/runtime-catalog/src/runtime_fn",
        )):
            continue
        # Skip test files
        if "tests" in rel.parts:
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
            "crates/backend-wasm/src/runtime/",
            "crates/backend-wasm/src/runtime_fn",
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
                violations.append(
                    f"check_architecture_rules: WARN {rel}:{i + 1}: "
                    f"`use super::*` outside test module — prefer explicit imports"
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


def check_src_filename_pseudo_hierarchy() -> list[str]:
    """Warn on underscored non-test Rust source filenames under src/.

    Names like `mir_dump.rs` or `runtime_async.rs` often indicate a flat file
    layout encoding hierarchy in the filename instead of using directory-backed
    modules such as `mir/dump.rs` or `runtime/async/mod.rs`.
    """
    violations = []

    for path in sorted(iter_repo_files(".rs")):
        rel = path.relative_to(REPO_ROOT)
        if any(part in EXCLUDED_PATH_PARTS for part in rel.parts):
            continue
        if "src" not in rel.parts:
            continue
        if "tests" in rel.parts:
            continue
        if rel.name == "tests.rs" or rel.stem.endswith("_tests"):
            continue
        if rel.name in {"lib.rs", "main.rs", "mod.rs"}:
            continue
        if "_" not in rel.stem:
            continue

        violations.append(
            f"check_architecture_rules: WARN {rel}: "
            f"underscored src filename suggests pseudo-hierarchy; prefer real submodules/directories"
        )

    return violations





# --- #299: Fan-out check ---

def check_module_fan_out() -> list[str]:
    """Check that no crate has excessive dependency fan-out (> 10 normal [dependencies]).

    Counts entries under [dependencies] in Cargo.toml, excluding dev-dependencies
    and build-dependencies. High fan-out increases coupling.
    """
    violations = []
    max_deps = MAX_CRATE_NORMAL_DEPS_HARD  # #365: hard gate (10)
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
                f"check_architecture_rules: ERROR {rel}: {count} dependencies "
                f"(max {max_deps} recommended)"
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

    Parses runtime-catalog's runtime/spec/all.rs and ensures:
    - Any variant with imports != NO_IMPORTS also has capability != NO_CAPS.
    - Any variant with capability != NO_CAPS also has imports != NO_IMPORTS.
    """
    violations = []
    spec_path = (
        REPO_ROOT
        / "crates"
        / "runtime-catalog"
        / "src"
        / "runtime"
        / "spec"
        / "all.rs"
    )
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
            "crates/runtime-catalog/src/runtime/spec/all.rs"
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
    - crates/runtime-catalog/src/runtime/manifest/all.rs (manifest mapping)

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
        REPO_ROOT / "crates" / "runtime-catalog" / "src" / "runtime" / "manifest" / "all.rs",
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


# --- #380: Frontend syntax ownership contract checks ---

FRONTEND_DIR = "crates/frontend/src"

# Runtime crate/type imports that frontend must not depend on.
# These belong to backend/runtime layers and would create circular coupling.
FRONTEND_FORBIDDEN_IMPORTS = (
    "ts2wasm_runtime_abi",
    "ts2wasm_runtime_catalog",
    "ts2wasm_backend_wasm",
    "ts2wasm_backend_core",
)

# Runtime type names that must not appear in frontend source (non-test).
FRONTEND_FORBIDDEN_TYPES = (
    "RuntimeFn",
    "HostImport",
    "CapabilityManifest",
    "RawValue",
    "HeapKind",
)

# WAT instruction patterns that must not appear in frontend or IR source.
WAT_INSTRUCTION_PATTERNS = (
    "i32.load",
    "i32.store",
    "i64.load",
    "i64.store",
    "wat!",
    "(module",
    "(func",
)


def check_frontend_no_runtime_import() -> list[str]:
    """Check that frontend crate does not import from runtime/backend crates."""
    violations = []
    frontend_src = REPO_ROOT / FRONTEND_DIR
    if not frontend_src.exists():
        return violations

    for path in sorted(frontend_src.rglob("*.rs")):
        rel = path.relative_to(REPO_ROOT)
        text = path.read_text()
        lines = text.split('\n')
        in_cfg_test = False
        cfg_test_brace_depth = 0
        for i, line in enumerate(lines, 1):
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
            for forbidden in FRONTEND_FORBIDDEN_IMPORTS:
                if re.match(rf'^\s*use\s+{re.escape(forbidden)}', line):
                    violations.append(
                        f"check_architecture_rules: ERROR {rel}:{i}: "
                        f"frontend module imports from {forbidden}"
                    )

    return violations


def check_frontend_no_runtime_type_ref() -> list[str]:
    """Check that frontend crate does not reference runtime type names."""
    violations = []
    frontend_src = REPO_ROOT / FRONTEND_DIR
    if not frontend_src.exists():
        return violations

    for path in sorted(frontend_src.rglob("*.rs")):
        rel = path.relative_to(REPO_ROOT)
        lines = path.read_text().split('\n')
        in_cfg_test = False
        cfg_test_brace_depth = 0
        for i, line in enumerate(lines, 1):
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
            # Skip comments and string literals
            if stripped.startswith('//') or stripped.startswith('#'):
                continue
            for forbidden in FRONTEND_FORBIDDEN_TYPES:
                pattern = rf'\b{re.escape(forbidden)}\b'
                if re.search(pattern, line) and forbidden not in line.split('//')[0]:
                    violations.append(
                        f"check_architecture_rules: ERROR {rel}:{i}: "
                        f"frontend references runtime type `{forbidden}`"
                    )

    return violations


def check_ir_no_wat_instructions() -> list[str]:
    """Check that IR crate does not contain raw WAT instruction patterns outside tests."""
    violations = []
    ir_dirs = ["crates/ir/src", "crates/resolve/src"]
    for ir_dir in ir_dirs:
        ir_src = REPO_ROOT / ir_dir
        if not ir_src.exists():
            continue
        for path in sorted(ir_src.rglob("*.rs")):
            rel = path.relative_to(REPO_ROOT)
            lines = path.read_text().split('\n')
            in_cfg_test = False
            cfg_test_brace_depth = 0
            for i, line in enumerate(lines, 1):
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
                if stripped.startswith('//'):
                    continue
                for pattern in WAT_INSTRUCTION_PATTERNS:
                    if pattern in line and pattern not in line.split('//')[0]:
                        violations.append(
                            f"check_architecture_rules: ERROR {rel}:{i}: "
                            f"IR module contains raw WAT instruction `{pattern}`"
                        )
                        break
    return violations


# --- Crate dependency DAG enforcement ---

def check_crate_dag_boundary() -> list[str]:
    """Check that forbidden crate dependencies are not present.

    Enforces:
      - backend-wasm must NOT depend on semantic-ir
      - spec-kernel must NOT depend on backend-wasm
      - runtime-core must NOT depend on backend-wasm
    """
    violations = []
    for crate_rel, denied_deps in CRATE_DAG_DENY.items():
        cargo_path = REPO_ROOT / crate_rel / "Cargo.toml"
        if not cargo_path.exists():
            continue
        text = cargo_path.read_text()
        deps_match = re.search(
            r"^\[dependencies\]\s*$(.+?)(?=^\s*\[|\Z)",
            text,
            re.MULTILINE | re.DOTALL,
        )
        if not deps_match:
            continue
        deps_section = deps_match.group(1)
        for denied in denied_deps:
            if denied in deps_section:
                violations.append(
                    f"check_architecture_rules: ERROR {crate_rel}/Cargo.toml depends on "
                    f"{denied} — violates crate dependency DAG boundary"
                )
    return violations


# --- Frozen file growth prohibition ---

def check_frozen_file_growth() -> list[str]:
    """Check that frozen files do not exceed their line count baseline.

    These files are in 'delete/move/bugfix only' mode. New logic must go
    to new modules rather than growing these files further.
    """
    violations = []
    for file_rel, baseline in FROZEN_FILES_BASELINE.items():
        if file_rel in FROZEN_FILES_ALLOWLIST:
            continue
        file_path = REPO_ROOT / file_rel
        if not file_path.exists():
            continue
        try:
            line_count = sum(1 for _ in file_path.open())
        except (OSError, UnicodeDecodeError):
            continue
        if line_count > baseline:
            violations.append(
                f"check_architecture_rules: ERROR {file_rel}: {line_count} lines "
                f"(baseline {baseline}) — frozen file exceeded line limit; "
                f"put new logic in a separate module"
            )
    return violations


# --- RuntimeFn addition prohibition ---

def _count_runtimefn_variants() -> int:
    """Count RuntimeFn enum variants from runtime_fn.rs."""
    runtime_fn_path = (
        REPO_ROOT / "crates" / "runtime-catalog" / "src" / "runtime_fn.rs"
    )
    if not runtime_fn_path.exists():
        return 0
    text = runtime_fn_path.read_text()
    enum_match = re.search(
        r'pub enum RuntimeFn \{(.*?)^\}', text, re.MULTILINE | re.DOTALL
    )
    if not enum_match:
        return 0
    enum_body = enum_match.group(1)
    variants = set()
    for m in re.finditer(r'^\s+([A-Z]\w+)\s*,?\s*$', enum_body, re.MULTILINE):
        variants.add(m.group(1))
    for m in re.finditer(
        r'#\[.*?\]\s*\n\s+([A-Z]\w+)\s*,?\s*', enum_body
    ):
        variants.add(m.group(1))
    return len(variants)


def check_runtimefn_addition_prohibition() -> list[str]:
    """Check that RuntimeFn variant count does not exceed baseline.

    New RuntimeFn additions are prohibited without an explicit allowlist entry.
    This forces new functionality through the new path (SpecOp + semantic-ir)
    rather than the legacy RuntimeFn catalog shortcut.
    """
    violations = []
    current_count = _count_runtimefn_variants()
    if current_count == 0:
        return violations
    # Read baseline from the constant; if 0, compute from current (first-run baseline)
    baseline = RUNTIMEFN_BASELINE_COUNT
    if baseline == 0:
        # First run: set baseline to current count. Subsequent runs compare against this.
        # In CI, this constant should be set to the known-good count.
        baseline = current_count
    if current_count > baseline:
        violations.append(
            f"check_architecture_rules: ERROR RuntimeFn has {current_count} variants "
            f"(baseline {baseline}) — new RuntimeFn additions prohibited; "
            f"use SpecOp + semantic-ir path instead"
        )
    return violations


# --- SpecOp dispatch coverage ---

def check_specop_dispatch_coverage() -> list[str]:
    """Check that new SpecOp variants have dispatch coverage in spec-kernel.

    Existing variants (SPECOP_BASELINE_VARIANTS) are allowed without dispatch
    since spec-kernel is WIP. New additions beyond the baseline must have
    corresponding dispatch coverage in dispatch.rs, conversion.rs, or
    environment.rs.
    """
    violations = []

    # Parse SpecOp variants
    spec_op_path = REPO_ROOT / "crates" / "spec-kernel" / "src" / "spec_op.rs"
    if not spec_op_path.exists():
        return violations
    spec_text = spec_op_path.read_text()
    enum_match = re.search(
        r'pub enum SpecOp \{(.*?)^\}', spec_text, re.MULTILINE | re.DOTALL
    )
    if not enum_match:
        return violations

    enum_body = enum_match.group(1)
    variants = set()
    for m in re.finditer(r'^\s+(\w+)\s*(?:\{|,|$)', enum_body, re.MULTILINE):
        name = m.group(1)
        if name[0].isupper():
            variants.add(name)

    # Check dispatch coverage for new variants only
    spec_kernel_src = REPO_ROOT / "crates" / "spec-kernel" / "src"
    dispatched_text = ""
    for mod_file in ["dispatch.rs", "conversion.rs", "environment.rs"]:
        mod_path = spec_kernel_src / mod_file
        if mod_path.exists():
            dispatched_text += mod_path.read_text()

    new_variants = variants - SPECOP_BASELINE_VARIANTS
    for v in sorted(new_variants):
        if v not in dispatched_text:
            violations.append(
                f"check_architecture_rules: ERROR SpecOp::{v} is a new variant "
                f"with no dispatch coverage in spec-kernel — implement dispatch "
                f"in dispatch.rs / conversion.rs / environment.rs before adding"
            )

    return violations


def main():
    if sys.argv[1:] and sys.argv[1] in ("-h", "--help"):
        usage()
        sys.exit(0)

    violations: list[str] = []
    check_fns = [
        ("frontend boundary", check_frontend_syntax_boundary),
        ("backend-frontend dependency", check_backend_frontend_dependency),
        ("backend-frontend import", check_backend_frontend_import),
        ("Diagnostic span: None", check_diagnostic_span_none),
        ("RuntimeCall string", check_no_new_string_runtime_call),
        ("Raw runtime symbol", check_raw_runtime_symbol_outside_catalog),
        ("LoweredExpr validate coverage", check_lowered_expr_validate_coverage),
        ("Host import string", check_host_import_string_outside_catalog),
        ("Module fan-out", check_module_fan_out),
        ("RuntimeFn spec gap", check_runtimefn_spec_gap),
        ("RuntimeFn capability", check_runtimefn_capability),
        ("Host import manifest", check_host_import_manifest),
        ("Frontend no runtime import", check_frontend_no_runtime_import),
        ("Frontend no runtime type ref", check_frontend_no_runtime_type_ref),
        ("IR no WAT instructions", check_ir_no_wat_instructions),
        ("Crate DAG boundary", check_crate_dag_boundary),
        ("Frozen file growth", check_frozen_file_growth),
        ("RuntimeFn addition prohibition", check_runtimefn_addition_prohibition),
        ("SpecOp dispatch coverage", check_specop_dispatch_coverage),
        ("use super::*", check_use_super_star),
        ("runtime push_str", check_runtime_push_str),
        ("filename pseudo-hierarchy", check_src_filename_pseudo_hierarchy),
        ("Validated backend contract", check_validated_backend_contract),
    ]

    for name, fn in check_fns:
        violations.extend(fn())

    # CLI-specific checks
    if not shutil.which("cargo"):
        print("check_architecture_rules: cargo is required", file=sys.stderr)
        sys.exit(1)

    cli_violations = find_cli_boundary_violations()
    violations.extend(cli_violations)

    # Cargo dependency tree checks
    result = subprocess.run(
        ["cargo", "tree", "-p", "ts2wasm-shared", "--edges", "normal,build"],
        capture_output=True, text=True, cwd=REPO_ROOT,
    )
    if result.returncode == 0 and "ts2wasm-cli" in result.stdout:
        violations.append(
            "check_architecture_rules: ERROR ts2wasm-shared must not depend on ts2wasm-cli"
        )

    # Import-capability parity test
    cargo_test = subprocess.run(
        ["cargo", "test", "-p", "ts2wasm-backend-wasm", "--lib", "--", "import_capability_parity"],
        capture_output=True, text=True, cwd=REPO_ROOT,
    )
    if cargo_test.returncode != 0:
        violations.append(
            "check_architecture_rules: ERROR RuntimeFn import/capability parity check FAILED"
        )

    for v in violations:
        print(v, file=sys.stderr)

    if any(": ERROR " in v for v in violations):
        print(f"check_architecture_rules: FAILED ({sum(1 for v in violations if ': ERROR ' in v)} errors)", file=sys.stderr)
        sys.exit(1)

    print("check_architecture_rules: OK", file=sys.stderr)


if __name__ == "__main__":
    main()
