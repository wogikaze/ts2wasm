use std::collections::BTreeSet;

use ts2wasm_runtime_catalog::{RuntimeFn, RuntimeSpec};

const RUNTIME_FN_SOURCE: &str = include_str!("../src/runtime_fn.rs");
const RUNTIME_SPEC_SOURCE: &str = include_str!("../src/runtime/spec/all.rs");

#[test]
fn runtime_fn_all_matches_enum_inventory() {
    let enum_variants = runtime_fn_enum_variants();
    let all_variants = runtime_fn_names(RuntimeFn::all());

    assert_unique("RuntimeFn::all", &all_variants);
    assert_sets_match(
        "RuntimeFn::all",
        &all_variants.iter().cloned().collect(),
        "RuntimeFn enum",
        &enum_variants,
    );
}

#[test]
fn emission_order_is_unique_and_complete_for_enum_inventory() {
    let enum_variants = runtime_fn_enum_variants();
    let emission_order = runtime_fn_names(RuntimeFn::emission_order());

    assert_unique("RuntimeFn::emission_order", &emission_order);
    assert_sets_match(
        "RuntimeFn::emission_order",
        &emission_order.iter().cloned().collect(),
        "RuntimeFn enum",
        &enum_variants,
    );
}

#[test]
fn emission_order_contains_every_declared_dependency() {
    let emission_order: BTreeSet<RuntimeFn> = RuntimeFn::emission_order().iter().copied().collect();
    let mut missing = Vec::new();

    for runtime_fn in RuntimeFn::all() {
        for dep in runtime_fn.spec().deps {
            if !emission_order.contains(dep) {
                missing.push(format!(
                    "RuntimeFn::{runtime_fn:?} depends on RuntimeFn::{dep:?}"
                ));
            }
        }
    }

    assert!(
        missing.is_empty(),
        "RuntimeFn dependencies missing from emission_order:\n  {}",
        missing.join("\n  ")
    );
}

#[test]
fn every_runtime_fn_has_explicit_runtime_spec_arm() {
    let enum_variants = runtime_fn_enum_variants();
    let spec_variants = explicit_runtime_spec_variants();

    assert_sets_match(
        "RuntimeSpec arms",
        &spec_variants,
        "RuntimeFn enum",
        &enum_variants,
    );

    for runtime_fn in RuntimeFn::all() {
        let spec: RuntimeSpec = runtime_fn.spec();
        assert!(
            !spec.symbol.is_empty(),
            "RuntimeFn::{runtime_fn:?} must declare a non-empty RuntimeSpec symbol"
        );
        assert!(
            spec.symbol.starts_with('$'),
            "RuntimeFn::{runtime_fn:?} RuntimeSpec symbol must be a wasm function symbol"
        );
    }
}

#[test]
fn runtime_fn_all_remains_an_explicit_inventory() {
    let all_body = function_body(RUNTIME_FN_SOURCE, "pub const fn all()");
    assert!(
        !all_body.contains("emission_order()"),
        "RuntimeFn::all must stay independent from emission_order()"
    );
    assert!(
        !all_body.contains("Self::emission_order"),
        "RuntimeFn::all must stay independent from emission_order()"
    );
    assert!(
        all_body.contains("Self::ReadStdinBytes"),
        "RuntimeFn::all should be an explicit Self:: variant inventory"
    );
}

fn runtime_fn_names(runtime_fns: &[RuntimeFn]) -> Vec<String> {
    runtime_fns
        .iter()
        .map(|runtime_fn| format!("{runtime_fn:?}"))
        .collect()
}

fn runtime_fn_enum_variants() -> BTreeSet<String> {
    let body = item_body(RUNTIME_FN_SOURCE, "pub enum RuntimeFn {");
    body.lines()
        .filter_map(|line| {
            let line = line.split("//").next().unwrap_or("").trim();
            if line.is_empty() || line.starts_with("///") || line.starts_with("#[") {
                return None;
            }
            let variant = line.strip_suffix(',')?;
            variant
                .chars()
                .next()
                .filter(|ch| ch.is_ascii_uppercase())
                .map(|_| variant.to_owned())
        })
        .collect()
}

fn explicit_runtime_spec_variants() -> BTreeSet<String> {
    RUNTIME_SPEC_SOURCE
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            let after_self = line.strip_prefix("Self::")?;
            let (variant, _) = after_self.split_once(" => RuntimeSpec")?;
            Some(variant.to_owned())
        })
        .collect()
}

fn assert_unique(label: &str, variants: &[String]) {
    let mut seen = BTreeSet::new();
    let mut duplicates = Vec::new();

    for variant in variants {
        if !seen.insert(variant.clone()) {
            duplicates.push(format!("RuntimeFn::{variant}"));
        }
    }

    assert!(
        duplicates.is_empty(),
        "{label} contains duplicate variants:\n  {}",
        duplicates.join("\n  ")
    );
}

fn assert_sets_match(
    left_label: &str,
    left: &BTreeSet<String>,
    right_label: &str,
    right: &BTreeSet<String>,
) {
    let missing: Vec<String> = right
        .difference(left)
        .map(|variant| format!("RuntimeFn::{variant}"))
        .collect();
    let extra: Vec<String> = left
        .difference(right)
        .map(|variant| format!("RuntimeFn::{variant}"))
        .collect();

    assert!(
        missing.is_empty() && extra.is_empty(),
        "{left_label} does not match {right_label}\nmissing from {left_label}:\n  {}\nextra in {left_label}:\n  {}",
        missing.join("\n  "),
        extra.join("\n  ")
    );
}

fn function_body<'a>(source: &'a str, signature: &str) -> &'a str {
    let function = source
        .split_once(signature)
        .unwrap_or_else(|| panic!("missing function signature: {signature}"))
        .1;
    item_body(function, "{")
}

fn item_body<'a>(source: &'a str, opener: &str) -> &'a str {
    let after_opener = source
        .split_once(opener)
        .unwrap_or_else(|| panic!("missing source opener: {opener}"))
        .1;
    let mut depth = 1usize;

    for (index, ch) in after_opener.char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return &after_opener[..index];
                }
            }
            _ => {}
        }
    }

    panic!("missing closing brace for source opener: {opener}");
}
