use crate::lowered::*;

impl<'a> super::Resolver<'a> {
    pub(super) fn module_id_for_specifier(&mut self, specifier: &str) -> usize {
        if let Some(id) = self.modules.module_ids.get(specifier) {
            return *id;
        }

        let id = self.modules.modules.len() + 1;
        self.modules.module_ids.insert(specifier.to_owned(), id);
        self.modules.modules.push(ModuleInfo {
            id,
            specifier: specifier.to_owned(),
            statements: Vec::new(),
            locals_count: 0,
        });
        id
    }
}
