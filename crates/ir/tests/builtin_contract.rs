//! Builtin contract tests — verify BuiltinId and BuiltinPropertyId contracts.
//!
//! These tests verify that every BuiltinId variant defines consistent arity
//! values and naming conventions, and that BuiltinPropertyId behaves correctly.
//! This provides early detection of regressions when new builtins are added.

use std::collections::HashSet;
use ts2wasm_ir::builtin::{BuiltinId, BuiltinPropertyId, BuiltinResult};

/// Collect all BuiltinId variants by iterating a sample list.
/// This list must be kept in sync with the enum definition.
fn all_builtin_ids() -> Vec<BuiltinId> {
    vec![
        BuiltinId::ConsoleLog,
        BuiltinId::ReadStdinUtf8,
        BuiltinId::FsReadFileSync,
        BuiltinId::FsWriteFileSync,
        BuiltinId::FsAppendFileSync,
        BuiltinId::ProcessArgv,
        BuiltinId::ProcessEnv,
        BuiltinId::ProcessExit,
        BuiltinId::PathJoin,
        BuiltinId::PathResolve,
        BuiltinId::PathBasename,
        BuiltinId::PathDirname,
        BuiltinId::CryptoRandomBytes,
        BuiltinId::InstanceOf,
        BuiltinId::MathPow,
        BuiltinId::IsNaN,
        BuiltinId::ParseInt,
        BuiltinId::ParseFloat,
        BuiltinId::IsFinite,
        BuiltinId::BooleanCoerce,
        BuiltinId::NumberCoerce,
        BuiltinId::EncodeURI,
        BuiltinId::DecodeURI,
        BuiltinId::Escape,
        BuiltinId::Unescape,
    ]
}

#[test]
fn builtin_contract_all_ids_have_expected_arity() {
    // Every variant's expected_arity must be >= min_arity
    for id in all_builtin_ids() {
        let expected = id.expected_arity();
        let min = id.min_arity();
        assert!(
            expected >= min,
            "BuiltinId::{id:?}: expected_arity({expected}) < min_arity({min})"
        );
        assert!(
            expected <= 3,
            "BuiltinId::{id:?}: expected_arity({expected}) > 3 (unusually high)"
        );
    }
}

#[test]
fn builtin_contract_all_ids_have_valid_min_arity() {
    for id in all_builtin_ids() {
        let min = id.min_arity();
        assert!(
            min <= 3,
            "BuiltinId::{id:?}: min_arity({min}) > 3 (unusually high)"
        );
    }
}

#[test]
fn builtin_contract_console_log_has_correct_arity() {
    assert_eq!(BuiltinId::ConsoleLog.expected_arity(), 1);
    assert_eq!(BuiltinId::ConsoleLog.min_arity(), 1);
}

#[test]
fn builtin_contract_instance_of_has_two_args() {
    assert_eq!(BuiltinId::InstanceOf.expected_arity(), 2);
}

#[test]
fn builtin_contract_math_pow_has_two_args() {
    assert_eq!(BuiltinId::MathPow.expected_arity(), 2);
}

#[test]
fn builtin_contract_read_stdin_has_zero_args() {
    assert_eq!(BuiltinId::ReadStdinUtf8.expected_arity(), 0);
    assert_eq!(BuiltinId::ReadStdinUtf8.min_arity(), 0);
}

#[test]
fn builtin_contract_process_exit_has_one_arg() {
    assert_eq!(BuiltinId::ProcessExit.expected_arity(), 1);
}

#[test]
fn builtin_contract_path_join_has_two_args() {
    assert_eq!(BuiltinId::PathJoin.expected_arity(), 2);
}

#[test]
fn builtin_contract_builtin_result_values() {
    // Verify that BuiltinResult has both expected variants
    match BuiltinResult::Value {
        BuiltinResult::Value => {}
        _ => panic!("unexpected BuiltinResult variant"),
    }
    match BuiltinResult::EffectOnly {
        BuiltinResult::EffectOnly => {}
        _ => panic!("unexpected BuiltinResult variant"),
    }
}

#[test]
fn builtin_contract_builtin_property_id_values() {
    // Verify that BuiltinPropertyId has at least Length
    match BuiltinPropertyId::Length {
        BuiltinPropertyId::Length => {}
    }
}

#[test]
fn builtin_contract_all_ids_have_unique_debug_names() {
    let ids = all_builtin_ids();
    let mut names = HashSet::new();
    for id in &ids {
        let name = format!("{id:?}");
        assert!(names.insert(name.clone()), "Duplicate Debug name: {name}");
    }
    // Verify we found all unique names
    assert_eq!(names.len(), ids.len());
}

#[test]
fn builtin_contract_console_log_specific_checks() {
    let id = BuiltinId::ConsoleLog;
    assert_eq!(id.expected_arity(), 1);
    assert_eq!(id.min_arity(), 1);
    let debug = format!("{id:?}");
    assert_eq!(debug, "ConsoleLog");
}

#[test]
fn builtin_contract_fs_read_file_sync_has_two_args() {
    assert_eq!(BuiltinId::FsReadFileSync.expected_arity(), 2);
}

#[test]
fn builtin_contract_fs_write_file_sync_has_two_args() {
    assert_eq!(BuiltinId::FsWriteFileSync.expected_arity(), 2);
}

#[test]
fn builtin_contract_parse_int_two_args() {
    assert_eq!(BuiltinId::ParseInt.expected_arity(), 2);
}

#[test]
fn builtin_contract_coercion_ids_have_one_arg() {
    assert_eq!(BuiltinId::BooleanCoerce.expected_arity(), 1);
    assert_eq!(BuiltinId::NumberCoerce.expected_arity(), 1);
}

#[test]
fn builtin_contract_uri_functions_have_one_arg() {
    assert_eq!(BuiltinId::EncodeURI.expected_arity(), 1);
    assert_eq!(BuiltinId::DecodeURI.expected_arity(), 1);
    assert_eq!(BuiltinId::Escape.expected_arity(), 1);
    assert_eq!(BuiltinId::Unescape.expected_arity(), 1);
}

#[test]
fn builtin_contract_crypto_bytes_has_one_arg() {
    assert_eq!(BuiltinId::CryptoRandomBytes.expected_arity(), 1);
}

#[test]
fn builtin_contract_unary_builtins_have_one_arg() {
    let unary_ids = vec![
        BuiltinId::IsNaN,
        BuiltinId::IsFinite,
        BuiltinId::ParseFloat,
        BuiltinId::PathResolve,
        BuiltinId::PathBasename,
        BuiltinId::PathDirname,
    ];
    for id in unary_ids {
        assert_eq!(id.expected_arity(), 1, "BuiltinId::{id:?} expected arity 1");
    }
}
