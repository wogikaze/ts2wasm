// Domain: Symbol -- auto-generated.
// Regenerate with: python3 scripts/refactor/extract-runtime-domains.py

            Self::SymbolNew => RuntimeSpec {
                symbol: "$symbol_new",
                deps: SYMBOL_NEW_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: SYMBOL_NEW_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::SymbolFor => RuntimeSpec {
                symbol: "$symbol_for",
                deps: SYMBOL_FOR_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: SYMBOL_FOR_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::SymbolKeyFor => RuntimeSpec {
                symbol: "$symbol_key_for",
                deps: SYMBOL_KEY_FOR_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
