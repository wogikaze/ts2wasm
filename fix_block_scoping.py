#!/usr/bin/env python3
"""Apply all changes for issue 5249: block-local class declarations."""

import re

# === 1. AST: Add Stmt::Block variant ===
ast_path = "crates/frontend/src/ast.rs"
with open(ast_path) as f:
    ast_content = f.read()

# Add Block variant after Continue
old = """    Continue {
        label: Option<String>,
        span: Span,
    },
}"""
new = """    Continue {
        label: Option<String>,
        span: Span,
    },
    Block {
        statements: Vec<Stmt>,
        span: Span,
    },
}"""
assert old in ast_content, "Could not find Continue in ast.rs"
ast_content = ast_content.replace(old, new, 1)

# Add Block to the span() method
old_span = """            | Self::Break { span, .. }
            | Self::Continue { span, .. } => *span,"""
new_span = """            | Self::Break { span, .. }
            | Self::Continue { span, .. }
            | Self::Block { span, .. } => *span,"""
assert old_span in ast_content, "Could not find span() pattern in ast.rs"
ast_content = ast_content.replace(old_span, new_span, 1)

with open(ast_path, "w") as f:
    f.write(ast_content)
print("ast.rs: Added Stmt::Block variant")

# === 2. Parser: Stop flattening blocks in parse_program ===
core_path = "crates/frontend/src/parser/statements_core.rs"
with open(core_path) as f:
    core_content = f.read()

old_parse = """            if matches!(self.peek(), Some(Token::LeftBrace)) {
                let block_stmts = self.block()?;
                statements.extend(block_stmts);
                continue;
            }"""
assert old_parse in core_content, "Could not find flattening block in parse_program"
core_content = core_content.replace(old_parse, """            if matches!(self.peek(), Some(Token::LeftBrace)) {
                statements.push(self.block_as_stmt()?);
                continue;
            }""", 1)
with open(core_path, "w") as f:
    f.write(core_content)
print("statements_core.rs: Updated parse_program to use block_as_stmt()")

# === 3. Parser: Add block_as_stmt() method, update block() to not flatten ===
general_path = "crates/frontend/src/parser/statements_general.rs"
with open(general_path) as f:
    general_content = f.read()

# Fix the nested block flattening in block() method
old_block_nest = """            // Handle nested block statements (e.g. `{ class C {} }`)
            if matches!(self.peek(), Some(Token::LeftBrace)) {
                let nested = self.block()?;
                statements.extend(nested);
                continue;
            }"""
assert old_block_nest in general_content, "Could not find nested block flattening in block()"
general_content = general_content.replace(old_block_nest, """            // Handle nested block statements (e.g. `{ class C {} }`)
            if matches!(self.peek(), Some(Token::LeftBrace)) {
                statements.push(self.block_as_stmt()?);
                continue;
            }""", 1)

# Add block_as_stmt() method after block()
old_block_end = """        Ok(statements)
    }

    fn statement_body"""
assert old_block_end in general_content, "Could not find block() end"
new_block_as_stmt = """        Ok(statements)
    }

    /// Parse a standalone block statement and return it wrapped in `Stmt::Block`.
    /// Unlike `block()`, this preserves the block boundary in the AST so that
    /// block-scoped declarations (e.g. `class C {}`) are distinguishable from
    /// top-level declarations during name resolution.
    fn block_as_stmt(&mut self) -> Result<Stmt, Diagnostic> {
        let left_brace = self.expect(TokenKind::LeftBrace)?;
        let mut stmts = Vec::new();
        while !self.consume(TokenKind::RightBrace) {
            if self.is_at_end() {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "unterminated block".to_owned(),
                    span: self.prev_span().or_else(|| self.peek_span()),
                });
            }
            if self.consume(TokenKind::Semicolon) {
                continue;
            }
            if let Some(stmt) = self.take_pending_statement() {
                stmts.push(stmt);
                continue;
            }
            if self.consume_erasable_typescript_declaration()? {
                continue;
            }
            if matches!(self.peek(), Some(Token::LeftBrace)) {
                stmts.push(self.block_as_stmt()?);
                continue;
            }
            stmts.push(self.statement()?);
        }
        let end_span = self.prev_span().unwrap_or(left_brace);
        Ok(Stmt::Block {
            statements: stmts,
            span: Span {
                start: left_brace.start,
                end: end_span.end,
            },
        })
    }

    fn statement_body"""
assert old_block_end in general_content, "Could not find block() end in general.rs"
general_content = general_content.replace(old_block_end, new_block_as_stmt, 1)

with open(general_path, "w") as f:
    f.write(general_content)
print("statements_general.rs: Added block_as_stmt() and fixed block() nested flattening")

# === 4. Name resolver: Handle Stmt::Block ===
resolver_path = "crates/ir/src/name_resolver.rs"
with open(resolver_path) as f:
    resolver_content = f.read()

# Skip class declarations inside blocks in the first pass
# The first pass collects class declarations only from the top-level program (not inside blocks)
# No change needed here - the current code only checks Stmt::ClassDecl at the program level,
# but since blocks are now preserved, inner ClassDecl within Stmt::Block won't be matched

# Add Stmt::Block handler in resolve_stmt's match arm
old_resolve_handle = """            Stmt::Continue { label, span } => {"""
new_resolve_handle = """            Stmt::Block { statements, span: block_span } => {
                self.enter_scope();
                // Register block-scoped class declarations in the current scope
                // so they don't collide with outer scope names during resolution.
                for stmt in statements {
                    if let Stmt::ClassDecl { name, .. } = stmt {
                        self.declare_variable(name, Some(*block_span), false)?;
                    }
                }
                let resolved = statements
                    .iter()
                    .map(|s| self.resolve_stmt(s))
                    .collect::<Result<Vec<_>, _>>()?;
                self.exit_scope();
                Ok(Stmt::Block {
                    statements: resolved,
                    span: *block_span,
                })
            }
            Stmt::Continue { label, span } => {"""
assert old_resolve_handle in resolver_content, "Could not find Continue handler in name_resolver.rs"
resolver_content = resolver_content.replace(old_resolve_handle, new_resolve_handle, 1)

with open(resolver_path, "w") as f:
    f.write(resolver_content)
print("name_resolver.rs: Added Stmt::Block handler")

# === 5. Builtin resolver: Handle Stmt::Block ===
builtin_path = "crates/ir/src/builtin_resolver.rs"
with open(builtin_path) as f:
    builtin_content = f.read()

# Find the fold_stmt function and add Stmt::Block handler before the fallthrough
# The fold_stmt function has a match with a final catch-all arm.
# We need to add Block before the import/etc fallthrough
fold_match = """            Stmt::ImportSideEffect { .. }
            | Stmt::ImportNamed { .. }
            | Stmt::ImportDefault { .. }
            | Stmt::ImportDefaultNamed { .. }
            | Stmt::ImportNamespace { .. }
            | Stmt::ImportDefaultNamespace { .. }
            | Stmt::ExportNamed { .. }
            | Stmt::ExportNamedFrom { .. }
            | Stmt::ExportAllFrom { .. }
            | Stmt::ExportNamespaceFrom { .. }
            | Stmt::ExportAssignment { .. }
            | Stmt::AmbientValueDecl { .. }
            | Stmt::Break { .. }
            | Stmt::Continue { .. } => stmt.clone(),"""
assert fold_match in builtin_content, "Could not find fold_stmt fallthrough"
block_fold = """            Stmt::Block { statements, .. } => {
                let folded = fold_stmts(statements, bindings, freeze, class_freezer)?;
                Stmt::Block {
                    statements: folded,
                    span: Span::generated("block"),
                }
            }
""" + fold_match
builtin_content = builtin_content.replace(fold_match, block_fold, 1)

# Add Stmt::Block to the resolve_stmt_with_outer_bindings match
# Find the Break match arm
old_break_arm = """        Stmt::Break { label, .. } => Ok(ResolvedStmt::Break {"""
new_break_arm = """        Stmt::Block { statements, .. } => {
            Ok(ResolvedStmt::Block {
                statements: statements
                    .iter()
                    .map(resolve_stmt)
                    .collect::<Result<Vec<_>, _>>()?,
            })
        }
        Stmt::Break { label, .. } => Ok(ResolvedStmt::Break {"""
assert old_break_arm in builtin_content, "Could not find Stmt::Break in resolve_stmt"
builtin_content = builtin_content.replace(old_break_arm, new_break_arm, 1)

with open(builtin_path, "w") as f:
    f.write(builtin_content)
print("builtin_resolver.rs: Added Stmt::Block handlers")

print("\nAll files patched successfully!")
