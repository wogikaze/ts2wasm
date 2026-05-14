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
// ABI metadata constants and struct
// ---------------------------------------------------------------------------

/// Current schema version for the `ts2wasm.abi` custom section.
pub const ABI_METADATA_SCHEMA_VERSION: u32 = 1;

/// Name of the ABI custom section in generated wasm modules.
pub const ABI_CUSTOM_SECTION_NAME: &str = "ts2wasm.abi";

/// Generator identifier for ABI metadata.
pub const ABI_GENERATOR: &str = "ts2wasm";

/// ABI metadata embedded in the `ts2wasm.abi` custom section of generated
/// wasm modules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbiMetadata {
    pub schema_version: u32,
    pub runtime_abi_version: u32,
    pub target: &'static str,
    pub target_profile: &'static str,
    pub features: &'static [&'static str],
    pub generator: &'static str,
}

impl AbiMetadata {
    pub const fn wasm32_wasi_p1() -> Self {
        Self {
            schema_version: ABI_METADATA_SCHEMA_VERSION,
            runtime_abi_version: 2,
            target: "wasm32-wasi-p1",
            target_profile: "wasi-p1",
            features: &["wasi-preview1", "standalone"],
            generator: ABI_GENERATOR,
        }
    }
}

impl Default for AbiMetadata {
    fn default() -> Self {
        Self::wasm32_wasi_p1()
    }
}

impl AbiMetadata {
    pub fn to_json(&self) -> String {
        let features_json = self
            .features
            .iter()
            .map(|f| format!("\"{f}\""))
            .collect::<Vec<_>>()
            .join(",");
        format!(
            r#"{{"schema_version":{},"runtime_abi_version":{},"target":"{}","target_profile":"{}","features":[{}],"generator":"{}"}}"#,
            self.schema_version,
            self.runtime_abi_version,
            self.target,
            self.target_profile,
            features_json,
            self.generator,
        )
    }

    /// Encode metadata as bytes for a WASM custom section payload.
    pub fn to_custom_section_payload(&self) -> Vec<u8> {
        self.to_json().into_bytes()
    }
}

// ---------------------------------------------------------------------------
// TargetProfile — canonical profile descriptor
// ---------------------------------------------------------------------------

/// Canonical execution profile describing the target environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TargetProfile {
    /// Profile name (e.g. "wasi-p1", "wasi-p1+node-shim").
    pub name: &'static str,
    /// Feature flags describing the execution environment.
    pub features: &'static [&'static str],
    /// Whether this profile is currently implemented by the backend.
    pub is_implemented: bool,
}

impl TargetProfile {
    pub const fn new(
        name: &'static str,
        features: &'static [&'static str],
        is_implemented: bool,
    ) -> Self {
        Self {
            name,
            features,
            is_implemented,
        }
    }
}

// ---------------------------------------------------------------------------
// TargetSpec — full target specification
// ---------------------------------------------------------------------------

/// Full target specification for metadata emission and validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetSpec {
    /// Canonical target name (e.g. "wasm32-wasi-p1").
    pub target: &'static str,
    /// Execution profile.
    pub profile: TargetProfile,
}

impl TargetSpec {
    pub const fn new(target: &'static str, profile: TargetProfile) -> Self {
        Self { target, profile }
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

    /// Return a `TargetProfile` for this execution target.
    pub const fn profile(self) -> TargetProfile {
        match self {
            Self::Wasm32WasiP1 => {
                TargetProfile::new("wasi-p1", &["wasi-preview1", "standalone"], true)
            }
            Self::Wasm32WasiP1NodeShim => TargetProfile::new(
                "wasi-p1+node-shim",
                &["wasi-preview1", "node-host-shim"],
                true,
            ),
            Self::Wasm32WasiGc => {
                TargetProfile::new("wasi-gc", &["wasi-preview1", "wasm-gc"], false)
            }
            Self::Wasm32Component => TargetProfile::new("component", &["component-model"], false),
        }
    }

    /// Return a `TargetSpec` for this execution target.
    pub const fn spec(self) -> TargetSpec {
        TargetSpec::new(self.manifest_target(), self.profile())
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

/// Try to parse a target string, rejecting future/unimplemented targets.
/// Returns `None` for unknown targets or unimplemented future targets
/// (wasm32-wasi-gc, wasm32-component).
pub fn parse_implemented_target(s: &str) -> Option<ExecutionTarget> {
    let target = ExecutionTarget::from_string(s)?;
    if target.is_implemented() {
        Some(target)
    } else {
        None
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

    // ---------------------------------------------------------------------------
    // TargetProfile / parse_implemented_target tests
    // ---------------------------------------------------------------------------

    #[test]
    fn target_profile_roundtrip() {
        let p = ExecutionTarget::Wasm32WasiP1.profile();
        assert_eq!(p.name, "wasi-p1");
        assert!(p.is_implemented);
        assert!(p.features.contains(&"standalone"));
    }

    #[test]
    fn target_spec_roundtrip() {
        let spec = ExecutionTarget::Wasm32WasiP1.spec();
        assert_eq!(spec.target, "wasm32-wasi-p1");
        assert_eq!(spec.profile.name, "wasi-p1");
    }

    #[test]
    fn parse_implemented_target_accepts_implemented() {
        assert_eq!(
            parse_implemented_target("wasm32-wasi-p1"),
            Some(ExecutionTarget::Wasm32WasiP1)
        );
        assert_eq!(
            parse_implemented_target("wasm32-wasi-p1+node-shim"),
            Some(ExecutionTarget::Wasm32WasiP1NodeShim)
        );
    }

    #[test]
    fn parse_implemented_target_rejects_future() {
        assert_eq!(parse_implemented_target("wasm32-wasi-gc"), None);
        assert_eq!(parse_implemented_target("wasm32-component"), None);
    }

    #[test]
    fn parse_implemented_target_rejects_unknown() {
        assert_eq!(parse_implemented_target("wasm32-unknown"), None);
        assert_eq!(parse_implemented_target(""), None);
    }

    #[test]
    fn parse_implemented_target_cannot_silently_mutate_standalone() {
        let standalone = parse_implemented_target("wasm32-wasi-p1").unwrap();
        let node_shim = parse_implemented_target("wasm32-wasi-p1+node-shim").unwrap();
        assert_ne!(standalone, node_shim);
        assert_eq!(standalone, ExecutionTarget::Wasm32WasiP1);
        assert_eq!(node_shim, ExecutionTarget::Wasm32WasiP1NodeShim);
    }

    #[test]
    fn target_profile_future_targets_are_not_implemented() {
        assert!(!ExecutionTarget::Wasm32WasiGc.profile().is_implemented);
        assert!(!ExecutionTarget::Wasm32Component.profile().is_implemented);
    }

    // ---------------------------------------------------------------------------
    // AbiMetadata tests
    // ---------------------------------------------------------------------------

    #[test]
    fn abi_metadata_default() {
        let meta = AbiMetadata::default();
        assert_eq!(meta.schema_version, ABI_METADATA_SCHEMA_VERSION);
        assert_eq!(meta.runtime_abi_version, 2);
        assert_eq!(meta.target, "wasm32-wasi-p1");
        assert_eq!(meta.target_profile, "wasi-p1");
        assert!(meta.features.contains(&"standalone"));
        assert_eq!(meta.generator, ABI_GENERATOR);
    }

    #[test]
    fn abi_metadata_to_json() {
        let meta = AbiMetadata::default();
        let json = meta.to_json();
        assert!(json.contains(r#""schema_version":1"#));
        assert!(json.contains(r#""runtime_abi_version":2"#));
        assert!(json.contains(r#""target":"wasm32-wasi-p1""#));
        assert!(json.contains(r#""generator":"ts2wasm""#));
    }

    #[test]
    fn abi_metadata_custom_section_payload() {
        let meta = AbiMetadata::default();
        let payload = meta.to_custom_section_payload();
        assert!(!payload.is_empty());
        let payload_str = String::from_utf8_lossy(&payload);
        assert!(payload_str.contains("ts2wasm"));
    }

    #[test]
    fn abi_custom_section_name() {
        assert_eq!(ABI_CUSTOM_SECTION_NAME, "ts2wasm.abi");
    }
}
