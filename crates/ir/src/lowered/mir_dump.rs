//! MIR dump utility — pretty-prints MIR programs for debugging and snapshots.
//!
//! This module provides `mir_dump` functions that render `MirExpr`, `MirStmt`,
//! `MirFunction`, and `MirProgram` as human-readable strings. This is used for:
//!
//! - Debugging during development
//! - Snapshot testing (serializing MIR to compare against golden files)
//! - Architecture coverage checks (verifying MIR structure)

use crate::lowered::RuntimeIntrinsic;
use crate::lowered::mir::{MirExpr, MirFunction, MirProgram, MirStmt};

/// Dump a `MirProgram` to a string.
pub fn dump_mir_program(program: &MirProgram, label: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!(";; MIR Program: {}\n", label));
    out.push_str(&format!(
        ";; Top-level locals: {:?}\n",
        program.top_level_locals
    ));
    out.push_str(";; Functions:\n");
    for func in &program.functions {
        out.push_str(&dump_mir_function(func));
    }
    out.push_str(";; Top-level statements:\n");
    for stmt in &program.top_level_statements {
        dump_mir_stmt(stmt, &mut out, 1);
    }
    out
}

/// Dump a `MirFunction` to a string.
pub fn dump_mir_function(func: &MirFunction) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "  (func ${} (params {:?}) (locals {:?}) (recursion {})\n",
        func.id.0, func.params, func.locals, func.recursion_depth,
    ));
    for stmt in &func.body {
        dump_mir_stmt(stmt, &mut out, 2);
    }
    out.push_str("  )\n");
    out
}

/// Dump a `MirStmt` to a string with indentation.
pub fn dump_mir_stmt(stmt: &MirStmt, out: &mut String, indent: usize) {
    let pad = "  ".repeat(indent);
    match stmt {
        MirStmt::Let { local, init } => {
            out.push_str(&format!("{};; let ${}\n", pad, local.0));
            dump_mir_expr(init, out, indent + 1);
        }
        MirStmt::Assign { local, init } => {
            out.push_str(&format!("{};; ${} =\n", pad, local.0));
            dump_mir_expr(init, out, indent + 1);
        }
        MirStmt::Expr(expr) => {
            dump_mir_expr(expr, out, indent);
        }
        MirStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            out.push_str(&format!("{};; if\n", pad));
            dump_mir_expr(condition, out, indent + 1);
            out.push_str(&format!("{};; then\n", pad));
            for s in then_body {
                dump_mir_stmt(s, out, indent + 1);
            }
            if !else_body.is_empty() {
                out.push_str(&format!("{};; else\n", pad));
                for s in else_body {
                    dump_mir_stmt(s, out, indent + 1);
                }
            }
        }
        MirStmt::While { condition, body } => {
            out.push_str(&format!("{};; while\n", pad));
            dump_mir_expr(condition, out, indent + 1);
            out.push_str(&format!("{};; do\n", pad));
            for s in body {
                dump_mir_stmt(s, out, indent + 1);
            }
        }
        MirStmt::Return(expr) => {
            out.push_str(&format!("{};; return\n", pad));
            dump_mir_expr(expr, out, indent + 1);
        }
        MirStmt::Throw(expr) => {
            out.push_str(&format!("{};; throw\n", pad));
            dump_mir_expr(expr, out, indent + 1);
        }
        MirStmt::TryCatch {
            try_body,
            catch_var,
            catch_body,
            finally_body,
        } => {
            out.push_str(&format!("{};; try\n", pad));
            for s in try_body {
                dump_mir_stmt(s, out, indent + 1);
            }
            if let Some(cv) = catch_var {
                out.push_str(&format!("{};; catch ${}\n", pad, cv.0));
            }
            if let Some(body) = catch_body {
                for s in body {
                    dump_mir_stmt(s, out, indent + 1);
                }
            }
            if let Some(body) = finally_body {
                out.push_str(&format!("{};; finally\n", pad));
                for s in body {
                    dump_mir_stmt(s, out, indent + 1);
                }
            }
        }
        MirStmt::Switch { expr, cases } => {
            out.push_str(&format!("{};; switch\n", pad));
            dump_mir_expr(expr, out, indent + 1);
            for (opt_key, body) in cases {
                match opt_key {
                    Some(key) => {
                        out.push_str(&format!("{};; case:\n", pad));
                        dump_mir_expr(key, out, indent + 1);
                    }
                    None => {
                        out.push_str(&format!("{};; default:\n", pad));
                    }
                }
                for s in body {
                    dump_mir_stmt(s, out, indent + 1);
                }
            }
        }
        MirStmt::Labeled { label, body } => {
            out.push_str(&format!("{};; labeled \"{}\"\n", pad, label));
            dump_mir_stmt(body, out, indent + 1);
        }
        MirStmt::Break { label } => {
            out.push_str(&format!("{};; break {:?}\n", pad, label));
        }
        MirStmt::Continue { label } => {
            out.push_str(&format!("{};; continue {:?}\n", pad, label));
        }
        MirStmt::ClassDecl {
            name,
            extends,
            constructor,
            methods,
            static_methods,
            private_fields,
        } => {
            out.push_str(&format!("{};; class {}\n", pad, name));
            if let Some(parent) = extends {
                out.push_str(&format!("{};;   extends {}\n", pad, parent));
            }
            if let Some(cid) = constructor {
                out.push_str(&format!("{};;   constructor func${}\n", pad, cid.0));
            }
            for (mname, mid) in methods {
                out.push_str(&format!("{};;   method {} func${}\n", pad, mname, mid.0));
            }
            for (mname, mid) in static_methods {
                out.push_str(&format!(
                    "{};;   static_method {} func${}\n",
                    pad, mname, mid.0
                ));
            }
            if !private_fields.is_empty() {
                out.push_str(&format!(
                    "{};;   private_fields {:?}\n",
                    pad, private_fields
                ));
            }
        }
        MirStmt::Export { name, expr } => {
            out.push_str(&format!("{};; export {}\n", pad, name));
            dump_mir_expr(expr, out, indent + 1);
        }
        MirStmt::ModuleExportsAssign { expr } => {
            out.push_str(&format!("{};; module.exports =\n", pad));
            dump_mir_expr(expr, out, indent + 1);
        }
    }
}

/// Dump a `MirExpr` to a string with indentation.
pub fn dump_mir_expr(expr: &MirExpr, out: &mut String, indent: usize) {
    let pad = "  ".repeat(indent);
    match expr {
        MirExpr::I32Const(n) => {
            out.push_str(&format!("{}i32.const {}\n", pad, n));
        }
        MirExpr::StringConst(s) => {
            out.push_str(&format!("{}\"{}\"\n", pad, s.escape_default()));
        }
        MirExpr::Local(local) => {
            out.push_str(&format!("{}local.get ${}\n", pad, local.0));
        }
        MirExpr::CallRuntime { intrinsic, args } => {
            out.push_str(&format!(
                "{}(call_runtime {:?}\n",
                pad,
                runtime_intrinsic_name(*intrinsic)
            ));
            for arg in args {
                dump_mir_expr(arg, out, indent + 1);
            }
            out.push_str(&format!("{})\n", pad));
        }
        MirExpr::CallFunction { func, args } => {
            out.push_str(&format!("{}(call_func ${}\n", pad, func.0));
            for arg in args {
                dump_mir_expr(arg, out, indent + 1);
            }
            out.push_str(&format!("{})\n", pad));
        }
        MirExpr::CallClosure { closure, args } => {
            out.push_str(&format!("{}(call_closure\n", pad));
            dump_mir_expr(closure, out, indent + 1);
            for arg in args {
                dump_mir_expr(arg, out, indent + 1);
            }
            out.push_str(&format!("{})\n", pad));
        }
        MirExpr::NewObject { props } => {
            out.push_str(&format!("{}(new_object\n", pad));
            for (k, v) in props {
                out.push_str(&format!("{}  \"{}\" ->\n", pad, k));
                dump_mir_expr(v, out, indent + 1);
            }
            out.push_str(&format!("{})\n", pad));
        }
        MirExpr::NewArray { elements } => {
            out.push_str(&format!("{}(new_array\n", pad));
            for elem in elements {
                dump_mir_expr(elem, out, indent + 1);
            }
            out.push_str(&format!("{})\n", pad));
        }
        MirExpr::LoadModule { module_id } => {
            out.push_str(&format!("{}(load_module {})\n", pad, module_id));
        }
        MirExpr::Block { stmts, result } => {
            out.push_str(&format!("{}(block\n", pad));
            for s in stmts {
                dump_mir_stmt(s, out, indent + 1);
            }
            out.push_str(&format!("{}  result:\n", pad));
            dump_mir_expr(result, out, indent + 1);
            out.push_str(&format!("{})\n", pad));
        }
    }
}

/// Get a debug name for a RuntimeIntrinsic variant.
fn runtime_intrinsic_name(intrinsic: RuntimeIntrinsic) -> &'static str {
    // For the dump, we use intrinsic's Debug representation
    // This avoids needing a `name()` method on RuntimeIntrinsic when
    // it's re-exported from runtime-catalog.
    match intrinsic {
        // Core
        _ if intrinsic == RuntimeIntrinsic::ReadStdinBytes => "ReadStdinBytes",
        _ if intrinsic == RuntimeIntrinsic::Write => "Write",
        _ if intrinsic == RuntimeIntrinsic::Copy => "Copy",
        _ if intrinsic == RuntimeIntrinsic::ValueToStringInto => "ValueToStringInto",
        _ if intrinsic == RuntimeIntrinsic::ErrorMessage => "ErrorMessage",
        _ if intrinsic == RuntimeIntrinsic::Log => "Log",
        _ if intrinsic == RuntimeIntrinsic::TruthyBool => "TruthyBool",
        _ if intrinsic == RuntimeIntrinsic::Not => "Not",
        _ if intrinsic == RuntimeIntrinsic::TypeOf => "TypeOf",
        _ if intrinsic == RuntimeIntrinsic::AllocHeap => "AllocHeap",
        _ if intrinsic == RuntimeIntrinsic::MemEqual => "MemEqual",
        _ if intrinsic == RuntimeIntrinsic::Index => "Index",
        _ if intrinsic == RuntimeIntrinsic::GetLength => "GetLength",
        _ if intrinsic == RuntimeIntrinsic::PropertyGet => "PropertyGet",
        _ if intrinsic == RuntimeIntrinsic::PropertySet => "PropertySet",
        _ if intrinsic == RuntimeIntrinsic::PropertyDelete => "PropertyDelete",
        _ if intrinsic == RuntimeIntrinsic::PropertyHas => "PropertyHas",
        _ if intrinsic == RuntimeIntrinsic::PrivateBrandTypeError => "PrivateBrandTypeError",
        // Number
        _ if intrinsic == RuntimeIntrinsic::NumberFromI32 => "NumberFromI32",
        _ if intrinsic == RuntimeIntrinsic::NumberToI32 => "NumberToI32",
        _ if intrinsic == RuntimeIntrinsic::NumberToExponential => "NumberToExponential",
        _ if intrinsic == RuntimeIntrinsic::NumberToFixed => "NumberToFixed",
        _ if intrinsic == RuntimeIntrinsic::NumberToPrecision => "NumberToPrecision",
        _ if intrinsic == RuntimeIntrinsic::NumberIsNaN => "NumberIsNaN",
        _ if intrinsic == RuntimeIntrinsic::NumberIsFinite => "NumberIsFinite",
        _ if intrinsic == RuntimeIntrinsic::NumberIsInteger => "NumberIsInteger",
        _ if intrinsic == RuntimeIntrinsic::NumberIsSafeInteger => "NumberIsSafeInteger",
        // BigInt
        _ if intrinsic == RuntimeIntrinsic::MakeBigIntLiteral => "MakeBigIntLiteral",
        _ if intrinsic == RuntimeIntrinsic::BigIntToString => "BigIntToString",
        _ if intrinsic == RuntimeIntrinsic::BigIntToBoolean => "BigIntToBoolean",
        _ if intrinsic == RuntimeIntrinsic::BigIntFromValue => "BigIntFromValue",
        _ if intrinsic == RuntimeIntrinsic::BigIntAsIntN => "BigIntAsIntN",
        _ if intrinsic == RuntimeIntrinsic::BigIntAsUintN => "BigIntAsUintN",
        _ if intrinsic == RuntimeIntrinsic::BigIntUnaryMinus => "BigIntUnaryMinus",
        _ if intrinsic == RuntimeIntrinsic::BigIntAdd => "BigIntAdd",
        _ if intrinsic == RuntimeIntrinsic::BigIntSub => "BigIntSub",
        _ if intrinsic == RuntimeIntrinsic::BigIntMul => "BigIntMul",
        _ if intrinsic == RuntimeIntrinsic::BigIntPow => "BigIntPow",
        _ if intrinsic == RuntimeIntrinsic::BigIntDiv => "BigIntDiv",
        _ if intrinsic == RuntimeIntrinsic::BigIntRem => "BigIntRem",
        _ if intrinsic == RuntimeIntrinsic::BigIntDivisionByZeroRangeError => {
            "BigIntDivisionByZeroRangeError"
        }
        _ if intrinsic == RuntimeIntrinsic::BigIntMixedArithmeticTypeError => {
            "BigIntMixedArithmeticTypeError"
        }
        _ if intrinsic == RuntimeIntrinsic::BigIntStringComparisonBoundaryError => {
            "BigIntStringComparisonBoundaryError"
        }
        _ if intrinsic == RuntimeIntrinsic::BigIntBitwiseNot => "BigIntBitwiseNot",
        _ if intrinsic == RuntimeIntrinsic::BigIntBitwiseAnd => "BigIntBitwiseAnd",
        _ if intrinsic == RuntimeIntrinsic::BigIntBitwiseOr => "BigIntBitwiseOr",
        _ if intrinsic == RuntimeIntrinsic::BigIntBitwiseXor => "BigIntBitwiseXor",
        _ if intrinsic == RuntimeIntrinsic::BigIntLeftShift => "BigIntLeftShift",
        _ if intrinsic == RuntimeIntrinsic::BigIntRightShift => "BigIntRightShift",
        _ if intrinsic == RuntimeIntrinsic::BigIntCompare => "BigIntCompare",
        // String
        _ if intrinsic == RuntimeIntrinsic::StringEqual => "StringEqual",
        _ if intrinsic == RuntimeIntrinsic::Concat => "Concat",
        _ if intrinsic == RuntimeIntrinsic::IsString => "IsString",
        // Arithmetic operators
        _ if intrinsic == RuntimeIntrinsic::Add => "Add",
        _ if intrinsic == RuntimeIntrinsic::AddFast => "AddFast",
        _ if intrinsic == RuntimeIntrinsic::Sub => "Sub",
        _ if intrinsic == RuntimeIntrinsic::SubFast => "SubFast",
        _ if intrinsic == RuntimeIntrinsic::Mul => "Mul",
        _ if intrinsic == RuntimeIntrinsic::MulFast => "MulFast",
        _ if intrinsic == RuntimeIntrinsic::Div => "Div",
        _ if intrinsic == RuntimeIntrinsic::DivFast => "DivFast",
        _ if intrinsic == RuntimeIntrinsic::Mod => "Mod",
        _ if intrinsic == RuntimeIntrinsic::ModFast => "ModFast",
        _ if intrinsic == RuntimeIntrinsic::BitwiseToI32 => "BitwiseToI32",
        _ if intrinsic == RuntimeIntrinsic::BitwiseAnd => "BitwiseAnd",
        _ if intrinsic == RuntimeIntrinsic::BitwiseXor => "BitwiseXor",
        _ if intrinsic == RuntimeIntrinsic::BitwiseOr => "BitwiseOr",
        _ if intrinsic == RuntimeIntrinsic::Negate => "Negate",
        _ if intrinsic == RuntimeIntrinsic::Less => "Less",
        _ if intrinsic == RuntimeIntrinsic::LessFast => "LessFast",
        _ if intrinsic == RuntimeIntrinsic::LessEqual => "LessEqual",
        _ if intrinsic == RuntimeIntrinsic::LessEqualFast => "LessEqualFast",
        _ if intrinsic == RuntimeIntrinsic::Greater => "Greater",
        _ if intrinsic == RuntimeIntrinsic::GreaterFast => "GreaterFast",
        _ if intrinsic == RuntimeIntrinsic::GreaterEqual => "GreaterEqual",
        _ if intrinsic == RuntimeIntrinsic::GreaterEqualFast => "GreaterEqualFast",
        _ if intrinsic == RuntimeIntrinsic::StrictEqual => "StrictEqual",
        _ if intrinsic == RuntimeIntrinsic::EqualEqual => "EqualEqual",
        _ if intrinsic == RuntimeIntrinsic::BangEqual => "BangEqual",
        _ if intrinsic == RuntimeIntrinsic::StrictNotEqual => "StrictNotEqual",
        _ if intrinsic == RuntimeIntrinsic::And => "And",
        _ if intrinsic == RuntimeIntrinsic::Or => "Or",
        // Array
        _ if intrinsic == RuntimeIntrinsic::ArrayGet => "ArrayGet",
        _ if intrinsic == RuntimeIntrinsic::ArrayIndexPresent => "ArrayIndexPresent",
        _ if intrinsic == RuntimeIntrinsic::ArrayBufferNew => "ArrayBufferNew",
        _ if intrinsic == RuntimeIntrinsic::ArrayPush => "ArrayPush",
        _ if intrinsic == RuntimeIntrinsic::ArrayPushGrow => "ArrayPushGrow",
        _ if intrinsic == RuntimeIntrinsic::ArrayPop => "ArrayPop",
        _ if intrinsic == RuntimeIntrinsic::ArraySlice => "ArraySlice",
        _ if intrinsic == RuntimeIntrinsic::ArrayConcat => "ArrayConcat",
        _ if intrinsic == RuntimeIntrinsic::ArrayMapValueToString => "ArrayMapValueToString",
        _ if intrinsic == RuntimeIntrinsic::ArrayMapUnaryPlus => "ArrayMapUnaryPlus",
        _ if intrinsic == RuntimeIntrinsic::ArrayMapStringSplit => "ArrayMapStringSplit",
        _ if intrinsic == RuntimeIntrinsic::ArrayMapArrayLikeIdentity => {
            "ArrayMapArrayLikeIdentity"
        }
        _ if intrinsic == RuntimeIntrinsic::ArrayMapArrayLikeDouble => "ArrayMapArrayLikeDouble",
        _ if intrinsic == RuntimeIntrinsic::ArraySortNumeric => "ArraySortNumeric",
        _ if intrinsic == RuntimeIntrinsic::ArrayJoin => "ArrayJoin",
        _ if intrinsic == RuntimeIntrinsic::ArrayReverse => "ArrayReverse",
        _ if intrinsic == RuntimeIntrinsic::ArrayIndexOf => "ArrayIndexOf",
        _ if intrinsic == RuntimeIntrinsic::ArrayIncludes => "ArrayIncludes",
        _ if intrinsic == RuntimeIntrinsic::ArrayFind => "ArrayFind",
        _ if intrinsic == RuntimeIntrinsic::ArrayFindIndex => "ArrayFindIndex",
        _ if intrinsic == RuntimeIntrinsic::ArrayFindLast => "ArrayFindLast",
        _ if intrinsic == RuntimeIntrinsic::ArrayFindLastIndex => "ArrayFindLastIndex",
        _ if intrinsic == RuntimeIntrinsic::ArrayFilter => "ArrayFilter",
        _ if intrinsic == RuntimeIntrinsic::ArrayEvery => "ArrayEvery",
        _ if intrinsic == RuntimeIntrinsic::ArraySome => "ArraySome",
        _ if intrinsic == RuntimeIntrinsic::ArrayReduce => "ArrayReduce",
        _ if intrinsic == RuntimeIntrinsic::ArrayReduceRight => "ArrayReduceRight",
        _ if intrinsic == RuntimeIntrinsic::ArrayLastIndexOf => "ArrayLastIndexOf",
        _ if intrinsic == RuntimeIntrinsic::ArrayForEach => "ArrayForEach",
        _ if intrinsic == RuntimeIntrinsic::ArrayMap => "ArrayMap",
        _ if intrinsic == RuntimeIntrinsic::ArrayAt => "ArrayAt",
        _ if intrinsic == RuntimeIntrinsic::ArrayFill => "ArrayFill",
        _ if intrinsic == RuntimeIntrinsic::ArrayFlat => "ArrayFlat",
        _ if intrinsic == RuntimeIntrinsic::ArrayPushOrSpread => "ArrayPushOrSpread",
        _ if intrinsic == RuntimeIntrinsic::ArrayCopyWithin => "ArrayCopyWithin",
        _ if intrinsic == RuntimeIntrinsic::ArrayWith => "ArrayWith",
        _ if intrinsic == RuntimeIntrinsic::ArrayToReversed => "ArrayToReversed",
        _ if intrinsic == RuntimeIntrinsic::ArrayToSorted => "ArrayToSorted",
        _ if intrinsic == RuntimeIntrinsic::ArrayToSpliced => "ArrayToSpliced",
        _ if intrinsic == RuntimeIntrinsic::ArrayValues => "ArrayValues",
        _ if intrinsic == RuntimeIntrinsic::ArrayKeys => "ArrayKeys",
        _ if intrinsic == RuntimeIntrinsic::ArrayEntries => "ArrayEntries",
        _ if intrinsic == RuntimeIntrinsic::ArrayShift => "ArrayShift",
        _ if intrinsic == RuntimeIntrinsic::ArrayUnshift => "ArrayUnshift",
        _ if intrinsic == RuntimeIntrinsic::ArraySplice => "ArraySplice",
        _ if intrinsic == RuntimeIntrinsic::ArrayIsArray => "ArrayIsArray",
        // Math
        _ if intrinsic == RuntimeIntrinsic::MathFloor => "MathFloor",
        _ if intrinsic == RuntimeIntrinsic::MathCeil => "MathCeil",
        _ if intrinsic == RuntimeIntrinsic::MathRound => "MathRound",
        _ if intrinsic == RuntimeIntrinsic::MathAbs => "MathAbs",
        _ if intrinsic == RuntimeIntrinsic::MathMax => "MathMax",
        _ if intrinsic == RuntimeIntrinsic::MathMin => "MathMin",
        _ if intrinsic == RuntimeIntrinsic::MathPow => "MathPow",
        _ if intrinsic == RuntimeIntrinsic::MathRandom => "MathRandom",
        _ if intrinsic == RuntimeIntrinsic::MathTrunc => "MathTrunc",
        _ if intrinsic == RuntimeIntrinsic::MathSign => "MathSign",
        // Date
        _ if intrinsic == RuntimeIntrinsic::DateNew => "DateNew",
        _ if intrinsic == RuntimeIntrinsic::DateNewLive => "DateNewLive",
        _ if intrinsic == RuntimeIntrinsic::DateNow => "DateNow",
        _ if intrinsic == RuntimeIntrinsic::DateEpochMsNowNumber => "DateEpochMsNowNumber",
        _ if intrinsic == RuntimeIntrinsic::DateGetTime => "DateGetTime",
        _ if intrinsic == RuntimeIntrinsic::DateToString => "DateToString",
        _ if intrinsic == RuntimeIntrinsic::DateGetLocalTimeField => "DateGetLocalTimeField",
        _ if intrinsic == RuntimeIntrinsic::DateToISOString => "DateToISOString",
        _ if intrinsic == RuntimeIntrinsic::DateGetTimezoneOffset => "DateGetTimezoneOffset",
        _ if intrinsic == RuntimeIntrinsic::DateGetUtcMilliseconds => "DateGetUtcMilliseconds",
        _ if intrinsic == RuntimeIntrinsic::DateGetUtcSeconds => "DateGetUtcSeconds",
        _ if intrinsic == RuntimeIntrinsic::DateGetUtcMinutes => "DateGetUtcMinutes",
        _ if intrinsic == RuntimeIntrinsic::DateGetUtcHours => "DateGetUtcHours",
        _ if intrinsic == RuntimeIntrinsic::DateGetUtcDay => "DateGetUtcDay",
        _ if intrinsic == RuntimeIntrinsic::DateGetUtcDate => "DateGetUtcDate",
        _ if intrinsic == RuntimeIntrinsic::DateGetUtcMonth => "DateGetUtcMonth",
        _ if intrinsic == RuntimeIntrinsic::DateGetUtcFullYear => "DateGetUtcFullYear",
        // String methods
        _ if intrinsic == RuntimeIntrinsic::StringCharAt => "StringCharAt",
        _ if intrinsic == RuntimeIntrinsic::StringAt => "StringAt",
        _ if intrinsic == RuntimeIntrinsic::StringSubstring => "StringSubstring",
        _ if intrinsic == RuntimeIntrinsic::StringSubstr => "StringSubstr",
        _ if intrinsic == RuntimeIntrinsic::StringSlice => "StringSlice",
        _ if intrinsic == RuntimeIntrinsic::StringIndexOf => "StringIndexOf",
        _ if intrinsic == RuntimeIntrinsic::StringLastIndexOf => "StringLastIndexOf",
        _ if intrinsic == RuntimeIntrinsic::StringLocaleCompare => "StringLocaleCompare",
        _ if intrinsic == RuntimeIntrinsic::StringIncludes => "StringIncludes",
        _ if intrinsic == RuntimeIntrinsic::StringPadStart => "StringPadStart",
        _ if intrinsic == RuntimeIntrinsic::StringPadEnd => "StringPadEnd",
        _ if intrinsic == RuntimeIntrinsic::StringRepeat => "StringRepeat",
        _ if intrinsic == RuntimeIntrinsic::StringSplit => "StringSplit",
        _ if intrinsic == RuntimeIntrinsic::StringTrim => "StringTrim",
        _ if intrinsic == RuntimeIntrinsic::StringTrimStart => "StringTrimStart",
        _ if intrinsic == RuntimeIntrinsic::StringTrimEnd => "StringTrimEnd",
        _ if intrinsic == RuntimeIntrinsic::StringStartsWith => "StringStartsWith",
        _ if intrinsic == RuntimeIntrinsic::StringEndsWith => "StringEndsWith",
        _ if intrinsic == RuntimeIntrinsic::StringMatch => "StringMatch",
        _ if intrinsic == RuntimeIntrinsic::StringSearch => "StringSearch",
        _ if intrinsic == RuntimeIntrinsic::StringToUpperCase => "StringToUpperCase",
        _ if intrinsic == RuntimeIntrinsic::StringToLowerCase => "StringToLowerCase",
        _ if intrinsic == RuntimeIntrinsic::StringCharCodeAt => "StringCharCodeAt",
        _ if intrinsic == RuntimeIntrinsic::StringCodePointAt => "StringCodePointAt",
        _ if intrinsic == RuntimeIntrinsic::StringIsWellFormed => "StringIsWellFormed",
        _ if intrinsic == RuntimeIntrinsic::StringToWellFormed => "StringToWellFormed",
        _ if intrinsic == RuntimeIntrinsic::StringFromCharCode => "StringFromCharCode",
        _ if intrinsic == RuntimeIntrinsic::StringFromCodePoint => "StringFromCodePoint",
        _ if intrinsic == RuntimeIntrinsic::StringReplace => "StringReplace",
        _ if intrinsic == RuntimeIntrinsic::StringReplaceAll => "StringReplaceAll",
        // RegExp
        _ if intrinsic == RuntimeIntrinsic::RegExpTest => "RegExpTest",
        _ if intrinsic == RuntimeIntrinsic::RegExpMatch => "RegExpMatch",
        _ if intrinsic == RuntimeIntrinsic::RegExpSearch => "RegExpSearch",
        _ if intrinsic == RuntimeIntrinsic::RegexpMatchInner => "RegexpMatchInner",
        // Object
        _ if intrinsic == RuntimeIntrinsic::ObjectKeys => "ObjectKeys",
        _ if intrinsic == RuntimeIntrinsic::ObjectSpread => "ObjectSpread",
        _ if intrinsic == RuntimeIntrinsic::SpreadViaIterator => "SpreadViaIterator",
        _ if intrinsic == RuntimeIntrinsic::ObjectValues => "ObjectValues",
        _ if intrinsic == RuntimeIntrinsic::ObjectEntries => "ObjectEntries",
        _ if intrinsic == RuntimeIntrinsic::ObjectHasOwnProperty => "ObjectHasOwnProperty",
        _ if intrinsic == RuntimeIntrinsic::ObjectHasOwn => "ObjectHasOwn",
        _ if intrinsic == RuntimeIntrinsic::ObjectGetOwnPropertyDescriptor => {
            "ObjectGetOwnPropertyDescriptor"
        }
        _ if intrinsic == RuntimeIntrinsic::ObjectGetPrototypeOf => "ObjectGetPrototypeOf",
        _ if intrinsic == RuntimeIntrinsic::ObjectSetPrototypeOf => "ObjectSetPrototypeOf",
        _ if intrinsic == RuntimeIntrinsic::ObjectFreeze => "ObjectFreeze",
        _ if intrinsic == RuntimeIntrinsic::ObjectSeal => "ObjectSeal",
        _ if intrinsic == RuntimeIntrinsic::ObjectPreventExtensions => "ObjectPreventExtensions",
        _ if intrinsic == RuntimeIntrinsic::ObjectIsExtensible => "ObjectIsExtensible",
        _ if intrinsic == RuntimeIntrinsic::ObjectIsSealed => "ObjectIsSealed",
        _ if intrinsic == RuntimeIntrinsic::ObjectIsFrozen => "ObjectIsFrozen",
        _ if intrinsic == RuntimeIntrinsic::ObjectDefineProperty => "ObjectDefineProperty",
        _ if intrinsic == RuntimeIntrinsic::ObjectAssign => "ObjectAssign",
        _ if intrinsic == RuntimeIntrinsic::ObjectCreate => "ObjectCreate",
        _ if intrinsic == RuntimeIntrinsic::ObjectIs => "ObjectIs",
        _ if intrinsic == RuntimeIntrinsic::ValueOf => "ValueOf",
        _ if intrinsic == RuntimeIntrinsic::InstanceOf => "InstanceOf",
        // Map/Set
        _ if intrinsic == RuntimeIntrinsic::MapNew => "MapNew",
        _ if intrinsic == RuntimeIntrinsic::MapGet => "MapGet",
        _ if intrinsic == RuntimeIntrinsic::MapSet => "MapSet",
        _ if intrinsic == RuntimeIntrinsic::MapHas => "MapHas",
        _ if intrinsic == RuntimeIntrinsic::MapDelete => "MapDelete",
        _ if intrinsic == RuntimeIntrinsic::MapValuesArray => "MapValuesArray",
        _ if intrinsic == RuntimeIntrinsic::SetNew => "SetNew",
        _ if intrinsic == RuntimeIntrinsic::SetAdd => "SetAdd",
        _ if intrinsic == RuntimeIntrinsic::SetHas => "SetHas",
        _ if intrinsic == RuntimeIntrinsic::SetDelete => "SetDelete",
        _ if intrinsic == RuntimeIntrinsic::SetSize => "SetSize",
        _ if intrinsic == RuntimeIntrinsic::SetClear => "SetClear",
        _ if intrinsic == RuntimeIntrinsic::SetForEach => "SetForEach",
        _ if intrinsic == RuntimeIntrinsic::MapClear => "MapClear",
        _ if intrinsic == RuntimeIntrinsic::MapSize => "MapSize",
        _ if intrinsic == RuntimeIntrinsic::MapForEach => "MapForEach",
        _ if intrinsic == RuntimeIntrinsic::MapEntriesArray => "MapEntriesArray",
        _ if intrinsic == RuntimeIntrinsic::SetFromArray => "SetFromArray",
        _ if intrinsic == RuntimeIntrinsic::SetValuesArray => "SetValuesArray",
        _ if intrinsic == RuntimeIntrinsic::SetPrototypeAddGet => "SetPrototypeAddGet",
        _ if intrinsic == RuntimeIntrinsic::SetPrototypeAddSet => "SetPrototypeAddSet",
        // WeakMap/WeakSet
        _ if intrinsic == RuntimeIntrinsic::WeakMapNew => "WeakMapNew",
        _ if intrinsic == RuntimeIntrinsic::WeakMapSet => "WeakMapSet",
        _ if intrinsic == RuntimeIntrinsic::WeakMapGet => "WeakMapGet",
        _ if intrinsic == RuntimeIntrinsic::WeakMapHas => "WeakMapHas",
        _ if intrinsic == RuntimeIntrinsic::WeakMapDelete => "WeakMapDelete",
        _ if intrinsic == RuntimeIntrinsic::WeakSetNew => "WeakSetNew",
        _ if intrinsic == RuntimeIntrinsic::WeakSetAdd => "WeakSetAdd",
        _ if intrinsic == RuntimeIntrinsic::WeakSetHas => "WeakSetHas",
        _ if intrinsic == RuntimeIntrinsic::WeakSetDelete => "WeakSetDelete",
        // Module
        _ if intrinsic == RuntimeIntrinsic::ModuleRequire => "ModuleRequire",
        _ if intrinsic == RuntimeIntrinsic::ModuleExportsSet => "ModuleExportsSet",
        _ if intrinsic == RuntimeIntrinsic::ModuleExportsAssign => "ModuleExportsAssign",
        // Node
        _ if intrinsic == RuntimeIntrinsic::FsReadFileSync => "FsReadFileSync",
        _ if intrinsic == RuntimeIntrinsic::FsWriteFileSync => "FsWriteFileSync",
        _ if intrinsic == RuntimeIntrinsic::FsAppendFileSync => "FsAppendFileSync",
        _ if intrinsic == RuntimeIntrinsic::ProcessArgv => "ProcessArgv",
        _ if intrinsic == RuntimeIntrinsic::ProcessEnv => "ProcessEnv",
        _ if intrinsic == RuntimeIntrinsic::ProcessExit => "ProcessExit",
        _ if intrinsic == RuntimeIntrinsic::PathJoin => "PathJoin",
        _ if intrinsic == RuntimeIntrinsic::PathResolve => "PathResolve",
        _ if intrinsic == RuntimeIntrinsic::PathBasename => "PathBasename",
        _ if intrinsic == RuntimeIntrinsic::PathDirname => "PathDirname",
        _ if intrinsic == RuntimeIntrinsic::CryptoRandomBytes => "CryptoRandomBytes",
        // Promise
        _ if intrinsic == RuntimeIntrinsic::PromiseConstructor => "PromiseConstructor",
        _ if intrinsic == RuntimeIntrinsic::PromiseResolve => "PromiseResolve",
        _ if intrinsic == RuntimeIntrinsic::PromiseReject => "PromiseReject",
        _ if intrinsic == RuntimeIntrinsic::PromiseThen => "PromiseThen",
        _ if intrinsic == RuntimeIntrinsic::PromiseCatch => "PromiseCatch",
        _ if intrinsic == RuntimeIntrinsic::PromiseAll => "PromiseAll",
        _ if intrinsic == RuntimeIntrinsic::PromiseRace => "PromiseRace",
        // Symbol
        _ if intrinsic == RuntimeIntrinsic::SymbolNew => "SymbolNew",
        _ if intrinsic == RuntimeIntrinsic::SymbolFor => "SymbolFor",
        _ if intrinsic == RuntimeIntrinsic::SymbolKeyFor => "SymbolKeyFor",
        // Encoding
        _ if intrinsic == RuntimeIntrinsic::EncodeURI => "EncodeURI",
        _ if intrinsic == RuntimeIntrinsic::DecodeURI => "DecodeURI",
        _ if intrinsic == RuntimeIntrinsic::Escape => "Escape",
        _ if intrinsic == RuntimeIntrinsic::Unescape => "Unescape",
        // Other
        _ if intrinsic == RuntimeIntrinsic::GetIterator => "GetIterator",
        _ if intrinsic == RuntimeIntrinsic::IteratorNext => "IteratorNext",
        _ if intrinsic == RuntimeIntrinsic::JsonStringify => "JsonStringify",
        _ if intrinsic == RuntimeIntrinsic::JsonParse => "JsonParse",
        _ if intrinsic == RuntimeIntrinsic::TypedArrayFromArray => "TypedArrayFromArray",
        _ if intrinsic == RuntimeIntrinsic::DataViewNew => "DataViewNew",
        _ if intrinsic == RuntimeIntrinsic::DataViewGetInt32 => "DataViewGetInt32",
        _ if intrinsic == RuntimeIntrinsic::DataViewSetInt32 => "DataViewSetInt32",
        _ if intrinsic == RuntimeIntrinsic::DataViewGetFloat64 => "DataViewGetFloat64",
        _ if intrinsic == RuntimeIntrinsic::DataViewSetFloat64 => "DataViewSetFloat64",
        // Global coerce
        _ if intrinsic == RuntimeIntrinsic::BooleanCoerce => "BooleanCoerce",
        _ if intrinsic == RuntimeIntrinsic::NumberCoerce => "NumberCoerce",
        // Task
        _ if intrinsic == RuntimeIntrinsic::TaskPoll => "TaskPoll",
        _ if intrinsic == RuntimeIntrinsic::TaskResult => "TaskResult",
        _ if intrinsic == RuntimeIntrinsic::TaskDrop => "TaskDrop",
        // Global
        _ if intrinsic == RuntimeIntrinsic::IsNaN => "IsNaN",
        _ if intrinsic == RuntimeIntrinsic::ParseInt => "ParseInt",
        _ if intrinsic == RuntimeIntrinsic::ParseFloat => "ParseFloat",
        _ if intrinsic == RuntimeIntrinsic::IsFinite => "IsFinite",
        // Pseudo-intrinsics
        _ if intrinsic == RuntimeIntrinsic::ArrayPushMany => "ArrayPushMany",
        _ if intrinsic == RuntimeIntrinsic::HeapClosureCall => "HeapClosureCall",
        _ if intrinsic == RuntimeIntrinsic::PrivateFieldGet => "PrivateFieldGet",
        _ if intrinsic == RuntimeIntrinsic::PrivateFieldSet => "PrivateFieldSet",
        _ if intrinsic == RuntimeIntrinsic::PrivateBrandCheck => "PrivateBrandCheck",
        _ => "Unknown",
    }
}

/// Trait for types that can dump their MIR representation.
pub trait MirDump {
    /// Dump this value as a MIR-formatted string.
    fn dump_mir(&self) -> String;
}

impl MirDump for MirProgram {
    fn dump_mir(&self) -> String {
        dump_mir_program(self, "program")
    }
}

impl MirDump for MirFunction {
    fn dump_mir(&self) -> String {
        dump_mir_function(self)
    }
}

impl MirDump for MirStmt {
    fn dump_mir(&self) -> String {
        let mut out = String::new();
        dump_mir_stmt(self, &mut out, 0);
        out
    }
}

impl MirDump for MirExpr {
    fn dump_mir(&self) -> String {
        let mut out = String::new();
        dump_mir_expr(self, &mut out, 0);
        out
    }
}
