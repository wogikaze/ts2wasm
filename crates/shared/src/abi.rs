/// Logical runtime ABI types for imports/exports described in `docs/14-runtime-abi.md`.
///
/// `JsVal` uses wasm text type `i64` here. Generated user wasm still passes tagged
/// JavaScript values as i32 inside the module body (`crates/cli` `WasmTaggedJsWire`);
/// any bridge between i32 tagged values and i64 `JsVal` at import boundaries must be
/// explicit and tested—do not widen/narrow implicitly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbiType {
    JsVal,
    Bool,
    Ptr,
    Len,
    Index,
    Status,
    F64,
}

impl AbiType {
    pub const fn wasm_repr(self) -> &'static str {
        match self {
            Self::JsVal => "i64",
            Self::Bool | Self::Ptr | Self::Len | Self::Index | Self::Status => "i32",
            Self::F64 => "f64",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbiFunction {
    pub name: &'static str,
    pub params: &'static [AbiType],
    pub result: Option<AbiType>,
}

impl AbiFunction {
    pub fn export_name(&self) -> String {
        format!("ts2wasm.rt.{}", self.name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeAbi {
    pub schema_version: u32,
    pub functions: &'static [AbiFunction],
}

impl RuntimeAbi {
    pub const V1: Self = Self {
        schema_version: 1,
        functions: &[
            AbiFunction {
                name: "make_undefined",
                params: &[],
                result: Some(AbiType::JsVal),
            },
            AbiFunction {
                name: "make_null",
                params: &[],
                result: Some(AbiType::JsVal),
            },
            AbiFunction {
                name: "make_bool",
                params: &[AbiType::Bool],
                result: Some(AbiType::JsVal),
            },
            AbiFunction {
                name: "make_number",
                params: &[AbiType::F64],
                result: Some(AbiType::JsVal),
            },
            AbiFunction {
                name: "make_string_utf8",
                params: &[AbiType::Ptr, AbiType::Len],
                result: Some(AbiType::JsVal),
            },
            AbiFunction {
                name: "to_boolean",
                params: &[AbiType::JsVal],
                result: Some(AbiType::Bool),
            },
            AbiFunction {
                name: "to_number",
                params: &[AbiType::JsVal],
                result: Some(AbiType::JsVal),
            },
            AbiFunction {
                name: "to_string",
                params: &[AbiType::JsVal],
                result: Some(AbiType::JsVal),
            },
            AbiFunction {
                name: "strict_equal",
                params: &[AbiType::JsVal, AbiType::JsVal],
                result: Some(AbiType::Bool),
            },
            AbiFunction {
                name: "abstract_equal",
                params: &[AbiType::JsVal, AbiType::JsVal],
                result: Some(AbiType::Bool),
            },
            AbiFunction {
                name: "new_object",
                params: &[],
                result: Some(AbiType::JsVal),
            },
            AbiFunction {
                name: "get_prop",
                params: &[AbiType::JsVal, AbiType::JsVal],
                result: Some(AbiType::JsVal),
            },
            AbiFunction {
                name: "set_prop",
                params: &[AbiType::JsVal, AbiType::JsVal, AbiType::JsVal],
                result: Some(AbiType::Status),
            },
            AbiFunction {
                name: "new_array",
                params: &[AbiType::Len],
                result: Some(AbiType::JsVal),
            },
            AbiFunction {
                name: "array_get",
                params: &[AbiType::JsVal, AbiType::Index],
                result: Some(AbiType::JsVal),
            },
            AbiFunction {
                name: "array_set",
                params: &[AbiType::JsVal, AbiType::Index, AbiType::JsVal],
                result: Some(AbiType::Status),
            },
            AbiFunction {
                name: "call",
                params: &[AbiType::JsVal, AbiType::JsVal, AbiType::Ptr, AbiType::Len],
                result: Some(AbiType::JsVal),
            },
            AbiFunction {
                name: "throw_value",
                params: &[AbiType::JsVal],
                result: Some(AbiType::Status),
            },
            AbiFunction {
                name: "string_trim",
                params: &[AbiType::JsVal],
                result: Some(AbiType::JsVal),
            },
            AbiFunction {
                name: "utf8_decode",
                params: &[AbiType::Ptr, AbiType::Len],
                result: Some(AbiType::JsVal),
            },
            AbiFunction {
                name: "utf8_encode",
                params: &[AbiType::JsVal, AbiType::Ptr],
                result: Some(AbiType::Len),
            },
        ],
    };

    pub fn find(&self, name: &str) -> Option<&AbiFunction> {
        self.functions.iter().find(|function| function.name == name)
    }
}

// ---------------------------------------------------------------------------
// ExecutionTarget — canonical target descriptor
// ---------------------------------------------------------------------------

/// Canonical execution target identifiers for the ts2wasm backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExecutionTarget {
    Wasm32WasiP1,
    Wasm32WasiP1NodeShim,
    Wasm32WasiGc,
    Wasm32Component,
}

impl ExecutionTarget {
    pub const fn manifest_target(self) -> &'static str {
        match self {
            Self::Wasm32WasiP1 => "wasm32-wasi-p1",
            Self::Wasm32WasiP1NodeShim => "wasm32-wasi-p1+node-shim",
            Self::Wasm32WasiGc => "wasm32-wasi-gc",
            Self::Wasm32Component => "wasm32-component",
        }
    }

    pub fn aliases(self) -> &'static [&'static str] {
        match self {
            Self::Wasm32WasiP1 => &["wasm32-wasi", "wasm32-wasi-p1"],
            Self::Wasm32WasiP1NodeShim => &[
                "wasm32-wasi+node-host",
                "wasm32-wasi-p1+node-host",
                "wasm32-wasi-p1+node-shim",
            ],
            Self::Wasm32WasiGc => &["wasm32-wasi-gc"],
            Self::Wasm32Component => &["wasm32-component"],
        }
    }

    pub const fn target_profile(self) -> &'static str {
        match self {
            Self::Wasm32WasiP1 => "wasi-p1",
            Self::Wasm32WasiP1NodeShim => "wasi-p1+node-shim",
            Self::Wasm32WasiGc => "wasi-gc",
            Self::Wasm32Component => "component",
        }
    }

    pub fn features(self) -> &'static [&'static str] {
        match self {
            Self::Wasm32WasiP1 => &["wasi-preview1", "standalone"],
            Self::Wasm32WasiP1NodeShim => &["wasi-preview1", "node-host-shim"],
            Self::Wasm32WasiGc => &["wasi-preview1", "wasm-gc"],
            Self::Wasm32Component => &["component-model"],
        }
    }

    pub const fn is_implemented(self) -> bool {
        matches!(self, Self::Wasm32WasiP1 | Self::Wasm32WasiP1NodeShim)
    }

    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "wasm32-wasi" | "wasm32-wasi-p1" => Some(Self::Wasm32WasiP1),
            "wasm32-wasi+node-host" | "wasm32-wasi-p1+node-host" | "wasm32-wasi-p1+node-shim" => {
                Some(Self::Wasm32WasiP1NodeShim)
            }
            "wasm32-wasi-gc" => Some(Self::Wasm32WasiGc),
            "wasm32-component" => Some(Self::Wasm32Component),
            _ => None,
        }
    }

    pub const fn is_wasi(self) -> bool {
        matches!(
            self,
            Self::Wasm32WasiP1 | Self::Wasm32WasiP1NodeShim | Self::Wasm32WasiGc
        )
    }
}

#[cfg(test)]
mod tests_abi {
    use super::*;

    #[test]
    fn execution_target_parse_roundtrip() {
        for target in &[
            ExecutionTarget::Wasm32WasiP1,
            ExecutionTarget::Wasm32WasiP1NodeShim,
        ] {
            let s = target.manifest_target();
            let parsed = ExecutionTarget::from_string(s).unwrap();
            assert_eq!(parsed, *target);
        }
    }

    #[test]
    fn execution_target_known_aliases_parse() {
        assert_eq!(
            ExecutionTarget::from_string("wasm32-wasi"),
            Some(ExecutionTarget::Wasm32WasiP1)
        );
        assert_eq!(
            ExecutionTarget::from_string("wasm32-wasi+node-host"),
            Some(ExecutionTarget::Wasm32WasiP1NodeShim)
        );
        assert_eq!(
            ExecutionTarget::from_string("wasm32-wasi-p1+node-host"),
            Some(ExecutionTarget::Wasm32WasiP1NodeShim)
        );
    }

    #[test]
    fn execution_target_unknown_string_returns_none() {
        assert_eq!(ExecutionTarget::from_string("wasm32-unknown"), None);
        assert_eq!(ExecutionTarget::from_string(""), None);
    }

    #[test]
    fn execution_target_implemented_targets() {
        assert!(ExecutionTarget::Wasm32WasiP1.is_implemented());
        assert!(ExecutionTarget::Wasm32WasiP1NodeShim.is_implemented());
        assert!(!ExecutionTarget::Wasm32WasiGc.is_implemented());
        assert!(!ExecutionTarget::Wasm32Component.is_implemented());
    }

    #[test]
    fn execution_target_wasi_check() {
        assert!(ExecutionTarget::Wasm32WasiP1.is_wasi());
        assert!(ExecutionTarget::Wasm32WasiP1NodeShim.is_wasi());
        assert!(ExecutionTarget::Wasm32WasiGc.is_wasi());
        assert!(!ExecutionTarget::Wasm32Component.is_wasi());
    }

    #[test]
    fn manifest_target_maps_correctly() {
        assert_eq!(
            ExecutionTarget::Wasm32WasiP1.manifest_target(),
            "wasm32-wasi-p1"
        );
        assert_eq!(
            ExecutionTarget::Wasm32WasiP1NodeShim.manifest_target(),
            "wasm32-wasi-p1+node-shim"
        );
        assert_eq!(
            ExecutionTarget::Wasm32WasiGc.manifest_target(),
            "wasm32-wasi-gc"
        );
        assert_eq!(
            ExecutionTarget::Wasm32Component.manifest_target(),
            "wasm32-component"
        );
    }

    #[test]
    fn target_profile_maps_correctly() {
        assert_eq!(ExecutionTarget::Wasm32WasiP1.target_profile(), "wasi-p1");
        assert_eq!(
            ExecutionTarget::Wasm32WasiP1NodeShim.target_profile(),
            "wasi-p1+node-shim"
        );
    }

    #[test]
    fn future_targets_have_features_list() {
        assert!(
            ExecutionTarget::Wasm32WasiGc
                .features()
                .contains(&"wasm-gc")
        );
        assert!(
            ExecutionTarget::Wasm32Component
                .features()
                .contains(&"component-model")
        );
    }

    #[test]
    fn jsval_is_i64_logical_repr_distinct_from_m0_i32_body() {
        assert_eq!(AbiType::JsVal.wasm_repr(), "i64");
        assert_eq!(AbiType::Bool.wasm_repr(), "i32");
        assert_eq!(AbiType::Ptr.wasm_repr(), "i32");
        assert_eq!(AbiType::F64.wasm_repr(), "f64");
    }

    #[test]
    fn export_names_are_namespaced() {
        let function = RuntimeAbi::V1.find("make_number").unwrap();
        assert_eq!(function.export_name(), "ts2wasm.rt.make_number");
    }

    #[test]
    fn required_m0_helpers_are_present() {
        for name in [
            "make_undefined",
            "make_number",
            "strict_equal",
            "get_prop",
            "call",
            "utf8_decode",
        ] {
            assert!(RuntimeAbi::V1.find(name).is_some(), "{name} is missing");
        }
    }
}
