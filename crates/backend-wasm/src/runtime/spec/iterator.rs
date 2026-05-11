// Domain: Iterator -- auto-generated.
// Regenerate with: python3 scripts/refactor/extract-runtime-domains.py

            Self::GetIterator => RuntimeSpec {
                symbol: "$get_iterator",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::EffectOnly,
            },
            Self::IteratorNext => RuntimeSpec {
                symbol: "$iterator_next",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::EffectOnly,
            },
