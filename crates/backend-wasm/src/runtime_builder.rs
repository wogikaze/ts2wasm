use super::emitter::WatEmitter;
use super::runtime_fn::RuntimeDomain;
use super::runtime_fn::RuntimeFn;

impl WatEmitter<'_> {
    /// Emit WAT runtime functions by domain dispatch.
    ///
    /// Iterates `RuntimeFn::emission_order()` and dispatches each required
    /// runtime function to its domain-specific handler. This replaces the
    /// previous 288-arm single match with a ~21-arm domain-level dispatch.
    pub(super) fn emit_runtime(&mut self, wat: &mut String) {
        // Emit UTF-8 helper functions required by string operations
        // These are plain WAT functions (not RuntimeFn) and must be emitted
        // before any function that calls them.
        self.emit_utf8_helpers(wat);
        for runtime_fn in RuntimeFn::emission_order() {
            if !self
                .link_plan
                .required_runtime_functions()
                .contains(runtime_fn)
            {
                continue;
            }
            match runtime_fn.domain() {
                RuntimeDomain::Core
                | RuntimeDomain::Operator
                | RuntimeDomain::TypeCoercion
                | RuntimeDomain::Number
                | RuntimeDomain::Json => self.emit_dispatch_core(*runtime_fn, wat),
                RuntimeDomain::BigInt => self.emit_dispatch_bigint(*runtime_fn, wat),
                RuntimeDomain::String | RuntimeDomain::RegExp => {
                    self.emit_dispatch_string(*runtime_fn, wat)
                }
                RuntimeDomain::Array | RuntimeDomain::TypedArray => {
                    self.emit_dispatch_array(*runtime_fn, wat)
                }
                RuntimeDomain::Object => self.emit_dispatch_object(*runtime_fn, wat),
                RuntimeDomain::MapSet => self.emit_dispatch_collections(*runtime_fn, wat),
                RuntimeDomain::Date
                | RuntimeDomain::Math
                | RuntimeDomain::Promise
                | RuntimeDomain::Task => self.emit_dispatch_date(*runtime_fn, wat),
                RuntimeDomain::Host
                | RuntimeDomain::Module
                | RuntimeDomain::Encoding
                | RuntimeDomain::Symbol
                | RuntimeDomain::Iterator => self.emit_dispatch_host(*runtime_fn, wat),
            }
        }
    }

    pub(super) fn emit_spread_via_iterator(&self, _wat: &mut String) {
        unreachable!("SpreadViaIterator is handled inline in RuntimeCall dispatch");
    }
}
