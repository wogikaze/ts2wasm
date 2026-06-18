// ---------------------------------------------------------------------------
// Environment Records — § 9.1
//
// Environment Records are the spec-level mechanism for tracking bindings
// (let, const, var, function, class, module imports).
// ---------------------------------------------------------------------------

use std::collections::HashMap;

use crate::property::PropertyKey;
use crate::value::Value;

// ---------------------------------------------------------------------------
// Environment ID — lightweight handle
// ---------------------------------------------------------------------------

/// A unique identifier for an environment record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EnvID(pub u32);

// ---------------------------------------------------------------------------
// Binding — a single name→value binding
// ---------------------------------------------------------------------------

/// A binding in an environment record — § 9.1.1.1.
#[derive(Debug, Clone)]
pub struct Binding {
    /// The name of the binding.
    pub name: String,
    /// The value (may be uninitialized for `let`/`const` before init).
    pub value: BindingValue,
    /// Whether this binding is mutable (`let`/`var`) or immutable (`const`).
    pub mutability: Mutability,
    /// Whether this binding can be deleted (only `var` in sloppy mode).
    pub deletable: bool,
    /// For `const`: has the binding been initialized?
    pub initialized: bool,
}

#[derive(Debug, Clone)]
pub enum BindingValue {
    /// Initialized with a value.
    Present(Value),
    /// Uninitialized — reading this binding before init throws ReferenceError.
    Uninitialized,
    /// TDZ (Temporal Dead Zone) marker.
    TDZ,
}

impl BindingValue {
    pub fn is_uninitialized(&self) -> bool {
        matches!(self, BindingValue::Uninitialized | BindingValue::TDZ)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mutability {
    /// `let` — can be reassigned.
    Mutable,
    /// `const` — cannot be reassigned.
    Immutable,
    /// `var` — can be reassigned and deleted.
    Var,
}

// ---------------------------------------------------------------------------
// EnvironmentRecord — § 9.1.1
// ---------------------------------------------------------------------------

/// An Environment Record — the spec's mechanism for tracking name bindings.
///
/// § 9.1.1 — Declarative Environment Records bind names introduced by
/// `let`, `const`, `class`, `function`, catch clauses, etc.
///
/// § 9.1.2 — Object Environment Records bind names introduced by `with`
/// statements.
///
/// § 9.1.3 — Function Environment Records are created when calling functions.
///
/// § 9.1.4 — Global Environment Records bind names at the top level.
#[derive(Debug, Clone)]
pub enum EnvironmentRecord {
    /// Declarative Environment Record — § 9.1.1.1
    Declarative(DeclarativeEnv),
    /// Object Environment Record — § 9.1.1.2
    Object(ObjectEnv),
    /// Function Environment Record — § 9.1.1.3
    Function(FunctionEnv),
    /// Global Environment Record — § 9.1.1.4
    Global(GlobalEnv),
    /// Module Environment Record — § 9.1.1.5
    Module(ModuleEnv),
}

impl EnvironmentRecord {
    /// Returns the outer environment, if any.
    pub fn outer(&self) -> Option<EnvID> {
        match self {
            EnvironmentRecord::Declarative(env) => env.outer,
            EnvironmentRecord::Object(env) => env.outer,
            EnvironmentRecord::Function(env) => env.outer,
            EnvironmentRecord::Global(env) => env.outer,
            EnvironmentRecord::Module(env) => env.declarative.outer,
        }
    }

    /// Returns `true` if this is a declarative environment record.
    pub fn is_declarative(&self) -> bool {
        matches!(self, EnvironmentRecord::Declarative(_))
    }

    /// Returns `true` if this is a global environment record.
    pub fn is_global(&self) -> bool {
        matches!(self, EnvironmentRecord::Global(_))
    }
}

// ---------------------------------------------------------------------------
// Declarative Environment Record — § 9.1.1.1
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct DeclarativeEnv {
    /// Bindings in this environment.
    pub bindings: HashMap<String, Binding>,
    /// The enclosing (outer) environment.
    pub outer: Option<EnvID>,
}

// ---------------------------------------------------------------------------
// Object Environment Record — § 9.1.1.2
// ---------------------------------------------------------------------------

/// An Object Environment Record — binds names from the properties of an object.
///
/// Used for `with` statements and global bindings.
#[derive(Debug, Clone)]
pub struct ObjectEnv {
    /// The binding object (the object whose properties are the bindings).
    pub binding_object: Value,
    /// Whether `with` statement binding is enabled (sloppy mode only).
    pub with_environment: bool,
    /// The enclosing (outer) environment.
    pub outer: Option<EnvID>,
}

// ---------------------------------------------------------------------------
// Function Environment Record — § 9.1.1.3
// ---------------------------------------------------------------------------

/// A Function Environment Record — created for each function call.
///
/// Extends DeclarativeEnv with `this` binding, `[[NewTarget]]`, and `[[HomeObject]]`.
#[derive(Debug, Clone)]
pub struct FunctionEnv {
    /// Bindings in this function's scope.
    pub bindings: HashMap<String, Binding>,
    /// The `this` binding value.
    pub this: Value,
    /// The `[[NewTarget]]` — non-empty when invoked via `new`.
    pub new_target: Option<Value>,
    /// The `[[HomeObject]]` — for super property access in methods.
    pub home_object: Option<Value>,
    /// The enclosing (outer) environment.
    pub outer: Option<EnvID>,
    /// Whether this function is in strict mode.
    pub strict: bool,
}

// ---------------------------------------------------------------------------
// Global Environment Record — § 9.1.1.4
// ---------------------------------------------------------------------------

/// A Global Environment Record — the root environment for scripts.
///
/// Combines a declarative record (for `let`, `const`, `class`, function
/// declarations) with an object record (for `var` and built-in globals).
#[derive(Debug, Clone)]
pub struct GlobalEnv {
    /// The declarative bindings (`let`, `const`, `class`, function decls).
    pub declarative: DeclarativeEnv,
    /// The object record for `var` bindings and built-in globals.
    pub object: ObjectEnv,
    /// Global `this` value.
    pub global_this: Value,
    /// The enclosing (outer) environment — always None for global.
    pub outer: Option<EnvID>,
}

// ---------------------------------------------------------------------------
// Module Environment Record — § 9.1.1.5
// ---------------------------------------------------------------------------

/// A Module Environment Record — extends DeclarativeEnv with import bindings.
#[derive(Debug, Clone)]
pub struct ModuleEnv {
    /// The underlying declarative bindings.
    pub declarative: DeclarativeEnv,
    /// Import bindings: local name → (module namespace, exported name).
    pub imports: HashMap<String, ImportBinding>,
}

/// An import binding — maps a local name to a remote export.
#[derive(Debug, Clone)]
pub struct ImportBinding {
    /// The module namespace object (the imported module's `ModuleNamespace`).
    pub module_namespace: Value,
    /// The exported name in the remote module.
    pub exported_name: PropertyKey,
}

// ---------------------------------------------------------------------------
// HasBinding, GetBindingValue, SetMutableBinding, etc.
// ---------------------------------------------------------------------------

impl DeclarativeEnv {
    /// § 9.1.1.1.1 — HasBinding(N).
    pub fn has_binding(&self, name: &str) -> bool {
        self.bindings.contains_key(name)
    }

    /// § 9.1.1.1.4 — GetBindingValue(N, S).
    pub fn get_binding_value(&self, name: &str, strict: bool) -> Result<Value, String> {
        let binding = self
            .bindings
            .get(name)
            .ok_or_else(|| format!("Uncaught ReferenceError: {name} is not defined"))?;
        if binding.value.is_uninitialized() {
            if strict {
                return Err(format!("ReferenceError: Cannot access '{name}' before initialization"));
            }
            return Err(format!("Uncaught ReferenceError: {name} is not defined"));
        }
        match &binding.value {
            BindingValue::Present(v) => Ok(v.clone()),
            _ => Err(format!("ReferenceError: {name} is not defined")),
        }
    }

    /// § 9.1.1.1.3 — SetMutableBinding(N, V, S).
    pub fn set_mutable_binding(
        &mut self,
        name: &str,
        value: Value,
        strict: bool,
    ) -> Result<(), String> {
        let binding = self
            .bindings
            .get_mut(name)
            .ok_or_else(|| format!("Uncaught ReferenceError: {name} is not defined"))?;
        match binding.mutability {
            Mutability::Immutable => {
                if strict {
                    return Err(format!("TypeError: Assignment to constant variable '{name}'"));
                }
                // In sloppy mode, silent failure for const reassignment.
                Ok(())
            }
            _ => {
                binding.value = BindingValue::Present(value);
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_binding(name: &str, value: Value, mutability: Mutability) -> Binding {
        Binding {
            name: name.to_owned(),
            value: BindingValue::Present(value),
            mutability,
            deletable: false,
            initialized: true,
        }
    }

    #[test]
    fn has_binding_returns_true_for_existing() {
        let env = DeclarativeEnv {
            bindings: HashMap::from([(
                "x".to_owned(),
                make_binding("x", Value::Number(42.0), Mutability::Mutable),
            )]),
            outer: None,
        };
        assert!(env.has_binding("x"));
        assert!(!env.has_binding("y"));
    }

    #[test]
    fn get_binding_value_returns_value() {
        let env = DeclarativeEnv {
            bindings: HashMap::from([(
                "x".to_owned(),
                make_binding("x", Value::Number(42.0), Mutability::Mutable),
            )]),
            outer: None,
        };
        assert!(matches!(
            env.get_binding_value("x", false),
            Ok(Value::Number(n)) if n == 42.0
        ));
    }

    #[test]
    fn get_binding_value_errors_for_missing() {
        let env = DeclarativeEnv {
            bindings: HashMap::new(),
            outer: None,
        };
        assert!(env.get_binding_value("x", false).is_err());
    }

    #[test]
    fn set_mutable_binding_updates_value() {
        let mut env = DeclarativeEnv {
            bindings: HashMap::from([(
                "x".to_owned(),
                make_binding("x", Value::Number(1.0), Mutability::Mutable),
            )]),
            outer: None,
        };
        env.set_mutable_binding("x", Value::Number(2.0), false).unwrap();
        assert!(matches!(
            env.get_binding_value("x", false),
            Ok(Value::Number(n)) if n == 2.0
        ));
    }

    #[test]
    fn set_immutable_binding_errors() {
        let mut env = DeclarativeEnv {
            bindings: HashMap::from([(
                "x".to_owned(),
                make_binding("x", Value::Number(1.0), Mutability::Immutable),
            )]),
            outer: None,
        };
        let result = env.set_mutable_binding("x", Value::Number(2.0), true);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("constant"));
    }
}
