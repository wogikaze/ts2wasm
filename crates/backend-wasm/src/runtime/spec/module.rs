// Domain: Module -- auto-generated.
// Regenerate with: python3 scripts/refactor/extract-runtime-domains.py

            Self::ModuleRequire => RuntimeSpec {
                symbol: "$module_require",
                deps: &[Self::AllocHeap],
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::ModuleExportsSet => RuntimeSpec {
                symbol: "$module_exports_set",
                deps: &[Self::AllocHeap, Self::PropertySet],
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::EffectOnly,
            },
            Self::ModuleExportsAssign => RuntimeSpec {
                symbol: "$module_exports_assign",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::EffectOnly,
            },
