use crate::builtin_resolved::ResolvedExpr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedArrayElement {
    Present(ResolvedExpr),
    Hole,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedObjectProp {
    KeyValue {
        key: String,
        value: ResolvedExpr,
    },
    Shorthand {
        key: String,
        value: ResolvedExpr,
    },
    ComputedKey {
        key: Box<ResolvedExpr>,
        value: ResolvedExpr,
    },
    MethodShorthand {
        key: String,
        value: ResolvedExpr,
    },
}

impl ResolvedObjectProp {
    pub fn static_key(&self) -> Option<&str> {
        match self {
            Self::KeyValue { key, .. }
            | Self::Shorthand { key, .. }
            | Self::MethodShorthand { key, .. } => Some(key),
            Self::ComputedKey { .. } => None,
        }
    }

    pub fn value(&self) -> &ResolvedExpr {
        match self {
            Self::KeyValue { value, .. }
            | Self::Shorthand { value, .. }
            | Self::ComputedKey { value, .. }
            | Self::MethodShorthand { value, .. } => value,
        }
    }

    pub fn computed_key(&self) -> Option<&ResolvedExpr> {
        match self {
            Self::ComputedKey { key, .. } => Some(key),
            _ => None,
        }
    }
}
