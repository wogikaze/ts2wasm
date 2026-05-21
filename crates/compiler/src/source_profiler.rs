use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

use serde::Serialize;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct ProfileKey {
    name: &'static str,
    file: &'static str,
    line: u32,
    module_path: &'static str,
}

#[derive(Debug, Clone, Default)]
struct ProfileAggregate {
    count: u64,
    total_ns: u128,
    max_ns: u128,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProfileRecord {
    pub name: &'static str,
    pub file: &'static str,
    pub line: u32,
    pub module_path: &'static str,
    pub count: u64,
    pub total_ns: u128,
    pub total_ms: f64,
    pub max_ns: u128,
    pub max_ms: f64,
    pub avg_ns: u128,
    pub avg_ms: f64,
}

#[derive(Debug)]
pub struct Scope {
    key: Option<ProfileKey>,
    started_at: Instant,
}

impl Scope {
    pub fn new(
        name: &'static str,
        file: &'static str,
        line: u32,
        module_path: &'static str,
    ) -> Self {
        let key = if enabled() {
            Some(ProfileKey {
                name,
                file,
                line,
                module_path,
            })
        } else {
            None
        };
        Self {
            key,
            started_at: Instant::now(),
        }
    }
}

impl Drop for Scope {
    fn drop(&mut self) {
        let Some(key) = self.key.take() else {
            return;
        };
        let elapsed = self.started_at.elapsed().as_nanos();
        let mut guard = profile_map().lock().expect("source profiler mutex poisoned");
        let entry = guard.entry(key).or_default();
        entry.count += 1;
        entry.total_ns += elapsed;
        entry.max_ns = entry.max_ns.max(elapsed);
    }
}

pub fn enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| {
        matches!(
            std::env::var("TS2WASM_SOURCE_PROFILE")
                .unwrap_or_default()
                .to_ascii_lowercase()
                .as_str(),
            "1" | "true" | "yes" | "on"
        )
    })
}

pub fn snapshot_and_reset() -> Vec<ProfileRecord> {
    if !enabled() {
        return Vec::new();
    }
    let mut guard = profile_map().lock().expect("source profiler mutex poisoned");
    let mut records: Vec<ProfileRecord> = guard
        .drain()
        .map(|(key, aggregate)| {
            let avg_ns = if aggregate.count == 0 {
                0
            } else {
                aggregate.total_ns / u128::from(aggregate.count)
            };
            ProfileRecord {
                name: key.name,
                file: key.file,
                line: key.line,
                module_path: key.module_path,
                count: aggregate.count,
                total_ns: aggregate.total_ns,
                total_ms: ns_to_ms(aggregate.total_ns),
                max_ns: aggregate.max_ns,
                max_ms: ns_to_ms(aggregate.max_ns),
                avg_ns,
                avg_ms: ns_to_ms(avg_ns),
            }
        })
        .collect();
    records.sort_by(|left, right| {
        right
            .total_ns
            .cmp(&left.total_ns)
            .then_with(|| left.name.cmp(right.name))
            .then_with(|| left.file.cmp(right.file))
            .then_with(|| left.line.cmp(&right.line))
    });
    records
}

fn ns_to_ms(ns: u128) -> f64 {
    ns as f64 / 1_000_000.0
}

fn profile_map() -> &'static Mutex<HashMap<ProfileKey, ProfileAggregate>> {
    static PROFILE_MAP: OnceLock<Mutex<HashMap<ProfileKey, ProfileAggregate>>> = OnceLock::new();
    PROFILE_MAP.get_or_init(|| Mutex::new(HashMap::new()))
}

#[macro_export]
macro_rules! source_profile_scope {
    ($name:literal) => {
        #[cfg(feature = "source-profiler")]
        let _ts2wasm_source_profile_scope = $crate::source_profiler::Scope::new(
            $name,
            file!(),
            line!(),
            module_path!(),
        );
    };
}
