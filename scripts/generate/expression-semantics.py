#!/usr/bin/env python3
"""Generate small expression-level TypeScript programs for differential testing.

Produces self-contained .ts files that exercise specific expression semantics
(arithmetic, comparison, logical, string, etc.) and print results via console.log.

Modes:
  --smoke       Generate a fixed set of basic expression programs (for CI validation)
  --seed <N>    Random seed for deterministic generation (default: 42)
  --count <N>   Number of expressions to generate (default: 10, smoke: fixed)
  --out <dir>   Output directory (default: /tmp/ts2wasm-expr-gen-<pid>)

Usage:
  python3 scripts/generate/expression-semantics.py --smoke
  python3 scripts/generate/expression-semantics.py --seed 42 --count 20
"""

import sys
import os
import random
import hashlib
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()


def usage():
    print("Usage:")
    print("  python3 scripts/generate/expression-semantics.py [--smoke] [--seed N] [--count N] [--out DIR]")
    print()
    print("  --smoke      Generate fixed set of basic expressions (for CI validation)")
    print("  --seed <N>   Random seed for deterministic generation (default: 42)")
    print("  --count <N>  Number of expressions to generate (default: 10)")
    print("  --out <DIR>  Output directory (default: /tmp/ts2wasm-expr-gen-<pid>)")
    sys.exit(0)


# ---------------------------------------------------------------------------
# Expression generators
# ---------------------------------------------------------------------------

def gen_binary_arithmetic(rng: random.Random) -> str:
    a = rng.randint(-100, 100)
    b = rng.randint(-100, 100)
    op = rng.choice(["+", "-", "*", "/", "%"])
    return f"console.log({a} {op} {b});"


def gen_unary_arithmetic(rng: random.Random) -> str:
    a = rng.randint(-100, 100)
    op = rng.choice(["+", "-", "!"])
    return f"console.log({op}{a});"


def gen_comparison(rng: random.Random) -> str:
    a = rng.randint(-50, 50)
    b = rng.randint(-50, 50)
    op = rng.choice(["===", "!==", "<", ">", "<=", ">=", "==", "!="])
    return f"console.log({a} {op} {b});"


def gen_logical(rng: random.Random) -> str:
    a = rng.choice([0, 1, -1, 42, None])
    b = rng.choice([0, 1, -1, 42, None])
    val_a = "true" if a is None else str(a)
    val_b = "true" if b is None else str(b)
    op = rng.choice(["&&", "||"])
    return f"console.log({val_a} {op} {val_b});"


def gen_ternary(rng: random.Random) -> str:
    cond = rng.choice(["true", "false", "1", "0"])
    a = rng.randint(1, 50)
    b = rng.randint(51, 100)
    return f"console.log({cond} ? {a} : {b});"


def gen_string_concat(rng: random.Random) -> str:
    s1 = rng.choice(["hello", "foo", "bar", "42", ""])
    s2 = rng.choice(["world", "baz", "qux", "13", "!"])
    return f'console.log("{s1}" + "{s2}");'


def gen_typeof(rng: random.Random) -> str:
    val = rng.choice(["42", '"hello"', "true", "undefined", "null"])
    return f"console.log(typeof {val});"


def gen_boolean_expr(rng: random.Random) -> str:
    a = rng.choice([0, 1, -5, 99])
    b = rng.choice([0, 1, -5, 99])
    op = rng.choice(["&&", "||"])
    return f"console.log(Boolean({a} {op} {b}));"


# Smoke test: fixed set of basic expressions for CI validation
SMOKE_EXPRESSIONS = [
    # Arithmetic
    "console.log(1 + 2);",
    "console.log(10 - 3);",
    "console.log(4 * 5);",
    "console.log(20 / 4);",
    "console.log(17 % 5);",
    # Unary
    "console.log(-42);",
    "console.log(+7);",
    "console.log(!true);",
    "console.log(!0);",
    # Comparison
    "console.log(1 === 1);",
    "console.log(2 !== 3);",
    "console.log(5 < 10);",
    "console.log(10 > 5);",
    "console.log(3 <= 3);",
    "console.log(4 >= 5);",
    "console.log(1 == true);",
    # Logical
    "console.log(true && false);",
    "console.log(true || false);",
    "console.log(0 && 42);",
    "console.log(0 || 42);",
    # Ternary
    "console.log(true ? 1 : 2);",
    "console.log(false ? 1 : 2);",
    # String
    'console.log("hello" + " " + "world");',
    # Typeof
    "console.log(typeof 42);",
    'console.log(typeof "hello");',
    "console.log(typeof true);",
    "console.log(typeof undefined);",
    "console.log(typeof null);",
]


def run_generator(rng: random.Random, count: int) -> list[str]:
    """Generate `count` expression programs (each returns a list of statements)."""
    generators = [
        gen_binary_arithmetic,
        gen_unary_arithmetic,
        gen_comparison,
        gen_logical,
        gen_ternary,
        gen_string_concat,
        gen_typeof,
        gen_boolean_expr,
    ]

    programs = []
    for _ in range(count):
        gen = rng.choice(generators)
        stmt = gen(rng)
        programs.append(stmt)

    return programs


def write_programs(programs: list[str], out_dir: Path) -> list[Path]:
    """Write each program as a separate .ts file. Returns list of file paths."""
    out_dir.mkdir(parents=True, exist_ok=True)
    paths = []
    for i, stmt in enumerate(programs):
        content = stmt + "\n"
        # Compute content hash for stable filenames
        file_hash = hashlib.sha256(content.encode()).hexdigest()[:12]
        file_path = out_dir / f"expr_{i:04d}_{file_hash}.ts"
        file_path.write_text(content)
        paths.append(file_path)
    return paths


def main():
    args = sys.argv[1:]

    if args and args[0] in ("-h", "--help"):
        usage()

    smoke_mode = False
    seed = 42
    count = 10
    out_dir = None

    i = 0
    while i < len(args):
        if args[i] == "--smoke":
            smoke_mode = True
            i += 1
        elif args[i] == "--seed" and i + 1 < len(args):
            seed = int(args[i + 1])
            i += 2
        elif args[i] == "--count" and i + 1 < len(args):
            count = int(args[i + 1])
            i += 2
        elif args[i] == "--out" and i + 1 < len(args):
            out_dir = Path(args[i + 1])
            i += 2
        else:
            print(f"expression_generator: unknown arg: {args[i]}", file=sys.stderr)
            usage()

    if out_dir is None:
        out_dir = Path(f"/tmp/ts2wasm-expr-gen-{os.getpid()}")

    rng = random.Random(seed)

    if smoke_mode:
        programs = SMOKE_EXPRESSIONS
        print(f"expression_generator: smoke mode: {len(programs)} fixed expressions", file=sys.stderr)
    else:
        programs = run_generator(rng, count)
        print(f"expression_generator: seed={seed} count={count}: {len(programs)} generated expressions", file=sys.stderr)

    paths = write_programs(programs, out_dir)

    # Print generated file paths (one per line for easy piping)
    for p in paths:
        print(p)

    return 0


if __name__ == "__main__":
    sys.exit(main())
