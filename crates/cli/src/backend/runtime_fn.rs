use crate::ir::lowered::BuiltinId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) enum RuntimeFn {
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
    Less,
    StrictEqual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum HostImport {
    FdWrite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Capability {
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
    pub result: RuntimeResult,
}

const NO_DEPS: &[RuntimeFn] = &[];
const NO_IMPORTS: &[HostImport] = &[];
const NO_CAPS: &[Capability] = &[];

const WRITE_DEPS: &[RuntimeFn] = &[];
const COPY_DEPS: &[RuntimeFn] = &[];
const VTS_DEPS: &[RuntimeFn] = &[RuntimeFn::Copy];
const LOG_DEPS: &[RuntimeFn] = &[RuntimeFn::Write, RuntimeFn::ValueToStringInto];
const STRING_EQUAL_DEPS: &[RuntimeFn] = &[RuntimeFn::IsString];
const CONCAT_DEPS: &[RuntimeFn] = &[RuntimeFn::ValueToStringInto];
const ADD_DEPS: &[RuntimeFn] = &[RuntimeFn::IsString, RuntimeFn::Concat];
const STRICT_EQUAL_DEPS: &[RuntimeFn] = &[RuntimeFn::IsString, RuntimeFn::StringEqual];

const IMPORT_FD_WRITE: &[HostImport] = &[HostImport::FdWrite];
const CAP_STDOUT_WRITE: &[Capability] = &[Capability::StdoutWrite];

impl RuntimeFn {
    pub(crate) const fn from_builtin(builtin: BuiltinId) -> Self {
        match builtin {
            BuiltinId::ConsoleLog => Self::Log,
        }
    }

    pub(crate) const fn spec(self) -> RuntimeSpec {
        match self {
            Self::Write => RuntimeSpec {
                symbol: "$write",
                deps: WRITE_DEPS,
                imports: IMPORT_FD_WRITE,
                capability: CAP_STDOUT_WRITE,
                result: RuntimeResult::EffectOnly,
            },
            Self::Copy => RuntimeSpec {
                symbol: "$copy",
                deps: COPY_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                result: RuntimeResult::EffectOnly,
            },
            Self::ValueToStringInto => RuntimeSpec {
                symbol: "$value_to_string_into",
                deps: VTS_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                result: RuntimeResult::Value,
            },
            Self::Log => RuntimeSpec {
                symbol: "$log",
                deps: LOG_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                result: RuntimeResult::EffectOnly,
            },
            Self::TruthyBool => RuntimeSpec {
                symbol: "$truthy_bool",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                result: RuntimeResult::Value,
            },
            Self::Not => RuntimeSpec {
                symbol: "$not",
                deps: &[Self::TruthyBool],
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                result: RuntimeResult::Value,
            },
            Self::StringEqual => RuntimeSpec {
                symbol: "$string_equal",
                deps: STRING_EQUAL_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                result: RuntimeResult::Value,
            },
            Self::Concat => RuntimeSpec {
                symbol: "$concat",
                deps: CONCAT_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                result: RuntimeResult::Value,
            },
            Self::IsString => RuntimeSpec {
                symbol: "$is_string",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                result: RuntimeResult::Value,
            },
            Self::Add => RuntimeSpec {
                symbol: "$add",
                deps: ADD_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                result: RuntimeResult::Value,
            },
            Self::Sub => RuntimeSpec {
                symbol: "$sub",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                result: RuntimeResult::Value,
            },
            Self::Less => RuntimeSpec {
                symbol: "$less",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                result: RuntimeResult::Value,
            },
            Self::StrictEqual => RuntimeSpec {
                symbol: "$strict_equal",
                deps: STRICT_EQUAL_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
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
            Self::Less,
            Self::StrictEqual,
        ]
    }
}
