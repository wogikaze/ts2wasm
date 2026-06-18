use crate::value::{EnvRef, ValueRef};

#[derive(Debug, Clone)]
pub enum RefBase {
    Value(ValueRef),
    Env(EnvRef),
    Unresolvable,
    Super(ValueRef),
}

#[derive(Debug, Clone)]
pub enum RefName {
    Id(String),
    PrivateName(String),
    Key(ValueRef),
}

#[derive(Debug, Clone)]
pub struct SemReference {
    pub base: RefBase,
    pub name: RefName,
    pub strict: bool,
    pub this_value: Option<ValueRef>,
}

impl SemReference {
    pub fn property(base: ValueRef, name: String, strict: bool) -> Self {
        Self {
            base: RefBase::Value(base),
            name: RefName::Id(name),
            strict,
            this_value: None,
        }
    }

    pub fn binding(env: EnvRef, name: String, strict: bool) -> Self {
        Self {
            base: RefBase::Env(env),
            name: RefName::Id(name),
            strict,
            this_value: None,
        }
    }

    pub fn unresolvable(strict: bool) -> Self {
        Self {
            base: RefBase::Unresolvable,
            name: RefName::Id(String::new()),
            strict,
            this_value: None,
        }
    }

    pub fn is_unresolvable(&self) -> bool {
        matches!(self.base, RefBase::Unresolvable)
    }

    pub fn is_property_reference(&self) -> bool {
        matches!(self.base, RefBase::Value(_))
    }

    pub fn is_environment_reference(&self) -> bool {
        matches!(self.base, RefBase::Env(_))
    }
}
