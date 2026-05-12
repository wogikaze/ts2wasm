use super::emitter::WatEmitter;
use super::runtime_fn::RuntimeFn;

impl WatEmitter<'_> {
    /// Dispatch MapSet domain runtime functions.
    pub(super) fn emit_dispatch_collections(&mut self, f: RuntimeFn, wat: &mut String) {
        match f {
            RuntimeFn::MapNew => self.emit_map_new(wat),
            RuntimeFn::MapGet => self.emit_map_get(wat),
            RuntimeFn::MapSet => self.emit_map_set(wat),
            RuntimeFn::MapHas => self.emit_map_has(wat),
            RuntimeFn::MapDelete => self.emit_map_delete(wat),
            RuntimeFn::MapValuesArray => self.emit_map_values_array(wat),
            RuntimeFn::MapClear => self.emit_map_clear(wat),
            RuntimeFn::MapSize => self.emit_map_size(wat),
            RuntimeFn::MapForEach => self.emit_map_for_each(wat),
            RuntimeFn::MapEntriesArray => self.emit_map_entries_array(wat),
            RuntimeFn::SetNew => self.emit_set_new(wat),
            RuntimeFn::SetAdd => self.emit_set_add(wat),
            RuntimeFn::SetHas => self.emit_set_has(wat),
            RuntimeFn::SetDelete => self.emit_set_delete(wat),
            RuntimeFn::SetSize => self.emit_set_size(wat),
            RuntimeFn::SetClear => self.emit_set_clear(wat),
            RuntimeFn::SetForEach => self.emit_set_for_each(wat),
            RuntimeFn::SetFromArray => self.emit_set_from_array(wat),
            RuntimeFn::SetValuesArray => self.emit_set_values_array(wat),
            RuntimeFn::SetPrototypeAddGet => self.emit_set_prototype_add_get(wat),
            RuntimeFn::SetPrototypeAddSet => self.emit_set_prototype_add_set(wat),
            RuntimeFn::WeakMapNew => self.emit_weak_map_new(wat),
            RuntimeFn::WeakMapSet => self.emit_weak_map_set(wat),
            RuntimeFn::WeakMapGet => self.emit_weak_map_get(wat),
            RuntimeFn::WeakMapHas => self.emit_weak_map_has(wat),
            RuntimeFn::WeakMapDelete => self.emit_weak_map_delete(wat),
            RuntimeFn::WeakSetNew => self.emit_weak_set_new(wat),
            RuntimeFn::WeakSetAdd => self.emit_weak_set_add(wat),
            RuntimeFn::WeakSetHas => self.emit_weak_set_has(wat),
            RuntimeFn::WeakSetDelete => self.emit_weak_set_delete(wat),
            _ => unreachable!("non-collections RuntimeFn routed to collections dispatch"),
        }
    }
}
