//! AlgoBuilder — DSL for constructing SpecAlgoPrograms.

use super::step::*;
use super::program::{SpecAlgoProgram, SpecBlock};
use crate::SpecOp;

/// Builder for constructing SpecAlgoPrograms with automatic local/block allocation.
#[derive(Debug)]
pub struct AlgoBuilder {
    next_local: u32,
    next_block: u32,
    blocks: Vec<SpecBlock>,
    current_block: Option<SpecBlockId>,
    entry_block: SpecBlockId,
}

impl AlgoBuilder {
    pub fn new() -> Self {
        Self {
            next_local: 0,
            next_block: 1,
            blocks: Vec::new(),
            current_block: None,
            entry_block: SpecBlockId(0),
        }
    }

    /// Allocate a new local.
    pub fn alloc_local(&mut self) -> SpecLocal {
        let local = SpecLocal(self.next_local);
        self.next_local += 1;
        local
    }

    /// Undefined constant.
    pub fn undefined(&mut self) -> SpecLocal {
        self.alloc_local()
    }

    /// Allocate a new block and return its ID.
    pub fn new_block(&mut self) -> SpecBlockId {
        let id = SpecBlockId(self.next_block);
        self.next_block += 1;
        id
    }

    /// Start emitting steps into a block.
    pub fn start_block(&mut self, id: SpecBlockId) {
        self.current_block = Some(id);
    }

    /// Emit a step into the current block.
    fn emit(&mut self, step: SpecAlgoStep) {
        let block_id = self.current_block.unwrap_or(self.entry_block);
        // Find or create the block
        if let Some(block) = self.blocks.iter_mut().find(|b| b.id == block_id) {
            block.steps.push(step);
        } else {
            self.blocks.push(SpecBlock {
                id: block_id,
                steps: vec![step],
            });
        }
    }

    // ── Property storage primitives ─────────────────────────────────

    pub fn own_property_lookup(&mut self, object: SpecLocal, key: SpecLocal) -> SpecLocal {
        let result = self.alloc_local();
        self.emit(SpecAlgoStep::OwnPropertyLookup { object, key, result_desc: result });
        result
    }

    pub fn own_property_insert(&mut self, object: SpecLocal, key: SpecLocal, desc: SpecLocal) {
        self.emit(SpecAlgoStep::OwnPropertyInsert { object, key, desc });
    }

    pub fn own_property_update(&mut self, object: SpecLocal, key: SpecLocal, desc: SpecLocal) {
        self.emit(SpecAlgoStep::OwnPropertyUpdate { object, key, desc });
    }

    pub fn get_prototype_slot(&mut self, object: SpecLocal) -> SpecLocal {
        let result = self.alloc_local();
        self.emit(SpecAlgoStep::GetPrototypeSlot { object, result_proto: result });
        result
    }

    pub fn own_property_delete(&mut self, object: SpecLocal, key: SpecLocal) -> SpecLocal {
        let result = self.alloc_local();
        self.emit(SpecAlgoStep::OwnPropertyDelete { object, key, result });
        result
    }

    pub fn set_prototype_slot(&mut self, object: SpecLocal, proto: SpecLocal) -> SpecLocal {
        let result = self.alloc_local();
        self.emit(SpecAlgoStep::SetPrototypeSlot { object, proto, result });
        result
    }

    pub fn is_extensible_bit(&mut self, object: SpecLocal) -> SpecLocal {
        let result = self.alloc_local();
        self.emit(SpecAlgoStep::IsExtensibleBit { object, result });
        result
    }

    pub fn own_property_keys_raw(&mut self, object: SpecLocal) -> SpecLocal {
        let result = self.alloc_local();
        self.emit(SpecAlgoStep::OwnPropertyKeysRaw { object, result });
        result
    }

    pub fn prevent_extensions_bit(&mut self, object: SpecLocal) {
        self.emit(SpecAlgoStep::PreventExtensionsBit { object });
    }

    // ── Descriptor operations ───────────────────────────────────────

    pub fn get_descriptor_value(&mut self, desc: SpecLocal) -> SpecLocal {
        let result = self.alloc_local();
        self.emit(SpecAlgoStep::GetDescriptorValue { desc, result });
        result
    }

    pub fn get_descriptor_getter(&mut self, desc: SpecLocal) -> SpecLocal {
        let result = self.alloc_local();
        self.emit(SpecAlgoStep::GetDescriptorGetter { desc, result });
        result
    }

    pub fn get_descriptor_setter(&mut self, desc: SpecLocal) -> SpecLocal {
        let result = self.alloc_local();
        self.emit(SpecAlgoStep::GetDescriptorSetter { desc, result });
        result
    }

    pub fn is_writable(&mut self, desc: SpecLocal) -> SpecLocal {
        let result = self.alloc_local();
        self.emit(SpecAlgoStep::IsWritable { desc, result });
        result
    }

    pub fn set_descriptor_value(&mut self, desc: SpecLocal, value: SpecLocal) {
        self.emit(SpecAlgoStep::SetDescriptorValue { desc, value });
    }

    pub fn create_data_descriptor(
        &mut self, value: SpecLocal, writable: bool, enumerable: bool, configurable: bool,
    ) -> SpecLocal {
        let result = self.alloc_local();
        self.emit(SpecAlgoStep::CreateDataDescriptor {
            value, writable, enumerable, configurable, result,
        });
        result
    }

    // ── Type queries ────────────────────────────────────────────────

    pub fn is_configurable(&mut self, desc: SpecLocal) -> SpecLocal {
        let result = self.alloc_local();
        self.emit(SpecAlgoStep::IsConfigurable { desc, result });
        result
    }

    pub fn is_data_descriptor(&mut self, desc: SpecLocal) -> SpecLocal {
        let result = self.alloc_local();
        self.emit(SpecAlgoStep::IsDataDescriptor { desc, result });
        result
    }

    pub fn is_undefined(&mut self, value: SpecLocal) -> SpecLocal {
        let result = self.alloc_local();
        let undef = self.alloc_local(); // placeholder for undefined constant
        self.emit(SpecAlgoStep::SameValue { x: value, y: undef, result });
        result
    }

    pub fn is_null(&mut self, value: SpecLocal) -> SpecLocal {
        let result = self.alloc_local();
        let undef = self.alloc_local();
        self.emit(SpecAlgoStep::SameValue { x: value, y: undef, result });
        result
    }

    // ── Control flow ────────────────────────────────────────────────

    pub fn branch_on_condition(
        &mut self, cond: SpecLocal, then_block: SpecBlockId, else_block: SpecBlockId,
    ) {
        self.emit(SpecAlgoStep::BranchOnCondition { cond, then_block, else_block });
    }

    pub fn return_normal(&mut self, value: SpecLocal) {
        self.emit(SpecAlgoStep::ReturnNormal { value });
    }

    pub fn return_undefined(&mut self) {
        let val = self.alloc_local();
        self.emit(SpecAlgoStep::ReturnNormal { value: val });
    }

    pub fn return_throw(&mut self, value: SpecLocal) {
        self.emit(SpecAlgoStep::ReturnThrow { value });
    }

    pub fn return_if_abrupt(&mut self, completion: SpecLocal, result_normal: SpecLocal) {
        self.emit(SpecAlgoStep::ReturnIfAbrupt { completion, result_normal });
    }

    pub fn jump(&mut self, block: SpecBlockId) {
        self.emit(SpecAlgoStep::Jump { block });
    }

    // ── Dispatch ────────────────────────────────────────────────────

    pub fn call_specop(&mut self, op: SpecOp, args: Vec<SpecLocal>) -> SpecLocal {
        let result = self.alloc_local();
        self.emit(SpecAlgoStep::CallSpecOp { op, args, result });
        result
    }

    pub fn call_runtime_primitive(&mut self, symbol: String, args: Vec<SpecLocal>) -> SpecLocal {
        let result = self.alloc_local();
        self.emit(SpecAlgoStep::CallRuntimePrimitive { symbol, args, result });
        result
    }

    pub fn call_function(
        &mut self, callee: SpecLocal, this_arg: SpecLocal, args: Vec<SpecLocal>,
    ) -> SpecLocal {
        let result = self.alloc_local();
        self.emit(SpecAlgoStep::CallFunction { callee, this_arg, args, result });
        result
    }

    /// Allocate a new object.
    pub fn allocate_object(&mut self) -> SpecLocal {
        let result = self.alloc_local();
        self.emit(SpecAlgoStep::AllocateObject { result });
        result
    }

    // ── Build ───────────────────────────────────────────────────────

    /// Build the final SpecAlgoProgram.
    pub fn build(self) -> SpecAlgoProgram {
        // Ensure the entry block exists
        let mut blocks = self.blocks;
        if !blocks.iter().any(|b| b.id == self.entry_block) {
            blocks.push(SpecBlock {
                id: self.entry_block,
                steps: vec![],
            });
        }
        SpecAlgoProgram::new(blocks, self.entry_block, self.next_local)
    }
}

impl Default for AlgoBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_allocates_locals() {
        let mut b = AlgoBuilder::new();
        let a = b.alloc_local();
        let b2 = b.alloc_local();
        assert_eq!(a.0, 0);
        assert_eq!(b2.0, 1);
    }

    #[test]
    fn builder_creates_program() {
        let mut b = AlgoBuilder::new();
        let obj = b.alloc_local();
        let key = b.alloc_local();
        let desc = b.own_property_lookup(obj, key);
        let is_data = b.is_data_descriptor(desc);
        let then_block = b.new_block();
        let else_block = b.new_block();
        b.branch_on_condition(is_data, then_block, else_block);
        b.start_block(then_block);
        let val = b.get_descriptor_value(desc);
        b.return_normal(val);
        b.start_block(else_block);
        b.return_undefined();
        let program = b.build();
        assert!(program.blocks.len() >= 2, "should have at least 2 blocks, got {}", program.blocks.len());
    }
}
