// Domain: Task -- auto-generated.
// Regenerate with: python3 scripts/refactor/extract-runtime-domains.py

            Self::TaskPoll => RuntimeSpec {
                symbol: "$task_poll",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::TaskResult => RuntimeSpec {
                symbol: "$task_result",
                deps: NO_DEPS,
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::Value,
            },
            Self::TaskDrop => RuntimeSpec {
                symbol: "$task_drop",
                deps: &[Self::AllocHeap],
                imports: NO_IMPORTS,
                capability: NO_CAPS,
                runtime_strings: NO_RUNTIME_STRINGS,
                result: RuntimeResult::EffectOnly,
            },
