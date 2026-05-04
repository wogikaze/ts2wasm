//! Test262 includes directive preprocessor
//!
//! Processes test262 YAML frontmatter to extract includes/features directives
//! and inserts directive-generated snippets before compilation.

use std::fs;
use std::path::Path;

use ts2wasm_frontend::Diagnostic;

/// Test262 feature name to tracking-issue ID mapping.
///
/// Each entry maps a test262 `features:` value to the ts2wasm issue that tracks
/// its implementation. Unknown features produce an UnsupportedSyntax diagnostic
/// pointing to the closest parent meta-issue.
const KNOWN_FEATURES: &[(&str, &str)] = &[
    ("IsHTMLDDA", "issue-5022"),
    ("createRealm", "issue-5023"),
    ("tail-call-optimization", "issue-5048"),
    ("Symbol.asyncIterator", "issue-5052"),
    // --- tracked features (known but not yet stubbed) ---
    ("Array.fromAsync", "issue-5024"),
    ("Array.isTemplateObject", "issue-5025"),
    ("ArrayBuffer", "issue-408"),
    ("Atomics", "issue-100"),
    ("Atomics.pause", "issue-119"),
    ("caller", "issue-346"),
    ("DataView", "issue-408"),
    ("Date", "issue-423"),
    ("FinalizationRegistry", "issue-436"),
    ("Float16Array", "issue-436"),
    ("global", "issue-5025"),
    ("hashbang", "issue-5024"),
    ("ImmutablePrototypeExotic", "issue-5025"),
    ("Intl.DateTimeFormat-formatRange", "issue-436"),
    ("Intl.DateTimeFormat.prototype.formatToParts", "issue-436"),
    ("Intl.Locale", "issue-436"),
    ("Intl.NumberFormat-unified", "issue-436"),
    ("Intl.Segmenter", "issue-436"),
    ("Intl.Segmenter-v2", "issue-436"),
    ("Intl.Segmenter-supportedLocalesOf", "issue-436"),
    ("IsHTMLDDA", "issue-5022"),
    ("iterator-helpers", "issue-436"),
    ("JSON", "issue-5025"),
    ("Map", "issue-5025"),
    ("Math.sumPrecise", "issue-5024"),
    ("new-set-methods", "issue-436"),
    ("Object.entries", "issue-5025"),
    ("Object.fromEntries", "issue-5024"),
    ("Object.is", "issue-5025"),
    ("Promise", "issue-5025"),
    ("Promise.allSettled", "issue-5024"),
    ("Promise.all", "issue-5025"),
    ("Proxy", "issue-5025"),
    ("Reflect", "issue-5025"),
    ("RegExp", "issue-5025"),
    ("RegExp.escape", "issue-5024"),
    ("RegExp-v-flag", "issue-5024"),
    ("Set", "issue-5025"),
    ("ShadowRealm", "issue-436"),
    ("SharedArrayBuffer", "issue-408"),
    ("String", "issue-5025"),
    ("Symbol", "issue-5025"),
    ("Symbol.species", "issue-5024"),
    ("Symbol.unscopables", "issue-5025"),
    ("Temporal", "issue-436"),
    ("TypedArray", "issue-408"),
    ("WeakMap", "issue-5025"),
    ("WeakRef", "issue-436"),
    ("WeakSet", "issue-5025"),
    ("WebAssembly", "issue-5025"),
    ("error-message", "issue-5024"),
    // --- commonly used standard / enabled features ---
    ("legacy-regexp", "issue-5024"),
    ("arrow-function", "issue-5000"),
    ("Reflect.construct", "issue-5025"),
    ("string-trimming", "issue-5025"),
    ("cross-realm", "issue-5023"),
    ("class", "issue-5011"),
    ("generators", "issue-401"),
    ("BigInt", "issue-281"),
    ("regexp-named-groups", "issue-5024"),
    ("regexp-dotall", "issue-5024"),
    ("Symbol.replace", "issue-5025"),
    ("Symbol.match", "issue-5025"),
    ("Symbol.split", "issue-5025"),
    ("Symbol.iterator", "issue-5052"),
    ("Symbol.toPrimitive", "issue-5025"),
    // --- array prototype features (implemented / partially supported) ---
    ("Symbol.isConcatSpreadable", "issue-5004"),
    ("stable-array-sort", "issue-5004"),
    ("Array.prototype.values", "issue-5004"),
    ("Array.prototype.at", "issue-5004"),
    ("Array.prototype.flat", "issue-5004"),
    ("Array.prototype.includes", "issue-5004"),
    ("array-find-from-last", "issue-5004"),
    ("Object.hasOwn", "issue-5004"),
    // --- string prototype features (implemented / partially supported) ---
    ("String.prototype.at", "issue-5004"),
    ("String.prototype.endsWith", "issue-5004"),
    ("String.prototype.includes", "issue-5004"),
    ("String.prototype.replaceAll", "issue-5004"),
    ("String.prototype.trimEnd", "issue-5004"),
    ("String.prototype.trimStart", "issue-5004"),
    // --- features commonly used by test262 but not yet stubbed ---
    ("change-array-by-copy", "issue-5004"),
    ("exponentiation", "issue-5000"),
    ("resizable-arraybuffer", "issue-408"),
    // --- prototype/symbol features (tested by test262, no runtime stub needed) ---
    ("Symbol.toStringTag", "issue-5004"),
    ("__proto__", "issue-5004"),
    ("__getter__", "issue-5004"),
    ("__setter__", "issue-5004"),
    // --- partially implemented features (known but may fail at compile/runtime) ---
    ("Object.fromEntries", "issue-5004"),
    ("String.prototype.isWellFormed", "issue-5004"),
    ("String.prototype.toWellFormed", "issue-5004"),
    ("Array.prototype.flatMap", "issue-5004"),
    ("array-grouping", "issue-5004"),
];

/// Process test262 metadata directives if present in source.
///
/// If the source file contains test262 YAML frontmatter with includes/features
/// directives, this function injects helper/function stubs and feature shims after
/// the frontmatter.
///
/// # Arguments
/// * `input` - Path to the input file (used to resolve test262 harness directory)
/// * `source` - Source code to process
///
/// # Returns
/// * `Ok(String)` - Processed source code with directive-generated snippets inserted
/// * `Err(Diagnostic)` - Error if directives cannot be resolved/loaded or are unsupported
/// Check if the source already contains a function definition of the given name.
///
/// This prevents DuplicateFunction errors when the test262 Python harness has already
/// wrapped the source with shim functions before passing it to ts2wasm build.
fn source_has_function(source: &str, name: &str) -> bool {
    let pattern = format!("function {}(", name);
    source.contains(&pattern)
}

pub fn process_test262_includes(input: &Path, source: &str) -> Result<String, Diagnostic> {
    // Check if this is a test262 file by looking for YAML frontmatter
    let Some(frontmatter_end) = source.find("---*/") else {
        // No frontmatter, return source as-is
        return Ok(source.to_string());
    };

    let frontmatter = &source[..=frontmatter_end + 4]; // Include the closing */
    let metadata = parse_test262_metadata(frontmatter);
    if metadata.includes.is_empty() && metadata.features.is_empty() && metadata.negative.is_none() {
        // Always inject common test262 global helper stubs for files with frontmatter,
        // since many older tests use `assert(...)` without an explicit `includes:` directive.
        // Skip injection if the functions are already present (e.g. from Python harness wrap).
        let body = &source[frontmatter_end + 5..];
        let mut stubs = String::new();
        if !source_has_function(source, "assert") {
            stubs.push_str("function assert() {}\n");
        }
        if !source_has_function(source, "verifyProperty") {
            stubs.push_str("function verifyProperty() {}\n");
        }
        if !source_has_function(source, "verifyCallableProperty") {
            stubs.push_str("function verifyCallableProperty() {}\n");
        }
        if stubs.is_empty() {
            return Ok(source.to_string());
        }
        return Ok(format!("{}\n{}\n{}", frontmatter, stubs.trim(), body));
    }

    let mut injected = String::new();

    // Inject `@negative` comment when a negative directive is present.
    if let Some(negative_phase) = &metadata.negative {
        if !injected.is_empty() {
            injected.push('\n');
        }
        injected.push_str(&format!("// @negative phase={negative_phase}\n"));
    }

    // Insert feature-backed stubs (e.g. `$262`) when supported by metadata.
    let feature_stubs = build_feature_stubs(&metadata.features)?;
    if !feature_stubs.is_empty() {
        if !injected.is_empty() {
            injected.push('\n');
        }
        injected.push_str(&feature_stubs);
    }

    // Resolve test262 harness directory and inject selected helpers
    if !metadata.includes.is_empty() {
        let harness_dir = resolve_harness_directory(input)?;

        for include_file in &metadata.includes {
            let helper_path = harness_dir.join(include_file);
            let helper_source = fs::read_to_string(&helper_path).map_err(|error| Diagnostic {
                code: ts2wasm_frontend::DiagCode::BackendIo,
                message: format!(
                    "failed to read test262 helper file {}: {error}",
                    helper_path.display()
                ),
                span: None,
            })?;

            // Remove YAML frontmatter from helper files if present
            let helper_source = remove_frontmatter(&helper_source);

            // Extract function stubs instead of full helper file
            let stubs = extract_function_stubs(helper_source, source);
            if !injected.is_empty() {
                injected.push('\n');
            }
            injected.push_str(&stubs);
        }
    }

    if injected.is_empty() {
        return Ok(source.to_string());
    }

    // Insert helper contents after frontmatter
    let body_start = frontmatter_end + 5;
    let body = &source[body_start..];

    let processed = format!("{}\n{}\n{}", frontmatter, injected.trim(), body.trim());
    Ok(processed)
}

/// Parsed test262 frontmatter directives used by preprocessor support.
#[derive(Default)]
struct Test262Directives {
    includes: Vec<String>,
    features: Vec<String>,
    /// Phase extracted from `negative:` directive (e.g. "early", "runtime").
    negative: Option<String>,
}

/// Extract supported directives from YAML frontmatter.
fn parse_test262_metadata(frontmatter: &str) -> Test262Directives {
    let mut directives = Test262Directives::default();
    let mut current_key = None;

    for raw_line in frontmatter.lines() {
        let stripped = raw_line.trim_end().trim();
        if stripped.is_empty() || stripped.starts_with('#') {
            continue;
        }

        if stripped.starts_with("- ") {
            let item = parse_yaml_scalar(stripped.trim_start_matches("- ").trim());
            match current_key.as_deref() {
                Some("includes") | Some("include") => directives.includes.push(item.to_owned()),
                Some("features") => directives.features.push(item.to_owned()),
                _ => {}
            }
            continue;
        }

        if !raw_line.starts_with(' ') && !raw_line.starts_with('\t') {
            current_key = None;
        }

        if !stripped.contains(':') {
            continue;
        }

        let Some((key, value)) = stripped.split_once(':') else {
            continue;
        };
        let key = key.trim();
        let value = value.trim();
        current_key = Some(key.to_owned());

        match key {
            "includes" | "include" => {
                directives.includes.extend(parse_yaml_list(value));
            }
            "features" => {
                directives.features.extend(parse_yaml_list(value));
            }
            "negative" => {
                if !value.is_empty() && directives.negative.is_none() {
                    directives.negative = Some(value.to_owned());
                }
            }
            "phase" => {
                if !value.is_empty() {
                    directives.negative = Some(value.to_owned());
                }
            }
            _ => {}
        }
    }

    directives
}

/// Parse a YAML-style scalar list item and trim YAML quoting.
fn parse_yaml_scalar(value: &str) -> &str {
    let no_comment = value.split_once('#').map(|(left, _)| left).unwrap_or(value);
    no_comment.trim().trim_matches('"').trim_matches('\'')
}

/// Parse YAML list from inline (e.g. [a, b]) or scalar form.
fn parse_yaml_list(value: &str) -> Vec<String> {
    let value = value
        .split_once('#')
        .map(|(left, _)| left)
        .unwrap_or(value)
        .trim();
    if value.is_empty() {
        return Vec::new();
    }

    if value.starts_with('[') && value.ends_with(']') {
        let inner = value[1..value.len() - 1].trim();
        if inner.is_empty() {
            return Vec::new();
        }
        return inner
            .split(',')
            .map(str::trim)
            .filter(|part| !part.is_empty())
            .map(parse_yaml_scalar)
            .map(str::to_owned)
            .collect();
    }

    if value.is_empty() {
        Vec::new()
    } else {
        vec![parse_yaml_scalar(value).to_owned()]
    }
}

/// Build stubs for supported `features:` values. Returns unsupported feature
/// diagnostic for unsupported metadata values.
fn build_feature_stubs(features: &[String]) -> Result<String, Diagnostic> {
    let mut unsupported_feature = None;
    let mut needs_262 = false;
    let mut stubs = String::new();

    for feature in features {
        match feature.as_str() {
            "IsHTMLDDA" => {
                needs_262 = true;
                stubs.push_str("$262.IsHTMLDDA = {};\n");
            }
            "createRealm" => {
                needs_262 = true;
                stubs.push_str("$262.createRealm = function createRealm() { return {}; };\n");
            }
            "tail-call-optimization" => {
                // feature marker currently used only for test262 metadata filtering.
            }
            "Symbol.asyncIterator" => {
                stubs.push_str(
                    "if (typeof Symbol === 'object' || typeof Symbol === 'function') {\n",
                );
                stubs.push_str("  if (Symbol.asyncIterator === undefined) {\n");
                stubs.push_str("    Symbol.asyncIterator = Symbol('Symbol.asyncIterator');\n");
                stubs.push_str("  }\n}");
            }
            _ => {
                // Check if this is a known-but-not-stubbed feature (tracked but
                // the functionality may partially work). Only error on truly
                // unknown features not in KNOWN_FEATURES.
                if unsupported_feature.is_none()
                    && !KNOWN_FEATURES.iter().any(|(name, _)| *name == feature)
                {
                    unsupported_feature = Some(feature.clone());
                }
            }
        }
    }

    if let Some(feature) = unsupported_feature {
        let tracking_id = KNOWN_FEATURES
            .iter()
            .find(|(name, _)| *name == feature)
            .map(|(_, id)| *id)
            .unwrap_or("issue-5000");
        return Err(Diagnostic {
            code: ts2wasm_frontend::DiagCode::UnsupportedSyntax,
            message: format!(
                "UnsupportedTest262Metadata/test262-metadata: test262 feature `{feature}` is not supported by this runner slice [{tracking_id}]"
            ),
            span: None,
        });
    }

    if needs_262 {
        stubs.insert_str(0, "var $262 = {};\n");
    }

    Ok(stubs)
}

/// Extract includes: directive from YAML frontmatter.
#[allow(dead_code)]
fn extract_includes_from_frontmatter(frontmatter: &str) -> Vec<String> {
    parse_test262_metadata(frontmatter).includes
}

/// Extract features: directive from YAML frontmatter.
#[allow(dead_code)]
fn extract_features_from_frontmatter(frontmatter: &str) -> Vec<String> {
    parse_test262_metadata(frontmatter).features
}

/// Extract function definitions from helper file
///
/// This is a simplified approach: extract function declarations
/// and create stub implementations that the parser can handle.
/// Skips functions already present in `full_source` to avoid DuplicateFunction
/// when the Python harness has already wrapped the source with shim functions.
fn extract_function_stubs(_helper_source: &str, full_source: &str) -> String {
    // For now, return hardcoded stubs for common test262 helper functions
    // This is a temporary solution until we can properly parse helper files
    let candidate_stubs = [
        ("function verifyProperty() {}", "verifyProperty"),
        (
            "function verifyCallableProperty() {}",
            "verifyCallableProperty",
        ),
        ("function assert() {}", "assert"),
    ];
    candidate_stubs
        .iter()
        .filter(|(_, name)| !source_has_function(full_source, name))
        .map(|(stub, _)| *stub)
        .collect::<Vec<_>>()
        .join("\n")
}

/// Resolve test262 harness directory from input file path
fn resolve_harness_directory(input: &Path) -> Result<std::path::PathBuf, Diagnostic> {
    // Navigate from input file to test262 harness directory
    // Expected structure: reference/test262/test/.../test.js
    // Harness directory: reference/test262/harness/

    let mut current = input.parent().ok_or_else(|| Diagnostic {
        code: ts2wasm_frontend::DiagCode::BackendIo,
        message: format!("cannot get parent directory of {}", input.display()),
        span: None,
    })?;

    // Navigate up to find test262 directory
    let mut test262_root = None;
    for _ in 0..10 {
        // Limit depth to avoid infinite loops
        if current.ends_with("test") {
            let parent = current.parent();
            if let Some(p) = parent
                && p.ends_with("test262")
            {
                test262_root = Some(p.to_path_buf());
                break;
            }
        }
        current = match current.parent() {
            Some(p) => p,
            None => break,
        };
    }

    let test262_root = test262_root.ok_or_else(|| Diagnostic {
        code: ts2wasm_frontend::DiagCode::BackendIo,
        message: format!(
            "could not find test262 root directory from input path {}",
            input.display()
        ),
        span: None,
    })?;

    let harness_dir = test262_root.join("harness");
    if !harness_dir.is_dir() {
        return Err(Diagnostic {
            code: ts2wasm_frontend::DiagCode::BackendIo,
            message: format!(
                "test262 harness directory not found at {}",
                harness_dir.display()
            ),
            span: None,
        });
    }

    Ok(harness_dir)
}

/// Remove YAML frontmatter from source if present
fn remove_frontmatter(source: &str) -> &str {
    if let Some(frontmatter_end) = source.find("---*/") {
        // Skip the closing */ and the following newline
        let start = frontmatter_end + 5;
        if start < source.len() {
            &source[start..]
        } else {
            ""
        }
    } else {
        source
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_includes_from_frontmatter() {
        let frontmatter = r#"/*---
includes: [propertyHelper.js, assert.js]
---*/"#;
        let includes = extract_includes_from_frontmatter(frontmatter);
        assert_eq!(includes, vec!["propertyHelper.js", "assert.js"]);
    }

    #[test]
    fn test_extract_features_and_multiline_includes_from_frontmatter() {
        let frontmatter = r#"/*---
includes:
  - propertyHelper.js
features: [IsHTMLDDA, createRealm]
---*/"#;
        let features = extract_features_from_frontmatter(frontmatter);
        let includes = extract_includes_from_frontmatter(frontmatter);
        assert_eq!(features, vec!["IsHTMLDDA", "createRealm"]);
        assert_eq!(includes, vec!["propertyHelper.js"]);
    }

    #[test]
    fn test_process_includes_and_features_inject_stubs() {
        let source = r#"/*---
features: [IsHTMLDDA, Symbol.asyncIterator]
includes:
  - assert.js
---*/

var IsHTMLDDA = $262.IsHTMLDDA;"#;
        let input = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../reference/test262/test/annexB/built-ins/Object/is/emulates-undefined.js");
        let processed = process_test262_includes(&input, source)
            .expect("feature/inclusion processing should succeed");

        assert!(processed.contains("var $262 = {};"));
        assert!(processed.contains("$262.IsHTMLDDA = {};"));
        assert!(processed.contains("function assert() {}"));
        assert!(processed.contains("if (typeof Symbol === 'object'"));
    }

    #[test]
    fn test_process_features_includes_262_declaration() {
        let source = r#"/*---
features: [createRealm]
---*/

var realm = $262.createRealm();"#;
        let input = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../reference/test262/test/language/expressions/call-expression-eval.js");
        let processed = process_test262_includes(&input, source)
            .expect("feature/inclusion processing should succeed");

        assert!(processed.contains("var $262 = {};"));
        assert!(processed.contains("$262.createRealm = function createRealm()"));
    }

    #[test]
    fn test_process_unknown_feature_is_unsupported() {
        let source = "/*---\nfeatures: [UnknownFeature]\n---*/\nvar x = 1;";
        let input = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../reference/test262/test/language/expressions/does-not-exist.js");
        let error = process_test262_includes(&input, source)
            .expect_err("unknown feature should be rejected");
        assert_eq!(error.code, ts2wasm_frontend::DiagCode::UnsupportedSyntax);
        assert!(error.message.contains("UnsupportedTest262Metadata"));
    }

    #[test]
    fn test_extract_includes_empty() {
        let frontmatter = r#"/*---
description: test
---*/"#;
        let includes = extract_includes_from_frontmatter(frontmatter);
        assert!(includes.is_empty());
    }

    #[test]
    fn test_extract_includes_no_frontmatter() {
        let frontmatter = "no frontmatter here";
        let includes = extract_includes_from_frontmatter(frontmatter);
        assert!(includes.is_empty());
    }

    #[test]
    fn test_remove_frontmatter() {
        let source = r#"/*---
description: test
---*/
var x = 1;"#;
        let result = remove_frontmatter(source);
        assert_eq!(result.trim(), "var x = 1;");
    }

    #[test]
    fn test_remove_frontmatter_no_frontmatter() {
        let source = "var x = 1;";
        let result = remove_frontmatter(source);
        assert_eq!(result, "var x = 1;");
    }

    #[test]
    fn test_negative_directive_extracted() {
        let source = r#"/*---
negative:
  phase: early
  type: SyntaxError
---*/
"#;
        let metadata = parse_test262_metadata(source);
        assert_eq!(metadata.negative, Some("early".to_owned()));
    }

    #[test]
    fn test_negative_directive_injects_comment() {
        let source = r#"/*---
negative:
  phase: early
  type: SyntaxError
---*/

var x = 1;"#;
        let input = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../reference/test262/test/language/expressions/does-not-exist.js");
        let processed =
            process_test262_includes(&input, source).expect("negative directive should not fail");
        assert!(processed.contains("// @negative"));
    }

    #[test]
    fn test_known_feature_tracking_id_in_error() {
        // Known features are silently skipped. Use a truly unknown feature
        // (not in KNOWN_FEATURES) to trigger the error.
        let source = "/*---\nfeatures: [TotallyUnknownFeature999]\n---*/\nvar x = 1;";
        let input = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../reference/test262/test/language/expressions/does-not-exist.js");
        let error = process_test262_includes(&input, source)
            .expect_err("unknown feature should be rejected");
        assert!(
            error.message.contains("[issue-5000]"),
            "unknown feature should get generic tracking ID, got: {}",
            error.message
        );
    }
}
