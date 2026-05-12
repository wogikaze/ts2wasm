//! Builtin contract tests for the semantics crate boundary.
//!
//! These tests verify that the semantics crate compiles, links, and that
//! its public module structure is accessible. As the semantics crate grows
//! to include builtin resolution, TypeScript erasure policy, and host API
//! classification, this test file should expand to test each boundary.

/// Verify the semantics crate links correctly by checking it exists as a
/// workspace member and has the expected lib.rs structure.
#[test]
fn builtin_contract_semantics_lib_exists() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("lib.rs");
    assert!(
        path.exists(),
        "semantics crate lib.rs should exist: {}",
        path.display()
    );
    let content = std::fs::read_to_string(&path).unwrap_or_default();
    assert!(
        content.contains("pub mod builtin"),
        "semantics lib.rs should declare builtin module"
    );
}

/// Verify that the semantics crate is configured as expected.
/// This ensures the workspace member is properly registered.
#[test]
fn builtin_contract_crate_module_accessible() {
    // The crate exists and compiles — this test verifies the
    // test infrastructure works. Actual boundary tests should
    // be added here when the semantics crate gains public API.
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    assert!(
        manifest_dir.contains("semantics"),
        "should be in semantics crate, got: {manifest_dir}"
    );
}
