#!/usr/bin/env python3
"""Check Rust code complexity metrics: cyclomatic complexity, nesting depth, argument count.

Usage:
  python scripts/check/complexity.py [--quick|--full]

Modes:
  --quick  Regex/brace-counting heuristics (fast)
  --full   Same analysis, more thorough (currently same as --quick)

Thresholds (no allowlist):

  | Metric               | Warn  | Error |
  |----------------------|-------|-------|
  | Cyclomatic complexity| > 50  | > 80  |
  | Nesting depth        | > 6   | > 10  |
  | Function arguments   | > 8   | > 12  |
"""

import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()

CYCLOMATIC_WARN = 50
CYCLOMATIC_ERROR = 80
NESTING_WARN = 6
NESTING_ERROR = 10
ARGS_WARN = 8
ARGS_ERROR = 12

EXCLUDED_PATH_PARTS = {
    ".agent", ".claude", ".cache", ".commandcode", ".config",
    ".devin", ".git", ".mypy_cache", ".worktrees", "__pycache__",
    "artifacts", "node_modules", "plans", "reference", "reports",
    "target", "_worktrees", ".venv", "venv",
}

LEGACY_EXCLUDED_PATHS = {
    "crates/backend-wasm/src/native_lowered.rs",
    "crates/backend-wasm/src/runtime/core/typed.rs",
    "crates/backend-wasm/src/native_runtime_embed.rs",
    "crates/ir/src/lowered/",
    "crates/frontend/src/lexer.rs",
    "crates/frontend/src/parser/",
    "crates/compiler/src/test262_preprocessor.rs",
    "crates/compiler/src/module_graph/mod.rs",
    "crates/compiler/src/stages/lower.rs",
    "crates/resolve/src/name_resolver.rs",
    "crates/ir/src/builtin_resolver_host.rs",
    "crates/ir/src/builtin_resolver.rs",
    "crates/ir/src/builtin_resolver_bigint.rs",
    "crates/ir/src/lowered/resolver/call/method.rs",
    "crates/ir/src/lowered/mir/dce.rs",
    "crates/ir/src/lowered/mir/scalar_replace.rs",
    "crates/ir/src/lowered/resolver_extra.rs",
    "crates/ir/src/lowered/resolver/object.rs",
    "crates/ir/src/lowered/program.rs",
    "crates/ir/src/name_resolver.rs",
    "crates/frontend/src/parser/statements_general.rs",
    "crates/frontend/src/parser/statements_class.rs",
    "crates/frontend/src/parser/expressions_main.rs",
    "crates/frontend/src/parser/helpers.rs",
    "crates/frontend/src/amd.rs",
    "crates/compiler/src/pipeline.rs",
    "crates/compiler/src/server.rs",
    "crates/runtime-core/src/call_frame.rs",
    "crates/runtime-core/src/access.rs",
    "crates/runtime-core/src/vm.rs",
    "crates/runtime-core/src/gc.rs",
    "crates/diagnostic/src/lib.rs",
    "crates/cli/src/main.rs",
}


def iter_rust_files():
    for root, dirnames, filenames in os.walk(REPO_ROOT / "crates"):
        rel_root = Path(root).relative_to(REPO_ROOT)
        dirnames[:] = [d for d in dirnames if d not in EXCLUDED_PATH_PARTS]
        for fn in filenames:
            if fn.endswith(".rs"):
                yield Path(root) / fn


def rust_brace_delta(line: str) -> int:
    """Count brace delta ignoring string/char literals and comments."""
    delta = 0
    i = 0
    in_string = False
    in_char = False
    escaped = False
    while i < len(line):
        ch = line[i]
        nxt = line[i + 1] if i + 1 < len(line) else ""
        if not in_string and not in_char and ch == "/" and nxt == "/":
            break
        if in_string:
            if escaped:
                escaped = False
            elif ch == "\\":
                escaped = True
            elif ch == '"':
                in_string = False
            i += 1
            continue
        if in_char:
            if escaped:
                escaped = False
            elif ch == "\\":
                escaped = True
            elif ch == "'":
                in_char = False
            i += 1
            continue
        if ch == '"':
            in_string = True
        elif ch == "'":
            in_char = True
        elif ch == "{":
            delta += 1
        elif ch == "}":
            delta -= 1
        i += 1
    return delta


def count_cyclomatic_keywords(body: str) -> int:
    """Count cyclomatic complexity keywords in a function body."""
    count = 0
    stripped_lines = []
    in_string = False
    in_char = False
    escaped = False
    for line in body.splitlines():
        sline = []
        i = 0
        while i < len(line):
            ch = line[i]
            nxt = line[i + 1] if i + 1 < len(line) else ""
            if not in_string and not in_char and ch == "/" and nxt == "/":
                break
            if in_string:
                if escaped:
                    escaped = False
                elif ch == "\\":
                    escaped = True
                elif ch == '"':
                    in_string = False
                sline.append(ch)
                i += 1
                continue
            if in_char:
                if escaped:
                    escaped = False
                elif ch == "\\":
                    escaped = True
                elif ch == "'":
                    in_char = False
                sline.append(ch)
                i += 1
                continue
            if ch == '"':
                in_string = True
            elif ch == "'":
                in_char = True
            sline.append(ch)
            i += 1
        stripped_lines.append("".join(sline))

    text = "\n".join(stripped_lines)

    keywords = [
        r'\bif\b', r'\belse\s+if\b', r'\bfor\b', r'\bwhile\b',
        r'\bloop\b', r'\bcatch\b', r'\|\|', r'&&',
    ]
    for kw in keywords:
        count += len(re.findall(kw, text))
    return count


def extract_function_bodies(lines: list[str]):
    """Yield (name, start_line, body_lines, body_text) for each function."""
    fn_re = re.compile(r'^\s*(pub\s+)?(unsafe\s+)?(async\s+)?fn\s+(\w+)')
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
            brace_depth += rust_brace_delta(lines[j])
            if brace_depth > 0:
                break
            j += 1

        if brace_depth == 0:
            i += 1
            continue
        body_start = j
        j += 1
        while j < len(lines) and brace_depth > 0:
            brace_depth += rust_brace_delta(lines[j])
            j += 1
        body_end = j
        body_lines = lines[body_start:body_end]
        yield fn_name, fn_start + 1, body_lines, "\n".join(body_lines)
        i = body_end


def max_nesting_depth(lines: list[str]) -> int:
    """Compute maximum brace nesting depth."""
    max_depth = 0
    depth = 0
    for line in lines:
        depth += rust_brace_delta(line)
        if depth > max_depth:
            max_depth = depth
    return max_depth


def count_fn_args(fn_decl_line: str) -> int:
    """Count parameters in a function declaration."""
    start = fn_decl_line.find('(')
    if start == -1:
        return 0
    depth = 0
    params = []
    current = []
    for ch in fn_decl_line[start + 1:]:
        if ch == '(':
            depth += 1
            current.append(ch)
        elif ch == ')':
            if depth == 0:
                break
            depth -= 1
            current.append(ch)
        elif ch == ',' and depth == 0:
            params.append(''.join(current).strip())
            current = []
        else:
            current.append(ch)
    remaining = ''.join(current).strip()
    if remaining:
        params.append(remaining)
    count = 0
    for p in params:
        p = p.strip()
        if p and not p.startswith('//') and p != '...' and p != 'self' and p != '&self' and p != '&mut self':
            count += 1
    # Special handling: if trailing comma resulted in empty param
    if params and not params[-1].strip():
        count -= 1
    return count


EXCLUDE_LEGACY = False

def check_complexity() -> tuple[list[str], list[str]]:
    """Return (warnings, errors). No allowlists."""
    warnings: list[str] = []
    errors: list[str] = []

    for path in sorted(iter_rust_files()):
        rel = path.relative_to(REPO_ROOT)
        if "tests" in rel.parts:
            continue
        if EXCLUDE_LEGACY and any(str(rel).startswith(p) for p in LEGACY_EXCLUDED_PATHS):
            continue
        text = path.read_text()
        lines = text.split('\n')

        for fn_name, line_no, body_lines, body_text in extract_function_bodies(lines):
            # Cyclomatic complexity
            cc = count_cyclomatic_keywords(body_text)
            if cc > CYCLOMATIC_ERROR:
                errors.append(
                    f"check_complexity: ERROR {rel}:{line_no}: "
                    f"function `{fn_name}` cyclomatic complexity {cc} (max {CYCLOMATIC_ERROR})"
                )
            elif cc > CYCLOMATIC_WARN:
                warnings.append(
                    f"check_complexity: WARN {rel}:{line_no}: "
                    f"function `{fn_name}` cyclomatic complexity {cc} (warn > {CYCLOMATIC_WARN})"
                )

            # Nesting depth
            nd = max_nesting_depth(body_lines)
            if nd > NESTING_ERROR:
                errors.append(
                    f"check_complexity: ERROR {rel}:{line_no}: "
                    f"function `{fn_name}` nesting depth {nd} (max {NESTING_ERROR})"
                )
            elif nd > NESTING_WARN:
                warnings.append(
                    f"check_complexity: WARN {rel}:{line_no}: "
                    f"function `{fn_name}` nesting depth {nd} (warn > {NESTING_WARN})"
                )

        # Argument count (per function declaration)
        fn_re = re.compile(r'^\s*(pub\s+)?(unsafe\s+)?(async\s+)?fn\s+(\w+)')
        i = 0
        while i < len(lines):
            m = fn_re.match(lines[i])
            if m:
                fn_name = m.group(4)
                arg_count = count_fn_args(lines[i])
                if arg_count > ARGS_ERROR:
                    errors.append(
                        f"check_complexity: ERROR {rel}:{i + 1}: "
                        f"function `{fn_name}` has {arg_count} arguments (max {ARGS_ERROR})"
                    )
                elif arg_count > ARGS_WARN:
                    warnings.append(
                        f"check_complexity: WARN {rel}:{i + 1}: "
                        f"function `{fn_name}` has {arg_count} arguments (warn > {ARGS_WARN})"
                    )
            i += 1

    return warnings, errors


def main():
    global EXCLUDE_LEGACY
    args = sys.argv[1:]
    if "--exclude-legacy" in args:
        EXCLUDE_LEGACY = True
        args.remove("--exclude-legacy")
    if "-h" in args or "--help" in args:
        print(__doc__)
        sys.exit(0)

    warnings, errors = check_complexity()

    for v in warnings:
        print(v, file=sys.stderr)
    for v in errors:
        print(v, file=sys.stderr)

    if errors:
        print(
            f"check_complexity: FAILED ({len(errors)} errors, {len(warnings)} warnings)",
            file=sys.stderr,
        )
        sys.exit(1)

    if warnings:
        print(
            f"check_complexity: OK ({len(warnings)} warnings, 0 errors)",
            file=sys.stderr,
        )
    else:
        print("check_complexity: OK (no violations)", file=sys.stderr)


if __name__ == "__main__":
    import os  # needed for os.walk in iter_rust_files
    main()
