//! Iterator SpecOps: GetIterator, IteratorNext, IteratorClose

use crate::algorithm::builder::AlgoBuilder;
use crate::algorithm::program::SpecAlgoProgram;
use crate::SpecOp;

pub fn build_get_iterator() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let object = env.alloc_local();

    // STUB(scaffold): Real GetIterator needs:
    // 1. Call @@iterator method on object
    // 2. Verify result is an object
    // For now, call SpecOp directly.
    let result = env.call_specop(
        SpecOp::GetIterator { object: object.0, sync: true },
        vec![object],
    );
    env.return_normal(result);
    env.build()
}

pub fn build_iterator_next() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let iterator = env.alloc_local();
    let ret_val = env.alloc_local();

    let result = env.call_specop(
        SpecOp::IteratorNext { iterator: iterator.0 },
        vec![iterator],
    );
    env.return_normal(result);
    env.build()
}

pub fn build_iterator_close() -> SpecAlgoProgram {
    let mut env = AlgoBuilder::new();
    let iterator = env.alloc_local();
    let completion = env.alloc_local();

    env.call_specop(
        SpecOp::IteratorClose { iterator: iterator.0, completion: completion.0 },
        vec![iterator, completion],
    );
    let ret_val = env.alloc_local();
    env.return_normal(ret_val);
    env.build()
}
