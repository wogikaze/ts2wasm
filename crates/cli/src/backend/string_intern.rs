use super::emitter::WatEmitter;
use crate::{align_to, wat_bytes};
use ts2wasm_runtime_abi::{Layout, ValueTag};

impl WatEmitter<'_> {
    pub(super) fn intern_string(&mut self, value: &str) -> u32 {
        if let Some(offset) = self.strings.get(value) {
            return *offset;
        }
        let offset = align_to(self.next_data_offset, Layout::ALIGN);
        self.next_data_offset = align_to(offset + 4 + value.len() as u32, Layout::ALIGN);
        self.strings.insert(value.to_owned(), offset);
        self.string_data.push((offset, value.to_owned()));
        offset
    }

    pub(super) fn string_value(&self, value: &str) -> u32 {
        self.strings[value] | ValueTag::STRING_TAG
    }

    pub(super) fn string_offset(&self, value: &str) -> u32 {
        self.strings[value]
    }

    pub(super) fn emit_data_segments(&self, wat: &mut String) {
        for (offset, value) in &self.string_data {
            let mut bytes = Vec::new();
            bytes.extend_from_slice(&(value.len() as u32).to_le_bytes());
            bytes.extend_from_slice(value.as_bytes());
            wat.push_str(&format!(
                "  (data (i32.const {offset}) \"{}\")\n",
                wat_bytes(&bytes)
            ));
        }
    }

    pub(super) fn string_len(&self, value: &str) -> u32 {
        value.len() as u32
    }
}
