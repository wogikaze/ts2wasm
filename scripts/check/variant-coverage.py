#!/usr/bin/env python3
"""Track which parser AST and lowered IR variants are exercised by fixtures.

Scans actual Rust enum definitions from:
- crates/syntax/src/ast.rs  (Stmt, Expr)
- crates/ir/src/lowered/types.rs  (LoweredStmt, LoweredExpr, LoweredBinaryOp, etc.)

Then scans fixture files and reports which variants have zero fixture coverage.

This is an info-only report (exit 0 always) — it's not a hard gate.

Usage: python3 scripts/check/variant-coverage.py
"""

import sys
import re
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
FIXTURES_DIR = REPO_ROOT / "fixtures"
SYNTAX_AST_PATH = REPO_ROOT / "crates" / "syntax" / "src" / "ast.rs"
IR_TYPES_PATH = REPO_ROOT / "crates" / "ir" / "src" / "lowered" / "types.rs"

# Regex to extract enum definitions from Rust source.
# Matches: pub enum EnumName {  ...  }
ENUM_RE = re.compile(
    r'pub enum (\w+) \{(.*?)\n\}',
    re.DOTALL,
)

# Regex to extract variant names (leading identifier at enum-member indentation).
# Matches `    VariantName` but NOT field lines like `    name: String,`
# by requiring the captured word NOT to be followed by `:` on the same line.
VARIANT_RE = re.compile(r'^\s{4}(\w+)(?!\s*:)', re.MULTILINE)


def parse_enum_variants(
    file_path: Path, *enum_names: str,
) -> dict[str, list[str]]:
    """Parse specific enum names from a Rust file and return {enum_name: [variant_names]}."""
    result: dict[str, list[str]] = {}
    if not file_path.exists():
        print(
            f"check_variant_coverage: WARNING: {file_path} not found",
            file=sys.stderr,
        )
        return result

    content = file_path.read_text(encoding="utf-8")
    for m in ENUM_RE.finditer(content):
        enum_name = m.group(1)
        if enum_names and enum_name not in enum_names:
            continue
        body = m.group(2)
        variants: list[str] = []
        for vm in VARIANT_RE.finditer(body):
            name = vm.group(1)
            if name not in variants and not name.startswith("_"):
                variants.append(name)
        result[enum_name] = variants
    return result


# Source-level patterns for AST statement variants.
# Keyed by variant name (Stmt::{variant}) → regex pattern.
STMT_PATTERNS: dict[str, str] = {
    "ImportSideEffect": r'\bimport\s+"',
    "ImportNamed": r'\bimport\s+\{',
    "ImportDefault": r'\bimport\s+\w+\s+from\b',
    "ImportDefaultNamed": r'\bimport\s+\w+\s*,\s*\{',
    "ImportNamespace": r'\bimport\s+\*\s+as\b',
    "ImportDefaultNamespace": r'\bimport\s+\w+\s*,\s*\*\s+as\b',
    "ExportNamed": r'\bexport\s+\{',
    "ExportNamedFrom": r'\bexport\s+\{.*\}\s+from\b',
    "ExportAllFrom": r'\bexport\s+\*\s+from\b',
    "ExportNamespaceFrom": r'\bexport\s+\*\s+as\b',
    "ExportDecl": r'\bexport\s+(?:function|class|let|const|var|async|default)\b',
    "ExportDefault": r'\bexport\s+default\b',
    "ExportAssignment": r'\bexport\s*=\s',
    "Let": r'\b(?:let|const|var)\s+\w+\s*(?:=|:)',
    "AmbientValueDecl": r'\bdeclare\s+\w+',
    "Assign": r'(?:^|;)\s*\w+\s*=(?!=)',
    "Expr": r'(?:^|;)\s*(?:\w+\s*\(|--?\w+|\w+\+\+|\[\w+|\+\+|--)',
    "If": r'\bif\s*\(',
    "While": r'\bwhile\s*\(',
    "Function": r'\bfunction\b',
    "Return": r'\breturn\s',
    "ClassDecl": r'\bclass\b',
    "EnumDecl": r'\benum\b',
    "TryCatch": r'\btry\b',
    "Throw": r'\bthrow\b',
    "Switch": r'\bswitch\s*\(',
    "DoWhile": r'\bdo\b',
    "For": r'\b(for|for\s*\()',
    "ForIn": r'\bfor\s*\(.*\bin\b',
    "ForOf": r'\bfor\s*\(.*\bof\b',
    "Labeled": r'\b\w+\s*:\s*(?:while|for|do|if|\{)',
    "Break": r'\bbreak\b',
    "Continue": r'\bcontinue\b',
    "Block": r'\{',
}

# Source-level patterns for AST expression variants.
EXPR_PATTERNS: dict[str, str] = {
    "Number": r'(?<!\w)\d+(?:\.\d+)?(?!n)(?!(?:\.\w|\w))',
    "BigInt": r'\d+n',
    "String": r'"[^"\n]*"|\'[^\'\n]*\'',
    "Bool": r'\b(?:true|false)\b',
    "Null": r'\bnull\b',
    "Undefined": r'\bundefined\b',
    "Await": r'\bawait\b',
    "Ident": r'\b\w+\b',
    "Unary": r'(?<!\w)(?:!|~|\+[+\s]|-[-\s])(?!=)',
    "Binary": r'(?<!\w)(?:\+|-|\*|/|%|&&?|\|\|?|<<|>>|>>>)(?!=)',
    "Member": r'\w+\.\w+',
    "OptionalMember": r'\w+\?\.\w+',
    "Call": r'\w+\s*\(',
    "OptionalCall": r'\w+\?\.?\s*\(',
    "Assign": r'\w+\s*=(?!=)',
    "LogicalAssign": r'\w+\s*(?:\|\|=|&&=|\?\?=)',
    "LogicalPropertyAssign": r'\w+\.\w+\s*(?:\|\|=|&&=|\?\?=)',
    "Array": r'\[\s*(?:\w|\d|"[^"]*"|\'[^\']*\'|,)',
    "Object": r'\{\s*(?:\w+|"[^"]+"|\'[^\']+\'|\[)',
    "Index": r'\w+\[',
    "OptionalIndex": r'\w+\?\.\[',
    "New": r'\bnew\b',
    "TypeOf": r'\btypeof\b',
    "InstanceOf": r'\binstanceof\b',
    "Ternary": r'\?\s*.*\s*:',
    "ArrowFn": r'=>',
    "FunctionExpr": r'\bfunction\b',
    "Spread": r'\.\.\.\w',
    "PropertyAssign": r'\w+\.\w+\s*=',
    "IndexAssign": r'\w+\[\w+\]\s*=',
    "ClassExpr": r'\bclass\b',
    "NewTarget": r'new\.target\b',
    "This": r'\bthis\b',
}

# Lowered statement variants with patterns derived from source-level features.
LOWERED_STMT_PATTERNS: dict[str, str] = {
    "Block": r'\{',
    "Let": r'\b(?:let|const|var)\s+\w+\s*=',
    "Assign": r'\w+\s*=(?!=)',
    "Expr": r'(?:^|;)\s*(?:\w+\s*\(|--?\w+|\w+\+\+|\+\+|--)',
    "If": r'\bif\s*\(',
    "While": r'\bwhile\s*\(',
    "Return": r'\breturn\s',
    "Throw": r'\bthrow\b',
    "TryFinally": r'\btry\b',
    "TryCatch": r'\btry\b.*\bcatch\b',
    "Switch": r'\bswitch\s*\(',
    "DoWhile": r'\bdo\b',
    "For": r'\b(for|for\s*\()',
    "ForIn": r'\bfor\s*\(.*\bin\b',
    "ForOf": r'\bfor\s*\(.*\bof\b',
    "Labeled": r'\b\w+\s*:\s*(?:while|for|do|if|\{)',
    "Break": r'\bbreak\b',
    "Continue": r'\bcontinue\b',
    "Export": r'\bexport\b',
    "ModuleExportsAssign": r'module\.exports\s*=',
    "ClassDecl": r'\bclass\b',
}

# Lowered expression variants with source-level patterns.
LOWERED_EXPR_PATTERNS: dict[str, str] = {
    "Number": r'(?<!\w)\d+(?:\.\d+)?(?!n)(?!(?:\.\w|\w))',
    "BigIntLiteral": r'\d+n',
    "String": r'"[^"\n]*"|\'[^\'\n]*\'',
    "Bool": r'\b(?:true|false)\b',
    "Null": r'\bnull\b',
    "Undefined": r'\bundefined\b',
    "Local": r'\w+',
    "EnvCellNew": r'\(function\(\)\s*\{',
    "EnvCellGet": r'\(function\(\)\s*\{',
    "EnvCellSet": r'\(function\(\)\s*\{',
    "Unary": r'(?<!\w)(?:!|~)(?!=)',
    "Binary": r'(?<!\w)(?:\+|-|\*|/|%|&&?|\|\|?|<<|>>|>>>)(?!=)',
    "PropertyIn": r'\bin\b',
    "PropertyInDynamic": r'\bin\b',
    "Call": r'\w+\s*\(',
    "Assign": r'\w+\s*=(?!=)',
    "LogicalAssign": r'\w+\s*(?:\|\|=|&&=|\?\?=)',
    "LogicalPropertyAssign": r'\w+\.\w+\s*(?:\|\|=|&&=|\?\?=)',
    "LogicalComputedPropertyAssign": r'\w+\[.*\]\s*(?:\|\|=|&&=|\?\?=)',
    "LogicalMemberAssign": r'\w+\.\w+\s*(?:\|\|=|&&=|\?\?=)',
    "ArrayNew": r'\[\s*(?:\w|\d|"[^"]*"|\'[^\']*\'|,)',
    "ArrayNewSparse": r'\[\s*,',
    "ArrayGet": r'\w+\[',
    "Index": r'\w+\[',
    "GetLength": r'\.length\b',
    "ObjectNew": r'\{\s*(?:\w+|"[^"]+"|\'[^\']+\'|\[)',
    "ErrorNew": r'\bnew\s+(?:Error|TypeError|RangeError|SyntaxError|ReferenceError)\b',
    "PropertyGet": r'\w+\.\w+',
    "OptionalPropertyGet": r'\w+\?\.\w+',
    "PropertyGetDynamic": r'\w+\[',
    "OptionalIndex": r'\w+\?\.\[',
    "OptionalCall": r'\w+\?\.?\s*\(',
    "MethodCall": r'\w+\.\w+\s*\(',
    "PromiseGetValue": r'\bawait\b',
    "RuntimeCall": r'\w+\.\w+\s*\(',
    "PropertySet": r'\w+\.\w+\s*=',
    "PropertyDelete": r'\bdelete\s+\w+\.\w+',
    "PropertyDeleteDynamic": r'\bdelete\s+\w+\[',
    "PropertySetDynamic": r'\w+\[.*\]\s*=',
    "New": r'\bnew\b',
    "ClassPrototype": r'\w+\.prototype\b',
    "BuiltinErrorPrototype": r'(?:Error|TypeError|RangeError|SyntaxError|ReferenceError)\.prototype\b',
    "ModuleLoad": r'\bimport\b|require\b',
    "Block": r'\{',
    "This": r'\bthis\b',
    "ArrowFn": r'=>',
}


def collect_fixture_files() -> list[Path]:
    """Collect all .ts fixture files, excluding parser-errors and negative dirs."""
    result: list[Path] = []
    if not FIXTURES_DIR.exists():
        return result

    for entry in sorted(FIXTURES_DIR.iterdir()):
        if not entry.is_dir():
            continue
        if entry.name in ("parser-errors", "negative"):
            continue
        for f in sorted(entry.rglob("*.ts")):
            result.append(f)
    return result


def scan_pattern_coverage(
    files: list[Path],
    patterns: dict[str, str],
) -> dict[str, bool]:
    """Determine which named patterns are covered by fixture content."""
    coverage: dict[str, bool] = {name: False for name in patterns}
    compiled_cache = {name: re.compile(pat) for name, pat in patterns.items()}

    for f in files:
        try:
            content = f.read_text(encoding="utf-8", errors="replace")
        except Exception:
            continue
        for name, compiled in compiled_cache.items():
            if not coverage[name] and compiled.search(content):
                coverage[name] = True

    return coverage


def main() -> None:
    args = sys.argv[1:]
    if args and args[0] in ("-h", "--help"):
        print("Usage: python3 scripts/check/variant-coverage.py")
        print()
        print("Scans fixtures/ for TS files and reports which parser AST and lowered IR")
        print("variants are exercised by existing fixtures.")
        print()
        print("Exit codes: 0 = info-only report, 1 = operational error")
        sys.exit(0)

    # Parse Rust enums for documentation/context.
    stmt_variants = parse_enum_variants(SYNTAX_AST_PATH, "Stmt").get("Stmt", [])
    expr_variants = parse_enum_variants(SYNTAX_AST_PATH, "Expr").get("Expr", [])
    lowered_stmt_variants = parse_enum_variants(IR_TYPES_PATH, "LoweredStmt").get(
        "LoweredStmt", []
    )
    lowered_expr_variants = parse_enum_variants(IR_TYPES_PATH, "LoweredExpr").get(
        "LoweredExpr", []
    )

    # Collect fixture files.
    files = collect_fixture_files()
    if not files:
        print(
            "check_variant_coverage: ERROR: no fixture files found", file=sys.stderr
        )
        sys.exit(1)

    print(f"check_variant_coverage: scanning {len(files)} fixture files...")
    print()

    # Scan each enum category.
    expr_coverage = scan_pattern_coverage(files, EXPR_PATTERNS)
    stmt_coverage = scan_pattern_coverage(files, STMT_PATTERNS)
    lowered_expr_coverage = scan_pattern_coverage(files, LOWERED_EXPR_PATTERNS)
    lowered_stmt_coverage = scan_pattern_coverage(files, LOWERED_STMT_PATTERNS)

    def print_variant_report(
        label: str,
        variants: list[str],
        coverage: dict[str, bool],
        patterns: dict[str, str],
    ):
        """Print coverage report for one enum category."""
        mapped = [v for v in variants if v in patterns]
        unmapped = [v for v in variants if v not in patterns]
        covered = sum(1 for v in mapped if coverage.get(v, False))
        total = len(mapped)

        print(f"=== {label} ({covered}/{total} mapped variants covered) ===")
        for v in sorted(mapped):
            status = "COVERED" if coverage.get(v, False) else "GAP"
            print(f"  {v}: {status}")
        if unmapped:
            print(f"\n  ({len(unmapped)} variants with no pattern mapping)")
            for v in sorted(unmapped):
                print(f"  {v}: NO_PATTERN")
        print()

    # Report per-category
    print_variant_report("Stmt (parser AST)", stmt_variants, stmt_coverage, STMT_PATTERNS)
    print_variant_report("Expr (parser AST)", expr_variants, expr_coverage, EXPR_PATTERNS)
    print_variant_report(
        "LoweredStmt (lowered IR)",
        lowered_stmt_variants,
        lowered_stmt_coverage,
        LOWERED_STMT_PATTERNS,
    )
    print_variant_report(
        "LoweredExpr (lowered IR)",
        lowered_expr_variants,
        lowered_expr_coverage,
        LOWERED_EXPR_PATTERNS,
    )

    # Summary across all categories
    all_patterns = {}
    all_patterns.update(STMT_PATTERNS)
    all_patterns.update(EXPR_PATTERNS)
    all_patterns.update(LOWERED_STMT_PATTERNS)
    all_patterns.update(LOWERED_EXPR_PATTERNS)

    all_coverage = {}
    all_coverage.update(stmt_coverage)
    all_coverage.update(expr_coverage)
    all_coverage.update(lowered_stmt_coverage)
    all_coverage.update(lowered_expr_coverage)

    covered = sum(1 for v in all_coverage.values() if v)
    total = len(all_coverage)
    print(f"check_variant_coverage: {covered}/{total} patterns matched by fixtures")

    # Identify gaps
    gaps = [name for name, cov in all_coverage.items() if not cov]
    if gaps:
        print("\nPattern gaps (zero fixture coverage):", file=sys.stderr)
        for g in sorted(gaps):
            # Find which category this gap belongs to
            if g in STMT_PATTERNS:
                print(f"  Stmt::{g}", file=sys.stderr)
            elif g in EXPR_PATTERNS:
                print(f"  Expr::{g}", file=sys.stderr)
            elif g in LOWERED_STMT_PATTERNS:
                print(f"  LoweredStmt::{g}", file=sys.stderr)
            elif g in LOWERED_EXPR_PATTERNS:
                print(f"  LoweredExpr::{g}", file=sys.stderr)

    # Exit 0 (info-only report, not a hard gate).
    print("\ncheck_variant_coverage: OK (info-only report)", file=sys.stderr)


if __name__ == "__main__":
    main()
