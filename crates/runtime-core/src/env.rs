use crate::value::TaggedValue;

pub type EnvRef = u32;

#[derive(Debug, Clone)]
pub struct Binding {
    pub name: String,
    pub value: TaggedValue,
    pub mutable: bool,
    pub initialized: bool,
    pub is_function: bool,
    pub is_var: bool,
    pub is_import: bool,
}

impl Binding {
    pub fn new(name: String, mutable: bool) -> Self {
        Self {
            name,
            value: TaggedValue::UNDEFINED,
            mutable,
            initialized: false,
            is_function: false,
            is_var: false,
            is_import: false,
        }
    }

    pub fn initialized_value(&self) -> Option<TaggedValue> {
        if self.initialized {
            Some(self.value)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImportBinding {
    pub local_name: String,
    pub module: ModuleIdent,
    pub import_name: String,
}

#[derive(Debug, Clone)]
pub struct PrivateName {
    pub description: String,
    pub brand: TaggedValue,
}

#[derive(Debug, Clone)]
pub struct ModuleIdent(pub String);

#[derive(Debug, Clone)]
pub enum EnvironmentRecord {
    Declarative {
        bindings: Vec<Binding>,
        outer: Option<EnvRef>,
    },
    Function {
        bindings: Vec<Binding>,
        outer: Option<EnvRef>,
        this_value: TaggedValue,
        new_target: TaggedValue,
    },
    Global {
        declarative_bindings: Vec<Binding>,
        object_bindings: Vec<Binding>,
        global_this: TaggedValue,
    },
    Object {
        binding_object: TaggedValue,
        provide_this: bool,
        outer: Option<EnvRef>,
    },
    Module {
        bindings: Vec<Binding>,
        import_bindings: Vec<ImportBinding>,
        outer: Option<EnvRef>,
    },
    Private {
        names: Vec<PrivateName>,
        outer: Option<EnvRef>,
    },
}

impl EnvironmentRecord {
    pub fn outer(&self) -> Option<EnvRef> {
        match self {
            Self::Declarative { outer, .. } => *outer,
            Self::Function { outer, .. } => *outer,
            Self::Global { .. } => None,
            Self::Object { outer, .. } => *outer,
            Self::Module { outer, .. } => *outer,
            Self::Private { outer, .. } => *outer,
        }
    }

    pub fn find_binding(&self, name: &str) -> Option<usize> {
        match self {
            Self::Declarative { bindings, .. }
            | Self::Function { bindings, .. }
            | Self::Module { bindings, .. } => bindings.iter().position(|b| b.name == name),
            Self::Global {
                declarative_bindings,
                object_bindings,
                ..
            } => declarative_bindings
                .iter()
                .position(|b| b.name == name)
                .or_else(|| object_bindings.iter().position(|b| b.name == name)),
            Self::Object { .. } | Self::Private { .. } => None,
        }
    }

    pub fn has_binding(&self, name: &str) -> bool {
        self.find_binding(name).is_some()
    }

    pub fn get_binding_value(&self, index: usize) -> Option<TaggedValue> {
        match self {
            Self::Declarative { bindings, .. }
            | Self::Function { bindings, .. }
            | Self::Module { bindings, .. } => {
                bindings.get(index).and_then(|b| b.initialized_value())
            }
            Self::Global {
                declarative_bindings,
                object_bindings,
                ..
            } => {
                let n = declarative_bindings.len();
                if index < n {
                    declarative_bindings
                        .get(index)
                        .and_then(|b| b.initialized_value())
                } else {
                    object_bindings.get(index - n).map(|b| b.value)
                }
            }
            Self::Object { .. } | Self::Private { .. } => None,
        }
    }
}
