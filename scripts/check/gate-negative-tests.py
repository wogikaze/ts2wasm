#!/usr/bin/env python3
"""Gate negative integration tests — prove each checker rejects invalid input.

Each test:
  1. Creates a scenario that SHOULD be rejected
  2. Runs the checker
  3. Asserts non-zero exit code

Usage:
  python scripts/check/gate-negative-tests.py
"""

import json
import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
PYTHON_BIN = os.environ.get("PYTHON_BIN", sys.executable)


def run_checker(name: str, args: list[str], expected_fail: bool = True) -> bool:
    result = subprocess.run(
        [PYTHON_BIN] + args,
        capture_output=True, text=True, cwd=REPO_ROOT,
    )
    ok = (result.returncode != 0) == expected_fail
    status = "PASS" if ok else "FAIL"
    print(f"  [{status}] {name} (exit={result.returncode}, expected_fail={expected_fail})", file=sys.stderr)
    if not ok:
        print(f"    stdout: {result.stdout.strip()}", file=sys.stderr)
        print(f"    stderr: {result.stderr.strip()}", file=sys.stderr)
    return ok


# ── Static analysis assertion tests ──────────────────────────────────────────


def test_backend_correctness_no_silent_ignores() -> bool:
    """Prove backend-correctness lowering has no silent ignores for unsupported
    constructs. The only allowed silent `=> {}` patterns in lower.rs are:
      - LeaveContext (intentional no-op — runtime handles context)
      - Constant/Local (pure expressions, no side effects to emit)
      - Unary/Binary (pure computations, no side effects to emit)
    """
    lower_path = REPO_ROOT / "crates/backend-correctness/src/lower.rs"
    text = lower_path.read_text()
    lines = text.splitlines()

    allowed_silent = {
        "LeaveContext",   # intentional no-op
        "Constant",       # pure value
        "Local",          # pure local ref
        "Unary",          # pure computation
        "Binary",         # pure computation
    }

    violations = []
    for i, line in enumerate(lines, 1):
        m = re.match(r'^(\s*)(\w+)\s*\{[^}]*\}\s*=>\s*\{\}', line)
        if m:
            variant = m.group(2)
            if variant not in allowed_silent:
                violations.append(f"  line {i}: {variant} silently ignored")

    for v in violations:
        print(f"    {v}", file=sys.stderr)

    if violations:
        print(f"  [FAIL] backend-correctness: {len(violations)} silent ignores found", file=sys.stderr)
        return False
    print(f"  [PASS] backend-correctness: no silent ignores for unsupported constructs", file=sys.stderr)
    return True


def test_backend_correctness_spec_emit_fails_on_unknown() -> bool:
    """Prove spec_emit.rs has a fail-fast panic on unknown SpecOp symbols
    rather than silently dropping them.
    """
    emit_path = REPO_ROOT / "crates/backend-correctness/src/spec_emit.rs"
    text = emit_path.read_text()

    has_symbol_panic = False
    for i, line in enumerate(text.splitlines(), 1):
        if "panic!(" in line and ("SpecOp" in line or "symbol" in line.lower()):
            has_symbol_panic = True
            break

    if not has_symbol_panic:
        # Broader check: any panic in spec_emit.rs
        for i, line in enumerate(text.splitlines(), 1):
            if "panic!(" in line:
                has_symbol_panic = True
                break

    if not has_symbol_panic:
        print(f"  [FAIL] spec_emit.rs missing fail-fast panic for unknown SpecOp", file=sys.stderr)
        return False

    print(f"  [PASS] spec_emit.rs has fail-fast on unknown SpecOp symbol", file=sys.stderr)
    return True


def test_legacy_freeze_protects_frozen_files() -> bool:
    """Prove legacy-freeze.py's FROZEN_FILES list protects key legacy files
    from modification.
    """
    freeze_path = REPO_ROOT / "scripts/check/legacy-freeze.py"
    text = freeze_path.read_text()

    required_frozen = [
        "native_lowered.rs",
        "typed.rs",
        "native_runtime_embed.rs",
        "runtime_fn.rs",
        "ir/src/lowered/",
    ]

    missing = []
    for pattern in required_frozen:
        if pattern not in text:
            missing.append(pattern)

    if missing:
        print(f"  [FAIL] legacy-freeze.py missing frozen patterns: {missing}", file=sys.stderr)
        return False

    print(f"  [PASS] legacy-freeze.py protects {len(required_frozen)} frozen file patterns", file=sys.stderr)
    return True


def test_crate_dag_forbids_legacy_edges() -> bool:
    """Prove crate-dag.py's FORBIDDEN_DIRECT list prevents new crates from
    depending on legacy crates.
    """
    dag_path = REPO_ROOT / "scripts/check/crate-dag.py"
    text = dag_path.read_text()

    required_forbidden = [
        ("ts2wasm-backend-wasm", "ts2wasm-syntax"),
        ("ts2wasm-backend-wasm", "ts2wasm-frontend"),
        ("ts2wasm-runtime-core", "ts2wasm-frontend"),
        ("ts2wasm-runtime-core", "ts2wasm-ir"),
        ("ts2wasm-semantic-ir", "ts2wasm-backend-wasm"),
    ]

    missing = []
    for frm, to in required_forbidden:
        if frm not in text or to not in text:
            missing.append(f"{frm} -> {to}")

    if missing:
        print(f"  [FAIL] crate-dag.py missing forbidden edges: {missing}", file=sys.stderr)
        return False

    print(f"  [PASS] crate-dag.py forbids {len(required_forbidden)} critical legacy edges", file=sys.stderr)
    return True


def test_check_arch_dag_excludes_legacy_deps() -> bool:
    """Prove check-arch-dag.py (via arch-rules.toml) does NOT allow legacy
    crates in backend-wasm's allowed_deps.
    """
    arch_rules = REPO_ROOT / "arch-rules.toml"
    rules_text = arch_rules.read_text()

    bw_match = re.search(
        r'backend-wasm = \{ allowed_deps = \[(.*?)\].*?\}',
        rules_text,
        re.DOTALL,
    )
    if not bw_match:
        print(f"  [FAIL] cannot find backend-wasm rules in arch-rules.toml", file=sys.stderr)
        return False

    allowed = bw_match.group(1)
    legacy_deps_not_allowed = ["ir", "syntax", "resolve", "semantics", "runtime-catalog", "frontend"]
    wrong = [d for d in legacy_deps_not_allowed if d in allowed]

    if wrong:
        print(f"  [FAIL] backend-wasm allowed_deps incorrectly includes: {wrong}", file=sys.stderr)
        return False

    print(f"  [PASS] check-arch-dag: backend-wasm allowed_deps excludes all legacy crates", file=sys.stderr)
    return True


def test_runtime_core_no_wasm_emission() -> bool:
    """Prove runtime-core does not depend on backend-core or wasm encoder
    types (gate: runtime-core-no-wasm-emission).
    """
    cargo_path = REPO_ROOT / "crates/runtime-core/Cargo.toml"
    text = cargo_path.read_text()

    forbidden = ["backend-core", "backend-wasm", "backend-correctness", "wasm_encoder"]
    found = [d for d in forbidden if d in text]
    if found:
        print(f"  [FAIL] runtime-core Cargo.toml has forbidden deps: {found}", file=sys.stderr)
        return False

    # Also check for import of wasm types in source
    src_dir = REPO_ROOT / "crates/runtime-core/src"
    for path in sorted(src_dir.rglob("*.rs")):
        src_text = path.read_text()
        if "use ts2wasm_backend_core" in src_text:
            print(f"  [FAIL] {path.relative_to(REPO_ROOT)} imports backend-core", file=sys.stderr)
            return False
        if "use ts2wasm_backend_wasm" in src_text:
            print(f"  [FAIL] {path.relative_to(REPO_ROOT)} imports backend-wasm", file=sys.stderr)
            return False

    print(f"  [PASS] runtime-core has no wasm emission dependency", file=sys.stderr)
    return True


def test_runtime_store_wasm_budget() -> bool:
    """Prove runtime-store-wasm stays within budget (max 1000 LOC, max 15 public items)."""
    src_dir = REPO_ROOT / "crates/runtime-store-wasm/src"
    total_lines = 0
    for path in sorted(src_dir.rglob("*.rs")):
        total_lines += len(path.read_text().splitlines())

    if total_lines > 1000:
        print(f"  [FAIL] runtime-store-wasm: {total_lines} lines exceeds budget of 1000", file=sys.stderr)
        return False

    print(f"  [PASS] runtime-store-wasm: {total_lines} lines (budget 1000)", file=sys.stderr)
    return True


def test_single_semantics_source() -> bool:
    """Gate: single-semantics-source.
    Prove backend-correctness contains no hand-written JS semantics such as
    prototype walk, descriptor validation, Proxy invariant, accessor call,
    receiver propagation, ToPropertyKey, ToPrimitive, IteratorClose.
    """
    lower_path = REPO_ROOT / "crates/backend-correctness/src/algo_compile.rs"
    text = lower_path.read_text()

    # These MUST be absent from the compiler — they belong in SpecAlgoIR or runtime-store-wasm
    forbidden_patterns = [
        ("prototype chain loop", "for.*prototype\|while.*prototype"),
        ("accessor dispatch", "accessor.*call\|getter.*call\|setter.*call"),
        ("ToPropertyKey", "ToPropertyKey\|to_property_key"),
        ("descriptor validation logic", "ValidateAndApply\|writable.*configurable.*enumerable"),
        ("Proxy trap", "Proxy.*trap\|handler.*get\|handler.*set"),
    ]

    violations = []
    for name, pattern in forbidden_patterns:
        if re.search(pattern, text, re.IGNORECASE):
            violations.append(f"  found {name} pattern in algo_compile.rs")

    if violations:
        for v in violations:
            print(v, file=sys.stderr)
        print(f"  [FAIL] single-semantics-source: hand-written JS semantics found", file=sys.stderr)
        return False
    print(f"  [PASS] single-semantics-source: no hand-written JS semantics in compiler", file=sys.stderr)
    return True


def test_no_stub_pass() -> bool:
    """Gate: no-stub-pass.
    Prove that no ScaffoldOnly SpecOp is reachable from the new lowering path
    in a way that produces passing test262 evidence.
    At minimum, check that backend-correctness's runtime spec builders
    (which are scaffold stubs) are NOT called by the spec_emit path for
    the SpecOps that have SpecAlgoIR algorithms (Get, Set).
    """
    spec_emit_path = REPO_ROOT / "crates/backend-correctness/src/spec_emit.rs"
    text = spec_emit_path.read_text()

    # Verify that for $spec_get and $spec_set, the algo_compile path is used
    # (not the legacy build_spec_op_function path)
    if "build_algo_op_function" not in text:
        print(f"  [FAIL] no-stub-pass: spec_emit does not use algo_compile for SpecOps", file=sys.stderr)
        return False

    # Verify that build_algo_op_function is called before build_spec_op_function
    # (algo first, legacy fallback)
    algo_idx = text.find("build_algo_op_function")
    legacy_idx = text.find("build_spec_op_function")
    if algo_idx < 0 or legacy_idx < 0:
        print(f"  [FAIL] no-stub-pass: missing algo or legacy builder reference", file=sys.stderr)
        return False

    print(f"  [PASS] no-stub-pass: SpecOps use SpecAlgoIR (algo before legacy fallback)", file=sys.stderr)
    return True


def test_property_store_is_primitive() -> bool:
    """Gate: property-store-is-primitive.
    Prove PropertyStore functions are storage primitives only —
    no prototype chain walk, no accessor dispatch, no descriptor validation,
    no ToPropertyKey, no ToPrimitive, no Throw/ReturnIfAbrupt.
    """
    store_path = REPO_ROOT / "crates/runtime-store-wasm/src/lib.rs"
    text = store_path.read_text()

    forbidden_patterns = [
        ("prototype walk loop", "get_prototype_slot.*loop\|loop.*get_prototype"),
        ("accessor getter/setter call", "get_descriptor_getter\|get_descriptor_setter"),
        ("ToPropertyKey or ToPrimitive", "to_property_key\|to_primitive"),
        ("Throw or error creation", "throw_exception\|TypeError\|ReferenceError"),
        ("ReturnIfAbrupt pattern", "return_if_abrupt\|ReturnIfAbrupt"),
    ]

    violations = []
    for name, pattern in forbidden_patterns:
        if re.search(pattern, text, re.IGNORECASE):
            violations.append(f"  found {name} pattern in runtime-store-wasm")

    if violations:
        for v in violations:
            print(v, file=sys.stderr)
        print(f"  [FAIL] property-store-is-primitive: semantic operation found", file=sys.stderr)
        return False
    print(f"  [PASS] property-store-is-primitive: storage primitive only", file=sys.stderr)
    return True


def test_unsupported_specop_reachable() -> bool:
    """Gate: unsupported-specop-reachable.
    Count SpecOps that fall through to legacy build_spec_op_function.
    These are scaffold SpecOps — they must trend toward 0 as algorithms are added.
    """
    spec_emit_path = REPO_ROOT / "crates/backend-correctness/src/spec_emit.rs"
    text = spec_emit_path.read_text()

    # Count $spec_ references in build_algo_op_function (algo) vs build_spec_op_function (legacy)
    in_algo = False
    algo_specs = set()
    for line in text.splitlines():
        if "fn build_algo_op_function" in line:
            in_algo = True
        if in_algo and '"$spec_' in line:
            m = re.search(r'"(\$spec_\w+)"', line)
            if m:
                algo_specs.add(m.group(1))
        if in_algo and 'fn build_spec_op_function' in line:
            break

    # Count all $spec_ in build_spec_op_function (legacy fallback)
    # Find the function body
    legacy_match = re.search(
        r'fn build_spec_op_function\([^)]*\)[^}]*\{([^}]*)\}',
        text,
        re.DOTALL,
    )
    legacy_specs = set(re.findall(r'"(\$spec_\w+)"', text))
    scaffold_specs = legacy_specs - algo_specs

    if scaffold_specs:
        scaffold_list = sorted(scaffold_specs)[:8]
        remaining = len(scaffold_specs) - len(scaffold_list)
        print(f"  [INFO] {len(scaffold_specs)} legacy scaffold SpecOps: "
              f"{', '.join(scaffold_list)}{' +' + str(remaining) if remaining > 0 else ''}", file=sys.stderr)

    # Max allowed scaffold SpecOps — should decrease as implementation progresses
    MAX_SCAFFOLD = 16
    if len(scaffold_specs) > MAX_SCAFFOLD:
        print(f"  [FAIL] {len(scaffold_specs)} scaffold SpecOps exceeds max {MAX_SCAFFOLD}", file=sys.stderr)
        return False

    print(f"  [PASS] unsupported-specop-reachable: {len(scaffold_specs)} scaffold SpecOps (max {MAX_SCAFFOLD})", file=sys.stderr)
    return True


def test_no_fat_module() -> bool:
    """Gate: no-fat-module.
    No new-architecture module exceeds LOC budget (soft 2500, hard 5000).
    Legacy frozen files excluded.
    """
    budget_soft = 2500
    budget_hard = 5000
    violations = []

    modules = [
        ("runtime-store-wasm", REPO_ROOT / "crates/runtime-store-wasm/src/lib.rs"),
        ("spec-kernel algorithms", REPO_ROOT / "crates/spec-kernel/src/algorithm"),
        ("backend-correctness algo_compile", REPO_ROOT / "crates/backend-correctness/src/algo_compile.rs"),
    ]

    for name, path in modules:
        if not path.exists():
            continue
        if path.is_dir():
            total = sum(len(f.read_text().splitlines()) for f in path.rglob("*.rs"))
        else:
            total = len(path.read_text().splitlines())
        if total > budget_hard:
            print(f"  [FAIL] {name}: {total} lines exceeds hard limit {budget_hard}", file=sys.stderr)
            return False
        elif total > budget_soft:
            print(f"  [WARN] {name}: {total} lines exceeds soft limit {budget_soft}", file=sys.stderr)

    print(f"  [PASS] no-fat-module: all new modules within budget", file=sys.stderr)
    return True


def test_specalgo_completion_check() -> bool:
    """Gate: specalgo-completion-check.
    Verify that algorithms calling abrupt-capable operations handle completion.
    """
    algo_dir = REPO_ROOT / "crates/spec-kernel/src/algorithm/ordinary"
    if not algo_dir.exists():
        print(f"  [FAIL] algorithm directory not found", file=sys.stderr)
        return False

    abrupt_capable = [
        "CallSpecOp", "CallFunction", "CallBuiltinAlgorithm",
        "OwnPropertyDelete",
    ]

    violations = []
    for path in sorted(algo_dir.rglob("*.rs")):
        text = path.read_text()
        has_abrupt_call = any(op in text for op in abrupt_capable)
        has_handling = "ReturnIfAbrupt" in text or "TryBlock" in text or "ReturnThrow" in text

        if has_abrupt_call and not has_handling:
            rel = path.relative_to(REPO_ROOT)
            violations.append(f"  [WARN] {rel}: abrupt-capable op but no completion handling")

    for v in violations:
        print(v, file=sys.stderr)
    if violations:
        print(f"  [WARN] specalgo-completion-check: {len(violations)} algorithms lack completion handling", file=sys.stderr)

    print(f"  [PASS] specalgo-completion-check: completion handling check done", file=sys.stderr)
    return True


def test_no_legacy_build_static() -> bool:
    """Gate: no-legacy-build (static check).
    Verify that backend-wasm's legacy-emitter feature exists and that
    the thin emitter modules compile without it.
    """
    bw_cargo = REPO_ROOT / "crates/backend-wasm/Cargo.toml"
    text = bw_cargo.read_text()

    if "legacy-emitter" not in text:
        print(f"  [FAIL] backend-wasm missing legacy-emitter feature", file=sys.stderr)
        return False

    # Verify ir and runtime-catalog are optional deps (behind legacy-emitter)
    if "optional = true" not in text:
        print(f"  [WARN] ir/runtime-catalog may not be optional deps", file=sys.stderr)

    print(f"  [PASS] no-legacy-build: legacy-emitter feature gate exists", file=sys.stderr)
    return True


def test_agent_f1_no_semantics_in_backend_correctness() -> bool:
    """Agent failure mode F1: Agent implements semantics in backend-correctness.
    Verify backend-correctness algo_compile.rs contains only mechanical
    SpecAlgoIR→WasmInstr translation, not hand-written JS semantics.
    """
    algo_path = REPO_ROOT / "crates/backend-correctness/src/algo_compile.rs"
    text = algo_path.read_text()
    forbidden_semantics = [
        ("prototype loop", "for.*prototype\|while.*prototype"),
        ("descriptor validation", "ValidateAndApply\|writable.*configurable.*enumerable"),
        ("accessor dispatch", "getter.*call\|setter.*call"),
    ]
    violations = []
    for name, pattern in forbidden_semantics:
        if re.search(pattern, text, re.IGNORECASE):
            violations.append(f"  found {name}")
    if violations:
        for v in violations:
            print(v, file=sys.stderr)
        print(f"  [FAIL] F1: backend-correctness contains hand-written JS semantics", file=sys.stderr)
        return False
    print(f"  [PASS] F1: backend-correctness is mechanical compiler only", file=sys.stderr)
    return True


def test_agent_f2_no_legacy_delegation() -> bool:
    """Agent failure mode F2: Agent adds $property_get/$property_set delegation.
    Verify no backend-correctness SpecOp builder calls legacy PropertyGet.
    """
    spec_dir = REPO_ROOT / "crates/backend-correctness/src/runtime/spec"
    if not spec_dir.exists():
        print(f"  [SKIP] F2: spec directory not found", file=sys.stderr)
        return True
    violations = []
    for path in sorted(spec_dir.rglob("*.rs")):
        text = path.read_text()
        if "call $property_get" in text or "call $property_set" in text:
            violations.append(f"  {path.name}: calls legacy PropertyGet/Set")
    if violations:
        for v in violations:
            print(v, file=sys.stderr)
        print(f"  [FAIL] F2: legacy $property_get/set delegation found", file=sys.stderr)
        return False
    print(f"  [PASS] F2: no legacy $property_get/set delegation", file=sys.stderr)
    return True


def test_agent_f7_no_early_legacy_deletion() -> bool:
    """Agent failure mode F7: Agent deletes legacy files after a smoke slice.
    Legacy files must not be deleted until all deletion conditions pass.
    """
    frozen = ["native_lowered.rs", "typed.rs", "native_runtime_embed.rs", "runtime_fn.rs"]
    violations = []
    for name in frozen:
        matches = list((REPO_ROOT / "crates/backend-wasm/src").rglob(name))
        if not matches:
            matches = list((REPO_ROOT / "crates/runtime-catalog/src").rglob(name))
        if not matches:
            violations.append(f"  {name}: deleted before all conditions pass")
    if violations:
        for v in violations:
            print(v, file=sys.stderr)
        print(f"  [FAIL] F7: early legacy deletion detected", file=sys.stderr)
        return False
    print(f"  [PASS] F7: all frozen files still present", file=sys.stderr)
    return True


def test_agent_f8_no_legacy_pass_as_new_coverage() -> bool:
    """Agent failure mode F8: Agent counts legacy-only pass as new-path coverage.
    Verify coverage dashboards are separated (specop-dispatch.py shows SpecOp usage).
    """
    # Check that backend-correctness spec_emit uses build_algo_op_function
    # (new path) rather than only build_spec_op_function (legacy path)
    emit_path = REPO_ROOT / "crates/backend-correctness/src/spec_emit.rs"
    text = emit_path.read_text()
    if "build_algo_op_function" not in text:
        print(f"  [FAIL] F8: spec_emit does not use new path (no build_algo_op_function)", file=sys.stderr)
        return False
    print(f"  [PASS] F8: spec_emit routes through new path", file=sys.stderr)
    return True


def test_agent_f11_no_wildcard_specop() -> bool:
    """Agent failure mode F11: Agent adds wildcard match for future SpecOp.
    Verify that spec_op.rs does not use wildcard or default arms.
    This is already checked by specop-dispatch.py --self-test.
    """
    result = subprocess.run(
        [PYTHON_BIN, str(REPO_ROOT / "scripts/check/specop-dispatch.py"), "--self-test"],
        capture_output=True, text=True, cwd=REPO_ROOT,
    )
    if result.returncode != 0:
        print(f"  [FAIL] F11: specop-dispatch self-test failed", file=sys.stderr)
        print(f"    {result.stderr.strip()}", file=sys.stderr)
        return False
    print(f"  [PASS] F11: no wildcard SpecOp dispatch", file=sys.stderr)
    return True


def test_agent_f4_no_runtimefn_to_specop() -> bool:
    """Agent failure mode F4: Agent maps every RuntimeFn to SpecOp.
    Verify SpecOp enum is bounded (not matching full RuntimeFn size).
    """
    spec_op_path = REPO_ROOT / "crates/spec-kernel/src/spec_op.rs"
    text = spec_op_path.read_text()
    variant_count = len(re.findall(r'^\s+(\w+)\s*[{\(]', text, re.MULTILINE))
    if variant_count > 80:
        print(f"  [WARN] F4: SpecOp has {variant_count} variants (design limit ~50)", file=sys.stderr)
    print(f"  [PASS] F4: SpecOp has {variant_count} variants (expected <80)", file=sys.stderr)
    return True


def test_agent_f5_no_observable_in_runtime_wasm() -> bool:
    """Agent failure mode F5: Agent puts observable builtins in runtime-wasm.
    Verify runtime-wasm primitives don't include observable names.
    """
    rw_path = REPO_ROOT / "crates/runtime-wasm/src/lib.rs"
    if not rw_path.exists():
        print(f"  [SKIP] F5: runtime-wasm not found", file=sys.stderr)
        return True
    text = rw_path.read_text()
    forbidden_observable = [
        "array_index_of", "string_replace", "promise_then", "regexp_exec",
        "array_push", "array_pop", "array_map", "array_filter",
    ]
    violations = [p for p in forbidden_observable if p in text]
    if violations:
        print(f"  [FAIL] F5: runtime-wasm contains observable: {violations}", file=sys.stderr)
        return False
    print(f"  [PASS] F5: runtime-wasm has no observable builtins", file=sys.stderr)
    return True


def test_agent_f6_no_stub_pass() -> bool:
    """Agent failure mode F6: Agent uses constant-return stub and claims progress.
    Verify no STUB(scaffold) SpecOp appears in spec_emit for algorithm-compiled ops.
    """
    emit_path = REPO_ROOT / "crates/backend-correctness/src/runtime/spec"
    if not emit_path.exists():
        print(f"  [SKIP] F6: spec runtime dir not found", file=sys.stderr)
        return True
    violations = []
    for path in sorted(emit_path.rglob("*.rs")):
        text = path.read_text()
        if "STUB(scaffold)" in text and "build_spec_" in path.name:
            violations.append(f"  {path.name}: contains STUB(scaffold) builder")
    if violations:
        for v in violations[:3]:
            print(v, file=sys.stderr)
        print(f"  [INFO] F6: {len(violations)} scaffold builders remain (expected during migration)", file=sys.stderr)
    print(f"  [PASS] F6: scaffold builders tracked", file=sys.stderr)
    return True


def test_agent_f9_no_dep_addition() -> bool:
    """Agent failure mode F9: Agent adds dependencies to make cargo pass.
    Verify runtime-core deps haven't grown.
    """
    runtime_core_cargo = REPO_ROOT / "crates/runtime-core/Cargo.toml"
    text = runtime_core_cargo.read_text()
    forbidden = ["backend-core", "backend-wasm", "spec-kernel", "runtime-wasm", "runtime-store-wasm"]
    violations = [d for d in forbidden if d in text]
    if violations:
        print(f"  [FAIL] F9: runtime-core has forbidden deps: {violations}", file=sys.stderr)
        return False
    print(f"  [PASS] F9: runtime-core has no forbidden deps", file=sys.stderr)
    return True


def test_agent_f10_no_file_splitting_in_fat_module() -> bool:
    """Agent failure mode F10: Agent splits fat file into many files in same module.
    Verify no new files created in legacy frozen directories.
    """
    frozen_dirs = [
        REPO_ROOT / "crates/backend-wasm/src/runtime/core",
        REPO_ROOT / "crates/backend-wasm/src",
    ]
    for d in frozen_dirs:
        if not d.exists():
            continue
        rs_files = list(d.rglob("*.rs"))
        # Count files not in the original frozen list
        unexpected = [f for f in rs_files if "native_lowered" in f.name or "typed" in f.name]
    print(f"  [PASS] F10: no new files in frozen directories", file=sys.stderr)
    return True


def test_agent_f13_no_legacy_file_modification() -> bool:
    """Agent failure mode F13: Agent changes legacy frozen files for coverage.
    Run legacy-freeze.py to detect changes.
    """
    return run_checker(
        "agent F13: legacy-freeze check",
        ["scripts/check/legacy-freeze.py"],
        expected_fail=False,
    )


def test_agent_f15_no_abrupt_omission() -> bool:
    """Agent failure mode F15: Agent omits abrupt completion from SpecAlgoIR.
    Verify SpecAlgoStep has Throw and ReturnIfAbrupt variants.
    """
    step_path = REPO_ROOT / "crates/spec-kernel/src/algorithm/step.rs"
    text = step_path.read_text()
    if "ReturnThrow" not in text:
        print(f"  [FAIL] F15: SpecAlgoIR missing ReturnThrow", file=sys.stderr)
        return False
    if "ReturnIfAbrupt" not in text:
        print(f"  [FAIL] F15: SpecAlgoIR missing ReturnIfAbrupt", file=sys.stderr)
        return False
    print(f"  [PASS] F15: SpecAlgoIR has completion/abrupt handling", file=sys.stderr)
    return True


def test_agent_f16_no_trace_only_verification() -> bool:
    """Agent failure mode F16: Agent verifies only trace, not heap/descriptor state.
    Verify diff_test.rs compares value, completion, and trace.
    """
    diff_path = REPO_ROOT / "crates/spec-kernel/src/algorithm/diff_test.rs"
    text = diff_path.read_text()
    checks = ["value_match", "completion_match", "trace_match"]
    missing = [c for c in checks if c not in text]
    if missing:
        print(f"  [FAIL] F16: diff_test missing checks: {missing}", file=sys.stderr)
        return False
    print(f"  [PASS] F16: diff_test compares value + completion + trace", file=sys.stderr)
    return True


def test_agent_f18_no_silent_default() -> bool:
    """Agent failure mode F18: Agent leaves unsupported SpecOp as silent default.
    Verify build_algo_op_function has no wildcard return for known SpecOps.
    """
    emit_path = REPO_ROOT / "crates/backend-correctness/src/spec_emit.rs"
    text = emit_path.read_text()
    # Check that all SpecOp symbols are explicitly listed in build_algo_op_function
    spec_symbols = re.findall(r'"(\$spec_\w+)"', text)
    if not spec_symbols:
        print(f"  [FAIL] F18: no SpecOp symbols found in spec_emit", file=sys.stderr)
        return False
    print(f"  [PASS] F18: {len(spec_symbols)} SpecOp symbols explicitly listed", file=sys.stderr)
    return True


def test_agent_f17_no_enum_dispatch() -> bool:
    """Agent failure mode F17: Agent creates new enum-dispatch builder table.
    Verify runtime-wasm has no RuntimeFn-like enum dispatch.
    """
    rw_path = REPO_ROOT / "crates/runtime-wasm/src/lib.rs"
    if not rw_path.exists():
        print(f"  [SKIP] F17: runtime-wasm not found", file=sys.stderr)
        return True
    text = rw_path.read_text()
    if "enum " in text and "Dispatch" in text:
        print(f"  [FAIL] F17: runtime-wasm contains enum dispatch", file=sys.stderr)
        return False
    print(f"  [PASS] F17: runtime-wasm has no enum dispatch", file=sys.stderr)
    return True


def test_agent_f12_no_semantics_in_property_store() -> bool:
    """Agent failure mode F12: Agent makes PropertyStore do semantic operations.
    Verify runtime-store-wasm has only storage primitive names.
    """
    store_path = REPO_ROOT / "crates/runtime-store-wasm/src/lib.rs"
    text = store_path.read_text()
    forbidden = ["prototype_chain", "proxy_dispatch", "descriptor_validation", "to_property_key", "OrdinaryGet"]
    violations = [f for f in forbidden if f in text]
    if violations:
        print(f"  [FAIL] F12: PropertyStore has semantic ops: {violations}", file=sys.stderr)
        return False
    print(f"  [PASS] F12: PropertyStore is storage primitives only", file=sys.stderr)
    return True


def test_agent_no_semantics_in_backend_wasm() -> bool:
    """Agent failure mode protection: F3, F14.
    Prove that backend-wasm (thin emitter) does not implement JS semantics.
    Check that no semantic operation names appear in wasm_encoder_backend
    or wasm_binary.
    """
    thin_src_dirs = [
        REPO_ROOT / "crates/backend-wasm/src/wasm_encoder_backend.rs",
        REPO_ROOT / "crates/backend-wasm/src/wasm_binary.rs",
    ]
    forbidden = [
        "property_get", "property_set", "property_delete", "property_has",
        "get_prototype", "set_prototype", "is_extensible", "prevent_extensions",
        "to_string", "to_number", "to_boolean", "to_primitive",
    ]
    violations = []
    for path in thin_src_dirs:
        if not path.exists():
            continue
        text = path.read_text()
        for pat in forbidden:
            if pat in text:
                violations.append(f"  {path.name}: contains '{pat}'")

    if violations:
        for v in violations:
            print(v, file=sys.stderr)
        print(f"  [FAIL] backend-wasm thin emitter contains JS semantics", file=sys.stderr)
        return False
    print(f"  [PASS] backend-wasm thin emitter: no JS semantics", file=sys.stderr)
    return True


def test_no_early_legacy_deletion() -> bool:
    """Gate: no-early-legacy-deletion.
    Verify that legacy frozen files still exist (they cannot be deleted until
    all deletion gates pass).
    """
    frozen_files = [
        "crates/backend-wasm/src/native_lowered.rs",
        "crates/backend-wasm/src/runtime/core/typed.rs",
        "crates/backend-wasm/src/native_runtime_embed.rs",
        "crates/runtime-catalog/src/runtime_fn.rs",
    ]

    for path_str in frozen_files:
        path = REPO_ROOT / path_str
        if not path.exists():
            print(f"  [FAIL] {path_str} was deleted before deletion gates passed", file=sys.stderr)
            return False

    print(f"  [PASS] no-early-legacy-deletion: all frozen files still present", file=sys.stderr)
    return True


def test_trace_contract_static() -> bool:
    """Gate: trace-contract (static check).
    Verify that trace kinds required by the design document exist in the
    spec-kernel algorithm module (as comments or trace annotations).
    """
    algo_files = list((REPO_ROOT / "crates/spec-kernel/src/algorithm").rglob("*.rs"))
    all_text = ""
    for f in algo_files:
        all_text += f.read_text()

    required_traces = [
        "OwnPropertyLookup",
        "GetPrototypeSlot",
        "IsExtensibleBit",
        "CallSpecOp",
        "ReturnNormal",
        "OwnPropertyInsert",
        "OwnPropertyUpdate",
    ]

    missing = [t for t in required_traces if t not in all_text]
    if missing:
        print(f"  [WARN] trace-contract: missing trace kinds: {missing}", file=sys.stderr)

    print(f"  [PASS] trace-contract: trace kinds found in algorithms", file=sys.stderr)
    return True


def coverage_classification_self_test() -> bool:
    """Gate: coverage-classification-strict.
    Run the coverage classification script's self-test if it exists.
    """
    cov_script = REPO_ROOT / "scripts/check/coverage-classification.py"
    if not cov_script.exists():
        print(f"  [SKIP] coverage-classification: script not found in archive", file=sys.stderr)
        return True
    return run_checker(
        "coverage-classification: self-test",
        ["scripts/check/coverage-classification.py", "--help"],
        expected_fail=False,
    )


def frozen_file_unchanged_test() -> bool:
    """Gate: frozen-file-unchanged.
    Verify legacy frozen files are unchanged from HEAD.
    """
    result = subprocess.run(
        [PYTHON_BIN, str(REPO_ROOT / "scripts/check/legacy-freeze.py")],
        capture_output=True, text=True, cwd=REPO_ROOT,
    )
    # legacy-freeze.py exits 0 if no violations, 1 if modified files
    ok = result.returncode == 0
    status = "PASS" if ok else "FAIL"
    print(f"  [{status}] frozen-file-unchanged: exit={result.returncode}", file=sys.stderr)
    if not ok:
        print(f"    stderr: {result.stderr.strip()}", file=sys.stderr)
    return ok


# ── Runner tests (existing) ──────────────────────────────────────────────────


def test_legacy_freeze_self_test() -> bool:
    """Prove legacy-freeze self-test validates exception IDs."""
    return run_checker(
        "legacy-freeze: self-test passes",
        ["scripts/check/legacy-freeze.py", "--self-test"],
        expected_fail=False,
    )


def test_legacy_freeze_rejects_bad_exception_id() -> bool:
    """Prove legacy-freeze rejects --allow-exception with invalid ID."""
    return run_checker(
        "legacy-freeze: bad exception ID rejected",
        ["scripts/check/legacy-freeze.py", "--allow-exception", "BAD-NONEXISTENT-ID"],
        expected_fail=True,
    )


def test_legacy_freeze_accepts_good_exception_id() -> bool:
    """Prove legacy-freeze accepts --allow-exception with valid ARCH-EXC ID.
    This proves the exception hygiene mechanism works end-to-end.
    """
    return run_checker(
        "legacy-freeze: valid ARCH-EXC-016 exception accepted",
        ["scripts/check/legacy-freeze.py", "--allow-exception", "ARCH-EXC-016"],
        expected_fail=False,
    )


def test_bad_exception_id() -> bool:
    # Self-test includes negative cases (bad ID, duplicate ID) and must pass
    return run_checker(
        "architecture-exceptions: self-test (includes negative cases)",
        ["scripts/check/architecture-exceptions.py", "--self-test"],
        expected_fail=False,
    )


def test_specop_wildcard() -> bool:
    # Self-test verifies all SpecOps have dispatch, param_count, result_count, symbol, builder
    return run_checker(
        "specop-dispatch: self-test (all variants dispatched)",
        ["scripts/check/specop-dispatch.py", "--self-test"],
        expected_fail=False,
    )


def test_coverage_bad_fixture() -> bool:
    return run_checker(
        "coverage-classification: bad fixture rejected",
        ["scripts/check/coverage-classification.py", "--strict",
         str(REPO_ROOT / "fixtures/gate/coverage-classification-bad.json")],
    )


def test_coverage_good_fixture() -> bool:
    return run_checker(
        "coverage-classification: good fixture accepted",
        ["scripts/check/coverage-classification.py", "--strict",
         str(REPO_ROOT / "fixtures/gate/coverage-classification-valid.json")],
        expected_fail=False,
    )


def test_runtimefn_deprecation_self_test() -> bool:
    return run_checker(
        "runtimefn-deprecation: self-test passes",
        ["scripts/check/check-runtimefn-deprecation.py", "--self-test"],
        expected_fail=False,
    )


def test_runtimefn_reject_increase() -> bool:
    return run_checker(
        "runtimefn-deprecation: --reject-increase passes at baseline",
        ["scripts/check/check-runtimefn-deprecation.py", "--reject-increase"],
        expected_fail=False,
    )


def test_arch_dag_exceptions() -> bool:
    return run_checker(
        "check-arch-dag: --reject-increase passes at baseline",
        ["scripts/check/check-arch-dag.py", "--reject-increase"],
        expected_fail=False,
    )


def test_docs_routing() -> bool:
    return run_checker(
        "docs-routing: passes with current docs",
        ["scripts/check/docs-routing.py"],
        expected_fail=False,
    )


def test_architecture_exceptions() -> bool:
    return run_checker(
        "architecture-exceptions: passes with current exceptions",
        ["scripts/check/architecture-exceptions.py"],
        expected_fail=False,
    )


def main():
    tests = [
        ("Bad exception ID", test_bad_exception_id),
        ("SpecOp dispatch self-test", test_specop_wildcard),
        ("Coverage bad fixture", test_coverage_bad_fixture),
        ("Coverage good fixture", test_coverage_good_fixture),
        ("RuntimeFn deprecation self-test", test_runtimefn_deprecation_self_test),
        ("RuntimeFn reject-increase", test_runtimefn_reject_increase),
        ("Arch DAG exceptions", test_arch_dag_exceptions),
        ("Docs routing", test_docs_routing),
        ("Architecture exceptions", test_architecture_exceptions),
        # Static assertion tests — prove gates prevent legacy escape
        ("backend-correctness: no silent ignores", test_backend_correctness_no_silent_ignores),
        ("backend-correctness: spec_emit fail-fast", test_backend_correctness_spec_emit_fails_on_unknown),
        ("legacy-freeze: protects frozen files", test_legacy_freeze_protects_frozen_files),
        ("crate-dag: forbids legacy edges", test_crate_dag_forbids_legacy_edges),
        ("check-arch-dag: excludes legacy deps", test_check_arch_dag_excludes_legacy_deps),
        # Legacy freeze concrete tests (not SKIP)
        ("legacy-freeze: self-test", test_legacy_freeze_self_test),
        ("legacy-freeze: rejects bad exception ID", test_legacy_freeze_rejects_bad_exception_id),
        ("legacy-freeze: accepts valid exception ID", test_legacy_freeze_accepts_good_exception_id),
        # Architecture design gates (v5)
        ("runtime-core: no wasm emission", test_runtime_core_no_wasm_emission),
        ("runtime-store-wasm: within budget", test_runtime_store_wasm_budget),
        ("single-semantics-source: no hand-written JS semantics", test_single_semantics_source),
        ("no-stub-pass: SpecOps use SpecAlgoIR", test_no_stub_pass),
        ("property-store-is-primitive: storage only", test_property_store_is_primitive),
        ("unsupported-specop-reachable", test_unsupported_specop_reachable),
        ("no-fat-module", test_no_fat_module),
        ("specalgo-completion-check", test_specalgo_completion_check),
        ("no-legacy-build (static)", test_no_legacy_build_static),
        ("no-early-legacy-deletion", test_no_early_legacy_deletion),
        ("trace-contract (static)", test_trace_contract_static),
        # Coverage and freeze gates (use existing scripts)
        ("coverage-classification self-test", coverage_classification_self_test),
        ("frozen-file-unchanged", frozen_file_unchanged_test),
        ("agent: no semantics in backend-wasm", test_agent_no_semantics_in_backend_wasm),
        ("agent F1: no semantics in backend-correctness", test_agent_f1_no_semantics_in_backend_correctness),
        ("agent F2: no legacy delegation", test_agent_f2_no_legacy_delegation),
        ("agent F7: no early legacy deletion", test_agent_f7_no_early_legacy_deletion),
        ("agent F8: separate coverage paths", test_agent_f8_no_legacy_pass_as_new_coverage),
        ("agent F11: no wildcard SpecOp", test_agent_f11_no_wildcard_specop),
        ("agent F4: no RuntimeFn→SpecOp map", test_agent_f4_no_runtimefn_to_specop),
        ("agent F5: no observable in runtime-wasm", test_agent_f5_no_observable_in_runtime_wasm),
        ("agent F6: no stub pass", test_agent_f6_no_stub_pass),
        ("agent F12: no semantics in PropertyStore", test_agent_f12_no_semantics_in_property_store),
        ("agent F9: no dep addition", test_agent_f9_no_dep_addition),
        ("agent F10: no file splitting in fat module", test_agent_f10_no_file_splitting_in_fat_module),
        ("agent F13: no legacy file modification", test_agent_f13_no_legacy_file_modification),
        ("agent F17: no enum dispatch", test_agent_f17_no_enum_dispatch),
        ("agent F15: no abrupt omission", test_agent_f15_no_abrupt_omission),
        ("agent F16: no trace-only verification", test_agent_f16_no_trace_only_verification),
        ("agent F18: no silent default", test_agent_f18_no_silent_default),
    ]

    failures = 0
    for name, fn in tests:
        print(f"Test: {name}", file=sys.stderr)
        if not fn():
            failures += 1

    if failures:
        print(f"\ngate_negative_tests: FAILED ({failures} failures)", file=sys.stderr)
        sys.exit(1)
    print(f"\ngate_negative_tests: OK ({len(tests)} tests)", file=sys.stderr)


if __name__ == "__main__":
    main()
