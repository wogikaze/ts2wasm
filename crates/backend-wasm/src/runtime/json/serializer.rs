use crate::emitter::WatEmitter;

impl WatEmitter<'_> {
    // JSON serialization (JSON.stringify) is currently implemented in
    // runtime/host/emit.rs. This module is reserved for future structured
    // serializer emission. See RuntimeFn::JsonStringify.
}
