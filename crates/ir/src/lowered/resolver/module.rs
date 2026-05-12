use crate::lowered::*;

impl super::Resolver {
    pub(super) fn module_id_for_specifier(&mut self, specifier: &str) -> usize {
        if let Some(id) = self.ctx.modules.module_ids.get(specifier) {
            return *id;
        }

        let id = self.ctx.modules.modules.len() + 1;
        self.ctx.modules.module_ids.insert(specifier.to_owned(), id);
        self.ctx.modules.modules.push(ModuleInfo {
            id,
            specifier: specifier.to_owned(),
            statements: Vec::new(),
            locals_count: 0,
        });
        id
    }
}
