use crate::shape::ShapeId;
use crate::value::TaggedValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementsKind {
    Generic,
    Int32,
    Float64,
}

pub struct ObjectHeader;

impl ObjectHeader {
    pub const SHAPE_ID_OFFSET: u32 = 0;
    pub const ELEMENTS_KIND_OFFSET: u32 = 4;
    pub const INLINE_PROPERTIES_OFFSET: u32 = 8;
}

#[derive(Debug, Clone)]
pub struct Object {
    pub shape_id: ShapeId,
    pub elements_kind: ElementsKind,
    pub inline_properties: Vec<TaggedValue>,
}

impl Object {
    pub fn new(shape_id: ShapeId, inline_capacity: usize) -> Self {
        Self {
            shape_id,
            elements_kind: ElementsKind::Generic,
            inline_properties: Vec::with_capacity(inline_capacity),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TypedArrayKind {
    Int8Array,
    Uint8Array,
    Uint8ClampedArray,
    Int16Array,
    Uint16Array,
    Int32Array,
    Uint32Array,
    BigInt64Array,
    BigUint64Array,
    Float32Array,
    Float64Array,
}

pub type ModuleRef = u32;

#[derive(Debug, Clone)]
pub enum ObjectKind {
    Ordinary(Object),
    Array(Object),
    StringExotic {
        primitive_value: TaggedValue,
    },
    ArgumentsExotic(Object),
    TypedArray {
        buffer: TaggedValue,
        kind: TypedArrayKind,
    },
    BoundFunction(Object),
    Proxy {
        target: TaggedValue,
        handler: TaggedValue,
    },
    ModuleNamespace {
        module: ModuleRef,
    },
}
