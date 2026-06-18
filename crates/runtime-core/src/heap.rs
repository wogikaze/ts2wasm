use ts2wasm_runtime_abi::Layout;

pub const HEAP_KIND_UNKNOWN: u32 = 0;
pub const HEAP_KIND_STRING: u32 = 4;
pub const HEAP_KIND_ARRAY: u32 = 8;
pub const HEAP_KIND_OBJECT: u32 = 12;
pub const HEAP_KIND_BIGINT: u32 = 16;
pub const HEAP_KIND_NUMBER: u32 = 20;

pub struct GcFlags;

impl GcFlags {
    pub const MARK: u32 = 1;
    pub const FINALIZABLE: u32 = 2;
    pub const KIND_MASK: u32 = 0xFC;
    pub const KIND_SHIFT: u32 = 2;
}

#[repr(C)]
pub struct GcHeader {
    pub flags_and_kind: u32,
    pub body_size: u32,
    pub sweep_next: u32,
    pub _reserved: u32,
}

impl GcHeader {
    pub const SIZE: u32 = Layout::GC_HEADER_SIZE;

    pub fn kind(&self) -> u32 {
        (self.flags_and_kind & GcFlags::KIND_MASK) >> GcFlags::KIND_SHIFT
    }

    pub fn set_kind(&mut self, kind: u32) {
        self.flags_and_kind =
            (self.flags_and_kind & !GcFlags::KIND_MASK) | (kind << GcFlags::KIND_SHIFT);
    }

    pub fn is_marked(&self) -> bool {
        self.flags_and_kind & GcFlags::MARK != 0
    }

    pub fn set_marked(&mut self, marked: bool) {
        if marked {
            self.flags_and_kind |= GcFlags::MARK;
        } else {
            self.flags_and_kind &= !GcFlags::MARK;
        }
    }
}
