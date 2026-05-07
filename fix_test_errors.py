#!/usr/bin/env python3
"""
Fix pre-existing test compilation errors in ts2wasm-backend-wasm.

The IR types changed (Span added to variants, LoweredFunction got recursion_depth).
This updates test code to match current type definitions.
"""

import re
import sys

def fix_lib_rs(content):
    """Fix test code in lib.rs"""
    # Fix 1: LoweredExpr::This -> LoweredExpr::This(Span::generated("test"))
    # Only fix the specific instance in test code (line 154)
    content = content.replace(
        "LoweredExpr::This,\n                Span::generated(\"test\")",
        "LoweredExpr::This(Span::generated(\"test\")),\n                Span::generated(\"test\")"
    )

    # Fix 2: LoweredStmt::Expr(LoweredExpr::String("hi".to_owned()))
    # -> LoweredStmt::Expr(LoweredExpr::String("hi".to_owned(), Span::generated("test")), Span::generated("test"))
    content = content.replace(
        "LoweredStmt::Expr(LoweredExpr::String(\"hi\".to_owned()))",
        "LoweredStmt::Expr(LoweredExpr::String(\"hi\".to_owned(), Span::generated(\"test\")), Span::generated(\"test\"))"
    )

    # Fix 3: LoweredExpr::Call { kind, args } missing span
    # Pattern: LoweredExpr::Call { kind: X, args: Y }
    # -> LoweredExpr::Call { kind: X, args: Y, span: Span::generated("test") }
    content = content.replace(
        "LoweredExpr::Call {\n                    kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),\n                    args: vec![LoweredExpr::Number(42, Span::generated(\"test\"))],\n                })",
        "LoweredExpr::Call {\n                    kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),\n                    args: vec![LoweredExpr::Number(42, Span::generated(\"test\"))],\n                    span: Span::generated(\"test\"),\n                })"
    )

    # Fix 4: LoweredStmt::Let(LocalId(0), LoweredExpr::Number(42, ...)) -> add span
    content = content.replace(
        "LoweredStmt::Let(LocalId(0), LoweredExpr::Number(42, Span::generated(\"test\")))",
        "LoweredStmt::Let(LocalId(0), LoweredExpr::Number(42, Span::generated(\"test\")), Span::generated(\"test\"))"
    )

    # Fix 5: LoweredExpr::Local(LocalId(0)) in Call args -> add span
    content = content.replace(
        "args: vec![LoweredExpr::Local(LocalId(0))]",
        "args: vec![LoweredExpr::Local(LocalId(0), Span::generated(\"test\"))]"
    )

    # Fix 6: LoweredExpr::Call { kind, args, ... } - full match needed
    # This handles direct_wasm_binary_mvp_binary_expression
    content = content.replace(
        "LoweredExpr::Call {\n                kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),\n                args: vec![LoweredExpr::Binary {\n                    left: Box::new(LoweredExpr::Number(10, Span::generated(\"test\"))),\n                    op: LoweredBinaryOp::Add,\n                    right: Box::new(LoweredExpr::Number(32, Span::generated(\"test\"))),\n                    span: Span::generated(\"test\"),\n                }],\n            })",
        "LoweredExpr::Call {\n                kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),\n                args: vec![LoweredExpr::Binary {\n                    left: Box::new(LoweredExpr::Number(10, Span::generated(\"test\"))),\n                    op: LoweredBinaryOp::Add,\n                    right: Box::new(LoweredExpr::Number(32, Span::generated(\"test\"))),\n                    span: Span::generated(\"test\"),\n                }],\n                span: Span::generated(\"test\"),\n            })"
    )

    # Fix 7: Multiple statements test - fix Expr(Call{...}) calls with missing span
    content = content.replace(
        "args: vec![LoweredExpr::String(\"hello\".to_owned())]",
        "args: vec![LoweredExpr::String(\"hello\".to_owned(), Span::generated(\"test\"))]"
    )

    # Fix 8: ObjectNew missing span
    content = content.replace(
        "LoweredExpr::ObjectNew {\n                props: vec![],\n                non_enumerable: 0,\n            })",
        "LoweredExpr::ObjectNew {\n                props: vec![],\n                non_enumerable: 0,\n                span: Span::generated(\"test\"),\n            })"
    )

    # Fix 9: Binary Add with String operands missing span on String
    content = content.replace(
        "LoweredExpr::String(\"a\".to_owned())",
        "LoweredExpr::String(\"a\".to_owned(), Span::generated(\"test\"))"
    )
    content = content.replace(
        "LoweredExpr::String(\"b\".to_owned())",
        "LoweredExpr::String(\"b\".to_owned(), Span::generated(\"test\"))"
    )

    # Fix 10: Let(ObjectNew) missing span - case in top_level_locals test
    content = content.replace(
        "LoweredExpr::ObjectNew {\n                    props: vec![],\n                    non_enumerable: 0,\n                },\n            )]",
        "LoweredExpr::ObjectNew {\n                    props: vec![],\n                    non_enumerable: 0,\n                    span: Span::generated(\"test\"),\n                },\n                Span::generated(\"test\"),\n            )]"
    )

    # Fix 11: Let(ObjectNew) in function body - case in function_locals test
    # Remove the span fix we already added... this is getting complex, let me do it differently
    # Actually let me check for the function_locals pattern:
    # LoweredStmt::Let(LocalId(0), LoweredExpr::ObjectNew {...})
    # which doesn't match our earlier fix since it's in a Vec

    return content

def fix_runtime_link_plan_tests(content):
    """Fix test code in runtime_link_plan.rs"""
    # Fix Expr(RuntimeCall{...}) missing span on RuntimeCall
    # Pattern: LoweredStmt::Expr(LoweredExpr::RuntimeCall { runtime_fn: X, args: Y })
    # -> add span: Span::generated("test") to RuntimeCall

    # Fix Expr(Local(...)) missing span on Expr's second arg
    # Pattern: LoweredStmt::Expr(LoweredExpr::Local(LocalId(X)))
    # -> LoweredStmt::Expr(LoweredExpr::Local(LocalId(X), Span::generated("test")), Span::generated("test"))

    # Fix bigint_runtime_arithmetic_selects_helper_deps and all others
    # Fix Expr(Local(id)) by adding span to Local
    content = re.sub(
        r'LoweredExpr::Local\(ts2wasm_ir::lowered::LocalId\((\d+)\)\)',
        r'LoweredExpr::Local(ts2wasm_ir::lowered::LocalId(\1), Span::generated("test"))',
        content
    )

    # Fix Expr(RuntimeCall{...}) by adding span to RuntimeCall (no trailing },
    # the Expr will need span too)
    content = re.sub(
        r'(LoweredExpr::RuntimeCall \{\s*runtime_fn: "[^"]*",\s*args: vec!\[[^\]]*\])\s*\}',
        r'\1, span: Span::generated("test") }',
        content
    )

    # Fix Expr(RuntimeCall{...}) by adding span to Expr
    content = re.sub(
        r'LoweredStmt::Expr\(LoweredExpr::BigIntLiteral \{',
        'LoweredStmt::Expr(LoweredExpr::BigIntLiteral {',
        content  # placeholder
    )

    # Fix LoweredStmt::Expr(...) with just one arg -> two args
    # After the RuntimeCall fix, we need to add Span to the Expr wrapper
    # But this is complex to do with regex. Let me handle it differently.

    return content

def fix_wasm_binary_rs(content):
    """Fix test code in wasm_binary.rs"""
    # Add Span import
    content = content.replace(
        "use ts2wasm_runtime_abi::ValueTag;",
        "use ts2wasm_frontend::Span;\nuse ts2wasm_runtime_abi::ValueTag;"
    )

    # Fix Expr(LoweredExpr, Span) - LoweredStmt::Expr now takes 2 args
    # Fix RuntimeCall missing span
    # Fix LoweredExpr::String missing span
    # Fix AllocLocal(_, _) - check if this still exists

    return content


def main():
    files = {}

    # Read all files
    for filepath in [
        "/home/wogikaze/wgkz/ts2wasm/crates/backend-wasm/src/lib.rs",
        "/home/wogikaze/wgkz/ts2wasm/crates/backend-wasm/src/runtime_link_plan.rs",
        "/home/wogikaze/wgkz/ts2wasm/crates/backend-wasm/src/wasm_binary.rs",
        "/home/wogikaze/wgkz/ts2wasm/crates/backend-wasm/src/string_intern.rs",
    ]:
        with open(filepath) as f:
            files[filepath] = f.read()

    # Fix lib.rs - use simple targeted replacements
    lib = files["/home/wogikaze/wgkz/ts2wasm/crates/backend-wasm/src/lib.rs"]

    # Fix 1: LoweredExpr::This -> LoweredExpr::This(Span)
    lib = lib.replace(
        "                LoweredExpr::This,\n                Span::generated(\"test\")",
        "                LoweredExpr::This(Span::generated(\"test\")),\n                Span::generated(\"test\")"
    )

    # Fix 2: LoweredStmt::Expr(LoweredExpr::String("hi")) -> add spans
    lib = lib.replace(
        'LoweredStmt::Expr(LoweredExpr::String("hi".to_owned()))',
        'LoweredStmt::Expr(LoweredExpr::String("hi".to_owned(), Span::generated("test")), Span::generated("test"))'
    )

    # Fix 3: LoweredExpr::Call with Builtin ConsoleLog, missing span
    # Pattern 1: Number(42) - single-arg call
    lib = lib.replace(
        "LoweredExpr::Call {\n                    kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),\n                    args: vec![LoweredExpr::Number(42, Span::generated(\"test\"))],\n                })",
        "LoweredExpr::Call {\n                    kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),\n                    args: vec![LoweredExpr::Number(42, Span::generated(\"test\"))],\n                    span: Span::generated(\"test\"),\n                })"
    )

    # Fix 4: LoweredStmt::Let(LocalId(0), Number) missing 3rd arg span
    lib = lib.replace(
        "LoweredStmt::Let(LocalId(0), LoweredExpr::Number(42, Span::generated(\"test\")))",
        "LoweredStmt::Let(LocalId(0), LoweredExpr::Number(42, Span::generated(\"test\")), Span::generated(\"test\"))"
    )

    # Fix 5: LoweredExpr::Local(LocalId(0)) inside Call args -> add span
    lib = lib.replace(
        "args: vec![LoweredExpr::Local(LocalId(0))]",
        "args: vec![LoweredExpr::Local(LocalId(0), Span::generated(\"test\"))]"
    )

    # Fix 6: Call with binary expr args missing span
    lib = lib.replace(
        "LoweredExpr::Call {\n                kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),\n                args: vec![LoweredExpr::Binary {\n                    left: Box::new(LoweredExpr::Number(10, Span::generated(\"test\"))),\n                    op: LoweredBinaryOp::Add,\n                    right: Box::new(LoweredExpr::Number(32, Span::generated(\"test\"))),\n                    span: Span::generated(\"test\"),\n                }],\n            })",
        "LoweredExpr::Call {\n                kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),\n                args: vec![LoweredExpr::Binary {\n                    left: Box::new(LoweredExpr::Number(10, Span::generated(\"test\"))),\n                    op: LoweredBinaryOp::Add,\n                    right: Box::new(LoweredExpr::Number(32, Span::generated(\"test\"))),\n                    span: Span::generated(\"test\"),\n                }],\n                span: Span::generated(\"test\"),\n            })"
    )

    # Fix 7: String("hello") calls missing Span
    lib = lib.replace(
        'args: vec![LoweredExpr::String("hello".to_owned())]',
        'args: vec![LoweredExpr::String("hello".to_owned(), Span::generated("test"))]'
    )

    # Fix 8: LoweredExpr::ObjectNew missing span in all occurrences
    lib = lib.replace(
        "LoweredExpr::ObjectNew {\n                props: vec![],\n                non_enumerable: 0,\n            })",
        "LoweredExpr::ObjectNew {\n                props: vec![],\n                non_enumerable: 0,\n                span: Span::generated(\"test\"),\n            })"
    )

    # Fix 9: String("a") and String("b") in Binary Add test
    lib = lib.replace(
        'LoweredExpr::String("a".to_owned())',
        'LoweredExpr::String("a".to_owned(), Span::generated("test"))'
    )
    lib = lib.replace(
        'LoweredExpr::String("b".to_owned())',
        'LoweredExpr::String("b".to_owned(), Span::generated("test"))'
    )

    # Fix 10: ObjectNew with props, non_enumerable and extra Let span
    lib = lib.replace(
        "LoweredExpr::ObjectNew {\n                    props: vec![],\n                    non_enumerable: 0,\n                },\n            )]",
        "LoweredExpr::ObjectNew {\n                    props: vec![],\n                    non_enumerable: 0,\n                    span: Span::generated(\"test\"),\n                },\n                Span::generated(\"test\"),\n            )]"
    )

    # Fix 11: ObjectNew in function body Let (lines 596-601)
    lib = lib.replace(
        "LoweredStmt::Let(\n                        LocalId(0),\n                        LoweredExpr::ObjectNew {\n                            props: vec![],\n                            non_enumerable: 0,\n                        },\n                    ),",
        "LoweredStmt::Let(\n                        LocalId(0),\n                        LoweredExpr::ObjectNew {\n                            props: vec![],\n                            non_enumerable: 0,\n                            span: Span::generated(\"test\"),\n                        },\n                        Span::generated(\"test\"),\n                    ),"
    )

    # Fix 12: LoweredStmt::Return(LoweredExpr::Local(LocalId(0))) -> add span to Local
    lib = lib.replace(
        "LoweredStmt::Return(LoweredExpr::Local(LocalId(0)))",
        "LoweredStmt::Return(LoweredExpr::Local(LocalId(0), Span::generated(\"test\")), Span::generated(\"test\"))"
    )

    # Fix 13: Call with no args (line 582-585) missing span
    lib = lib.replace(
        "LoweredExpr::Call {\n                kind: FunctionCallKind::User(FuncId(0)),\n                args: vec![],\n            })",
        "LoweredExpr::Call {\n                kind: FunctionCallKind::User(FuncId(0)),\n                args: vec![],\n                span: Span::generated(\"test\"),\n            })"
    )

    # Fix 14: Call with String args (lines 364-371) - both String needs span AND Call needs span
    lib = lib.replace(
        "LoweredExpr::Call {\n                    kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),\n                    args: vec![LoweredExpr::String(\"hello\".to_owned(), Span::generated(\"test\"))],\n                })",
        "LoweredExpr::Call {\n                    kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),\n                    args: vec![LoweredExpr::String(\"hello\".to_owned(), Span::generated(\"test\"))],\n                    span: Span::generated(\"test\"),\n                })"
    )
    lib = lib.replace(
        "LoweredExpr::Call {\n                    kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),\n                    args: vec![LoweredExpr::Number(42, Span::generated(\"test\"))],\n                })",
        "LoweredExpr::Call {\n                    kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),\n                    args: vec![LoweredExpr::Number(42, Span::generated(\"test\"))],\n                    span: Span::generated(\"test\"),\n                })"
    )

    files["/home/wogikaze/wgkz/ts2wasm/crates/backend-wasm/src/lib.rs"] = lib

    # Fix runtime_link_plan.rs - Expr(RuntimeCall{...}) missing span on RuntimeCall + Expr
    rlp = files["/home/wogikaze/wgkz/ts2wasm/crates/backend-wasm/src/runtime_link_plan.rs"]

    # Fix all LoweredExpr::Local(id) -> LoweredExpr::Local(id, Span::generated("test"))
    rlp = re.sub(
        r'LoweredExpr::Local\(ts2wasm_ir::lowered::LocalId\((\d+)\)\)(?!\s*,)',
        r'LoweredExpr::Local(ts2wasm_ir::lowered::LocalId(\1), Span::generated("test"))',
        rlp
    )

    # Fix all LoweredExpr::RuntimeCall { ..., } missing span
    rlp = re.sub(
        r'(LoweredExpr::RuntimeCall \{\s*runtime_fn: "[^"]*",\s*args: vec!\[[^\]]*\])\s*\}',
        r'\1,\n                        span: Span::generated("test") }',
        rlp
    )

    # Fix all LoweredStmt::Expr(LoweredExpr::Local(...)) that need Expr span
    rlp = re.sub(
        r'LoweredStmt::Expr\((LoweredExpr::Local\([^)]+\))\)',
        r'LoweredStmt::Expr(\1, Span::generated("test"))',
        rlp
    )

    # Fix LoweredStmt::Expr with RuntimeCall - add Span as second arg
    rlp = re.sub(
        r'LoweredStmt::Expr\((LoweredExpr::RuntimeCall \{.*?span: Span::generated\("test"\) \})\)',
        r'LoweredStmt::Expr(\1, Span::generated("test"))',
        rlp
    )

    # Add missing BigIntLiteral span
    rlp = re.sub(
        r'(LoweredExpr::BigIntLiteral \{\s*decimal: "[^"]*",\s*sign: \d+,\s*limb_low: \d+u?,\s*limb_high: \d+u?\s*)\}',
        r'\1, span: Span::generated("test") }',
        rlp
    )

    files["/home/wogikaze/wgkz/ts2wasm/crates/backend-wasm/src/runtime_link_plan.rs"] = rlp

    # Fix wasm_binary.rs
    wb = files["/home/wogikaze/wgkz/ts2wasm/crates/backend-wasm/src/wasm_binary.rs"]
    wb = wb.replace(
        "use ts2wasm_runtime_abi::ValueTag;",
        "use ts2wasm_frontend::Span;\nuse ts2wasm_runtime_abi::ValueTag;"
    )
    # Fix LoweredStmt::Expr(LoweredExpr::String(...)) -> add Split
    wb = re.sub(
        r'LoweredStmt::Expr\(LoweredExpr::String\("([^"]+)"\.to_owned\(\)\)\)',
        r'LoweredStmt::Expr(LoweredExpr::String("\1".to_owned(), Span::generated("test")), Span::generated("test"))',
        wb
    )
    # Fix RuntimeCall missing span
    wb = re.sub(
        r'(LoweredExpr::RuntimeCall \{\s*runtime_fn: "[^"]*",\s*args: vec!\[[^\]]*\])\s*\}',
        r'\1, span: Span::generated("test") }',
        wb
    )
    # Fix BigIntLiteral missing span
    wb = re.sub(
        r'(LoweredExpr::BigIntLiteral \{\s*decimal: "[^"]*",\s*sign: \d+,\s*limb_low: \d+u?,\s*limb_high: \d+u?\s*)\}',
        r'\1, span: Span::generated("test") }',
        wb
    )
    files["/home/wogikaze/wgkz/ts2wasm/crates/backend-wasm/src/wasm_binary.rs"] = wb

    # Fix string_intern.rs
    si = files["/home/wogikaze/wgkz/ts2wasm/crates/backend-wasm/src/string_intern.rs"]
    # Fix Any LoweredStmt::Expr with single arg
    si = re.sub(
        r'LoweredStmt::Expr\(LoweredExpr::RuntimeCall \{\s*runtime_fn: "([^"]*)",\s*args: vec!\[\]\s*\}\)',
        r'LoweredStmt::Expr(LoweredExpr::RuntimeCall { runtime_fn: "\1", args: vec![], span: Span::generated("test") }, Span::generated("test"))',
        si
    )
    files["/home/wogikaze/wgkz/ts2wasm/crates/backend-wasm/src/string_intern.rs"] = si

    # Write all files
    for filepath, content in files.items():
        with open(filepath, 'w') as f:
            f.write(content)
        print(f"Fixed: {filepath}")

if __name__ == "__main__":
    main()
