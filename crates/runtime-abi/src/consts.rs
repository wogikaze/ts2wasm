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
}
