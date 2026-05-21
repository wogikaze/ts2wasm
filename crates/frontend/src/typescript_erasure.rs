/// ErasureReport: records which TypeScript constructs were erased or rejected
/// during frontend processing. This is the machine-readable record consumed by
/// coverage reporting and triage tooling.
///
/// Ownership: this module belongs to the frontend crate. It records what the
/// parser could recognize and what was erased or rejected. It does NOT assign
/// runtime policy or call backend types.
use crate::ast::Stmt;
use crate::diagnostic::Span;

/// Kinds of TypeScript syntax that the frontend can erase before runtime lowering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ErasureKind {
    TypeAnnotation,
    InterfaceDecl,
    TypeAliasDecl,
    AmbientDecl,
    AmbientNamespace,
    GenericParams,
    AsAssertion,
    Satisfies,
    ConstAssertion,
    TypeOnlyImport,
}

/// A serialization-friendly span representation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SpanRecord {
    pub start: usize,
    pub end: usize,
}

/// A single erased TypeScript construct.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErasedSyntax {
    pub kind: ErasureKind,
    pub span: SpanRecord,
}

/// An unsupported TypeScript construct that was rejected with a diagnostic.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UnsupportedTsSyntax {
    pub feature: String,
    pub span: SpanRecord,
    pub diagnostic_code: String,
    pub tracking: Option<String>,
}

/// Complete erasure report for a parsed program.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErasureReport {
    pub erased: Vec<ErasedSyntax>,
    pub unsupported: Vec<UnsupportedTsSyntax>,
}

fn to_span_record(s: Span) -> SpanRecord {
    SpanRecord {
        start: s.start,
        end: s.end,
    }
}

/// Collect an erasure report from a parsed program's statements.
///
/// This scans the statement list for TypeScript-only constructs and categorizes
/// them as erased or unsupported. It does NOT lower or emit any code; it only
/// reports what the frontend recognized.
pub fn collect_erasure_report(program: &[Stmt]) -> ErasureReport {
    let mut erased = Vec::new();
    let mut unsupported = Vec::new();

    for stmt in program {
        collect_from_stmt(stmt, &mut erased, &mut unsupported);
    }

    ErasureReport {
        erased,
        unsupported,
    }
}

fn collect_from_stmt(
    stmt: &Stmt,
    erased: &mut Vec<ErasedSyntax>,
    unsupported: &mut Vec<UnsupportedTsSyntax>,
) {
    match stmt {
        // Note: Stmt::EnumDecl exists in the AST but the current parser
        // erases enums at the token level (see parser/statements_ts.rs),
        // so no EnumDecl statements are produced. When the parser is
        // updated to emit EnumDecl nodes, this arm can be enabled.
        // Stmt::EnumDecl { span, .. } => { }
        Stmt::AmbientValueDecl { span, .. } => {
            erased.push(ErasedSyntax {
                kind: ErasureKind::AmbientDecl,
                span: to_span_record(*span),
            });
        }
        Stmt::Function {
            is_ambient: true,
            span,
            ..
        } => {
            erased.push(ErasedSyntax {
                kind: ErasureKind::AmbientDecl,
                span: to_span_record(*span),
            });
        }
        Stmt::ImportNamed {
            import_type: true,
            span,
            ..
        } => {
            erased.push(ErasedSyntax {
                kind: ErasureKind::TypeOnlyImport,
                span: to_span_record(*span),
            });
        }
        Stmt::ExportDecl { declaration, .. } => {
            if let Stmt::Function {
                is_ambient: true,
                span,
                ..
            } = declaration.as_ref()
            {
                erased.push(ErasedSyntax {
                    kind: ErasureKind::AmbientDecl,
                    span: to_span_record(*span),
                });
            }
            collect_from_stmt(declaration, erased, unsupported);
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Stmt;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    /// Helper: parse a source string and return the resulting statements.
    fn parse_source(source: &str) -> Vec<Stmt> {
        let lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("tokenize failed");
        let mut parser = Parser::new(tokens, source);
        parser.parse_program().expect("parse failed")
    }

    #[test]
    fn erasure_report_ambient_decl() {
        // AmbientValueDecl: declare var/let/const
        let source = "declare var x: number;";
        let stmts = parse_source(source);
        let report = collect_erasure_report(&stmts);
        assert_eq!(report.erased.len(), 1, "should erase ambient decl");
        assert_eq!(report.erased[0].kind, ErasureKind::AmbientDecl);
    }

    #[test]
    fn erasure_report_ambient_function() {
        let source = "declare function foo(): void;";
        let stmts = parse_source(source);
        let report = collect_erasure_report(&stmts);
        assert_eq!(report.erased.len(), 1, "should erase ambient function");
        assert_eq!(report.erased[0].kind, ErasureKind::AmbientDecl);
    }

    #[test]
    fn erasure_report_enum_erased_at_parser_level() {
        // The parser currently erases enums at the token level (no Stmt produced).
        // When the parser emits EnumDecl nodes, update this test.
        let source = "enum Color { Red, Green, Blue }";
        let stmts = parse_source(source);
        let report = collect_erasure_report(&stmts);
        // Enums are fully consumed by the parser without producing Stmt nodes
        assert_eq!(report.erased.len(), 0);
        assert_eq!(report.unsupported.len(), 0);
    }

    #[test]
    fn erasure_report_type_only_import() {
        let source = "import type { Foo } from './mod';";
        let stmts = parse_source(source);
        let report = collect_erasure_report(&stmts);
        assert_eq!(report.erased.len(), 1, "type-only import should be erased");
        assert_eq!(report.erased[0].kind, ErasureKind::TypeOnlyImport);
    }

    #[test]
    fn erasure_report_empty_program() {
        let stmts: Vec<Stmt> = vec![];
        let report = collect_erasure_report(&stmts);
        assert_eq!(report.erased.len(), 0);
        assert_eq!(report.unsupported.len(), 0);
    }

    #[test]
    fn erasure_report_keeps_runtime_code() {
        let source = "let x = 42; console.log(x);";
        let stmts = parse_source(source);
        let report = collect_erasure_report(&stmts);
        assert_eq!(report.erased.len(), 0);
        assert_eq!(report.unsupported.len(), 0);
    }

    #[test]
    fn erasure_report_json_serialization() {
        let report = ErasureReport {
            erased: vec![ErasedSyntax {
                kind: ErasureKind::AmbientDecl,
                span: SpanRecord { start: 0, end: 17 },
            }],
            unsupported: vec![UnsupportedTsSyntax {
                feature: "enum".to_string(),
                span: SpanRecord { start: 0, end: 4 },
                diagnostic_code: "UnsupportedTypeScriptSyntax".to_string(),
                tracking: None,
            }],
        };
        let json = serde_json::to_string_pretty(&report).expect("serialization failed");
        assert!(json.contains("AmbientDecl"));
        assert!(json.contains("UnsupportedTypeScriptSyntax"));
        assert!(json.contains("erased"));
        assert!(json.contains("unsupported"));
    }
}
