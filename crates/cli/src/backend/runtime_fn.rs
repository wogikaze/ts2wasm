use crate::ir::builtin::BuiltinId;
use crate::runtime::consts::RuntimeString;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) enum RuntimeFn {
    /// M6-1 skeleton for stdin path. Real UTF-8 decode/runtime behavior is added in later M6 slices.
    ReadStdinUtf8,
    Write,
    Copy,
    ValueToStringInto,
    Log,
    TruthyBool,
    Not,
    StringEqual,
    Concat,
    IsString,
    Add,
    Sub,
    Negate,
    Less,
    Greater,
    StrictEqual,
    And,
    Or,
    /// Bump-allocate `size` bytes on the heap, aligned to `Layout::ALIGN`.
    AllocHeap,
    /// Byte-by-byte memory equality check used by `PropertyGet`.
    MemEqual,
    /// Load an element from a heap array by tagged-int index.
    ArrayGet,
    /// Read the `.length` of a string or array (i32 at offset 0 of heap ptr).
    GetLength,
    /// Linear-scan property lookup on a heap object.
    PropertyGet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) enum HostImport {
    FdRead,
    FdWrite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) enum Capability {
    StdinRead,
    StdoutWrite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum RuntimeResult {
    Value,
    EffectOnly,
}

pub(crate) struct RuntimeSpec {
    pub symbol: &'static str,
    pub deps: &'static [RuntimeFn],
    pub imports: &'static [HostImport],
    pub capability: &'static [Capability],
    pub runtime_strings: &'static [&'static str],
    pub result: RuntimeResult,
}

const NO_DEPS: &[RuntimeFn] = &[];
const NO_IMPORTS: &[HostImport] = &[];
const NO_CAPS: &[Capability] = &[];
const NO_RUNTIME_STRINGS: &[&str] = &[];

const READ_STDIN_DEPS: &[RuntimeFn] = &[RuntimeFn::AllocHeap, RuntimeFn::Copy];
const WRITE_DEPS: &[RuntimeFn] = &[];
const COPY_DEPS: &[RuntimeFn] = &[];
const VTS_DEPS: &[RuntimeFn] = &[RuntimeFn::Copy];
const LOG_DEPS: &[RuntimeFn] = &[RuntimeFn::Write, RuntimeFn::ValueToStringInto];
const STRING_EQUAL_DEPS: &[RuntimeFn] = &[RuntimeFn::IsString];
const CONCAT_DEPS: &[RuntimeFn] = &[RuntimeFn::ValueToStringInto];
const ADD_DEPS: &[RuntimeFn] = &[RuntimeFn::IsString, RuntimeFn::Concat];
const STRICT_EQUAL_DEPS: &[RuntimeFn] = &[RuntimeFn::IsString, RuntimeFn::StringEqual];
const AND_DEPS: &[RuntimeFn] = &[RuntimeFn::TruthyBool];
const OR_DEPS: &[RuntimeFn] = &[RuntimeFn::TruthyBool];

const IMPORT_FD_READ: &[HostImport] = &[HostImport::FdRead];
const IMPORT_FD_WRITE: &[HostImport] = &[HostImport::FdWrite];
const CAP_STDIN_READ: &[Capability] = &[Capability::StdinRead];
const CAP_STDOUT_WRITE: &[Capability] = &[Capability::StdoutWrite];
const VTS_RUNTIME_STRINGS: &[&str] = &[
    RuntimeString::UNDEFINED,
    RuntimeString::NULL,
    RuntimeString::FALSE,
    RuntimeString::TRUE,
];
const LOG_RUNTIME_STRINGS: &[&str] = &[RuntimeString::NEWLINE];

impl RuntimeFn {
    pub(crate) const fn from_builtin(builtin: BuiltinId) -> Self {
        match builtin {
            BuiltinId::ConsoleLog => Self::Log,
            BuiltinId::ReadStdinUtf8 => Self::ReadStdinUtf8,
        }
    }

    pub(crate) const fn spec(self) -> RuntimeSpec {
        match self {
            Self::ReadStdinUtf8 => RuntimeSpec {
                symbol: "$read_stdin_utf8",
                deps: READ_STDIN_DEPS,
                imports: IMPORT_FD_READ,
                capability: CAP_STDIN_READ,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::Write => RuntimeSpec {
                symbol: "$write",
                deps: WRITE_DEPS,
                imports: IMPORT_FD_WRITE,
                capability: CAP_STDOUT_WRITE,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::EffectOnly,
            },
            Self::Copy => RuntimeSpec {
                symbol: "$copy",
                deps: COPY_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::EffectOnly,
            },
            Self::ValueToStringInto => RuntimeSpec {
                symbol: "$value_to_string_into",
                deps: VTS_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: VTS_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::Log => RuntimeSpec {
                symbol: "$log",
                deps: LOG_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: LOG_RUNTIME_STRINGS,
                result: RuntimeResult::EffectOnly,
            },
            Self::TruthyBool => RuntimeSpec {
                symbol: "$truthy_bool",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::Not => RuntimeSpec {
                symbol: "$not",
                deps: &[Self::TruthyBool],
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::StringEqual => RuntimeSpec {
                symbol: "$string_equal",
                deps: STRING_EQUAL_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::Concat => RuntimeSpec {
                symbol: "$concat",
                deps: CONCAT_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::IsString => RuntimeSpec {
                symbol: "$is_string",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::Add => RuntimeSpec {
                symbol: "$add",
                deps: ADD_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::Sub => RuntimeSpec {
                symbol: "$sub",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::Negate => RuntimeSpec {
                symbol: "$negate",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::Less => RuntimeSpec {
                symbol: "$less",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::Greater => RuntimeSpec {
                symbol: "$greater",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::StrictEqual => RuntimeSpec {
                symbol: "$strict_equal",
                deps: STRICT_EQUAL_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::And => RuntimeSpec {
                symbol: "$and",
                deps: AND_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::Or => RuntimeSpec {
                symbol: "$or",
                deps: OR_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::AllocHeap => RuntimeSpec {
                symbol: "$alloc_heap",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::MemEqual => RuntimeSpec {
                symbol: "$mem_equal",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::ArrayGet => RuntimeSpec {
                symbol: "$array_get",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::GetLength => RuntimeSpec {
                symbol: "$get_length",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::PropertyGet => RuntimeSpec {
                symbol: "$property_get",
                deps: &[Self::MemEqual],
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
        }
    }

    pub(crate) const fn symbol(self) -> &'static str {
        self.spec().symbol
    }

    pub(crate) const fn result(self) -> RuntimeResult {
        self.spec().result
    }

    pub(crate) const fn is_value(self) -> bool {
        matches!(self.result(), RuntimeResult::Value)
    }

    pub(crate) const fn emission_order() -> &'static [RuntimeFn] {
        &[
            Self::ReadStdinUtf8,
            Self::Write,
            Self::Copy,
            Self::ValueToStringInto,
            Self::Log,
            Self::TruthyBool,
            Self::Not,
            Self::StringEqual,
            Self::Concat,
            Self::IsString,
            Self::Add,
            Self::Sub,
            Self::Negate,
            Self::Less,
            Self::Greater,
            Self::StrictEqual,
            Self::And,
            Self::Or,
            Self::AllocHeap,
            Self::MemEqual,
            Self::ArrayGet,
            Self::GetLength,
            Self::PropertyGet,
        ]
    }

    #[cfg(test)]
    pub(crate) const fn all() -> &'static [RuntimeFn] {
        &[
            Self::ReadStdinUtf8,
            Self::Write,
            Self::Copy,
            Self::ValueToStringInto,
            Self::Log,
            Self::TruthyBool,
            Self::Not,
            Self::StringEqual,
            Self::Concat,
            Self::IsString,
            Self::Add,
            Self::Sub,
            Self::Negate,
            Self::Less,
            Self::Greater,
            Self::StrictEqual,
            Self::And,
            Self::Or,
            Self::AllocHeap,
            Self::MemEqual,
            Self::ArrayGet,
            Self::GetLength,
            Self::PropertyGet,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::RuntimeFn;

    #[test]
    fn emission_order_is_unique_and_complete() {
        let order = RuntimeFn::emission_order();
        let all = RuntimeFn::all();
        assert_eq!(order.len(), all.len());
        for item in all {
            assert_eq!(
                order.iter().filter(|candidate| *candidate == item).count(),
                1
            );
        }
    }

    #[test]
    fn emission_order_contains_all_dependencies() {
        let order = RuntimeFn::emission_order();
        for runtime_fn in order {
            for dep in runtime_fn.spec().deps {
                assert!(
                    order.contains(dep),
                    "missing dependency {:?} for {:?} in emission order",
                    dep,
                    runtime_fn
                );
            }
        }
    }
}
