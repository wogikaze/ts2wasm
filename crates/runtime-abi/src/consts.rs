pub struct RuntimeConst;

impl RuntimeConst {
    pub const ZERO: i32 = 0;
    pub const ONE: i32 = 1;
    pub const TEN: i32 = 10;
    pub const STDOUT_FD: i32 = 1;
    pub const ASCII_ZERO: i32 = 48;
    pub const ASCII_MINUS: i32 = 45;
}

pub struct RuntimeString;

impl RuntimeString {
    pub const UNDEFINED: &'static str = "undefined";
    pub const NULL: &'static str = "null";
    pub const FALSE: &'static str = "false";
    pub const TRUE: &'static str = "true";
    pub const NEWLINE: &'static str = "\n";
    pub const JSON_PARSE_SYNTAX_ERROR: &'static str = "SyntaxError: JSON.parse invalid JSON\n";
    pub const BIGINT_MIXED_ARITHMETIC_TYPE_ERROR: &'static str =
        "TypeError: Cannot mix BigInt and other types, use explicit conversions\n";
    pub const BIGINT_DIVISION_BY_ZERO_RANGE_ERROR: &'static str = "RangeError: Division by zero\n";
    pub const BIGINT_STRING_COMPARISON_BOUNDARY_ERROR: &'static str = "issue-375: BigInt/String comparison StringToBigInt value is outside the runtime comparison boundary\n";
    pub const PRIVATE_BRAND_TYPE_ERROR: &'static str =
        "TypeError: Cannot read private member from an object whose class did not declare it\n";
}
