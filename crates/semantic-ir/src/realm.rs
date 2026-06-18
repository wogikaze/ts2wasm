// ---------------------------------------------------------------------------
// Realm — § 9.3
//
// A Realm is the spec's mechanism for representing a JavaScript execution
// environment with its own set of built-in objects, global environment,
// and intrinsics.
// ---------------------------------------------------------------------------

use std::collections::HashMap;

use crate::env::EnvID;
use crate::value::{ObjectID, Value};

// ---------------------------------------------------------------------------
// Realm ID — lightweight handle
// ---------------------------------------------------------------------------

/// A unique identifier for a Realm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RealmID(pub u32);

// ---------------------------------------------------------------------------
// Realm — § 9.3
// ---------------------------------------------------------------------------

/// A Realm — the top-level execution environment for JavaScript.
///
/// Each Realm has its own set of built-in objects, global environment, and
/// intrinsic objects. This is essential for correct behavior of code that
/// creates multiple realms (e.g., iframes, vm.createContext).
#[derive(Debug, Clone)]
pub struct Realm {
    /// The global environment record.
    pub global_env: EnvID,
    /// The global object (`globalThis`).
    pub global_object: ObjectID,
    /// The intrinsics map — named built-in objects like `Object`, `Array`, etc.
    pub intrinsics: IntrinsicMap,
    /// The current Realm's [[IsTemplate]] flag for tagged templates.
    pub is_template: bool,
    /// The host-defined intrinsics.
    pub host_intrinsics: HashMap<String, Value>,
}

impl Realm {
    /// Create a new Realm with default intrinsics.
    pub fn new(global_env: EnvID, global_object: ObjectID) -> Self {
        Self {
            global_env,
            global_object,
            intrinsics: IntrinsicMap::default(),
            is_template: false,
            host_intrinsics: HashMap::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// IntrinsicMap — named built-in objects
// ---------------------------------------------------------------------------

/// A map of intrinsic names to their object IDs.
///
/// § 9.3.2 — Intrinsic Objects. Each Realm has a set of intrinsic objects
/// that are created when the Realm is initialized. These include the
/// standard built-in constructors (`Object`, `Array`, `Function`, etc.)
/// and their prototypes.
#[derive(Debug, Clone, Default)]
pub struct IntrinsicMap {
    /// The named intrinsics — keyed by intrinsic name.
    intrinsics: HashMap<IntrinsicName, ObjectID>,
    /// The prototype objects for built-in types.
    prototypes: HashMap<IntrinsicName, ObjectID>,
}

impl IntrinsicMap {
    /// Get an intrinsic by name.
    pub fn get(&self, name: IntrinsicName) -> Option<ObjectID> {
        self.intrinsics.get(&name).copied()
    }

    /// Get a prototype by name.
    pub fn get_prototype(&self, name: IntrinsicName) -> Option<ObjectID> {
        self.prototypes.get(&name).copied()
    }

    /// Register an intrinsic.
    pub fn set(&mut self, name: IntrinsicName, object: ObjectID) {
        self.intrinsics.insert(name, object);
    }

    /// Register a prototype.
    pub fn set_prototype(&mut self, name: IntrinsicName, object: ObjectID) {
        self.prototypes.insert(name, object);
    }
}

// ---------------------------------------------------------------------------
// IntrinsicName — § 20
// ---------------------------------------------------------------------------

/// Named intrinsic objects — § 20.
///
/// These are the standard built-in objects that each Realm has.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntrinsicName {
    // -- Fundamental objects --
    Object,
    ObjectPrototype,
    Function,
    FunctionPrototype,

    // -- Fundamental objects: Boolean, Number, String --
    Boolean,
    BooleanPrototype,
    Number,
    NumberPrototype,
    String,
    StringPrototype,

    // -- Fundamental objects: Symbol --
    Symbol,
    SymbolPrototype,

    // -- Fundamental objects: BigInt --
    BigInt,
    BigIntPrototype,

    // -- Fundamental objects: Error --
    Error,
    ErrorPrototype,
    EvalError,
    EvalErrorPrototype,
    RangeError,
    RangeErrorPrototype,
    ReferenceError,
    ReferenceErrorPrototype,
    SyntaxError,
    SyntaxErrorPrototype,
    TypeError,
    TypeErrorPrototype,
    URIError,
    URIErrorPrototype,

    // -- Keyed collections --
    Array,
    ArrayPrototype,
    Map,
    MapPrototype,
    Set,
    SetPrototype,
    WeakMap,
    WeakMapPrototype,
    WeakSet,
    WeakSetPrototype,

    // -- Structured data --
    ArrayBuffer,
    ArrayBufferPrototype,
    SharedArrayBuffer,
    SharedArrayBufferPrototype,
    DataView,
    DataViewPrototype,
    Json,
    Promise,
    PromisePrototype,

    // -- Iteration --
    IteratorPrototype,
    ArrayIteratorPrototype,
    MapIteratorPrototype,
    SetIteratorPrototype,
    StringIteratorPrototype,

    // -- Reflect --
    Reflect,

    // -- Proxy --
    Proxy,

    // -- Date --
    Date,
    DatePrototype,

    // -- RegExp --
    RegExp,
    RegExpPrototype,

    // -- Math --
    Math,

    // -- JSON --
    // Already listed above as Json

    // -- ArrayBuffer and SharedArrayBuffer --
    // Already listed above

    // -- Atomics --
    Atomics,

    // -- FinalizationRegistry --
    FinalizationRegistry,
    FinalizationRegistryPrototype,

    // -- WeakRef --
    WeakRef,
    WeakRefPrototype,

    // -- Generator --
    GeneratorFunction,
    GeneratorPrototype,
    GeneratorFunctionPrototype,

    // -- Async --
    AsyncFunction,
    AsyncFunctionPrototype,
    AsyncGeneratorFunction,
    AsyncGeneratorPrototype,

    // -- Iterator helpers --
    Iterator,
    IteratorPrototype_,
    WrapperIteratorPrototype,

    // -- AggregateError --
    AggregateError,
    AggregateErrorPrototype,

    // -- SuppressedError --
    SuppressedError,
    SuppressedErrorPrototype,

    // -- Temporal --
    Temporal,
    TemporalPlainDate,
    TemporalPlainDateTime,
    TemporalPlainTime,
    TemporalPlainYearMonth,
    TemporalPlainMonthDay,
    TemporalZonedDateTime,
    TemporalDuration,
    TemporalCalendar,
    TemporalTimeZone,
    TemporalInstant,
    TemporalNow,
}

impl IntrinsicName {
    /// Returns the string name of the intrinsic (used for `globalThis[name]` lookup).
    pub fn as_str(&self) -> &'static str {
        match self {
            IntrinsicName::Object => "Object",
            IntrinsicName::ObjectPrototype => "Object",
            IntrinsicName::Function => "Function",
            IntrinsicName::FunctionPrototype => "Function",
            IntrinsicName::Boolean => "Boolean",
            IntrinsicName::BooleanPrototype => "Boolean",
            IntrinsicName::Number => "Number",
            IntrinsicName::NumberPrototype => "Number",
            IntrinsicName::String => "String",
            IntrinsicName::StringPrototype => "String",
            IntrinsicName::Symbol => "Symbol",
            IntrinsicName::SymbolPrototype => "Symbol",
            IntrinsicName::BigInt => "BigInt",
            IntrinsicName::BigIntPrototype => "BigInt",
            IntrinsicName::Error => "Error",
            IntrinsicName::ErrorPrototype => "Error",
            IntrinsicName::Array => "Array",
            IntrinsicName::ArrayPrototype => "Array",
            IntrinsicName::Map => "Map",
            IntrinsicName::MapPrototype => "Map",
            IntrinsicName::Set => "Set",
            IntrinsicName::SetPrototype => "Set",
            IntrinsicName::Promise => "Promise",
            IntrinsicName::PromisePrototype => "Promise",
            IntrinsicName::RegExp => "RegExp",
            IntrinsicName::RegExpPrototype => "RegExp",
            IntrinsicName::Date => "Date",
            IntrinsicName::DatePrototype => "Date",
            IntrinsicName::Math => "Math",
            IntrinsicName::Json => "JSON",
            IntrinsicName::Reflect => "Reflect",
            IntrinsicName::Proxy => "Proxy",
            // For error types, return the name directly
            IntrinsicName::EvalError => "EvalError",
            IntrinsicName::RangeError => "RangeError",
            IntrinsicName::ReferenceError => "ReferenceError",
            IntrinsicName::SyntaxError => "SyntaxError",
            IntrinsicName::TypeError => "TypeError",
            IntrinsicName::URIError => "URIError",
            IntrinsicName::AggregateError => "AggregateError",
            IntrinsicName::SuppressedError => "SuppressedError",
            // For prototypes, return the constructor name
            IntrinsicName::EvalErrorPrototype => "EvalError",
            IntrinsicName::RangeErrorPrototype => "RangeError",
            IntrinsicName::ReferenceErrorPrototype => "ReferenceError",
            IntrinsicName::SyntaxErrorPrototype => "SyntaxError",
            IntrinsicName::TypeErrorPrototype => "TypeError",
            IntrinsicName::URIErrorPrototype => "URIError",
            IntrinsicName::AggregateErrorPrototype => "AggregateError",
            IntrinsicName::SuppressedErrorPrototype => "SuppressedError",
            // Other types
            IntrinsicName::ArrayBuffer => "ArrayBuffer",
            IntrinsicName::ArrayBufferPrototype => "ArrayBuffer",
            IntrinsicName::SharedArrayBuffer => "SharedArrayBuffer",
            IntrinsicName::SharedArrayBufferPrototype => "SharedArrayBuffer",
            IntrinsicName::DataView => "DataView",
            IntrinsicName::DataViewPrototype => "DataView",
            IntrinsicName::WeakMap => "WeakMap",
            IntrinsicName::WeakMapPrototype => "WeakMap",
            IntrinsicName::WeakSet => "WeakSet",
            IntrinsicName::WeakSetPrototype => "WeakSet",
            IntrinsicName::WeakRef => "WeakRef",
            IntrinsicName::WeakRefPrototype => "WeakRef",
            IntrinsicName::FinalizationRegistry => "FinalizationRegistry",
            IntrinsicName::FinalizationRegistryPrototype => "FinalizationRegistry",
            IntrinsicName::Atomics => "Atomics",
            IntrinsicName::Temporal => "Temporal",
            IntrinsicName::TemporalNow => "Temporal.Now",
            IntrinsicName::IteratorPrototype => "Iterator",
            IntrinsicName::IteratorPrototype_ => "Iterator",
            IntrinsicName::WrapperIteratorPrototype => "Iterator",
            IntrinsicName::ArrayIteratorPrototype => "Array Iterator",
            IntrinsicName::MapIteratorPrototype => "Map Iterator",
            IntrinsicName::SetIteratorPrototype => "Set Iterator",
            IntrinsicName::StringIteratorPrototype => "String Iterator",
            IntrinsicName::GeneratorFunction => "GeneratorFunction",
            IntrinsicName::GeneratorPrototype => "Generator",
            IntrinsicName::GeneratorFunctionPrototype => "GeneratorFunction",
            IntrinsicName::AsyncFunction => "AsyncFunction",
            IntrinsicName::AsyncFunctionPrototype => "AsyncFunction",
            IntrinsicName::AsyncGeneratorFunction => "AsyncGeneratorFunction",
            IntrinsicName::AsyncGeneratorPrototype => "AsyncGenerator",
            IntrinsicName::Iterator => "Iterator",
            IntrinsicName::TemporalPlainDate => "Temporal.PlainDate",
            IntrinsicName::TemporalPlainDateTime => "Temporal.PlainDateTime",
            IntrinsicName::TemporalPlainTime => "Temporal.PlainTime",
            IntrinsicName::TemporalPlainYearMonth => "Temporal.PlainYearMonth",
            IntrinsicName::TemporalPlainMonthDay => "Temporal.PlainMonthDay",
            IntrinsicName::TemporalZonedDateTime => "Temporal.ZonedDateTime",
            IntrinsicName::TemporalDuration => "Temporal.Duration",
            IntrinsicName::TemporalCalendar => "Temporal.Calendar",
            IntrinsicName::TemporalTimeZone => "Temporal.TimeZone",
            IntrinsicName::TemporalInstant => "Temporal.Instant",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn realm_new_has_default_intrinsics() {
        let realm = Realm::new(EnvID(0), ObjectID(0));
        assert!(!realm.is_template);
        assert!(realm.host_intrinsics.is_empty());
    }

    #[test]
    fn intrinsics_set_and_get() {
        let mut map = IntrinsicMap::default();
        map.set(IntrinsicName::Object, ObjectID(1));
        assert_eq!(map.get(IntrinsicName::Object), Some(ObjectID(1)));
        assert_eq!(map.get(IntrinsicName::Array), None);
    }

    #[test]
    fn intrinsic_name_as_str() {
        assert_eq!(IntrinsicName::Object.as_str(), "Object");
        assert_eq!(IntrinsicName::Array.as_str(), "Array");
        assert_eq!(IntrinsicName::Math.as_str(), "Math");
    }
}
