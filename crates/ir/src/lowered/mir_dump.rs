//! MIR dump utility — pretty-prints MIR programs for debugging and snapshots.
//!
//! This module provides `mir_dump` functions that render `MirExpr`, `MirStmt`,
//! `MirFunction`, and `MirProgram` as human-readable strings. This is used for:
//!
//! - Debugging during development
//! - Snapshot testing (serializing MIR to compare against golden files)
//! - Architecture coverage checks (verifying MIR structure)

use crate::lowered::RuntimeFn;
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

/// Get a debug name for a RuntimeFn variant.
fn runtime_intrinsic_name(intrinsic: RuntimeFn) -> &'static str {
    // For the dump, we use intrinsic's Debug representation
    // This avoids needing a `name()` method on RuntimeFn when
    // it's re-exported from runtime-catalog.
    match intrinsic {
        // Core
        _ if intrinsic == RuntimeFn::ReadStdinBytes => "ReadStdinBytes",
        _ if intrinsic == RuntimeFn::Write => "Write",
        _ if intrinsic == RuntimeFn::Copy => "Copy",
        _ if intrinsic == RuntimeFn::ValueToStringInto => "ValueToStringInto",
        _ if intrinsic == RuntimeFn::ErrorMessage => "ErrorMessage",
        _ if intrinsic == RuntimeFn::Log => "Log",
        _ if intrinsic == RuntimeFn::TruthyBool => "TruthyBool",
        _ if intrinsic == RuntimeFn::Not => "Not",
        _ if intrinsic == RuntimeFn::TypeOf => "TypeOf",
        _ if intrinsic == RuntimeFn::AllocHeap => "AllocHeap",
        _ if intrinsic == RuntimeFn::MemEqual => "MemEqual",
        _ if intrinsic == RuntimeFn::Index => "Index",
        _ if intrinsic == RuntimeFn::GetLength => "GetLength",
        _ if intrinsic == RuntimeFn::PropertyGet => "PropertyGet",
        _ if intrinsic == RuntimeFn::PropertySet => "PropertySet",
        _ if intrinsic == RuntimeFn::PropertyDelete => "PropertyDelete",
        _ if intrinsic == RuntimeFn::PropertyHas => "PropertyHas",
        _ if intrinsic == RuntimeFn::PrivateBrandTypeError => "PrivateBrandTypeError",
        // Number
        _ if intrinsic == RuntimeFn::NumberFromI32 => "NumberFromI32",
        _ if intrinsic == RuntimeFn::NumberToI32 => "NumberToI32",
        _ if intrinsic == RuntimeFn::NumberToExponential => "NumberToExponential",
        _ if intrinsic == RuntimeFn::NumberToFixed => "NumberToFixed",
        _ if intrinsic == RuntimeFn::NumberToPrecision => "NumberToPrecision",
        _ if intrinsic == RuntimeFn::NumberIsNaN => "NumberIsNaN",
        _ if intrinsic == RuntimeFn::NumberIsFinite => "NumberIsFinite",
        _ if intrinsic == RuntimeFn::NumberIsInteger => "NumberIsInteger",
        _ if intrinsic == RuntimeFn::NumberIsSafeInteger => "NumberIsSafeInteger",
        // BigInt
        _ if intrinsic == RuntimeFn::MakeBigIntLiteral => "MakeBigIntLiteral",
        _ if intrinsic == RuntimeFn::BigIntToString => "BigIntToString",
        _ if intrinsic == RuntimeFn::BigIntToBoolean => "BigIntToBoolean",
        _ if intrinsic == RuntimeFn::BigIntFromValue => "BigIntFromValue",
        _ if intrinsic == RuntimeFn::BigIntAsIntN => "BigIntAsIntN",
        _ if intrinsic == RuntimeFn::BigIntAsUintN => "BigIntAsUintN",
        _ if intrinsic == RuntimeFn::BigIntUnaryMinus => "BigIntUnaryMinus",
        _ if intrinsic == RuntimeFn::BigIntAdd => "BigIntAdd",
        _ if intrinsic == RuntimeFn::BigIntSub => "BigIntSub",
        _ if intrinsic == RuntimeFn::BigIntMul => "BigIntMul",
        _ if intrinsic == RuntimeFn::BigIntPow => "BigIntPow",
        _ if intrinsic == RuntimeFn::BigIntDiv => "BigIntDiv",
        _ if intrinsic == RuntimeFn::BigIntRem => "BigIntRem",
        _ if intrinsic == RuntimeFn::BigIntDivisionByZeroRangeError => {
            "BigIntDivisionByZeroRangeError"
        }
        _ if intrinsic == RuntimeFn::BigIntMixedArithmeticTypeError => {
            "BigIntMixedArithmeticTypeError"
        }
        _ if intrinsic == RuntimeFn::BigIntStringComparisonBoundaryError => {
            "BigIntStringComparisonBoundaryError"
        }
        _ if intrinsic == RuntimeFn::BigIntBitwiseNot => "BigIntBitwiseNot",
        _ if intrinsic == RuntimeFn::BigIntBitwiseAnd => "BigIntBitwiseAnd",
        _ if intrinsic == RuntimeFn::BigIntBitwiseOr => "BigIntBitwiseOr",
        _ if intrinsic == RuntimeFn::BigIntBitwiseXor => "BigIntBitwiseXor",
        _ if intrinsic == RuntimeFn::BigIntLeftShift => "BigIntLeftShift",
        _ if intrinsic == RuntimeFn::BigIntRightShift => "BigIntRightShift",
        _ if intrinsic == RuntimeFn::BigIntCompare => "BigIntCompare",
        // String
        _ if intrinsic == RuntimeFn::StringEqual => "StringEqual",
        _ if intrinsic == RuntimeFn::Concat => "Concat",
        _ if intrinsic == RuntimeFn::IsString => "IsString",
        // Arithmetic operators
        _ if intrinsic == RuntimeFn::Add => "Add",
        _ if intrinsic == RuntimeFn::AddFast => "AddFast",
        _ if intrinsic == RuntimeFn::Sub => "Sub",
        _ if intrinsic == RuntimeFn::SubFast => "SubFast",
        _ if intrinsic == RuntimeFn::Mul => "Mul",
        _ if intrinsic == RuntimeFn::MulFast => "MulFast",
        _ if intrinsic == RuntimeFn::Div => "Div",
        _ if intrinsic == RuntimeFn::DivFast => "DivFast",
        _ if intrinsic == RuntimeFn::Mod => "Mod",
        _ if intrinsic == RuntimeFn::ModFast => "ModFast",
        _ if intrinsic == RuntimeFn::BitwiseToI32 => "BitwiseToI32",
        _ if intrinsic == RuntimeFn::BitwiseAnd => "BitwiseAnd",
        _ if intrinsic == RuntimeFn::BitwiseXor => "BitwiseXor",
        _ if intrinsic == RuntimeFn::BitwiseOr => "BitwiseOr",
        _ if intrinsic == RuntimeFn::Negate => "Negate",
        _ if intrinsic == RuntimeFn::Less => "Less",
        _ if intrinsic == RuntimeFn::LessFast => "LessFast",
        _ if intrinsic == RuntimeFn::LessEqual => "LessEqual",
        _ if intrinsic == RuntimeFn::LessEqualFast => "LessEqualFast",
        _ if intrinsic == RuntimeFn::Greater => "Greater",
        _ if intrinsic == RuntimeFn::GreaterFast => "GreaterFast",
        _ if intrinsic == RuntimeFn::GreaterEqual => "GreaterEqual",
        _ if intrinsic == RuntimeFn::GreaterEqualFast => "GreaterEqualFast",
        _ if intrinsic == RuntimeFn::StrictEqual => "StrictEqual",
        _ if intrinsic == RuntimeFn::EqualEqual => "EqualEqual",
        _ if intrinsic == RuntimeFn::BangEqual => "BangEqual",
        _ if intrinsic == RuntimeFn::StrictNotEqual => "StrictNotEqual",
        _ if intrinsic == RuntimeFn::And => "And",
        _ if intrinsic == RuntimeFn::Or => "Or",
        // Array
        _ if intrinsic == RuntimeFn::ArrayGet => "ArrayGet",
        _ if intrinsic == RuntimeFn::ArrayIndexPresent => "ArrayIndexPresent",
        _ if intrinsic == RuntimeFn::ArrayBufferNew => "ArrayBufferNew",
        _ if intrinsic == RuntimeFn::ArrayPush => "ArrayPush",
        _ if intrinsic == RuntimeFn::ArrayPushGrow => "ArrayPushGrow",
        _ if intrinsic == RuntimeFn::ArrayPop => "ArrayPop",
        _ if intrinsic == RuntimeFn::ArraySlice => "ArraySlice",
        _ if intrinsic == RuntimeFn::ArrayConcat => "ArrayConcat",
        _ if intrinsic == RuntimeFn::ArrayMapValueToString => "ArrayMapValueToString",
        _ if intrinsic == RuntimeFn::ArrayMapUnaryPlus => "ArrayMapUnaryPlus",
        _ if intrinsic == RuntimeFn::ArrayMapStringSplit => "ArrayMapStringSplit",
        _ if intrinsic == RuntimeFn::ArrayMapArrayLikeIdentity => {
            "ArrayMapArrayLikeIdentity"
        }
        _ if intrinsic == RuntimeFn::ArrayMapArrayLikeDouble => "ArrayMapArrayLikeDouble",
        _ if intrinsic == RuntimeFn::ArraySortNumeric => "ArraySortNumeric",
        _ if intrinsic == RuntimeFn::ArrayJoin => "ArrayJoin",
        _ if intrinsic == RuntimeFn::ArrayReverse => "ArrayReverse",
        _ if intrinsic == RuntimeFn::ArrayIndexOf => "ArrayIndexOf",
        _ if intrinsic == RuntimeFn::ArrayIncludes => "ArrayIncludes",
        _ if intrinsic == RuntimeFn::ArrayFind => "ArrayFind",
        _ if intrinsic == RuntimeFn::ArrayFindIndex => "ArrayFindIndex",
        _ if intrinsic == RuntimeFn::ArrayFindLast => "ArrayFindLast",
        _ if intrinsic == RuntimeFn::ArrayFindLastIndex => "ArrayFindLastIndex",
        _ if intrinsic == RuntimeFn::ArrayFilter => "ArrayFilter",
        _ if intrinsic == RuntimeFn::ArrayEvery => "ArrayEvery",
        _ if intrinsic == RuntimeFn::ArraySome => "ArraySome",
        _ if intrinsic == RuntimeFn::ArrayReduce => "ArrayReduce",
        _ if intrinsic == RuntimeFn::ArrayReduceRight => "ArrayReduceRight",
        _ if intrinsic == RuntimeFn::ArrayLastIndexOf => "ArrayLastIndexOf",
        _ if intrinsic == RuntimeFn::ArrayForEach => "ArrayForEach",
        _ if intrinsic == RuntimeFn::ArrayMap => "ArrayMap",
        _ if intrinsic == RuntimeFn::ArrayAt => "ArrayAt",
        _ if intrinsic == RuntimeFn::ArrayFill => "ArrayFill",
        _ if intrinsic == RuntimeFn::ArrayFlat => "ArrayFlat",
        _ if intrinsic == RuntimeFn::ArrayPushOrSpread => "ArrayPushOrSpread",
        _ if intrinsic == RuntimeFn::ArrayCopyWithin => "ArrayCopyWithin",
        _ if intrinsic == RuntimeFn::ArrayWith => "ArrayWith",
        _ if intrinsic == RuntimeFn::ArrayToReversed => "ArrayToReversed",
        _ if intrinsic == RuntimeFn::ArrayToSorted => "ArrayToSorted",
        _ if intrinsic == RuntimeFn::ArrayToSpliced => "ArrayToSpliced",
        _ if intrinsic == RuntimeFn::ArrayValues => "ArrayValues",
        _ if intrinsic == RuntimeFn::ArrayKeys => "ArrayKeys",
        _ if intrinsic == RuntimeFn::ArrayEntries => "ArrayEntries",
        _ if intrinsic == RuntimeFn::ArrayShift => "ArrayShift",
        _ if intrinsic == RuntimeFn::ArrayUnshift => "ArrayUnshift",
        _ if intrinsic == RuntimeFn::ArraySplice => "ArraySplice",
        _ if intrinsic == RuntimeFn::ArrayIsArray => "ArrayIsArray",
        // Math
        _ if intrinsic == RuntimeFn::MathFloor => "MathFloor",
        _ if intrinsic == RuntimeFn::MathCeil => "MathCeil",
        _ if intrinsic == RuntimeFn::MathRound => "MathRound",
        _ if intrinsic == RuntimeFn::MathAbs => "MathAbs",
        _ if intrinsic == RuntimeFn::MathMax => "MathMax",
        _ if intrinsic == RuntimeFn::MathMin => "MathMin",
        _ if intrinsic == RuntimeFn::MathPow => "MathPow",
        _ if intrinsic == RuntimeFn::MathRandom => "MathRandom",
        _ if intrinsic == RuntimeFn::MathTrunc => "MathTrunc",
        _ if intrinsic == RuntimeFn::MathSign => "MathSign",
        // Date
        _ if intrinsic == RuntimeFn::DateNew => "DateNew",
        _ if intrinsic == RuntimeFn::DateNewLive => "DateNewLive",
        _ if intrinsic == RuntimeFn::DateNow => "DateNow",
        _ if intrinsic == RuntimeFn::DateEpochMsNowNumber => "DateEpochMsNowNumber",
        _ if intrinsic == RuntimeFn::DateGetTime => "DateGetTime",
        _ if intrinsic == RuntimeFn::DateToString => "DateToString",
        _ if intrinsic == RuntimeFn::DateGetLocalTimeField => "DateGetLocalTimeField",
        _ if intrinsic == RuntimeFn::DateToISOString => "DateToISOString",
        _ if intrinsic == RuntimeFn::DateGetTimezoneOffset => "DateGetTimezoneOffset",
        _ if intrinsic == RuntimeFn::DateGetUtcMilliseconds => "DateGetUtcMilliseconds",
        _ if intrinsic == RuntimeFn::DateGetUtcSeconds => "DateGetUtcSeconds",
        _ if intrinsic == RuntimeFn::DateGetUtcMinutes => "DateGetUtcMinutes",
        _ if intrinsic == RuntimeFn::DateGetUtcHours => "DateGetUtcHours",
        _ if intrinsic == RuntimeFn::DateGetUtcDay => "DateGetUtcDay",
        _ if intrinsic == RuntimeFn::DateGetUtcDate => "DateGetUtcDate",
        _ if intrinsic == RuntimeFn::DateGetUtcMonth => "DateGetUtcMonth",
        _ if intrinsic == RuntimeFn::DateGetUtcFullYear => "DateGetUtcFullYear",
        // String methods
        _ if intrinsic == RuntimeFn::StringCharAt => "StringCharAt",
        _ if intrinsic == RuntimeFn::StringAt => "StringAt",
        _ if intrinsic == RuntimeFn::StringSubstring => "StringSubstring",
        _ if intrinsic == RuntimeFn::StringSubstr => "StringSubstr",
        _ if intrinsic == RuntimeFn::StringSlice => "StringSlice",
        _ if intrinsic == RuntimeFn::StringIndexOf => "StringIndexOf",
        _ if intrinsic == RuntimeFn::StringLastIndexOf => "StringLastIndexOf",
        _ if intrinsic == RuntimeFn::StringLocaleCompare => "StringLocaleCompare",
        _ if intrinsic == RuntimeFn::StringIncludes => "StringIncludes",
        _ if intrinsic == RuntimeFn::StringPadStart => "StringPadStart",
        _ if intrinsic == RuntimeFn::StringPadEnd => "StringPadEnd",
        _ if intrinsic == RuntimeFn::StringRepeat => "StringRepeat",
        _ if intrinsic == RuntimeFn::StringSplit => "StringSplit",
        _ if intrinsic == RuntimeFn::StringTrim => "StringTrim",
        _ if intrinsic == RuntimeFn::StringTrimStart => "StringTrimStart",
        _ if intrinsic == RuntimeFn::StringTrimEnd => "StringTrimEnd",
        _ if intrinsic == RuntimeFn::StringStartsWith => "StringStartsWith",
        _ if intrinsic == RuntimeFn::StringEndsWith => "StringEndsWith",
        _ if intrinsic == RuntimeFn::StringMatch => "StringMatch",
        _ if intrinsic == RuntimeFn::StringSearch => "StringSearch",
        _ if intrinsic == RuntimeFn::StringToUpperCase => "StringToUpperCase",
        _ if intrinsic == RuntimeFn::StringToLowerCase => "StringToLowerCase",
        _ if intrinsic == RuntimeFn::StringCharCodeAt => "StringCharCodeAt",
        _ if intrinsic == RuntimeFn::StringCodePointAt => "StringCodePointAt",
        _ if intrinsic == RuntimeFn::StringIsWellFormed => "StringIsWellFormed",
        _ if intrinsic == RuntimeFn::StringToWellFormed => "StringToWellFormed",
        _ if intrinsic == RuntimeFn::StringFromCharCode => "StringFromCharCode",
        _ if intrinsic == RuntimeFn::StringFromCodePoint => "StringFromCodePoint",
        _ if intrinsic == RuntimeFn::StringReplace => "StringReplace",
        _ if intrinsic == RuntimeFn::StringReplaceAll => "StringReplaceAll",
        // RegExp
        _ if intrinsic == RuntimeFn::RegExpTest => "RegExpTest",
        _ if intrinsic == RuntimeFn::RegExpMatch => "RegExpMatch",
        _ if intrinsic == RuntimeFn::RegExpSearch => "RegExpSearch",
        _ if intrinsic == RuntimeFn::RegexpMatchInner => "RegexpMatchInner",
        // Object
        _ if intrinsic == RuntimeFn::ObjectKeys => "ObjectKeys",
        _ if intrinsic == RuntimeFn::ObjectSpread => "ObjectSpread",
        _ if intrinsic == RuntimeFn::SpreadViaIterator => "SpreadViaIterator",
        _ if intrinsic == RuntimeFn::ObjectValues => "ObjectValues",
        _ if intrinsic == RuntimeFn::ObjectEntries => "ObjectEntries",
        _ if intrinsic == RuntimeFn::ObjectHasOwnProperty => "ObjectHasOwnProperty",
        _ if intrinsic == RuntimeFn::ObjectHasOwn => "ObjectHasOwn",
        _ if intrinsic == RuntimeFn::ObjectGetOwnPropertyDescriptor => {
            "ObjectGetOwnPropertyDescriptor"
        }
        _ if intrinsic == RuntimeFn::ObjectGetPrototypeOf => "ObjectGetPrototypeOf",
        _ if intrinsic == RuntimeFn::ObjectSetPrototypeOf => "ObjectSetPrototypeOf",
        _ if intrinsic == RuntimeFn::ObjectFreeze => "ObjectFreeze",
        _ if intrinsic == RuntimeFn::ObjectSeal => "ObjectSeal",
        _ if intrinsic == RuntimeFn::ObjectPreventExtensions => "ObjectPreventExtensions",
        _ if intrinsic == RuntimeFn::ObjectIsExtensible => "ObjectIsExtensible",
        _ if intrinsic == RuntimeFn::ObjectIsSealed => "ObjectIsSealed",
        _ if intrinsic == RuntimeFn::ObjectIsFrozen => "ObjectIsFrozen",
        _ if intrinsic == RuntimeFn::ObjectDefineProperty => "ObjectDefineProperty",
        _ if intrinsic == RuntimeFn::ObjectAssign => "ObjectAssign",
        _ if intrinsic == RuntimeFn::ObjectCreate => "ObjectCreate",
        _ if intrinsic == RuntimeFn::ObjectIs => "ObjectIs",
        _ if intrinsic == RuntimeFn::ValueOf => "ValueOf",
        _ if intrinsic == RuntimeFn::InstanceOf => "InstanceOf",
        // Map/Set
        _ if intrinsic == RuntimeFn::MapNew => "MapNew",
        _ if intrinsic == RuntimeFn::MapGet => "MapGet",
        _ if intrinsic == RuntimeFn::MapSet => "MapSet",
        _ if intrinsic == RuntimeFn::MapHas => "MapHas",
        _ if intrinsic == RuntimeFn::MapDelete => "MapDelete",
        _ if intrinsic == RuntimeFn::MapValuesArray => "MapValuesArray",
        _ if intrinsic == RuntimeFn::SetNew => "SetNew",
        _ if intrinsic == RuntimeFn::SetAdd => "SetAdd",
        _ if intrinsic == RuntimeFn::SetHas => "SetHas",
        _ if intrinsic == RuntimeFn::SetDelete => "SetDelete",
        _ if intrinsic == RuntimeFn::SetSize => "SetSize",
        _ if intrinsic == RuntimeFn::SetClear => "SetClear",
        _ if intrinsic == RuntimeFn::SetForEach => "SetForEach",
        _ if intrinsic == RuntimeFn::MapClear => "MapClear",
        _ if intrinsic == RuntimeFn::MapSize => "MapSize",
        _ if intrinsic == RuntimeFn::MapForEach => "MapForEach",
        _ if intrinsic == RuntimeFn::MapEntriesArray => "MapEntriesArray",
        _ if intrinsic == RuntimeFn::SetFromArray => "SetFromArray",
        _ if intrinsic == RuntimeFn::SetValuesArray => "SetValuesArray",
        _ if intrinsic == RuntimeFn::SetPrototypeAddGet => "SetPrototypeAddGet",
        _ if intrinsic == RuntimeFn::SetPrototypeAddSet => "SetPrototypeAddSet",
        // WeakMap/WeakSet
        _ if intrinsic == RuntimeFn::WeakMapNew => "WeakMapNew",
        _ if intrinsic == RuntimeFn::WeakMapSet => "WeakMapSet",
        _ if intrinsic == RuntimeFn::WeakMapGet => "WeakMapGet",
        _ if intrinsic == RuntimeFn::WeakMapHas => "WeakMapHas",
        _ if intrinsic == RuntimeFn::WeakMapDelete => "WeakMapDelete",
        _ if intrinsic == RuntimeFn::WeakSetNew => "WeakSetNew",
        _ if intrinsic == RuntimeFn::WeakSetAdd => "WeakSetAdd",
        _ if intrinsic == RuntimeFn::WeakSetHas => "WeakSetHas",
        _ if intrinsic == RuntimeFn::WeakSetDelete => "WeakSetDelete",
        // Module
        _ if intrinsic == RuntimeFn::ModuleRequire => "ModuleRequire",
        _ if intrinsic == RuntimeFn::ModuleExportsSet => "ModuleExportsSet",
        _ if intrinsic == RuntimeFn::ModuleExportsAssign => "ModuleExportsAssign",
        // Node
        _ if intrinsic == RuntimeFn::FsReadFileSync => "FsReadFileSync",
        _ if intrinsic == RuntimeFn::FsWriteFileSync => "FsWriteFileSync",
        _ if intrinsic == RuntimeFn::FsAppendFileSync => "FsAppendFileSync",
        _ if intrinsic == RuntimeFn::ProcessArgv => "ProcessArgv",
        _ if intrinsic == RuntimeFn::ProcessEnv => "ProcessEnv",
        _ if intrinsic == RuntimeFn::ProcessExit => "ProcessExit",
        _ if intrinsic == RuntimeFn::PathJoin => "PathJoin",
        _ if intrinsic == RuntimeFn::PathResolve => "PathResolve",
        _ if intrinsic == RuntimeFn::PathBasename => "PathBasename",
        _ if intrinsic == RuntimeFn::PathDirname => "PathDirname",
        _ if intrinsic == RuntimeFn::CryptoRandomBytes => "CryptoRandomBytes",
        // Promise
        _ if intrinsic == RuntimeFn::PromiseConstructor => "PromiseConstructor",
        _ if intrinsic == RuntimeFn::PromiseResolve => "PromiseResolve",
        _ if intrinsic == RuntimeFn::PromiseReject => "PromiseReject",
        _ if intrinsic == RuntimeFn::PromiseThen => "PromiseThen",
        _ if intrinsic == RuntimeFn::PromiseCatch => "PromiseCatch",
        _ if intrinsic == RuntimeFn::PromiseAll => "PromiseAll",
        _ if intrinsic == RuntimeFn::PromiseRace => "PromiseRace",
        // Symbol
        _ if intrinsic == RuntimeFn::SymbolNew => "SymbolNew",
        _ if intrinsic == RuntimeFn::SymbolFor => "SymbolFor",
        _ if intrinsic == RuntimeFn::SymbolKeyFor => "SymbolKeyFor",
        // Encoding
        _ if intrinsic == RuntimeFn::EncodeURI => "EncodeURI",
        _ if intrinsic == RuntimeFn::DecodeURI => "DecodeURI",
        _ if intrinsic == RuntimeFn::Escape => "Escape",
        _ if intrinsic == RuntimeFn::Unescape => "Unescape",
        // Other
        _ if intrinsic == RuntimeFn::GetIterator => "GetIterator",
        _ if intrinsic == RuntimeFn::IteratorNext => "IteratorNext",
        _ if intrinsic == RuntimeFn::JsonStringify => "JsonStringify",
        _ if intrinsic == RuntimeFn::JsonParse => "JsonParse",
        _ if intrinsic == RuntimeFn::TypedArrayFromArray => "TypedArrayFromArray",
        _ if intrinsic == RuntimeFn::DataViewNew => "DataViewNew",
        _ if intrinsic == RuntimeFn::DataViewGetInt32 => "DataViewGetInt32",
        _ if intrinsic == RuntimeFn::DataViewSetInt32 => "DataViewSetInt32",
        _ if intrinsic == RuntimeFn::DataViewGetFloat64 => "DataViewGetFloat64",
        _ if intrinsic == RuntimeFn::DataViewSetFloat64 => "DataViewSetFloat64",
        // Global coerce
        _ if intrinsic == RuntimeFn::BooleanCoerce => "BooleanCoerce",
        _ if intrinsic == RuntimeFn::NumberCoerce => "NumberCoerce",
        // Task
        _ if intrinsic == RuntimeFn::TaskPoll => "TaskPoll",
        _ if intrinsic == RuntimeFn::TaskResult => "TaskResult",
        _ if intrinsic == RuntimeFn::TaskDrop => "TaskDrop",
        // Global
        _ if intrinsic == RuntimeFn::IsNaN => "IsNaN",
        _ if intrinsic == RuntimeFn::ParseInt => "ParseInt",
        _ if intrinsic == RuntimeFn::ParseFloat => "ParseFloat",
        _ if intrinsic == RuntimeFn::IsFinite => "IsFinite",
        // Pseudo-intrinsics
        _ if intrinsic == RuntimeFn::ArrayPushMany => "ArrayPushMany",
        _ if intrinsic == RuntimeFn::HeapClosureCall => "HeapClosureCall",
        _ if intrinsic == RuntimeFn::PrivateFieldGet => "PrivateFieldGet",
        _ if intrinsic == RuntimeFn::PrivateFieldSet => "PrivateFieldSet",
        _ if intrinsic == RuntimeFn::PrivateBrandCheck => "PrivateBrandCheck",
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
