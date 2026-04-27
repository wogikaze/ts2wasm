use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Deserialize;

use crate::{DiagCode, Diagnostic, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeScriptCheckReport {
    pub typescript_version: Option<String>,
    pub diagnostics: Vec<TypeScriptDiagnostic>,
}

impl TypeScriptCheckReport {
    pub fn is_ok(&self) -> bool {
        self.diagnostics.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeScriptDiagnostic {
    pub code: u32,
    pub category: String,
    pub message: String,
    pub file: Option<PathBuf>,
    pub start: Option<usize>,
    pub length: Option<usize>,
    pub line: Option<usize>,
    pub character: Option<usize>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OracleReport {
    diagnostics: Vec<OracleDiagnostic>,
    typescript_version: Option<String>,
    error: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OracleDiagnostic {
    code: u32,
    category: String,
    message: String,
    file: Option<PathBuf>,
    start: Option<usize>,
    length: Option<usize>,
    line: Option<usize>,
    character: Option<usize>,
}

pub fn collect_typescript_diagnostics(input: &Path) -> Result<TypeScriptCheckReport, Diagnostic> {
    let script = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("scripts/check/typescript-oracle.js");
    let output = Command::new("node")
        .arg(&script)
        .arg(input)
        .output()
        .map_err(|error| Diagnostic {
            code: DiagCode::BackendIo,
            message: format!("failed to execute TypeScript compiler oracle: {error}"),
            span: None,
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let report: OracleReport = serde_json::from_str(&stdout).map_err(|error| Diagnostic {
        code: DiagCode::BackendIo,
        message: format!(
            "failed to parse TypeScript compiler oracle JSON: {error}; stdout: {stdout}; stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        ),
        span: None,
    })?;

    if !output.status.success() {
        return Err(Diagnostic {
            code: DiagCode::BackendIo,
            message: report.error.unwrap_or_else(|| {
                format!(
                    "TypeScript compiler oracle failed with status {}",
                    output.status
                )
            }),
            span: None,
        });
    }

    Ok(TypeScriptCheckReport {
        typescript_version: report.typescript_version,
        diagnostics: report
            .diagnostics
            .into_iter()
            .map(|diagnostic| TypeScriptDiagnostic {
                code: diagnostic.code,
                category: diagnostic.category,
                message: diagnostic.message,
                file: diagnostic.file,
                start: diagnostic.start,
                length: diagnostic.length,
                line: diagnostic.line,
                character: diagnostic.character,
            })
            .collect(),
    })
}

pub fn check_typescript_file(input: &Path) -> Result<(), Diagnostic> {
    let report = collect_typescript_diagnostics(input)?;
    match report.diagnostics.first() {
        Some(diagnostic) => Err(typecheck_diagnostic(diagnostic)),
        None => Ok(()),
    }
}

fn typecheck_diagnostic(diagnostic: &TypeScriptDiagnostic) -> Diagnostic {
    let location = match (&diagnostic.file, diagnostic.line, diagnostic.character) {
        (Some(file), Some(line), Some(character)) => {
            format!("{}:{line}:{character}: ", file.display())
        }
        _ => String::new(),
    };
    let span = match (diagnostic.start, diagnostic.length) {
        (Some(start), Some(length)) => Some(Span {
            start,
            end: start.saturating_add(length),
        }),
        _ => None,
    };
    Diagnostic {
        code: DiagCode::TypeScriptTypeCheck,
        message: format!(
            "{location}typescript TS{} {}: {}",
            diagnostic.code, diagnostic.category, diagnostic.message
        ),
        span,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("fixtures/basics-types")
            .join(name)
    }

    fn skip_if_oracle_missing(
        result: Result<TypeScriptCheckReport, Diagnostic>,
    ) -> Option<TypeScriptCheckReport> {
        match result {
            Ok(report) => Some(report),
            Err(error)
                if error.code == DiagCode::BackendIo
                    && error
                        .message
                        .contains("failed to load TypeScript compiler API") =>
            {
                eprintln!(
                    "skip-with-reason: TypeScript compiler API is not installed; run npm install"
                );
                None
            }
            Err(error) => panic!("unexpected TypeScript oracle error: {error}"),
        }
    }

    #[test]
    fn accepts_valid_typescript_fixture() {
        let Some(report) =
            skip_if_oracle_missing(collect_typescript_diagnostics(&fixture("types.ts")))
        else {
            return;
        };
        assert!(report.is_ok(), "unexpected diagnostics: {report:?}");
        assert!(report.typescript_version.is_some());
    }

    #[test]
    fn propagates_typescript_diagnostic() {
        let result = check_typescript_file(&fixture("type-error.ts"));
        match result {
            Err(error) if error.code == DiagCode::TypeScriptTypeCheck => {
                assert!(error.message.contains("TS2322"), "{error}");
                assert!(error.message.contains("Type 'string'"), "{error}");
                assert!(error.span.is_some());
            }
            Err(error)
                if error.code == DiagCode::BackendIo
                    && error
                        .message
                        .contains("failed to load TypeScript compiler API") =>
            {
                eprintln!(
                    "skip-with-reason: TypeScript compiler API is not installed; run npm install"
                );
            }
            other => panic!("expected TypeScript type diagnostic, got {other:?}"),
        }
    }
}
