#!/usr/bin/env python3
"""Apply all edits for issue 5186: ExportAssignment AST variant."""

import sys

# 1. ast.rs: Add ExportAssignment variant and update Stmt::span()
with open("crates/frontend/src/ast.rs") as f:
    content = f.read()

# Add variant after ExportDefault
if "ExportAssignment" not in content:
    content = content.replace(
        "    ExportDefault {\n        expr: Expr,\n        default_span: Span,\n        span: Span,\n    },\n    Let {",
        "    ExportDefault {\n        expr: Expr,\n        default_span: Span,\n        span: Span,\n    },\n    ExportAssignment {\n        expr: Expr,\n        span: Span,\n    },\n    Let {"
    )
    content = content.replace(
        "| Self::ExportDefault { span, .. }\n            | Self::Let { span, .. }",
        "| Self::ExportDefault { span, .. }\n            | Self::ExportAssignment { span, .. }\n            | Self::Let { span, .. }"
    )
    with open("crates/frontend/src/ast.rs", "w") as f:
        f.write(content)
    print("  ast.rs: added ExportAssignment variant")
else:
    print("  ast.rs: already has ExportAssignment")

# 2. statements_general.rs: Change Stmt::Expr to Stmt::ExportAssignment
with open("crates/frontend/src/parser/statements_general.rs") as f:
    content = f.read()

if "Stmt::ExportAssignment" not in content:
    content = content.replace(
        "            Ok(Stmt::Expr {\n                expr,\n                span: Span {\n                    start: export_span.start,\n                    end,\n                },\n            })",
        "            Ok(Stmt::ExportAssignment {\n                expr,\n                span: Span {\n                    start: export_span.start,\n                    end,\n                },\n            })"
    )
    with open("crates/frontend/src/parser/statements_general.rs", "w") as f:
        f.write(content)
    print("  statements_general.rs: changed export= to ExportAssignment")
else:
    print("  statements_general.rs: already has ExportAssignment")

# 3. dump.rs: Add ExportAssignment formatting (only if not already present)
with open("crates/compiler/src/dump.rs") as f:
    content = f.read()

if "Stmt::ExportAssignment" in content:
    print("  dump.rs: already has ExportAssignment")
else:
    content = content.replace(
        '        Stmt::ExportDefault { expr, .. } => {\n            let _ = writeln!(out, "export default {};", unparse_expr(expr));\n        }',
        '        Stmt::ExportDefault { expr, .. } => {\n            let _ = writeln!(out, "export default {};", unparse_expr(expr));\n        }\n        Stmt::ExportAssignment { expr, .. } => {\n            let _ = writeln!(out, "export = {};", unparse_expr(expr));\n        }'
    )
    with open("crates/compiler/src/dump.rs", "w") as f:
        f.write(content)
    print("  dump.rs: added ExportAssignment formatting")

# 4. compiler/src/lib.rs: Add ExportAssignment to validate_stmt
with open("crates/compiler/src/lib.rs") as f:
    content = f.read()

if "ExportAssignment" not in content:
    content = content.replace(
        "| Stmt::ExportDefault { .. } => Ok(()),\n        Stmt::ExportDecl { declaration, .. } => {",
        "| Stmt::ExportDefault { .. }\n        | Stmt::ExportAssignment { .. } => Ok(()),\n        Stmt::ExportDecl { declaration, .. } => {"
    )
    with open("crates/compiler/src/lib.rs", "w") as f:
        f.write(content)
    print("  compiler/lib.rs: added ExportAssignment to validate_stmt")
else:
    print("  compiler/lib.rs: already has ExportAssignment")

# 5-13. IR crate files
ir_files = {
    "crates/ir/src/builtin_resolver_outer.rs": [
        ("| Stmt::ExportDefault { .. }\n        | Stmt::AmbientValueDecl { .. }",
         "| Stmt::ExportDefault { .. }\n        | Stmt::ExportAssignment { .. }\n        | Stmt::AmbientValueDecl { .. }"),
        ("| Stmt::ExportDefault { .. }\n        | Stmt::Break { .. }",
         "| Stmt::ExportDefault { .. }\n        | Stmt::ExportAssignment { .. }\n        | Stmt::Break { .. }"),
    ],
    "crates/ir/src/builtin_resolver_bigint.rs": [
        ("| Stmt::ExportDefault { .. }\n            | Stmt::AmbientValueDecl { .. }",
         "| Stmt::ExportDefault { .. }\n            | Stmt::ExportAssignment { .. }\n            | Stmt::AmbientValueDecl { .. }"),
    ],
    "crates/ir/src/builtin_resolver_bigint_ops.rs": [
        ("| Stmt::ExportDefault { .. }\n        | Stmt::Break { .. }",
         "| Stmt::ExportDefault { .. }\n        | Stmt::ExportAssignment { .. }\n        | Stmt::Break { .. }"),
    ],
    "crates/ir/src/builtin_resolver_class_features.rs": [
        ("| Stmt::ExportDefault { .. }\n        | Stmt::Let { .. }",
         "| Stmt::ExportDefault { .. }\n        | Stmt::ExportAssignment { .. }\n        | Stmt::Let { .. }"),
        ("| Stmt::ExportDefault { span, .. } => Err(static_block_unsupported(",
         "| Stmt::ExportDefault { span, .. }\n        | Stmt::ExportAssignment { span, .. } => Err(static_block_unsupported("),
    ],
    "crates/ir/src/builtin_resolver.rs": [
        ("| Stmt::ExportNamespaceFrom { .. }\n            | Stmt::AmbientValueDecl { .. }",
         "| Stmt::ExportNamespaceFrom { .. }\n            | Stmt::ExportAssignment { .. }\n            | Stmt::AmbientValueDecl { .. }"),
        ("| Stmt::ExportDefault { .. } => Err(Diagnostic {",
         "| Stmt::ExportDefault { .. }\n        | Stmt::ExportAssignment { .. } => Err(Diagnostic {"),
    ],
    "crates/frontend/src/parser/statements_class.rs": [
        ("| Stmt::ExportDefault { .. } => {}",
         "| Stmt::ExportDefault { .. }\n            | Stmt::ExportAssignment { .. } => {}"),
    ],
}

for filepath, replacements in ir_files.items():
    with open(filepath) as f:
        content = f.read()
    changed = False
    for old, new in replacements:
        if old in content and "ExportAssignment" not in content:
            content = content.replace(old, new)
            changed = True
            print(f"  {filepath}: updated")
        else:
            print(f"  {filepath}: already has ExportAssignment or no match")
    if changed:
        with open(filepath, "w") as f:
            f.write(content)

# name_resolver.rs: add ExportAssignment arm
with open("crates/ir/src/name_resolver.rs") as f:
    content = f.read()

if "ExportAssignment" not in content:
    old = '''            Stmt::ExportDefault { span, .. } => {
                Err(unsupported_module_decl(*span, "default export"))
            }'''
    new = '''            Stmt::ExportDefault { span, .. } => {
                Err(unsupported_module_decl(*span, "default export"))
            }
            Stmt::ExportAssignment { span, .. } => {
                Err(unsupported_module_decl(*span, "export assignment"))
            }'''
    content = content.replace(old, new, 1)
    with open("crates/ir/src/name_resolver.rs", "w") as f:
        f.write(content)
    print("  name_resolver.rs: added ExportAssignment arm")
else:
    print("  name_resolver.rs: already has ExportAssignment")

# Tests: frontend parser tests
with open("crates/frontend/src/parser/tests.rs") as f:
    content = f.read()

if "parses_export_assignment_expression" not in content:
    # Insert before the closing } of the mod tests block
    # Find the last line that is just "}"
    lines = content.split('\n')
    for i in range(len(lines) - 1, -1, -1):
        if lines[i].strip() == '}':
            test_code = '''
    #[test]
    fn parses_export_assignment_expression() {
        let stmts = parse_program("export = foo;").unwrap();
        assert_eq!(stmts.len(), 1);
        match &stmts[0] {
            Stmt::ExportAssignment { expr, span } => {
                assert!(matches!(expr, Expr::Ident { name, .. } if name == "foo"));
                assert_eq!(span.start, 0);
                assert_eq!(span.end, 14);
            }
            other => panic!("expected ExportAssignment, got {other:?}"),
        }
    }
'''
            lines.insert(i, test_code)
            break
    content = '\n'.join(lines)
    with open("crates/frontend/src/parser/tests.rs", "w") as f:
        f.write(content)
    print("  parser/tests.rs: added test")
else:
    print("  parser/tests.rs: already has test")

# CLI tests
with open("crates/cli/tests/parser_ast_structures.rs") as f:
    content = f.read()

if "export_assignment_creates_ast_node" not in content:
    content = content.rstrip() + """

#[test]
fn export_assignment_creates_ast_node() {
    let stmts = parse("export = foo;");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::ExportAssignment { expr, .. } => {
            assert!(matches!(expr, Expr::Ident { name, .. } if name == "foo"));
        }
        other => panic!("expected ExportAssignment, got {other:?}"),
    }
}"""
    with open("crates/cli/tests/parser_ast_structures.rs", "w") as f:
        f.write(content)
    print("  parser_ast_structures.rs: added test")
else:
    print("  parser_ast_structures.rs: already has test")

print("\nAll edits applied successfully.")
