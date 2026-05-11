use std::collections::{HashMap, HashSet};

use super::program_builtins::looks_like_regexp_literal;
use crate::builtin_resolved::ResolvedExpr;
use crate::lowered::*;
use ts2wasm_shared::{DiagCode, Diagnostic, Span};

impl<'a> super::Resolver<'a> {
    pub(super) fn append_class_method_captures(
        &self,
        method_id: FuncId,
        lowered_args: &mut Vec<LoweredExpr>,
    ) -> Result<(), Diagnostic> {
        let Some(captures) = self.functions.class_method_captures.get(&method_id) else {
            return Ok(());
        };
        let mutable_captures = self
            .functions
            .class_method_mutable_captures
            .get(&method_id)
            .map(Vec::as_slice)
            .unwrap_or(&[]);

        for capture in captures {
            let local = self.resolve_local(capture).map_err(|_| Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-289: class method capture `{capture}` is not available at this call site; escaped class lexical environments require heap environment support"
                ),
                span: None,

                phase: None,})?;
            if mutable_captures.contains(capture) && !self.captures.env_cell_locals.contains(&local)
            {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "issue-301: mutable class method capture `{capture}` is not available as an environment cell at this call site"
                    ),
                    span: None,

                    phase: None,
                });
            }
            lowered_args.push(LoweredExpr::Local(local, Span::generated("local")));
        }

        Ok(())
    }

    pub(super) fn resolve_class_method(&self, class_name: &str, method: &str) -> Option<FuncId> {
        let mut current = Some(class_name.to_owned());
        while let Some(class) = current {
            if let Some(id) = self
                .classes
                .class_method_ids
                .get(&(class.clone(), method.to_owned()))
                .copied()
            {
                return Some(id);
            }
            current = self
                .classes
                .class_parents
                .get(&class)
                .and_then(|p| p.clone());
        }
        None
    }

    pub(super) fn resolve_static_class_method(
        &self,
        class_name: &str,
        method: &str,
    ) -> Option<FuncId> {
        let mut current = Some(class_name.to_owned());
        while let Some(class) = current {
            if let Some(id) = self
                .classes
                .class_static_method_ids
                .get(&(class.clone(), method.to_owned()))
                .copied()
            {
                return Some(id);
            }
            current = self
                .classes
                .class_parents
                .get(&class)
                .and_then(|p| p.clone());
        }
        None
    }

    pub(super) fn current_private_method_id(&self, method: &str) -> Option<FuncId> {
        let class_name = self.classes.current_class.as_ref()?;
        self.classes
            .class_method_ids
            .get(&(class_name.clone(), method.to_owned()))
            .copied()
    }

    pub(super) fn current_static_private_method_id(&self, method: &str) -> Option<FuncId> {
        let class_name = self.classes.current_class.as_ref()?;
        self.classes
            .class_static_method_ids
            .get(&(class_name.clone(), method.to_owned()))
            .copied()
    }

    pub(super) fn current_static_private_field_local_name(&self, key: &str) -> Option<String> {
        let class_name = self.classes.current_class.as_ref()?;
        let field_name = key.strip_prefix('#')?;
        self.classes
            .class_static_private_fields
            .get(class_name)
            .and_then(|fields| fields.get(field_name))
            .cloned()
    }

    pub(super) fn current_static_private_getter_id(&self, key: &str) -> Option<FuncId> {
        let class_name = self.classes.current_class.as_ref()?;
        let getter_name = key.strip_prefix('#')?;
        self.classes
            .class_static_method_ids
            .get(&(class_name.clone(), format!("#get::{getter_name}")))
            .copied()
    }

    pub(super) fn current_static_private_setter_id(&self, key: &str) -> Option<FuncId> {
        let class_name = self.classes.current_class.as_ref()?;
        let setter_name = key.strip_prefix('#')?;
        self.classes
            .class_static_method_ids
            .get(&(class_name.clone(), format!("#set::{setter_name}")))
            .copied()
    }

    pub(super) fn is_same_class_static_private_receiver(&self, object: &ResolvedExpr) -> bool {
        match object {
            ResolvedExpr::This { .. } => self.resolve_local("this").is_err(),
            ResolvedExpr::Ident(name) => {
                self.classes.current_class.as_deref() == Some(name.as_str())
            }
            _ => false,
        }
    }

    pub(super) fn current_private_getter_id(&self, key: &str) -> Option<FuncId> {
        let class_name = self.classes.current_class.as_ref()?;
        self.private_getter_id_for_class(class_name, key)
    }

    pub(super) fn private_getter_id_for_class(
        &self,
        class_name: &str,
        key: &str,
    ) -> Option<FuncId> {
        let getter_name = key.strip_prefix('#')?;
        self.classes
            .class_method_ids
            .get(&(class_name.to_owned(), format!("#get::{getter_name}")))
            .copied()
    }

    pub(super) fn current_private_setter_id(&self, key: &str) -> Option<FuncId> {
        let class_name = self.classes.current_class.as_ref()?;
        self.private_setter_id_for_class(class_name, key)
    }

    pub(super) fn private_setter_id_for_class(
        &self,
        class_name: &str,
        key: &str,
    ) -> Option<FuncId> {
        let setter_name = key.strip_prefix('#')?;
        self.classes
            .class_method_ids
            .get(&(class_name.to_owned(), format!("#set::{setter_name}")))
            .copied()
    }

    pub(super) fn private_field_brand_and_slot(
        &self,
        _object: &ResolvedExpr,
        key: &str,
        span: Span,
    ) -> Result<(u32, usize), Diagnostic> {
        let Some(field_name) = key.strip_prefix('#') else {
            return Err(Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!("private field slot lookup requires private key, got `{key}`"),
                span: Some(span),

                phase: None,
            });
        };
        let class_name = self.classes.current_class.as_ref().ok_or_else(|| Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!(
                "issue-255: private field `#{field_name}` access requires declaring class context"
            ),
            span: Some(span),

            phase: None,
        })?;
        let Some(mut slot) = self
            .classes
            .class_private_fields
            .get(class_name)
            .and_then(|fields| fields.get(field_name))
            .copied()
        else {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-255: private field `#{field_name}` is not declared in class `{class_name}`"
                ),
                span: Some(span),

                phase: None,
            });
        };
        slot += self.ancestor_private_slot_count(class_name);
        let brand = self.private_brand_for_class(class_name, Some(span))?;
        Ok((brand, slot))
    }

    fn root_class_name(&self, class_name: &str) -> String {
        let mut current = class_name.to_owned();
        while let Some(parent) = self
            .classes
            .class_parents
            .get(&current)
            .and_then(|p| p.clone())
        {
            current = parent;
        }
        current
    }

    pub(super) fn private_brand_for_class(
        &self,
        class_name: &str,
        span: Option<Span>,
    ) -> Result<u32, Diagnostic> {
        let root = self.root_class_name(class_name);
        let constructor = self
            .classes
            .class_constructor_ids
            .get(&root)
            .copied()
            .ok_or_else(|| Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!("private brand lookup requires constructor for class `{root}`"),
                span,

                phase: None,
            })?;
        u32::try_from(constructor.0.saturating_add(1)).map_err(|_| Diagnostic {
            code: DiagCode::InvariantViolation,
            message: format!("private brand for class `{class_name}` exceeds u32"),
            span,

            phase: None,
        })
    }

    pub(super) fn ancestor_private_slot_count(&self, class_name: &str) -> usize {
        match self
            .classes
            .class_parents
            .get(class_name)
            .and_then(|p| p.as_ref())
        {
            Some(parent) => self.private_slot_count(parent),
            None => 0,
        }
    }

    pub(super) fn private_slot_count(&self, class_name: &str) -> usize {
        let own = self
            .classes
            .class_private_fields
            .get(class_name)
            .map_or(0, HashMap::len);
        own + self.ancestor_private_slot_count(class_name)
    }

    pub(super) fn class_has_instance_private_brand(&self, class_name: &str) -> bool {
        self.private_slot_count(class_name) > 0
            || self
                .classes
                .class_method_ids
                .keys()
                .any(|(owner, method)| owner == class_name && method.starts_with('#'))
    }

    pub(super) fn is_object_key_enumeration_leak(
        &self,
        object: &ResolvedExpr,
        method: &str,
        args: &[ResolvedExpr],
    ) -> bool {
        matches!(object, ResolvedExpr::Ident(name) if name == "Object")
            && matches!(method, "keys" | "values" | "entries")
            && args
                .first()
                .is_some_and(|arg| self.expr_has_private_progress_storage(arg))
    }

    pub(super) fn expr_has_private_progress_storage(&self, expr: &ResolvedExpr) -> bool {
        match expr {
            ResolvedExpr::This { .. } => self
                .classes
                .current_class
                .as_ref()
                .is_some_and(|class_name| self.class_has_private_progress_storage(class_name)),
            ResolvedExpr::Ident(name) => self
                .resolve_local(name)
                .ok()
                .is_some_and(|local| self.local_has_private_progress_storage(local)),
            ResolvedExpr::New { class_name, .. } => {
                self.class_has_private_progress_storage(class_name)
            }
            _ => false,
        }
    }

    pub(super) fn local_has_private_progress_storage(&self, local: LocalId) -> bool {
        self.classes
            .local_classes
            .get(&local)
            .is_some_and(|class_name| self.class_has_private_progress_storage(class_name))
    }

    pub(super) fn class_has_private_progress_storage(&self, class_name: &str) -> bool {
        self.classes
            .class_private_fields
            .get(class_name)
            .is_some_and(|fields| !fields.is_empty())
    }

    pub(super) fn is_date_receiver(&self, expr: &ResolvedExpr) -> bool {
        match expr {
            ResolvedExpr::New { class_name, .. } => class_name == "Date",
            ResolvedExpr::Ident(name) => self
                .resolve_local(name)
                .ok()
                .and_then(|local_id| self.classes.local_classes.get(&local_id))
                .is_some_and(|class_name| class_name == "Date"),
            _ => false,
        }
    }

    pub(super) fn is_unsupported_regexp_compile_receiver(
        &self,
        expr: &ResolvedExpr,
        method: &str,
    ) -> bool {
        if method != "compile" {
            return false;
        }
        match expr {
            ResolvedExpr::String(raw) if looks_like_regexp_literal(raw) => true,
            ResolvedExpr::New { class_name, .. } => class_name == "RegExp",
            ResolvedExpr::Ident(name) => self.resolve_local(name).ok().is_some_and(|local_id| {
                self.facts.regexp_literal_locals.contains(&local_id)
                    || self
                        .classes
                        .local_classes
                        .get(&local_id)
                        .is_some_and(|class_name| class_name == "RegExp")
            }),
            _ => false,
        }
    }

    pub(super) fn class_prototype_ref(
        &self,
        class_name: &str,
    ) -> Result<ClassPrototypeRef, Diagnostic> {
        let constructor = self
            .classes.class_constructor_ids
            .get(class_name)
            .copied()
            .ok_or_else(|| Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-207: instanceof right-hand side must be a supported class constructor `{}`",
                    class_name
                ),
                span: None,

                phase: None,})?;

        let mut parent_constructors = Vec::new();
        let mut current = self
            .classes
            .class_parents
            .get(class_name)
            .and_then(|p| p.clone());
        while let Some(parent) = current {
            let parent_constructor = self
                .classes
                .class_constructor_ids
                .get(&parent)
                .copied()
                .ok_or_else(|| Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "issue-207: superclass constructor `{}` is not available for instanceof",
                        parent
                    ),
                    span: None,

                    phase: None,
                })?;
            parent_constructors.push(parent_constructor);
            current = self
                .classes
                .class_parents
                .get(&parent)
                .and_then(|p| p.clone());
        }

        Ok(ClassPrototypeRef {
            constructor,
            parent_constructors,
        })
    }

    pub(super) fn infer_class_for_expr(&self, expr: &ResolvedExpr) -> Option<String> {
        match expr {
            ResolvedExpr::New { class_name, .. } => Some(class_name.clone()),
            ResolvedExpr::Array(_) => Some("Array".to_owned()),
            ResolvedExpr::Ident(name) => self
                .resolve_local(name)
                .ok()
                .and_then(|local_id| self.classes.local_classes.get(&local_id).cloned()),
            _ => None,
        }
    }
}
