#![allow(dead_code)]

pub(super) const CLOSURE_SENTINEL: i32 = -2;
pub(super) const CLOSURE_CAPTURE_COUNT_OFFSET: u32 = 8;
pub(super) const CLOSURE_CAPTURE_SLOTS_OFFSET: u32 = 16;
pub(super) const CLOSURE_CAPTURE_SLOT_SIZE: u32 = 4;
pub(super) const CLASS_INSTANCE_PUBLIC_SLOT_CAPACITY: u32 = 16;
pub(super) const PRIVATE_FIELD_SLOT_SIZE: u32 = 4;
pub(super) const PRIVATE_FIELD_COUNT_MASK: u32 = 0xffff;
pub(super) const BIGINT_FROM_STRING_ABORT_MESSAGE: &str =
    "issue-333: BigInt(string) runtime invalid or out-of-range input\n";
