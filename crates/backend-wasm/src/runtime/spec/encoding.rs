// Domain: Encoding -- auto-generated.
// Regenerate with: python3 scripts/refactor/extract-runtime-domains.py

            Self::EncodeURI => RuntimeSpec {
                symbol: "$encode_uri",
                deps: NO_DEPS,
                imports: IMPORT_ENCODE_URI,
                capability: CAP_HOST_ENCODE_URI,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::DecodeURI => RuntimeSpec {
                symbol: "$decode_uri",
                deps: NO_DEPS,
                imports: IMPORT_DECODE_URI,
                capability: CAP_HOST_DECODE_URI,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::Escape => RuntimeSpec {
                symbol: "$escape",
                deps: NO_DEPS,
                imports: IMPORT_ESCAPE,
                capability: CAP_HOST_ESCAPE,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::Unescape => RuntimeSpec {
                symbol: "$unescape",
                deps: NO_DEPS,
                imports: IMPORT_UNESCAPE,
                capability: CAP_HOST_UNESCAPE,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
