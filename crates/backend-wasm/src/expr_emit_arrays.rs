use crate::wat_writer::WatWriter;

impl WatEmitter<'_> {
    fn emit_array_push_many_call(
        &self,
        writer: &mut WatWriter,
        args: &[LoweredExpr],
        indent: usize,
        frame: &LocalFrame,
    ) {
        let Some((array, values)) = args.split_first() else {
            return;
        };
        if values.is_empty() {
            self.emit_expr(writer, array, indent, frame);
            writer.line_fmt(
                indent,
                format_args!("(call {})", RuntimeFn::GetLength.symbol()),
            );
            return;
        }

        let arr_tmp = frame.heap_base_tmp();
        let val_tmp = frame.heap_value_tmp();

        // Save the array/object reference
        self.emit_expr(writer, array, indent, frame);
        writer.local_set(indent, arr_tmp);
        {
            let pad = " ".repeat(indent);
            self.emit_gc_root_mirror_index(writer.output_mut(), &pad, arr_tmp, frame);
        }

        // Branch: objects use $array_push (property_set), arrays use $array_push_grow + presence mask
        let inner = format!("{pad}  ", pad = " ".repeat(indent));
        let inner2 = format!("{pad}    ", pad = " ".repeat(indent));
        let inner3 = format!("{pad}      ", pad = " ".repeat(indent));
        writer.push_str(&format!(
            "{pad}(if (i32.eq\n\
             {inner}(i32.and (local.get {arr_tmp}) (i32.const {tag_mask}))\n\
             {inner}(i32.const {object_tag}))\n\
             {inner}(then\n",
            pad = " ".repeat(indent),
            inner = inner,
            tag_mask = ValueTag::TAG_MASK,
            object_tag = ValueTag::OBJECT,
        ));
        // Object path: $array_push each value, drop intermediate results
        for value in values {
            writer.push_str(&format!(
                "{inner2}(drop\n\
                 {inner3}(call {}\n\
                 {inner3}  (local.get {arr_tmp})\n\
                 {inner3}  ",
                RuntimeFn::ArrayPush.symbol(),
            ));
            self.emit_expr(writer, value, indent + 6, frame);
            writer.push_str(&format!("{inner3}))\n",));
        }
        writer.push_str(&format!("{inner})(else\n",));
        // Array path: $array_push_grow each value + presence mask update
        for value in values {
            writer.push_str(&format!(
                "{inner2}(local.set {arr_tmp}\n\
                 {inner3}(call {}\n\
                 {inner3}  (local.get {arr_tmp})\n\
                 {inner3}  ",
                RuntimeFn::ArrayPushGrow.symbol(),
            ));
            self.emit_expr(writer, value, indent + 6, frame);
            writer.push_str(&format!("{inner3}))\n",));
            {
                let i2 = inner2.clone();
                self.emit_gc_root_mirror_index(writer.output_mut(), &i2, arr_tmp, frame);
            }
            // Update presence mask: presence_word |= (1 << (new_len - 1))
            let p = inner2.clone();
            writer.push_str(&format!(
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
        writer.push_str(&format!("{inner}))\n"));
        // Return GetLength of (potentially new) array
        writer.push_str(&format!(
            "{pad}(call {}\n\
             {pad}  (local.get {arr_tmp})\n\
             {pad})\n",
            RuntimeFn::GetLength.symbol(),
            pad = " ".repeat(indent),
        ));
    }

    fn emit_array_push_grow_call(
        &self,
        writer: &mut WatWriter,
        args: &[LoweredExpr],
        indent: usize,
        frame: &LocalFrame,
    ) {
        let [array, value] = args else {
            return;
        };
        let old_array = frame.heap_base_tmp();
        let pushed_value = frame.heap_value_tmp();
        self.emit_expr(writer, array, indent, frame);
        writer.local_set(indent, old_array);
        {
            let pad = " ".repeat(indent);
            self.emit_gc_root_mirror_index(writer.output_mut(), &pad, old_array, frame);
        }
        self.emit_expr(writer, value, indent, frame);
        writer.local_set(indent, pushed_value);
        {
            let pad = " ".repeat(indent);
            self.emit_gc_root_mirror_index(writer.output_mut(), &pad, pushed_value, frame);
        }
        writer.push_str(&format!(
            "{pad}(local.get {old_array})\n\
             {pad}(local.get {pushed_value})\n\
             {pad}(call {})\n",
            RuntimeFn::ArrayPushGrow.symbol(),
            pad = " ".repeat(indent),
        ));
    }

    fn emit_array_literal(
        &self,
        writer: &mut WatWriter,
        elements: &[LoweredExpr],
        indent: usize,
        frame: &LocalFrame,
    ) {
        let elem_count = elements.len();
        let capacity = std::cmp::max(4, elem_count);
        let size = Layout::ARRAY_HEADER_SIZE + (capacity as u32) * 4;
        writer.line_fmt(
            indent,
            format_args!(
                "(local.set {} (call {} (i32.const {})))",
                frame.heap_base_tmp(),
                RuntimeFn::AllocHeap.symbol(),
                size,
            ),
        );
        {
            let pad = " ".repeat(indent);
            self.emit_gc_root_mirror_index(writer.output_mut(), &pad, frame.heap_base_tmp(), frame);
        }
        writer.line_fmt(
            indent,
            format_args!(
                "(i32.store (local.get {}) (i32.const {}))",
                frame.heap_base_tmp(),
                elem_count,
            ),
        );
        writer.line_fmt(
            indent,
            format_args!(
                "(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const {}))",
                frame.heap_base_tmp(),
                Layout::ARRAY_CAPACITY_OFFSET,
                capacity,
            ),
        );
        writer.line_fmt(
            indent,
            format_args!(
                "(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const 1))",
                frame.heap_base_tmp(),
                Layout::ARRAY_PRESENCE_WORD_COUNT_OFFSET,
            ),
        );
        writer.line_fmt(
            indent,
            format_args!(
                "(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const {}))",
                frame.heap_base_tmp(),
                Layout::ARRAY_ELEMENTS_OFFSET_OFFSET,
                Layout::ARRAY_HEADER_SIZE,
            ),
        );
        writer.line_fmt(
            indent,
            format_args!(
                "(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const {}))",
                frame.heap_base_tmp(),
                Layout::ARRAY_PRESENCE_WORDS_OFFSET,
                array_presence_mask(elem_count),
            ),
        );
        let child_frame = frame.child_temp_frame();
        for (i, elem) in elements.iter().enumerate() {
            let offset = Layout::ARRAY_HEADER_SIZE + (i as u32) * 4;
            self.emit_expr(writer, elem, indent, &child_frame);
            writer.local_set(indent, child_frame.heap_value_tmp());
            writer.line_fmt(
                indent,
                format_args!(
                    "(i32.store (i32.add (local.get {}) (i32.const {})) (local.get {}))",
                    frame.heap_base_tmp(),
                    offset,
                    child_frame.heap_value_tmp(),
                ),
            );
        }
        writer.line_fmt(
            indent,
            format_args!(
                "(i32.or (local.get {}) (i32.const {}))",
                frame.heap_base_tmp(),
                ValueTag::ARRAY_TAG,
            ),
        );
    }

    fn emit_sparse_array_literal(
        &self,
        writer: &mut WatWriter,
        slots: &[LoweredArraySlot],
        indent: usize,
        frame: &LocalFrame,
    ) {
        let elem_count = slots.len();
        let capacity = std::cmp::max(4, elem_count);
        let size = Layout::ARRAY_HEADER_SIZE + (capacity as u32) * 4;
        writer.line_fmt(
            indent,
            format_args!(
                "(local.set {} (call {} (i32.const {})))",
                frame.heap_base_tmp(),
                RuntimeFn::AllocHeap.symbol(),
                size,
            ),
        );
        {
            let pad = " ".repeat(indent);
            self.emit_gc_root_mirror_index(writer.output_mut(), &pad, frame.heap_base_tmp(), frame);
        }
        writer.line_fmt(
            indent,
            format_args!(
                "(i32.store (local.get {}) (i32.const {}))",
                frame.heap_base_tmp(),
                elem_count,
            ),
        );
        writer.line_fmt(
            indent,
            format_args!(
                "(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const {}))",
                frame.heap_base_tmp(),
                Layout::ARRAY_CAPACITY_OFFSET,
                capacity,
            ),
        );
        writer.line_fmt(
            indent,
            format_args!(
                "(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const 1))",
                frame.heap_base_tmp(),
                Layout::ARRAY_PRESENCE_WORD_COUNT_OFFSET,
            ),
        );
        writer.line_fmt(
            indent,
            format_args!(
                "(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const {}))",
                frame.heap_base_tmp(),
                Layout::ARRAY_ELEMENTS_OFFSET_OFFSET,
                Layout::ARRAY_HEADER_SIZE,
            ),
        );
        let mut mask = 0u32;
        for (i, slot) in slots.iter().enumerate() {
            if matches!(slot, LoweredArraySlot::Present(_)) && i < 32 {
                mask |= 1u32 << i;
            }
        }
        writer.line_fmt(
            indent,
            format_args!(
                "(i32.store (i32.add (local.get {}) (i32.const {})) (i32.const {}))",
                frame.heap_base_tmp(),
                Layout::ARRAY_PRESENCE_WORDS_OFFSET,
                mask as i32,
            ),
        );
        let child_frame = frame.child_temp_frame();
        for (i, slot) in slots.iter().enumerate() {
            let offset = Layout::ARRAY_HEADER_SIZE + (i as u32) * 4;
            match slot {
                LoweredArraySlot::Present(elem) => {
                    self.emit_expr(writer, elem, indent, &child_frame)
                }
                LoweredArraySlot::Hole => {
                    writer.i32_const(indent, ValueTag::UNDEFINED);
                }
            }
            writer.local_set(indent, child_frame.heap_value_tmp());
            writer.line_fmt(
                indent,
                format_args!(
                    "(i32.store (i32.add (local.get {}) (i32.const {})) (local.get {}))",
                    frame.heap_base_tmp(),
                    offset,
                    child_frame.heap_value_tmp(),
                ),
            );
        }
        writer.line_fmt(
            indent,
            format_args!(
                "(i32.or (local.get {}) (i32.const {}))",
                frame.heap_base_tmp(),
                ValueTag::ARRAY_TAG,
            ),
        );
    }
}
