impl WatEmitter<'_> {
    fn emit_array_push_many_call(
        &self,
        wat: &mut String,
        args: &[LoweredExpr],
        indent: usize,
        frame: &LocalFrame,
    ) {
        let Some((array, values)) = args.split_first() else {
            return;
        };
        let pad = " ".repeat(indent);
        if values.is_empty() {
            self.emit_expr(wat, array, indent, frame);
            wat.push_str(&format!("{pad}(call {})\n", RuntimeFn::GetLength.symbol()));
            return;
        }

        let arr_tmp = frame.heap_base_tmp();
        let val_tmp = frame.heap_value_tmp();

        // Save the array/object reference
        self.emit_expr(wat, array, indent, frame);
        wat.push_str(&format!("{pad}(local.set {arr_tmp})\n"));
        self.emit_gc_root_mirror_index(wat, &pad, arr_tmp, frame);

        // Branch: objects use $array_push (property_set), arrays use $array_push_grow + presence mask
        let inner = format!("{pad}  ");
        let inner2 = format!("{pad}    ");
        let inner3 = format!("{pad}      ");
        wat.push_str(&format!(
            "{pad}(if (i32.eq\n\
             {inner}(i32.and (local.get {arr_tmp}) (i32.const {tag_mask}))\n\
             {inner}(i32.const {object_tag}))\n\
             {inner}(then\n",
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
            inner = inner,
        ));
        // Object path: $array_push each value, drop intermediate results
        for value in values {
            wat.push_str(&format!(
                "{inner2}(drop\n\
                 {inner3}(call {}\n\
                 {inner3}  (local.get {arr_tmp})\n\
                 {inner3}  ",
                RuntimeFn::ArrayPush.symbol(),
            ));
            self.emit_expr(wat, value, indent + 6, frame);
            wat.push_str(&format!("{inner3}))\n",));
        }
        wat.push_str(&format!("{inner})(else\n",));
        // Array path: $array_push_grow each value + presence mask update
        for value in values {
            wat.push_str(&format!(
                "{inner2}(local.set {arr_tmp}\n\
                 {inner3}(call {}\n\
                 {inner3}  (local.get {arr_tmp})\n\
                 {inner3}  ",
                RuntimeFn::ArrayPushGrow.symbol(),
            ));
            self.emit_expr(wat, value, indent + 6, frame);
            wat.push_str(&format!("{inner3}))\n",));
            self.emit_gc_root_mirror_index(wat, &inner2, arr_tmp, frame);
            // Update presence mask: presence_word |= (1 << (new_len - 1))
            let p = inner2.clone();
            wat.push_str(&format!(
                "{p}(local.set {val_tmp}\n\
                 {inner3}(i32.sub\n\
                 {inner3}  (i32.load\n\
                 {inner3}    (i32.and (local.get {arr_tmp}) (i32.const {heap_mask})))\n\
                 {inner3}  (i32.const {one})))\n\
                 {p}(i32.store\n\
                 {inner3}(i32.add\n\
                 {inner3}  (i32.and (local.get {arr_tmp}) (i32.const {heap_mask}))\n\
                 {inner3}  (i32.const {presence_offset}))\n\
                 {inner3}(i32.or\n\
                 {inner3}  (i32.load\n\
                 {inner3}    (i32.add\n\
                 {inner3}      (i32.and (local.get {arr_tmp}) (i32.const {heap_mask}))\n\
                 {inner3}      (i32.const {presence_offset})))\n\
                 {inner3}  (i32.shl\n\
                 {inner3}    (i32.const {one})\n\
                 {inner3}    (local.get {val_tmp}))))\n",
                heap_mask = ValueTag::HEAP_MASK,
                presence_offset = Layout::ARRAY_PRESENCE_WORDS_OFFSET,
                one = RuntimeConst::ONE,
            ));
        }
        wat.push_str(&format!("{inner}))\n"));
        // Return GetLength of (potentially new) array
        wat.push_str(&format!(
            "{pad}(call {}\n\
             {pad}  (local.get {arr_tmp})\n\
             {pad})\n",
            RuntimeFn::GetLength.symbol(),
        ));
    }

    fn emit_array_push_grow_call(
        &self,
        wat: &mut String,
        args: &[LoweredExpr],
        indent: usize,
        frame: &LocalFrame,
    ) {
        let [array, value] = args else {
            return;
        };
        let pad = " ".repeat(indent);
        let old_array = frame.heap_base_tmp();
        let pushed_value = frame.heap_value_tmp();
        self.emit_expr(wat, array, indent, frame);
        wat.push_str(&format!("{pad}(local.set {old_array})\n"));
        self.emit_gc_root_mirror_index(wat, &pad, old_array, frame);
        self.emit_expr(wat, value, indent, frame);
        wat.push_str(&format!("{pad}(local.set {pushed_value})\n"));
        self.emit_gc_root_mirror_index(wat, &pad, pushed_value, frame);
        wat.push_str(&format!(
            "{pad}(local.get {old_array})\n\
             {pad}(local.get {pushed_value})\n\
             {pad}(call {})\n",
            RuntimeFn::ArrayPushGrow.symbol()
        ));
    }

    fn emit_array_literal(
        &self,
        wat: &mut String,
        elements: &[LoweredExpr],
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let elem_count = elements.len();
        let capacity = std::cmp::max(4, elem_count);
        let size = Layout::ARRAY_HEADER_SIZE + (capacity as u32) * 4;
        wat.push_str(&format!(
            "{pad}(local.set {} (call {} (i32.const {})))\n",
            frame.heap_base_tmp(),
            RuntimeFn::AllocHeap.symbol(),
            size,
        ));
        self.emit_gc_root_mirror_index(wat, &pad, frame.heap_base_tmp(), frame);
        wat.push_str(&format!(
            "{pad}(i32.store (local.get {}) (i32.const {}))\n",
            frame.heap_base_tmp(),
            elem_count,
        ));
        wat.push_str(&format!(
            "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const {}))\n",
            frame.heap_base_tmp(),
            Layout::ARRAY_CAPACITY_OFFSET,
            capacity,
        ));
        wat.push_str(&format!(
            "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const 1))\n",
            frame.heap_base_tmp(),
            Layout::ARRAY_PRESENCE_WORD_COUNT_OFFSET,
        ));
        wat.push_str(&format!(
            "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const {}))\n",
            frame.heap_base_tmp(),
            Layout::ARRAY_ELEMENTS_OFFSET_OFFSET,
            Layout::ARRAY_HEADER_SIZE,
        ));
        wat.push_str(&format!(
            "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const {}))\n",
            frame.heap_base_tmp(),
            Layout::ARRAY_PRESENCE_WORDS_OFFSET,
            array_presence_mask(elem_count),
        ));
        let child_frame = frame.child_temp_frame();
        for (i, elem) in elements.iter().enumerate() {
            let offset = Layout::ARRAY_HEADER_SIZE + (i as u32) * 4;
            self.emit_expr(wat, elem, indent, &child_frame);
            wat.push_str(&format!(
                "{pad}(local.set {})\n",
                child_frame.heap_value_tmp(),
            ));
            wat.push_str(&format!(
                "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (local.get {}))\n",
                frame.heap_base_tmp(),
                offset,
                child_frame.heap_value_tmp(),
            ));
        }
        wat.push_str(&format!(
            "{pad}(i32.or (local.get {}) (i32.const {}))\n",
            frame.heap_base_tmp(),
            ValueTag::ARRAY_TAG,
        ));
    }

    fn emit_sparse_array_literal(
        &self,
        wat: &mut String,
        slots: &[LoweredArraySlot],
        indent: usize,
        frame: &LocalFrame,
    ) {
        let pad = " ".repeat(indent);
        let elem_count = slots.len();
        let capacity = std::cmp::max(4, elem_count);
        let size = Layout::ARRAY_HEADER_SIZE + (capacity as u32) * 4;
        wat.push_str(&format!(
            "{pad}(local.set {} (call {} (i32.const {})))\n",
            frame.heap_base_tmp(),
            RuntimeFn::AllocHeap.symbol(),
            size,
        ));
        self.emit_gc_root_mirror_index(wat, &pad, frame.heap_base_tmp(), frame);
        wat.push_str(&format!(
            "{pad}(i32.store (local.get {}) (i32.const {}))\n",
            frame.heap_base_tmp(),
            elem_count,
        ));
        wat.push_str(&format!(
            "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const {}))\n",
            frame.heap_base_tmp(),
            Layout::ARRAY_CAPACITY_OFFSET,
            capacity,
        ));
        wat.push_str(&format!(
            "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const 1))\n",
            frame.heap_base_tmp(),
            Layout::ARRAY_PRESENCE_WORD_COUNT_OFFSET,
        ));
        wat.push_str(&format!(
            "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const {}))\n",
            frame.heap_base_tmp(),
            Layout::ARRAY_ELEMENTS_OFFSET_OFFSET,
            Layout::ARRAY_HEADER_SIZE,
        ));
        let mut mask = 0u32;
        for (i, slot) in slots.iter().enumerate() {
            if matches!(slot, LoweredArraySlot::Present(_)) && i < 32 {
                mask |= 1u32 << i;
            }
        }
        wat.push_str(&format!(
            "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const {}))\n",
            frame.heap_base_tmp(),
            Layout::ARRAY_PRESENCE_WORDS_OFFSET,
            mask as i32,
        ));
        let child_frame = frame.child_temp_frame();
        for (i, slot) in slots.iter().enumerate() {
            let offset = Layout::ARRAY_HEADER_SIZE + (i as u32) * 4;
            match slot {
                LoweredArraySlot::Present(elem) => self.emit_expr(wat, elem, indent, &child_frame),
                LoweredArraySlot::Hole => {
                    wat.push_str(&format!("{pad}(i32.const {})\n", ValueTag::UNDEFINED))
                }
            }
            wat.push_str(&format!(
                "{pad}(local.set {})\n",
                child_frame.heap_value_tmp()
            ));
            wat.push_str(&format!(
                "{pad}(i32.store (i32.add (local.get {}) (i32.const {})) (local.get {}))\n",
                frame.heap_base_tmp(),
                offset,
                child_frame.heap_value_tmp(),
            ));
        }
        wat.push_str(&format!(
            "{pad}(i32.or (local.get {}) (i32.const {}))\n",
            frame.heap_base_tmp(),
            ValueTag::ARRAY_TAG,
        ));
    }


}