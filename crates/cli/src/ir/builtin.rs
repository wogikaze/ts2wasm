#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum BuiltinId {
    ConsoleLog,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum BuiltinPropertyId {
    Length,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BuiltinResult {
    Value,
    EffectOnly,
}

impl BuiltinId {
    pub(crate) const fn expected_arity(self) -> usize {
        match self {
            Self::ConsoleLog => 1,
        }
    }

    pub(crate) const fn result(self) -> BuiltinResult {
        match self {
            Self::ConsoleLog => BuiltinResult::EffectOnly,
        }
    }
}
