/// Semantic domain classification for RuntimeFn variants.
/// Enables domain-based dispatch in the runtime builder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum RuntimeDomain {
    Core,
    Operator,
    TypeCoercion,
    Number,
    BigInt,
    String,
    Array,
    Object,
    MapSet,
    TypedArray,
    Date,
    Math,
    Json,
    RegExp,
    Promise,
    Task,
    Symbol,
    Iterator,
    Module,
    Host,
    Encoding,
}
