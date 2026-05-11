//! Class environment — class hierarchy context for the lowering pass.
//!
//! This module groups the Resolver's class-related fields into a single
//! `ClassEnv` struct. It tracks class hierarchy, constructor/method IDs,
//! private fields, and the current class context during lowering.
//!
//! Current scope:
//! - `Classes` struct from resolver/mod.rs
//! - All class-related tracking (constructors, methods, parents, private fields)

use std::collections::HashMap;

use crate::lowered::types::{
    ClassConstructorMap, ClassMethodMap, ClassPrivateFieldSlots, ClassStaticPrivateFields,
};
use crate::lowered::{FuncId, LocalId};

/// Class hierarchy context for the lowering pass.
///
/// Tracks all class-related state during lowering: constructor and method
/// IDs, inheritance relationships, private field slots, and the current
/// class being lowered (for `super` and `new.target` resolution).
pub struct ClassEnv {
    /// Map from class name to constructor FuncId.
    pub class_constructor_ids: ClassConstructorMap,
    /// Map from (class_name, method_name) to instance method FuncId.
    pub class_method_ids: ClassMethodMap,
    /// Map from (class_name, method_name) to static method FuncId.
    pub class_static_method_ids: ClassMethodMap,
    /// Map from class name to optional parent class name.
    pub class_parents: HashMap<String, Option<String>>,
    /// Private field slot assignments: class_name → { field_name → slot_index }.
    pub class_private_fields: ClassPrivateFieldSlots,
    /// Static private field initializers: class_name → { field_name → local_name }.
    pub class_static_private_fields: ClassStaticPrivateFields,
    /// Map from local ID to the class name it was inferred to be (e.g., for `any`-typed vars initialized with class instances).
    pub local_classes: HashMap<LocalId, String>,
    /// Map from local ID to function-valued properties on object literals.
    pub object_function_props: HashMap<LocalId, HashMap<String, FuncId>>,
    /// The name of the class currently being lowered (for super/new.target/method resolution).
    pub current_class: Option<String>,
    /// Whether the current position is inside a class constructor.
    pub in_constructor: bool,
}

impl ClassEnv {
    /// Create a new empty ClassEnv.
    pub fn new() -> Self {
        Self {
            class_constructor_ids: HashMap::new(),
            class_method_ids: HashMap::new(),
            class_static_method_ids: HashMap::new(),
            class_parents: HashMap::new(),
            class_private_fields: HashMap::new(),
            class_static_private_fields: HashMap::new(),
            local_classes: HashMap::new(),
            object_function_props: HashMap::new(),
            current_class: None,
            in_constructor: false,
        }
    }

    /// Create a ClassEnv with pre-populated constructor and method maps.
    pub fn with_class_maps(
        class_constructor_ids: ClassConstructorMap,
        class_method_ids: ClassMethodMap,
        class_static_method_ids: ClassMethodMap,
        class_parents: HashMap<String, Option<String>>,
        class_private_fields: ClassPrivateFieldSlots,
        class_static_private_fields: ClassStaticPrivateFields,
    ) -> Self {
        Self {
            class_constructor_ids,
            class_method_ids,
            class_static_method_ids,
            class_parents,
            class_private_fields,
            class_static_private_fields,
            local_classes: HashMap::new(),
            object_function_props: HashMap::new(),
            current_class: None,
            in_constructor: false,
        }
    }

    /// Get the constructor FuncId for a class name.
    pub fn constructor(&self, class_name: &str) -> Option<FuncId> {
        self.class_constructor_ids.get(class_name).copied()
    }

    /// Get the instance method FuncId for a class+method name.
    pub fn method(&self, class_name: &str, method_name: &str) -> Option<FuncId> {
        self.class_method_ids
            .get(&(class_name.to_owned(), method_name.to_owned()))
            .copied()
    }

    /// Get the static method FuncId for a class+method name.
    pub fn static_method(&self, class_name: &str, method_name: &str) -> Option<FuncId> {
        self.class_static_method_ids
            .get(&(class_name.to_owned(), method_name.to_owned()))
            .copied()
    }

    /// Check if a class extends another class.
    pub fn parent(&self, class_name: &str) -> Option<Option<&String>> {
        self.class_parents.get(class_name).map(|p| p.as_ref())
    }

    /// Get the inferred class name for a local.
    pub fn local_class(&self, local: LocalId) -> Option<&String> {
        self.local_classes.get(&local)
    }

    /// Set the current class context. Returns the previous class name.
    pub fn enter_class(&mut self, class_name: &str) -> Option<String> {
        self.current_class.replace(class_name.to_owned())
    }

    /// Restore the previous class context.
    pub fn exit_class(&mut self, previous: Option<String>) {
        self.current_class = previous;
    }

    /// Get the private field slot index for a class+field.
    pub fn private_field_slot(&self, class_name: &str, field: &str) -> Option<usize> {
        self.class_private_fields
            .get(class_name)
            .and_then(|fields| fields.get(field))
            .copied()
    }
}

impl Default for ClassEnv {
    fn default() -> Self {
        Self::new()
    }
}
