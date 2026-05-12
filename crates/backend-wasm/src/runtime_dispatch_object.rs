use super::emitter::WatEmitter;
use super::runtime_fn::RuntimeFn;

impl WatEmitter<'_> {
    /// Dispatch Object domain runtime functions.
    pub(super) fn emit_dispatch_object(&mut self, f: RuntimeFn, wat: &mut String) {
        match f {
            RuntimeFn::PropertyGet => self.emit_property_get(wat),
            RuntimeFn::PropertySet => self.emit_property_set(wat),
            RuntimeFn::PropertyDelete => self.emit_property_delete(wat),
            RuntimeFn::PropertyHas => self.emit_property_has(wat),
            RuntimeFn::ObjectKeys => self.emit_object_keys(wat),
            RuntimeFn::ObjectSpread => self.emit_object_spread(wat),
            RuntimeFn::SpreadViaIterator => self.emit_spread_via_iterator(wat),
            RuntimeFn::ObjectValues => self.emit_object_values(wat),
            RuntimeFn::ObjectEntries => self.emit_object_entries(wat),
            RuntimeFn::ObjectHasOwnProperty => self.emit_object_has_own_property(wat),
            RuntimeFn::ObjectHasOwn => self.emit_object_has_own(wat),
            RuntimeFn::ObjectGetOwnPropertyDescriptor => {
                self.emit_object_get_own_property_descriptor(wat)
            }
            RuntimeFn::ObjectGetPrototypeOf => self.emit_object_get_prototype_of(wat),
            RuntimeFn::ObjectSetPrototypeOf => self.emit_object_set_prototype_of(wat),
            RuntimeFn::ObjectFreeze => self.emit_object_freeze(wat),
            RuntimeFn::ObjectSeal => self.emit_object_seal(wat),
            RuntimeFn::ObjectPreventExtensions => self.emit_object_prevent_extensions(wat),
            RuntimeFn::ObjectIsExtensible => self.emit_object_is_extensible(wat),
            RuntimeFn::ObjectIsSealed => self.emit_object_is_sealed(wat),
            RuntimeFn::ObjectIsFrozen => self.emit_object_is_frozen(wat),
            RuntimeFn::ObjectDefineProperty => self.emit_object_define_property(wat),
            RuntimeFn::ObjectAssign => self.emit_object_assign(wat),
            RuntimeFn::ObjectCreate => self.emit_object_create(wat),
            RuntimeFn::ObjectIs => self.emit_object_is(wat),
            _ => unreachable!("non-object RuntimeFn routed to object dispatch"),
        }
    }
}
