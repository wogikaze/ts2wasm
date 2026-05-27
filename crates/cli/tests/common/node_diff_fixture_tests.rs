use super::*;

pub(super) const BIGINT_ISSUE_370: &str = "issue-370:";
pub(super) const BIGINT_ISSUE_373_TOPRIMITIVE_STRING_BOUNDARY: &str = "issue-373: direct object ToPrimitive toString string returns that are invalid or outside the signed-i32 StringToBigInt comparison boundary require source-backed diagnostics in this slice";

#[path = "node_diff_fixture_tests/part_1.rs"]
mod part_1;
#[path = "node_diff_fixture_tests/part_2.rs"]
mod part_2;
#[path = "node_diff_fixture_tests/part_3.rs"]
mod part_3;
#[path = "node_diff_fixture_tests/part_4.rs"]
mod part_4;
#[path = "node_diff_fixture_tests/part_5.rs"]
mod part_5;
#[path = "node_diff_fixture_tests/part_6.rs"]
mod part_6;
