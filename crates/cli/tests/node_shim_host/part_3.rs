use super::*;

#[test]
fn static_direct_eval_rejects_return_statement() {
    let fixture = "fixtures/core-semantics/direct-eval-return-unsupported.ts";
    assert_build_fails_with(
        fixture,
        "UnsupportedSyntax",
        "return statement is not valid in eval source",
    );
}
