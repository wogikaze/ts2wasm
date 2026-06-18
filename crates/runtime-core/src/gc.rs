use crate::heap::GcHeader;
use crate::value::TaggedValue;

pub type HandleScopeId = u32;

#[derive(Debug, Clone)]
pub struct GcRoot {
    pub value: TaggedValue,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct HandleScope {
    roots: Vec<GcRoot>,
    parent: Option<HandleScopeId>,
}

impl HandleScope {
    pub fn new() -> Self {
        Self {
            roots: Vec::new(),
            parent: None,
        }
    }

    pub fn push_root(&mut self, value: TaggedValue, description: &str) {
        self.roots.push(GcRoot {
            value,
            description: description.to_string(),
        });
    }

    pub fn pop_root(&mut self) -> Option<GcRoot> {
        self.roots.pop()
    }

    pub fn roots(&self) -> &[GcRoot] {
        &self.roots
    }

    pub fn root_count(&self) -> usize {
        self.roots.len()
    }
}

impl Default for HandleScope {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct GcState {
    pub heap_base: u32,
    pub heap_current: u32,
    pub heap_limit: u32,
    pub allocation_count: u64,
}

impl GcState {
    pub fn new(heap_base: u32, heap_limit: u32) -> Self {
        Self {
            heap_base,
            heap_current: heap_base,
            heap_limit,
            allocation_count: 0,
        }
    }

    pub fn alloc(&mut self, size: u32) -> Option<u32> {
        let total = GcHeader::SIZE + size;
        if self.heap_current + total > self.heap_limit {
            return None;
        }
        let ptr = self.heap_current;
        self.heap_current += total;
        self.allocation_count += 1;
        Some(ptr + GcHeader::SIZE)
    }

    pub fn remaining(&self) -> u32 {
        self.heap_limit.saturating_sub(self.heap_current)
    }
}
