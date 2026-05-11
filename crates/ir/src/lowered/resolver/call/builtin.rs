use crate::lowered::*;
use ts2wasm_shared::{DiagCode, Diagnostic};
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
    let (open_prefix, open_suffix, close_tag) = match method {
        "anchor" => ("<a name=\"", "\"", "</a>"),
        "big" => ("<big>", "", "</big>"),
        "blink" => ("<blink>", "", "</blink>"),
        "bold" => ("<b>", "", "</b>"),
        "fixed" => ("<tt>", "", "</tt>"),
        "fontcolor" => ("<font color=\"", "\"", "</font>"),
        "fontsize" => ("<font size=\"", "\"", "</font>"),
        "italics" => ("<i>", "", "</i>"),
        "link" => ("<a href=\"", "\"", "</a>"),
        "small" => ("<small>", "", "</small>"),
        "strike" => ("<strike>", "", "</strike>"),
        "sub" => ("<sub>", "", "</sub>"),
        "sup" => ("<sup>", "", "</sup>"),
        _ => {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("String.prototype.{method} is not supported in this milestone"),
                span: Some(span),

                phase: None,
            });
        }
    };

    let mut result = LoweredExpr::RuntimeCall {
        intrinsic: RuntimeFn::Concat,
        args: vec![
            object,
            LoweredExpr::String(close_tag.to_owned(), Span::generated("str")),
        ],

        span: Span::generated("runtime_call"),
    };

    let has_arg = !open_suffix.is_empty();
    if has_arg {
        let needs_escaping = matches!(method, "anchor" | "fontcolor" | "fontsize" | "link");
        let mut arg = args.into_iter().next().unwrap_or(LoweredExpr::String(
            "undefined".to_owned(),
            Span::generated("str"),
        ));
        // Spec requires escaping " as &quot; in attribute values (B.2.3.10, B.2.3.6, etc.)
        if needs_escaping {
            arg = LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::StringReplaceAll,
                args: vec![
                    arg,
                    LoweredExpr::String("\"".to_owned(), Span::generated("str")),
                    LoweredExpr::String("&quot;".to_owned(), Span::generated("str")),
                ],

                span: Span::generated("runtime_call"),
            };
        }
        result = LoweredExpr::RuntimeCall {
            intrinsic: RuntimeFn::Concat,
            args: vec![
                arg,
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::Concat,
                    args: vec![
                        LoweredExpr::String(open_suffix.to_owned(), Span::generated("str")),
                        LoweredExpr::RuntimeCall {
                            intrinsic: RuntimeFn::Concat,
                            args: vec![
                                LoweredExpr::String(">".to_owned(), Span::generated("str")),
                                result,
                            ],

                            span: Span::generated("runtime_call"),
                        },
                    ],
                    span: Span::generated("RuntimeCall"),
                },
            ],
            span: Span::generated("RuntimeCall"),
        };
    }

    Ok(LoweredExpr::RuntimeCall {
        intrinsic: RuntimeFn::Concat,
        args: vec![
            LoweredExpr::String(open_prefix.to_owned(), Span::generated("str")),
            result,
        ],
        span: Span::generated("runtime_call"),
    })
}
