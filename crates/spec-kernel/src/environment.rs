use ts2wasm_runtime_core::env::{Binding, EnvRef, EnvironmentRecord};
use ts2wasm_runtime_core::value::TaggedValue;

pub struct EnvironmentOps;

impl EnvironmentOps {
    pub fn get_binding_value(env: &EnvironmentRecord, name: &str) -> Option<TaggedValue> {
        let index = env.find_binding(name)?;
        env.get_binding_value(index)
    }

    pub fn set_mutable_binding(
        env: &mut EnvironmentRecord,
        name: &str,
        value: TaggedValue,
    ) -> bool {
        let index = match env.find_binding(name) {
            Some(i) => i,
            None => return false,
        };
        match env {
            EnvironmentRecord::Declarative { bindings, .. }
            | EnvironmentRecord::Function { bindings, .. }
            | EnvironmentRecord::Module { bindings, .. } => {
                if let Some(b) = bindings.get_mut(index) {
                    if b.mutable || !b.initialized {
                        b.value = value;
                        b.initialized = true;
                        return true;
                    }
                }
                false
            }
            EnvironmentRecord::Global {
                declarative_bindings,
                object_bindings,
                ..
            } => {
                let n = declarative_bindings.len();
                if index < n {
                    if let Some(b) = declarative_bindings.get_mut(index) {
                        b.value = value;
                        b.initialized = true;
                        return true;
                    }
                } else if let Some(b) = object_bindings.get_mut(index - n) {
                    b.value = value;
                    return true;
                }
                false
            }
            _ => false,
        }
    }

    pub fn create_binding(env: &mut EnvironmentRecord, name: &str, mutable: bool) -> bool {
        match env {
            EnvironmentRecord::Declarative { bindings, .. }
            | EnvironmentRecord::Function { bindings, .. }
            | EnvironmentRecord::Module { bindings, .. } => {
                bindings.push(Binding::new(name.to_string(), mutable));
                true
            }
            EnvironmentRecord::Global {
                declarative_bindings,
                ..
            } => {
                declarative_bindings.push(Binding::new(name.to_string(), mutable));
                true
            }
            _ => false,
        }
    }

    pub fn initialize_binding(env: &mut EnvironmentRecord, name: &str, value: TaggedValue) -> bool {
        match env {
            EnvironmentRecord::Declarative { bindings, .. }
            | EnvironmentRecord::Function { bindings, .. }
            | EnvironmentRecord::Module { bindings, .. } => {
                if let Some(b) = bindings.iter_mut().find(|b| b.name == name) {
                    b.value = value;
                    b.initialized = true;
                    return true;
                }
                false
            }
            EnvironmentRecord::Global {
                declarative_bindings,
                ..
            } => {
                if let Some(b) = declarative_bindings.iter_mut().find(|b| b.name == name) {
                    b.value = value;
                    b.initialized = true;
                    return true;
                }
                false
            }
            _ => false,
        }
    }

    pub fn has_binding(env: &EnvironmentRecord, name: &str) -> bool {
        env.has_binding(name)
    }
}
