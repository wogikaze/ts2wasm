use crate::env::EnvRef;
use crate::value::TaggedValue;

pub type FunctionId = u32;

#[derive(Debug, Clone)]
pub enum FunctionKind {
    Script,
    Arrow,
    Method,
    ClassConstructor,
    Getter,
    Setter,
    Async { inner: Box<FunctionKind> },
    Generator { inner: Box<FunctionKind> },
    AsyncGenerator { inner: Box<FunctionKind> },
    Builtin,
    Bound,
}

#[derive(Debug, Clone)]
pub struct FunctionObject {
    pub id: FunctionId,
    pub kind: FunctionKind,
    pub name: String,
    pub length: u32,
    pub scope: EnvRef,
    pub prototype: TaggedValue,
    pub home_object: Option<TaggedValue>,
    pub is_strict: bool,
    pub has_rest_param: bool,
    pub param_count: u32,
    pub body_start: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct Closure {
    pub function: FunctionId,
    pub captures: Vec<(String, TaggedValue)>,
    pub env: EnvRef,
}

impl Closure {
    pub fn new(function: FunctionId, env: EnvRef) -> Self {
        Self { function, captures: Vec::new(), env }
    }

    pub fn capture(&mut self, name: String, value: TaggedValue) {
        if let Some(pos) = self.captures.iter().position(|(n, _)| *n == name) {
            self.captures[pos].1 = value;
        } else {
            self.captures.push((name, value));
        }
    }
}
