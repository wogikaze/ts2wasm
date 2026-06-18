pub type ShapeId = u32;
pub const SHAPE_EMPTY: ShapeId = 0;

pub struct CapabilityFlags;

impl CapabilityFlags {
    pub const EXTENSIBLE: u8 = 1;
    pub const SEALED: u8 = 2;
    pub const FROZEN: u8 = 4;

    pub fn is_extensible(flags: u8) -> bool {
        flags & Self::EXTENSIBLE != 0
    }
    pub fn is_sealed(flags: u8) -> bool {
        flags & Self::SEALED != 0
    }
    pub fn is_frozen(flags: u8) -> bool {
        flags & Self::FROZEN != 0
    }
}

#[derive(Debug, Clone)]
pub struct ShapeProperty {
    pub name: String,
    pub offset: u32,
    pub attributes: u8,
}

#[derive(Debug, Clone)]
pub struct Shape {
    pub id: ShapeId,
    pub parent: Option<ShapeId>,
    pub transition_property: Option<String>,
    pub capability_flags: u8,
    pub property_count: u32,
    pub properties: Vec<ShapeProperty>,
}

impl Shape {
    pub fn new(id: ShapeId) -> Self {
        Self {
            id,
            parent: None,
            transition_property: None,
            capability_flags: CapabilityFlags::EXTENSIBLE,
            property_count: 0,
            properties: Vec::new(),
        }
    }

    pub fn find_property(&self, name: &str) -> Option<&ShapeProperty> {
        self.properties.iter().find(|p| p.name == name)
    }

    pub fn find_property_offset(&self, name: &str) -> Option<u32> {
        self.find_property(name).map(|p| p.offset)
    }

    pub fn with_capability(mut self, flags: u8) -> Self {
        self.capability_flags = flags;
        self
    }

    pub fn is_extensible(&self) -> bool {
        CapabilityFlags::is_extensible(self.capability_flags)
    }
}
