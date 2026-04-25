/// Logical runtime ABI types for imports/exports described in `docs/15-runtime-abi.md`.
///
/// `JsVal` uses wasm text type `i64` here. Generated M0 user wasm still passes **tagged
/// JavaScript values as i32** inside the module body (`crates/cli` `M0WasmTaggedValue`);
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

#[cfg(test)]
mod tests {
    use super::*;

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
