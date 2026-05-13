use crate::lowered::*;
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;

/// Returns true if `method` is an HTML wrapper (Annex B String.prototype method
/// that can be lowered to Concat calls at the IR level).
pub(super) fn is_html_wrapper_string_method(method: &str) -> bool {
    matches!(
        method,
        "anchor"
            | "big"
            | "blink"
            | "bold"
            | "fixed"
            | "fontcolor"
            | "fontsize"
            | "italics"
            | "link"
            | "small"
            | "strike"
            | "sub"
            | "sup"
    )
}

/// Lower an HTML wrapper String.prototype method to nested Concat runtime calls.
pub(super) fn lower_html_wrapper_string_method(
    method: &str,
    object: LoweredExpr,
    args: Vec<LoweredExpr>,
    span: Span,
) -> Result<LoweredExpr, Diagnostic> {
    let (open_prefix, attr_suffix, close_tag) = match method {
        "anchor" => ("<a name=\"", Some("\">"), "</a>"),
        "big" => ("<big>", None, "</big>"),
        "blink" => ("<blink>", None, "</blink>"),
        "bold" => ("<b>", None, "</b>"),
        "fixed" => ("<tt>", None, "</tt>"),
        "fontcolor" => ("<font color=\"", Some("\">"), "</font>"),
        "fontsize" => ("<font size=\"", Some("\">"), "</font>"),
        "italics" => ("<i>", None, "</i>"),
        "link" => ("<a href=\"", Some("\">"), "</a>"),
        "small" => ("<small>", None, "</small>"),
        "strike" => ("<strike>", None, "</strike>"),
        "sub" => ("<sub>", None, "</sub>"),
        "sup" => ("<sup>", None, "</sup>"),
        _ => {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("String.prototype.{method} is not supported in this milestone"),
                span: Some(span),

                phase: None,
            });
        }
    };

    fn string_lit(value: &str) -> LoweredExpr {
        LoweredExpr::String(value.to_owned(), Span::generated("str"))
    }

    fn concat(left: LoweredExpr, right: LoweredExpr) -> LoweredExpr {
        LoweredExpr::RuntimeCall {
            intrinsic: RuntimeFn::Concat,
            args: vec![left, right],
            span: Span::generated("runtime_call"),
        }
    }

    let mut result = string_lit(open_prefix);

    if let Some(attr_suffix) = attr_suffix {
        let needs_escaping = matches!(method, "anchor" | "fontcolor" | "fontsize" | "link");
        let mut attr = args
            .into_iter()
            .next()
            .unwrap_or_else(|| string_lit("undefined"));
        if needs_escaping {
            attr = LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::StringReplaceAll,
                args: vec![attr, string_lit("\""), string_lit("&quot;")],
                span: Span::generated("runtime_call"),
            };
        }
        result = concat(result, attr);
        result = concat(result, string_lit(attr_suffix));
    }

    result = concat(result, object);
    Ok(concat(result, string_lit(close_tag)))
}
