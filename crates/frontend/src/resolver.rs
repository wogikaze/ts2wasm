pub struct Scope {
    symbols: Vec<Vec<(String, SymbolKind)>>,
    current_depth: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    Variable,
    Function,
    Class,
    Import,
    Type,
    Namespace,
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            symbols: vec![Vec::new()],
            current_depth: 0,
        }
    }

    pub fn enter_scope(&mut self) {
        self.current_depth += 1;
        if self.current_depth >= self.symbols.len() {
            self.symbols.push(Vec::new());
        }
    }

    pub fn exit_scope(&mut self) {
        if self.current_depth > 0 {
            if self.current_depth < self.symbols.len() {
                self.symbols[self.current_depth].clear();
            }
            self.current_depth -= 1;
        }
    }

    pub fn declare(&mut self, name: &str, kind: SymbolKind) -> bool {
        if self.current_depth < self.symbols.len() {
            let depth = self.current_depth;
            // Check for duplicate in current scope
            if self.symbols[depth].iter().any(|(n, _)| n == name) {
                return false; // Duplicate declaration
            }
            self.symbols[depth].push((name.to_owned(), kind));
            true
        } else {
            false
        }
    }

    pub fn resolve(&self, name: &str) -> Option<&SymbolKind> {
        for depth in (0..=self.current_depth).rev() {
            if depth < self.symbols.len() {
                for (n, kind) in self.symbols[depth].iter().rev() {
                    if n == name {
                        return Some(kind);
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_resolve() {
        let mut scope = Scope::new();
        assert!(scope.declare("x", SymbolKind::Variable));
        assert_eq!(scope.resolve("x"), Some(&SymbolKind::Variable));
        assert_eq!(scope.resolve("y"), None);
    }

    #[test]
    fn nested_scope() {
        let mut scope = Scope::new();
        scope.declare("x", SymbolKind::Variable);
        scope.enter_scope();
        assert_eq!(scope.resolve("x"), Some(&SymbolKind::Variable));
        scope.declare("y", SymbolKind::Function);
        assert_eq!(scope.resolve("y"), Some(&SymbolKind::Function));
        scope.exit_scope();
        assert_eq!(scope.resolve("y"), None);
        assert_eq!(scope.resolve("x"), Some(&SymbolKind::Variable));
    }

    #[test]
    fn shadowing() {
        let mut scope = Scope::new();
        scope.declare("x", SymbolKind::Variable);
        scope.enter_scope();
        scope.declare("x", SymbolKind::Function);
        assert_eq!(scope.resolve("x"), Some(&SymbolKind::Function));
    }

    #[test]
    fn duplicate_declaration() {
        let mut scope = Scope::new();
        assert!(scope.declare("x", SymbolKind::Variable));
        assert!(!scope.declare("x", SymbolKind::Variable));
    }
}
