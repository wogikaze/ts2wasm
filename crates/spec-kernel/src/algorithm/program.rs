//! SpecAlgoProgram — a complete algorithm as a sequence of SpecAlgoSteps
//! organized into basic blocks.

use std::collections::HashMap;
use super::step::{SpecAlgoStep, SpecBlockId};

/// A basic block in a SpecAlgoProgram.
#[derive(Debug, Clone)]
pub struct SpecBlock {
    pub id: SpecBlockId,
    pub steps: Vec<SpecAlgoStep>,
}

/// A complete ECMAScript algorithm program.
///
/// Each algorithm is a flat list of typed steps organized into basic blocks.
/// Blocks are connected by Branch/Jump instructions.
#[derive(Debug, Clone)]
pub struct SpecAlgoProgram {
    pub blocks: Vec<SpecBlock>,
    pub entry_block: SpecBlockId,
    pub local_count: u32,
}

impl SpecAlgoProgram {
    pub fn new(blocks: Vec<SpecBlock>, entry_block: SpecBlockId, local_count: u32) -> Self {
        Self { blocks, entry_block, local_count }
    }

    /// Collect all steps from all blocks in order.
    pub fn all_steps(&self) -> Vec<&SpecAlgoStep> {
        let mut all = Vec::new();
        for block in &self.blocks {
            for step in &block.steps {
                all.push(step);
            }
        }
        all
    }

    /// Check if a step kind exists anywhere in the program.
    pub fn contains<P>(&self, predicate: P) -> bool
    where
        P: Fn(&SpecAlgoStep) -> bool,
    {
        self.all_steps().into_iter().any(predicate)
    }
}

/// A map from SpecOp symbol to its algorithm program.
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct SpecAlgoRegistry {
    pub algorithms: HashMap<String, SpecAlgoProgram>,
}

impl SpecAlgoRegistry {
    pub fn new() -> Self {
        Self { algorithms: HashMap::new() }
    }

    pub fn register(&mut self, name: &str, program: SpecAlgoProgram) {
        self.algorithms.insert(name.to_owned(), program);
    }

    pub fn get(&self, name: &str) -> Option<&SpecAlgoProgram> {
        self.algorithms.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SpecOp;
    use crate::algorithm::step::Completion;

    #[test]
    fn empty_program_has_no_steps() {
        let program = SpecAlgoProgram::new(vec![], SpecBlockId(0), 0);
        assert!(program.all_steps().is_empty());
    }

    #[test]
    fn program_with_one_block() {
        use SpecAlgoStep::*;
        let block = SpecBlock {
            id: SpecBlockId(0),
            steps: vec![
                OwnPropertyLookup {
                    object: SpecLocal(0),
                    key: SpecLocal(1),
                    result_desc: SpecLocal(2),
                },
            ],
        };
        let program = SpecAlgoProgram::new(vec![block], SpecBlockId(0), 3);
        assert_eq!(program.all_steps().len(), 1);
    }

    #[test]
    fn registry_registers_and_retrieves() {
        let mut reg = SpecAlgoRegistry::new();
        let program = SpecAlgoProgram::new(vec![], SpecBlockId(0), 0);
        reg.register("OrdinaryGet", program);
        assert!(reg.get("OrdinaryGet").is_some());
        assert!(reg.get("UnknownGet").is_none());
    }

    #[test]
    fn program_contains_predicate() {
        use SpecAlgoStep::*;
        let block = SpecBlock {
            id: SpecBlockId(0),
            steps: vec![
                ReturnNormal { value: SpecLocal(0) },
            ],
        };
        let program = SpecAlgoProgram::new(vec![block], SpecBlockId(0), 1);
        assert!(program.contains(|s| matches!(s, ReturnNormal { .. })));
        assert!(!program.contains(|s| matches!(s, ReturnThrow { .. })));
    }
}
