// Domain: Json -- auto-generated.
// Regenerate with: python3 scripts/refactor/extract-runtime-domains.py

            Self::JsonStringify => RuntimeSpec {
                symbol: "$json_stringify",
                deps: JSON_STRINGIFY_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: JSON_STRINGIFY_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::JsonParse => RuntimeSpec {
                symbol: "$json_parse",
                deps: JSON_PARSE_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: JSON_PARSE_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
