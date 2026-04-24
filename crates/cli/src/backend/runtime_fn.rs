#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum RuntimeFn {
    Log,
    TruthyBool,
    Not,
    Add,
    Sub,
    Less,
    StrictEqual,
}

impl RuntimeFn {
    pub(crate) const fn symbol(self) -> &'static str {
        match self {
            Self::Log => "$log",
            Self::TruthyBool => "$truthy_bool",
            Self::Not => "$not",
            Self::Add => "$add",
            Self::Sub => "$sub",
            Self::Less => "$less",
            Self::StrictEqual => "$strict_equal",
        }
    }
}
