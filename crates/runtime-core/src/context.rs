use crate::env::EnvRef;
use crate::realm::RealmRef;
use crate::value::TaggedValue;

pub type ContextRef = u32;
pub type FunctionRef = u32;

#[derive(Debug, Clone)]
pub enum ThisValue {
    Global,
    Strict(TaggedValue),
    Primitive(TaggedValue),
}

#[derive(Debug, Clone)]
pub enum ScriptOrModule {
    Script { url: String },
    Module { url: String },
}

#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub lexical_env: EnvRef,
    pub variable_env: EnvRef,
    pub private_env: Option<EnvRef>,
    pub realm: RealmRef,
    pub function: Option<FunctionRef>,
    pub script_or_module: Option<ScriptOrModule>,
}

impl ExecutionContext {
    pub fn new_global(realm: RealmRef, global_env: EnvRef) -> Self {
        Self {
            lexical_env: global_env,
            variable_env: global_env,
            private_env: None,
            realm,
            function: None,
            script_or_module: None,
        }
    }

    pub fn new_function(
        realm: RealmRef,
        lexical_env: EnvRef,
        variable_env: EnvRef,
        function: FunctionRef,
    ) -> Self {
        Self {
            lexical_env,
            variable_env,
            private_env: None,
            realm,
            function: Some(function),
            script_or_module: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExecutionContextStack {
    stack: Vec<ExecutionContext>,
}

impl ExecutionContextStack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, ctx: ExecutionContext) {
        self.stack.push(ctx);
    }

    pub fn pop(&mut self) -> Option<ExecutionContext> {
        self.stack.pop()
    }

    pub fn top(&self) -> Option<&ExecutionContext> {
        self.stack.last()
    }

    pub fn top_mut(&mut self) -> Option<&mut ExecutionContext> {
        self.stack.last_mut()
    }

    pub fn depth(&self) -> usize {
        self.stack.len()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

impl Default for ExecutionContextStack {
    fn default() -> Self {
        Self::new()
    }
}
