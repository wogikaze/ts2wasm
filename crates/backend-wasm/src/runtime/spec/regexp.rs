// Domain: RegExp -- auto-generated.
// Regenerate with: python3 scripts/refactor/extract-runtime-domains.py

            Self::RegExpTest => RuntimeSpec {
                symbol: "$regexp_test",
                deps: REGEXP_TEST_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::RegExpMatch => RuntimeSpec {
                symbol: "$regexp_match",
                deps: REGEXP_MATCH_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::RegExpSearch => RuntimeSpec {
                symbol: "$regexp_search",
                deps: REGEXP_SEARCH_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::RegexpMatchInner => RuntimeSpec {
                symbol: "$regexp_match_inner",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
