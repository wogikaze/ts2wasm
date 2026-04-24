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
    fn jsval_is_i64_for_core_wasm_backend() {
        assert_eq!(AbiType::JsVal.wasm_repr(), "i64");
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
