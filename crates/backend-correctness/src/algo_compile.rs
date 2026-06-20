//! SpecAlgoIR → WasmInstr mechanical compiler.
//!
//! Translates SpecAlgoProgram (from spec-kernel) into WasmInstr sequences.
//! This is a PURE MECHANICAL translation — NO JS SEMANTICS HERE.
//! Every WasmInstr must be traceable to a SpecAlgoStep from spec-kernel.

use std::collections::HashMap;

use ts2wasm_backend_core::wasm_ir::{WasmFunction, WasmInstr, WasmValType, WasmBlockType};
use ts2wasm_spec_kernel::algorithm::{
    CompletionKind, SpecAlgoProgram, SpecAlgoStep, SpecBlock, SpecBlockId, SpecLocal,
};
use ts2wasm_spec_kernel::SpecOp;

struct CompileCtx<'a> {
    blocks: HashMap<SpecBlockId, &'a SpecBlock>,
}

/// Compile a SpecAlgoProgram into a WasmFunction.
pub fn compile_algo_to_wasm(
    name: &str,
    program: &SpecAlgoProgram,
    params: Vec<WasmValType>,
    results: Vec<WasmValType>,
) -> WasmFunction {
    let mut body: Vec<WasmInstr> = Vec::new();

    // Build a block lookup map
    let block_map: HashMap<SpecBlockId, &SpecBlock> = program.blocks.iter()
        .map(|b| (b.id, b))
        .collect();
    let ctx = CompileCtx { blocks: block_map };

    // Compile the entry block
    if let Some(entry) = ctx.blocks.get(&program.entry_block) {
        for step in &entry.steps {
            compile_step(step, &mut body, &ctx);
        }
    }

    if !body.iter().any(|i| matches!(i, WasmInstr::End | WasmInstr::Return)) {
        body.push(WasmInstr::End);
    }

    WasmFunction {
        symbol: name.into(),
        params,
        results,
        locals: vec![],
        body,
    }
}

/// Compile the steps of a block into the body, inlined.
fn compile_block_steps(block_id: SpecBlockId, body: &mut Vec<WasmInstr>, ctx: &CompileCtx) {
    if let Some(block) = ctx.blocks.get(&block_id) {
        for step in &block.steps {
            compile_step(step, body, ctx);
        }
    }
}

fn compile_step(step: &SpecAlgoStep, body: &mut Vec<WasmInstr>, ctx: &CompileCtx) {
    match step {
        // ── Property storage primitives ─────────────────────────────
        SpecAlgoStep::OwnPropertyLookup { object, key, result_desc } => {
            body.push(WasmInstr::LocalGet(object.0 as usize));
            body.push(WasmInstr::LocalGet(key.0 as usize));
            body.push(WasmInstr::Call("$own_property_lookup".into()));
            body.push(WasmInstr::LocalSet(result_desc.0 as usize));
        }
        SpecAlgoStep::OwnPropertyInsert { object, key, desc } => {
            body.push(WasmInstr::LocalGet(object.0 as usize));
            body.push(WasmInstr::LocalGet(key.0 as usize));
            body.push(WasmInstr::LocalGet(desc.0 as usize));
            body.push(WasmInstr::Call("$own_property_insert".into()));
        }
        SpecAlgoStep::OwnPropertyUpdate { object, key, desc } => {
            body.push(WasmInstr::LocalGet(object.0 as usize));
            body.push(WasmInstr::LocalGet(key.0 as usize));
            body.push(WasmInstr::LocalGet(desc.0 as usize));
            body.push(WasmInstr::Call("$own_property_update".into()));
        }
        SpecAlgoStep::OwnPropertyDelete { object, key, result } => {
            body.push(WasmInstr::LocalGet(object.0 as usize));
            body.push(WasmInstr::LocalGet(key.0 as usize));
            body.push(WasmInstr::Call("$own_property_delete".into()));
            body.push(WasmInstr::LocalSet(result.0 as usize));
        }
        SpecAlgoStep::OwnPropertyKeysRaw { object, result } => {
            body.push(WasmInstr::LocalGet(object.0 as usize));
            body.push(WasmInstr::Call("$own_property_keys_raw".into()));
            body.push(WasmInstr::LocalSet(result.0 as usize));
        }
        SpecAlgoStep::GetPrototypeSlot { object, result_proto } => {
            body.push(WasmInstr::LocalGet(object.0 as usize));
            body.push(WasmInstr::Call("$get_prototype_slot".into()));
            body.push(WasmInstr::LocalSet(result_proto.0 as usize));
        }
        SpecAlgoStep::SetPrototypeSlot { object, proto, result } => {
            body.push(WasmInstr::LocalGet(object.0 as usize));
            body.push(WasmInstr::LocalGet(proto.0 as usize));
            body.push(WasmInstr::Call("$set_prototype_slot".into()));
            body.push(WasmInstr::LocalSet(result.0 as usize));
        }
        SpecAlgoStep::IsExtensibleBit { object, result } => {
            body.push(WasmInstr::LocalGet(object.0 as usize));
            body.push(WasmInstr::Call("$is_extensible_bit".into()));
            body.push(WasmInstr::LocalSet(result.0 as usize));
        }
        SpecAlgoStep::PreventExtensionsBit { object } => {
            body.push(WasmInstr::LocalGet(object.0 as usize));
            body.push(WasmInstr::Call("$prevent_extensions_bit".into()));
        }

        // ── Descriptor operations (scaffold) ──────────────────────────
        SpecAlgoStep::GetDescriptorValue { .. }
        | SpecAlgoStep::SetDescriptorValue { .. }
        | SpecAlgoStep::GetDescriptorGetter { .. }
        | SpecAlgoStep::GetDescriptorSetter { .. } => {
            body.push(WasmInstr::Nop);
        }
        SpecAlgoStep::CreateDataDescriptor { value, writable: _, enumerable: _, configurable: _, result } => {
            body.push(WasmInstr::LocalGet(value.0 as usize));
            body.push(WasmInstr::LocalSet(result.0 as usize));
        }
        SpecAlgoStep::CreateAccessorDescriptor { get, result, .. } => {
            body.push(WasmInstr::LocalGet(get.0 as usize));
            body.push(WasmInstr::LocalSet(result.0 as usize));
        }

        // ── Type queries (scaffold) ──────────────────────────────────
        SpecAlgoStep::IsCallable { result, .. }
        | SpecAlgoStep::IsConstructor { result, .. }
        | SpecAlgoStep::IsPropertyKey { result, .. }
        | SpecAlgoStep::IsDataDescriptor { result, .. }
        | SpecAlgoStep::IsAccessorDescriptor { result, .. }
        | SpecAlgoStep::IsGenericDescriptor { result, .. }
        | SpecAlgoStep::IsWritable { result, .. }
        | SpecAlgoStep::IsConfigurable { result, .. }
        | SpecAlgoStep::IsEnumerable { result, .. }
        | SpecAlgoStep::SameValue { result, .. }
        | SpecAlgoStep::SameValueZero { result, .. }
        | SpecAlgoStep::TypeOf { result, .. } => {
            body.push(WasmInstr::I32Const(1));
            body.push(WasmInstr::LocalSet(result.0 as usize));
        }

        // ── Allocation (scaffold) ────────────────────────────────────
        SpecAlgoStep::AllocateObject { result } => {
            body.push(WasmInstr::Call("$heap_alloc_object".into()));
            body.push(WasmInstr::LocalSet(result.0 as usize));
        }
        SpecAlgoStep::AllocateArray { result } => {
            body.push(WasmInstr::Call("$heap_alloc_array".into()));
            body.push(WasmInstr::LocalSet(result.0 as usize));
        }
        SpecAlgoStep::AllocateFunction { result } => {
            body.push(WasmInstr::Call("$heap_alloc_function".into()));
            body.push(WasmInstr::LocalSet(result.0 as usize));
        }

        // ── Control flow ────────────────────────────────────────────
        SpecAlgoStep::ReturnNormal { value } => {
            body.push(WasmInstr::LocalGet(value.0 as usize));
            body.push(WasmInstr::Return);
        }
        SpecAlgoStep::ReturnThrow { value } => {
            body.push(WasmInstr::LocalGet(value.0 as usize));
            body.push(WasmInstr::Call("$throw_exception".into()));
            body.push(WasmInstr::Unreachable);
        }
        SpecAlgoStep::ReturnCompletion { completion } => {
            match completion.kind {
                CompletionKind::Normal => {
                    body.push(WasmInstr::LocalGet(completion.value.0 as usize));
                    body.push(WasmInstr::Return);
                }
                CompletionKind::Throw => {
                    body.push(WasmInstr::LocalGet(completion.value.0 as usize));
                    body.push(WasmInstr::Call("$throw_exception".into()));
                    body.push(WasmInstr::Unreachable);
                }
                _ => {
                    body.push(WasmInstr::Unreachable);
                }
            }
        }
        SpecAlgoStep::ReturnIfAbrupt { completion, result_normal } => {
            body.push(WasmInstr::LocalGet(completion.0 as usize));
            body.push(WasmInstr::Call("$is_throw_completion".into()));
            body.push(WasmInstr::If { result_ty: WasmBlockType::Empty });
            body.push(WasmInstr::LocalGet(completion.0 as usize));
            body.push(WasmInstr::Call("$throw_exception".into()));
            body.push(WasmInstr::Unreachable);
            body.push(WasmInstr::Else);
            body.push(WasmInstr::LocalGet(completion.0 as usize));
            body.push(WasmInstr::LocalSet(result_normal.0 as usize));
            body.push(WasmInstr::End);
        }
        SpecAlgoStep::BranchOnCondition { cond, then_block, else_block } => {
            body.push(WasmInstr::LocalGet(cond.0 as usize));
            body.push(WasmInstr::If { result_ty: WasmBlockType::Empty });
            // Inline then-block steps directly inside the If branch
            compile_block_steps(*then_block, body, ctx);
            body.push(WasmInstr::Else);
            // Inline else-block steps directly inside the Else branch
            compile_block_steps(*else_block, body, ctx);
            body.push(WasmInstr::End);
        }
        SpecAlgoStep::Jump { .. } => {
            // Jump is a no-op in linear compilation — control flow
            // follows naturally from inlined block contents.
        }
        SpecAlgoStep::Loop { header } => {
            body.push(WasmInstr::Loop(format!("loop_{}", header.0)));
            compile_block_steps(*header, body, ctx);
            body.push(WasmInstr::End);
        }

        // ── Try / cleanup (scaffold) ────────────────────────────────
        SpecAlgoStep::TryBlock { .. } |
        SpecAlgoStep::CompletionMapInstall { .. } |
        SpecAlgoStep::IteratorClose { .. } => {
            body.push(WasmInstr::Nop);
        }

        // ── Dispatch ───────────────────────────────────────────────
        SpecAlgoStep::CallSpecOp { op, args, result } => {
            for arg in args {
                body.push(WasmInstr::LocalGet(arg.0 as usize));
            }
            body.push(WasmInstr::Call(spec_op_symbol(op)));
            body.push(WasmInstr::LocalSet(result.0 as usize));
        }
        SpecAlgoStep::CallBuiltinAlgorithm { algorithm, args, result } => {
            for arg in args {
                body.push(WasmInstr::LocalGet(arg.0 as usize));
            }
            body.push(WasmInstr::Call(format!("$builtin_algorithm_{}", algorithm)));
            body.push(WasmInstr::LocalSet(result.0 as usize));
        }
        SpecAlgoStep::CallRuntimePrimitive { symbol, args, result } => {
            for arg in args {
                body.push(WasmInstr::LocalGet(arg.0 as usize));
            }
            body.push(WasmInstr::Call(symbol.clone()));
            body.push(WasmInstr::LocalSet(result.0 as usize));
        }
        SpecAlgoStep::CallFunction { callee, this_arg, args, result } => {
            body.push(WasmInstr::LocalGet(callee.0 as usize));
            body.push(WasmInstr::LocalGet(this_arg.0 as usize));
            for arg in args {
                body.push(WasmInstr::LocalGet(arg.0 as usize));
            }
            body.push(WasmInstr::Call("$call_function".into()));
            body.push(WasmInstr::LocalSet(result.0 as usize));
        }

        // ── Context / environment (scaffold) ─────────────────────────
        SpecAlgoStep::EnterExecutionContext { .. } |
        SpecAlgoStep::LeaveExecutionContext |
        SpecAlgoStep::GetBindingValue { .. } |
        SpecAlgoStep::SetMutableBinding { .. } |
        SpecAlgoStep::CreateBinding { .. } |
        SpecAlgoStep::InitializeBinding { .. } |
        SpecAlgoStep::ResolveBinding { .. } |
        SpecAlgoStep::DeleteBinding { .. } |
        SpecAlgoStep::HasBinding { .. } |
        SpecAlgoStep::GetRealmIntrinsic { .. } |
        SpecAlgoStep::GetActiveScriptOrModule { .. } |
        SpecAlgoStep::HostResolveImportedModule { .. } |
        SpecAlgoStep::GetIterator { .. } |
        SpecAlgoStep::IteratorNext { .. } |
        SpecAlgoStep::IteratorComplete { .. } |
        SpecAlgoStep::IteratorValue { .. } |
        SpecAlgoStep::CreateIterResultObject { .. } => {
            body.push(WasmInstr::Nop);
        }
    }
}

fn spec_op_symbol(op: &SpecOp) -> String {
    match op {
        SpecOp::Get { .. } => "$spec_get".into(),
        SpecOp::Set { .. } => "$spec_set".into(),
        SpecOp::GetOwnProperty { .. } => "$spec_get_own_property".into(),
        SpecOp::DefineOwnProperty { .. } => "$spec_define_own_property".into(),
        SpecOp::Delete { .. } => "$spec_delete".into(),
        SpecOp::HasProperty { .. } => "$spec_has_property".into(),
        SpecOp::GetPrototypeOf { .. } => "$spec_get_prototype_of".into(),
        SpecOp::SetPrototypeOf { .. } => "$spec_set_prototype_of".into(),
        SpecOp::IsExtensible { .. } => "$spec_is_extensible".into(),
        SpecOp::PreventExtensions { .. } => "$spec_prevent_extensions".into(),
        SpecOp::OwnPropertyKeys { .. } => "$spec_own_property_keys".into(),
        SpecOp::Call { .. } => "$spec_call".into(),
        SpecOp::Construct { .. } => "$spec_construct".into(),
        SpecOp::CreateDataProperty { .. } => "$spec_create_data_property".into(),
        SpecOp::SetIntegrityLevel { .. } => "$spec_set_integrity_level".into(),
        SpecOp::TestIntegrityLevel { .. } => "$spec_test_integrity_level".into(),
        SpecOp::ToPrimitive { .. } => "$spec_to_primitive".into(),
        SpecOp::ToNumber { .. } => "$spec_to_number".into(),
        SpecOp::ToNumeric { .. } => "$spec_to_numeric".into(),
        SpecOp::ToPropertyKey { .. } => "$spec_to_property_key".into(),
        SpecOp::ToObject { .. } => "$spec_to_object".into(),
        SpecOp::ToBoolean { .. } => "$spec_to_boolean".into(),
        SpecOp::ToString { .. } => "$spec_to_string".into(),
        SpecOp::GetBindingValue { .. } => "$spec_get_binding_value".into(),
        SpecOp::SetMutableBinding { .. } => "$spec_set_mutable_binding".into(),
        SpecOp::CreateBinding { .. } => "$spec_create_binding".into(),
        SpecOp::InitializeBinding { .. } => "$spec_initialize_binding".into(),
        SpecOp::ResolveBinding { .. } => "$spec_resolve_binding".into(),
        SpecOp::GetIterator { .. } => "$spec_get_iterator".into(),
        SpecOp::IteratorNext { .. } => "$spec_iterator_next".into(),
        SpecOp::IteratorClose { .. } => "$spec_iterator_close".into(),
        SpecOp::GetModuleNamespace { .. } => "$spec_get_module_namespace".into(),
        SpecOp::Return { .. } => "$spec_return".into(),
        SpecOp::Throw { .. } => "$spec_throw".into(),
        SpecOp::PushStringConstant { .. } => "$spec_push_string_constant".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_spec_kernel::algorithm::{SpecAlgoProgram, SpecBlock, SpecBlockId, SpecLocal, SpecAlgoStep};

    #[test]
    fn compile_empty_program() {
        let program = SpecAlgoProgram::new(vec![], SpecBlockId(0), 0);
        let func = compile_algo_to_wasm("$test_empty", &program, vec![], vec![WasmValType::I32]);
        assert!(func.symbol.contains("test_empty"));
        assert!(!func.body.is_empty());
    }

    #[test]
    fn compile_return_normal() {
        let block = SpecBlock {
            id: SpecBlockId(0),
            steps: vec![SpecAlgoStep::ReturnNormal { value: SpecLocal(0) }],
        };
        let program = SpecAlgoProgram::new(vec![block], SpecBlockId(0), 1);
        let func = compile_algo_to_wasm("$test_return", &program, vec![WasmValType::I32], vec![WasmValType::I32]);
        let has_return = func.body.iter().any(|i| matches!(i, WasmInstr::Return));
        assert!(has_return, "expected Return instruction");
    }

    #[test]
    fn compile_own_property_lookup() {
        let block = SpecBlock {
            id: SpecBlockId(0),
            steps: vec![
                SpecAlgoStep::OwnPropertyLookup {
                    object: SpecLocal(0), key: SpecLocal(1), result_desc: SpecLocal(2),
                },
                SpecAlgoStep::ReturnNormal { value: SpecLocal(2) },
            ],
        };
        let program = SpecAlgoProgram::new(vec![block], SpecBlockId(0), 3);
        let func = compile_algo_to_wasm("$test_lookup", &program, vec![WasmValType::I32; 2], vec![WasmValType::I32]);
        let has_call = func.body.iter().any(|i| matches!(i, WasmInstr::Call(name) if name == "$own_property_lookup"));
        assert!(has_call, "expected Call($own_property_lookup)");
    }

    #[test]
    fn compile_branch_on_condition_inlines_blocks() {
        // Create a simple program with entry block + 2 branch targets
        let entry = SpecBlock {
            id: SpecBlockId(0),
            steps: vec![
                SpecAlgoStep::BranchOnCondition {
                    cond: SpecLocal(0),
                    then_block: SpecBlockId(1),
                    else_block: SpecBlockId(2),
                },
            ],
        };
        let then_block = SpecBlock {
            id: SpecBlockId(1),
            steps: vec![SpecAlgoStep::ReturnNormal { value: SpecLocal(1) }],
        };
        let else_block = SpecBlock {
            id: SpecBlockId(2),
            steps: vec![SpecAlgoStep::ReturnNormal { value: SpecLocal(2) }],
        };
        let program = SpecAlgoProgram::new(vec![entry, then_block, else_block], SpecBlockId(0), 3);
        let func = compile_algo_to_wasm("$test_branch", &program, vec![WasmValType::I32], vec![WasmValType::I32]);

        // Verify If/Else/End structure exists
        let has_if = func.body.iter().any(|i| matches!(i, WasmInstr::If { .. }));
        let has_else = func.body.iter().any(|i| matches!(i, WasmInstr::Else));
        assert!(has_if, "branch must emit If");
        assert!(has_else, "branch must emit Else");
    }
}
