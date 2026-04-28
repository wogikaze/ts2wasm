use crate::{DiagCode, Diagnostic, Span};

pub fn validate_type_reference_directives(source: &str) -> Result<(), Diagnostic> {
    if has_skip_lib_check(source) {
        return Ok(());
    }

    let mut previous_line_ts_ignore = false;
    let mut offset = 0;
    for raw_line in source.split_inclusive('\n') {
        let line = raw_line.trim_end_matches(['\r', '\n']);
        let trimmed = line.trim_start();
        let indent = line.len() - trimmed.len();
        let line_start = offset + indent;

        if let Some(package_name) = reference_types_package(trimmed) {
            if previous_line_ts_ignore {
                previous_line_ts_ignore = false;
                offset += raw_line.len();
                continue;
            }

            let package_start = trimmed
                .find(package_name)
                .map(|start| line_start + start)
                .unwrap_or(line_start);
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-227: triple-slash reference types directive for `{package_name}` requires type package resolution, which is not supported in this milestone"
                ),
                span: Some(Span {
                    start: package_start,
                    end: package_start + package_name.len(),
                }),
            });
        }

        previous_line_ts_ignore = is_ts_ignore_line(trimmed);
        offset += raw_line.len();
    }

    Ok(())
}

fn reference_types_package(line: &str) -> Option<&str> {
    let rest = line.strip_prefix("///")?.trim_start();
    let rest = rest.strip_prefix("<reference")?;
    let types_start = rest.find("types")?;
    let after_types = rest[types_start + "types".len()..].trim_start();
    let after_equals = after_types.strip_prefix('=')?.trim_start();
    let quote = after_equals.chars().next()?;
    if quote != '"' && quote != '\'' {
        return None;
    }
    let value_start = quote.len_utf8();
    let value = &after_equals[value_start..];
    let value_end = value.find(quote)?;
    Some(&value[..value_end])
}

fn is_ts_ignore_line(line: &str) -> bool {
    line.strip_prefix("//")
        .is_some_and(|comment| comment.trim_start().starts_with("@ts-ignore"))
}

fn has_skip_lib_check(source: &str) -> bool {
    let compact = source
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .flat_map(char::to_lowercase)
        .collect::<String>();
    compact.contains("\"skiplibcheck\":true") || compact.contains("skiplibcheck:true")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_missing_type_reference_directive_with_issue_link() {
        let error = validate_type_reference_directives(
            "/// <reference types=\"cookie-session\"/>\nconsole.log(1);\n",
        )
        .unwrap_err();

        assert_eq!(error.code, DiagCode::UnsupportedSyntax);
        assert!(error.message.contains("issue-227"));
        assert!(error.message.contains("cookie-session"));
        assert_eq!(error.span, Some(Span { start: 22, end: 36 }));
    }

    #[test]
    fn ts_ignore_suppresses_immediately_following_type_reference_directive() {
        validate_type_reference_directives(
            "// @ts-ignore\n/// <reference types=\"cookie-session\"/>\nconsole.log(1);\n",
        )
        .unwrap();
    }

    #[test]
    fn skip_lib_check_suppresses_type_reference_directive_diagnostics() {
        validate_type_reference_directives(
            r#"
// @filename: /node_modules/foo/index.d.ts
/// <reference types="cookie-session"/>
export const foo = 1;

// @filename: /tsconfig.json
{
    "compilerOptions": {
        "strict": true,
        "skipLibCheck": true
    }
}
"#,
        )
        .unwrap();
    }

    #[test]
    fn ignores_non_types_reference_directives() {
        validate_type_reference_directives(
            "/// <reference path=\"./other.d.ts\"/>\nconsole.log(1);\n",
        )
        .unwrap();
    }
}
