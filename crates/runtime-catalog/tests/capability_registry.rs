use std::collections::BTreeSet;

use ts2wasm_runtime_catalog::{
    Capability, HostAbi, HostImport, RuntimeFn, RuntimeLinkPlan, emit_link_plan_snapshot,
    validate_runtime_link_plan,
};

#[derive(Debug, Clone, Copy)]
struct RuntimeImportCase {
    runtime_fn: RuntimeFn,
    imports: &'static [HostImport],
    capabilities: &'static [Capability],
}

const HOST_IMPORT_RUNTIME_CASES: &[RuntimeImportCase] = &[
    RuntimeImportCase {
        runtime_fn: RuntimeFn::ReadStdinBytes,
        imports: &[HostImport::FdRead],
        capabilities: &[Capability::StdinRead],
    },
    RuntimeImportCase {
        runtime_fn: RuntimeFn::Write,
        imports: &[HostImport::FdWrite],
        capabilities: &[Capability::StdoutWrite],
    },
    RuntimeImportCase {
        runtime_fn: RuntimeFn::DateNewLive,
        imports: &[HostImport::ClockTimeGet],
        capabilities: &[Capability::WasiClockRealtime],
    },
    RuntimeImportCase {
        runtime_fn: RuntimeFn::DateNow,
        imports: &[HostImport::ClockTimeGet],
        capabilities: &[Capability::WasiClockRealtime],
    },
    RuntimeImportCase {
        runtime_fn: RuntimeFn::MathRandom,
        imports: &[HostImport::RandomGet],
        capabilities: &[Capability::WasiRandom],
    },
    RuntimeImportCase {
        runtime_fn: RuntimeFn::FsReadFileSync,
        imports: &[
            HostImport::PathOpen,
            HostImport::FdRead,
            HostImport::FdClose,
        ],
        capabilities: &[Capability::WasiFilesystemRead],
    },
    RuntimeImportCase {
        runtime_fn: RuntimeFn::FsWriteFileSync,
        imports: &[
            HostImport::PathOpen,
            HostImport::FdWrite,
            HostImport::FdClose,
        ],
        capabilities: &[Capability::WasiFilesystemWrite],
    },
    RuntimeImportCase {
        runtime_fn: RuntimeFn::FsAppendFileSync,
        imports: &[HostImport::FsAppendFileSync],
        capabilities: &[Capability::HostFsAppendFileSync],
    },
    RuntimeImportCase {
        runtime_fn: RuntimeFn::ProcessArgv,
        imports: &[HostImport::ArgsSizesGet, HostImport::ArgsGet],
        capabilities: &[Capability::WasiArgs],
    },
    RuntimeImportCase {
        runtime_fn: RuntimeFn::ProcessEnv,
        imports: &[HostImport::EnvironSizesGet, HostImport::EnvironGet],
        capabilities: &[Capability::WasiEnv],
    },
    RuntimeImportCase {
        runtime_fn: RuntimeFn::ProcessExit,
        imports: &[HostImport::ProcessExit],
        capabilities: &[Capability::HostProcessExit],
    },
    RuntimeImportCase {
        runtime_fn: RuntimeFn::PathJoin,
        imports: &[HostImport::PathJoin],
        capabilities: &[Capability::HostPathJoin],
    },
    RuntimeImportCase {
        runtime_fn: RuntimeFn::PathResolve,
        imports: &[HostImport::PathResolve],
        capabilities: &[Capability::HostPathResolve],
    },
    RuntimeImportCase {
        runtime_fn: RuntimeFn::PathBasename,
        imports: &[HostImport::PathBasename],
        capabilities: &[Capability::HostPathBasename],
    },
    RuntimeImportCase {
        runtime_fn: RuntimeFn::PathDirname,
        imports: &[HostImport::PathDirname],
        capabilities: &[Capability::HostPathDirname],
    },
    RuntimeImportCase {
        runtime_fn: RuntimeFn::CryptoRandomBytes,
        imports: &[HostImport::CryptoRandomBytes],
        capabilities: &[Capability::HostCryptoRandomBytes],
    },
    RuntimeImportCase {
        runtime_fn: RuntimeFn::EncodeURI,
        imports: &[HostImport::EncodeURI],
        capabilities: &[Capability::HostEncodeURI],
    },
    RuntimeImportCase {
        runtime_fn: RuntimeFn::DecodeURI,
        imports: &[HostImport::DecodeURI],
        capabilities: &[Capability::HostDecodeURI],
    },
    RuntimeImportCase {
        runtime_fn: RuntimeFn::Escape,
        imports: &[HostImport::Escape],
        capabilities: &[Capability::HostEscape],
    },
    RuntimeImportCase {
        runtime_fn: RuntimeFn::Unescape,
        imports: &[HostImport::Unescape],
        capabilities: &[Capability::HostUnescape],
    },
    RuntimeImportCase {
        runtime_fn: RuntimeFn::DateToString,
        imports: &[HostImport::DateToString],
        capabilities: &[Capability::HostDateToString],
    },
    RuntimeImportCase {
        runtime_fn: RuntimeFn::DateGetLocalTimeField,
        imports: &[HostImport::DateGetLocalTimeField],
        capabilities: &[Capability::HostDateGetLocalTimeField],
    },
    RuntimeImportCase {
        runtime_fn: RuntimeFn::DateToISOString,
        imports: &[HostImport::DateToISOString],
        capabilities: &[Capability::HostDateToISOString],
    },
    RuntimeImportCase {
        runtime_fn: RuntimeFn::DateGetTimezoneOffset,
        imports: &[HostImport::DateGetTimezoneOffset],
        capabilities: &[Capability::HostDateGetTimezoneOffset],
    },
];

const ALL_HOST_IMPORTS: &[HostImport] = &[
    HostImport::FdRead,
    HostImport::FdWrite,
    HostImport::PathOpen,
    HostImport::FdClose,
    HostImport::WasiProcExit,
    HostImport::ClockTimeGet,
    HostImport::ClockResGet,
    HostImport::RandomGet,
    HostImport::ArgsSizesGet,
    HostImport::ArgsGet,
    HostImport::EnvironSizesGet,
    HostImport::EnvironGet,
    HostImport::FsReadFileSync,
    HostImport::FsWriteFileSync,
    HostImport::FsAppendFileSync,
    HostImport::ProcessExit,
    HostImport::PathJoin,
    HostImport::PathResolve,
    HostImport::PathBasename,
    HostImport::PathDirname,
    HostImport::CryptoRandomBytes,
    HostImport::EncodeURI,
    HostImport::DecodeURI,
    HostImport::Escape,
    HostImport::Unescape,
    HostImport::DateToString,
    HostImport::DateGetLocalTimeField,
    HostImport::DateToISOString,
    HostImport::DateGetTimezoneOffset,
];

const NON_RUNTIME_LINK_PLAN_HOST_IMPORTS: &[HostImport] = &[
    // Backend policy, not runtime-catalog policy: the backend always inserts
    // WASI proc_exit for program termination.
    HostImport::WasiProcExit,
    // Declared metadata without a current RuntimeFn link-plan producer.
    HostImport::ClockResGet,
    HostImport::FsReadFileSync,
    HostImport::FsWriteFileSync,
];

const ALL_CAPABILITIES: &[Capability] = &[
    Capability::StdinRead,
    Capability::StdoutWrite,
    Capability::WasiClockRealtime,
    Capability::WasiRandom,
    Capability::WasiArgs,
    Capability::WasiEnv,
    Capability::WasiFilesystemRead,
    Capability::WasiFilesystemWrite,
    Capability::WasiFilesystemAppend,
    Capability::HostFsReadFileSync,
    Capability::HostFsWriteFileSync,
    Capability::HostFsAppendFileSync,
    Capability::HostProcessExit,
    Capability::HostPathJoin,
    Capability::HostPathResolve,
    Capability::HostPathBasename,
    Capability::HostPathDirname,
    Capability::HostCryptoRandomBytes,
    Capability::HostEncodeURI,
    Capability::HostDecodeURI,
    Capability::HostEscape,
    Capability::HostUnescape,
    Capability::HostDateToString,
    Capability::HostDateGetLocalTimeField,
    Capability::HostDateToISOString,
    Capability::HostDateGetTimezoneOffset,
];

const NON_RUNTIME_LINK_PLAN_CAPABILITIES: &[Capability] = &[
    Capability::WasiFilesystemAppend,
    Capability::HostFsReadFileSync,
    Capability::HostFsWriteFileSync,
];

#[test]
fn host_import_manifest_names_match_specs() {
    for import in ALL_HOST_IMPORTS {
        let spec = import.spec();
        assert!(
            !spec.module.is_empty(),
            "{import:?} module should not be empty"
        );
        assert!(!spec.name.is_empty(), "{import:?} name should not be empty");
        assert!(
            !spec.wat_symbol.is_empty(),
            "{import:?} wat symbol should not be empty"
        );

        let expected_manifest_name = format!("{}.{}", spec.module, spec.name);
        assert_eq!(
            import.manifest_name(),
            expected_manifest_name,
            "{import:?} manifest name should be derived from its spec"
        );
    }
}

#[test]
fn capability_manifest_names_are_unique_and_non_empty() {
    let mut names = BTreeSet::new();

    for capability in ALL_CAPABILITIES {
        let name = capability.manifest_name();
        assert!(
            !name.is_empty(),
            "{capability:?} manifest name should not be empty"
        );
        assert!(
            names.insert(name),
            "{capability:?} duplicates capability manifest name {name}"
        );
    }
}

#[test]
fn host_imports_are_runtime_linked_or_explicitly_non_runtime_policy() {
    let linked_imports = linked_host_imports();
    let non_runtime_imports = NON_RUNTIME_LINK_PLAN_HOST_IMPORTS
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();

    for import in ALL_HOST_IMPORTS {
        assert!(
            linked_imports.contains(import) || non_runtime_imports.contains(import),
            "{import:?} must be covered by a RuntimeLinkPlan case or explicitly listed as non-runtime policy"
        );
    }
}

#[test]
fn capabilities_are_runtime_linked_or_explicitly_non_runtime_policy() {
    let linked_capabilities = linked_capabilities();
    let non_runtime_capabilities = NON_RUNTIME_LINK_PLAN_CAPABILITIES
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();

    for capability in ALL_CAPABILITIES {
        assert!(
            linked_capabilities.contains(capability)
                || non_runtime_capabilities.contains(capability),
            "{capability:?} must be covered by a RuntimeLinkPlan case or explicitly listed as non-runtime policy"
        );
    }
}

#[test]
fn runtime_functions_with_host_imports_have_explicit_capabilities() {
    for case in HOST_IMPORT_RUNTIME_CASES {
        let spec = case.runtime_fn.spec();
        assert_eq!(
            spec.imports, case.imports,
            "{:?} import registry drifted; update the consistency case",
            case.runtime_fn
        );
        assert_eq!(
            spec.capability, case.capabilities,
            "{:?} capability registry drifted; do not add imports without an explicit capability decision",
            case.runtime_fn
        );
        assert!(
            !spec.imports.is_empty(),
            "{:?} consistency case should cover a host-importing runtime function",
            case.runtime_fn
        );
        assert!(
            !spec.capability.is_empty(),
            "{:?} imports host APIs but has no explicit capability marker",
            case.runtime_fn
        );
    }
}

#[test]
fn runtime_link_plan_populates_imports_capabilities_and_manifest_target() {
    for case in HOST_IMPORT_RUNTIME_CASES {
        let plan = populated_plan_for(case.runtime_fn);
        let validated = validate_runtime_link_plan(plan).expect("link plan should validate");
        let plan = validated.plan();

        for import in case.imports {
            assert!(
                plan.required_imports().contains(import),
                "{:?} should require import {:?}; imports were {:?}",
                case.runtime_fn,
                import,
                plan.required_imports()
            );
        }
        for capability in case.capabilities {
            assert!(
                plan.required_capabilities().contains(capability),
                "{:?} should require capability {:?}; capabilities were {:?}",
                case.runtime_fn,
                capability,
                plan.required_capabilities()
            );
        }

        let expected_target = if case
            .imports
            .iter()
            .any(|import| matches!(import.spec().abi, HostAbi::NodeShim))
        {
            "wasm32-wasi-p1+node-shim"
        } else {
            "wasm32-wasi-p1"
        };
        assert_eq!(
            plan.manifest_target(),
            expected_target,
            "{:?} manifest target should track host ABI policy",
            case.runtime_fn
        );
    }
}

#[test]
fn runtime_link_plan_capability_reasons_match_capabilities() {
    for case in HOST_IMPORT_RUNTIME_CASES {
        let plan = populated_plan_for(case.runtime_fn);

        for capability in case.capabilities {
            let capability_key = capability.manifest_name();
            let reasons = plan
                .capability_reasons()
                .get(capability_key)
                .unwrap_or_else(|| {
                    panic!(
                        "{:?} should emit a reason for capability {capability_key}",
                        case.runtime_fn
                    )
                });
            let expected_reason = match case.runtime_fn {
                RuntimeFn::DateNow => "Date.now".to_owned(),
                RuntimeFn::DateNewLive => "new Date()".to_owned(),
                runtime_fn => {
                    format!(
                        "required by runtime function: {}",
                        runtime_fn.manifest_name()
                    )
                }
            };
            assert!(
                reasons.iter().any(|reason| reason == &expected_reason),
                "{:?} capability {capability_key} should include reason {expected_reason:?}; got {reasons:?}",
                case.runtime_fn
            );
        }
    }
}

#[test]
fn emitted_link_plan_snapshot_reflects_imports_capabilities_and_target() {
    let plan = populated_plan_for(RuntimeFn::FsAppendFileSync);
    let snapshot: serde_json::Value = serde_json::from_str(&emit_link_plan_snapshot(&plan))
        .expect("link plan snapshot should be valid JSON");

    assert_eq!(
        snapshot["manifest_target"], "wasm32-wasi-p1+node-shim",
        "node-shim imports should change the manifest target"
    );
    assert!(
        snapshot["imports"]
            .as_array()
            .expect("imports should be an array")
            .iter()
            .any(|value| value == "host.fs.appendFileSync"),
        "snapshot should include the host import: {snapshot:#}"
    );
    assert!(
        snapshot["capabilities"]
            .as_array()
            .expect("capabilities should be an array")
            .iter()
            .any(|value| value == "host.fs.appendFileSync"),
        "snapshot should include the matching capability: {snapshot:#}"
    );
}

fn populated_plan_for(runtime_fn: RuntimeFn) -> RuntimeLinkPlan {
    let mut plan = RuntimeLinkPlan::default();
    plan.add_required_runtime(runtime_fn);
    plan.populate_derived_sets();
    plan
}

fn linked_host_imports() -> BTreeSet<HostImport> {
    HOST_IMPORT_RUNTIME_CASES
        .iter()
        .flat_map(|case| case.imports.iter().copied())
        .collect()
}

fn linked_capabilities() -> BTreeSet<Capability> {
    HOST_IMPORT_RUNTIME_CASES
        .iter()
        .flat_map(|case| case.capabilities.iter().copied())
        .collect()
}
