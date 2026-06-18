// ---------------------------------------------------------------------------
// PropertyKey — the type of property keys on objects
// ---------------------------------------------------------------------------

use crate::value::{SymbolID, Value};

/// A valid property key — either a string or symbol.
///
/// This corresponds to the ECMAScript `property-key` type (§ 7.3.21 IsPropertyKey).
/// Property keys are the domain of object property names.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PropertyKey {
    /// String property key (the common case).
    String(String),
    /// Symbol property key (unique per Symbol).
    Symbol(SymbolID),
}

impl PropertyKey {
    /// Create a property key from a string.
    pub fn string(s: impl Into<String>) -> Self {
        PropertyKey::String(s.into())
    }

    /// Create a property key from a symbol.
    pub fn symbol(id: SymbolID) -> Self {
        PropertyKey::Symbol(id)
    }

    /// Returns `true` if this is a string key.
    pub fn is_string(&self) -> bool {
        matches!(self, PropertyKey::String(_))
    }

    /// Returns `true` if this is a symbol key.
    pub fn is_symbol(&self) -> bool {
        matches!(self, PropertyKey::Symbol(_))
    }

    /// Returns the string key, or `None` if this is a symbol.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            PropertyKey::String(s) => Some(s),
            PropertyKey::Symbol(_) => None,
        }
    }
}

impl From<&str> for PropertyKey {
    fn from(s: &str) -> Self {
        PropertyKey::String(s.to_owned())
    }
}

impl From<String> for PropertyKey {
    fn from(s: String) -> Self {
        PropertyKey::String(s)
    }
}

impl From<SymbolID> for PropertyKey {
    fn from(s: SymbolID) -> Self {
        PropertyKey::Symbol(s)
    }
}

impl std::fmt::Display for PropertyKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PropertyKey::String(s) => write!(f, "{s}"),
            PropertyKey::Symbol(_) => write!(f, "[Symbol]"),
        }
    }
}

/// Convert a Value to a PropertyKey.
///
/// § 7.1.21 — ToPropertyKey. Returns the PropertyKey if the value is a
/// String or Symbol. For other types, falls through to ToPrimitive (which
/// is defined elsewhere).
pub fn value_to_property_key(value: &Value) -> Option<PropertyKey> {
    match value {
        Value::String(s) => Some(PropertyKey::String(s.clone())),
        Value::Symbol(s) => Some(PropertyKey::Symbol(*s)),
        _ => None,
    }
}
