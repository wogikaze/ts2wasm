use crate::value::TaggedValue;

#[derive(Debug, Clone, Copy)]
pub struct PropertyAttributes(u8);

impl PropertyAttributes {
    pub const WRITABLE: u8 = 0b001;
    pub const ENUMERABLE: u8 = 0b010;
    pub const CONFIGURABLE: u8 = 0b100;

    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn all() -> Self {
        Self(Self::WRITABLE | Self::ENUMERABLE | Self::CONFIGURABLE)
    }

    pub fn writable(self) -> bool {
        self.0 & Self::WRITABLE != 0
    }
    pub fn enumerable(self) -> bool {
        self.0 & Self::ENUMERABLE != 0
    }
    pub fn configurable(self) -> bool {
        self.0 & Self::CONFIGURABLE != 0
    }
    pub fn bits(self) -> u8 {
        self.0
    }
    pub fn from_bits(bits: u8) -> Self {
        Self(bits & 0b111)
    }

    pub fn set_writable(&mut self, v: bool) {
        if v { self.0 |= Self::WRITABLE; } else { self.0 &= !Self::WRITABLE; }
    }
    pub fn set_enumerable(&mut self, v: bool) {
        if v { self.0 |= Self::ENUMERABLE; } else { self.0 &= !Self::ENUMERABLE; }
    }
    pub fn set_configurable(&mut self, v: bool) {
        if v { self.0 |= Self::CONFIGURABLE; } else { self.0 &= !Self::CONFIGURABLE; }
    }
}

#[derive(Debug, Clone)]
pub enum PropertyDescriptor {
    Data {
        value: TaggedValue,
        writable: bool,
        enumerable: bool,
        configurable: bool,
    },
    Accessor {
        get: TaggedValue,
        set: TaggedValue,
        enumerable: bool,
        configurable: bool,
    },
}

impl PropertyDescriptor {
    pub fn is_data(&self) -> bool {
        matches!(self, Self::Data { .. })
    }

    pub fn is_accessor(&self) -> bool {
        matches!(self, Self::Accessor { .. })
    }

    pub fn enumerable(&self) -> bool {
        match self {
            Self::Data { enumerable, .. } => *enumerable,
            Self::Accessor { enumerable, .. } => *enumerable,
        }
    }

    pub fn configurable(&self) -> bool {
        match self {
            Self::Data { configurable, .. } => *configurable,
            Self::Accessor { configurable, .. } => *configurable,
        }
    }

    pub fn writable(&self) -> bool {
        match self {
            Self::Data { writable, .. } => *writable,
            Self::Accessor { .. } => false,
        }
    }

    pub fn attributes(&self) -> PropertyAttributes {
        let mut attrs = PropertyAttributes::empty();
        if self.writable() { attrs.set_writable(true); }
        if self.enumerable() { attrs.set_enumerable(true); }
        if self.configurable() { attrs.set_configurable(true); }
        attrs
    }

    pub fn data(value: TaggedValue, writable: bool, enumerable: bool, configurable: bool) -> Self {
        Self::Data { value, writable, enumerable, configurable }
    }

    pub fn accessor(get: TaggedValue, set: TaggedValue, enumerable: bool, configurable: bool) -> Self {
        Self::Accessor { get, set, enumerable, configurable }
    }

    pub fn from_attributes(value: TaggedValue, attrs: PropertyAttributes) -> Self {
        Self::Data {
            value,
            writable: attrs.writable(),
            enumerable: attrs.enumerable(),
            configurable: attrs.configurable(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CompactDescriptor(u32, TaggedValue);

impl CompactDescriptor {
    pub fn new(value: TaggedValue, attrs: PropertyAttributes, is_accessor: bool) -> Self {
        let bits = (attrs.bits() as u32) | ((is_accessor as u32) << 3);
        Self(bits, value)
    }

    pub fn value(&self) -> TaggedValue {
        self.1
    }

    pub fn attributes(&self) -> PropertyAttributes {
        PropertyAttributes::from_bits((self.0 & 0b111) as u8)
    }

    pub fn is_accessor(&self) -> bool {
        self.0 & (1 << 3) != 0
    }
}
