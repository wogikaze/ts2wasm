use super::emitter::WatEmitter;
use super::runtime_fn::StringOrigin;
use super::wat_writer::WatModuleBuilder;
use crate::{align_to, wat_bytes};
use ts2wasm_runtime_abi::{Layout, ValueTag};

impl WatEmitter<'_> {
    pub(super) fn intern_string(&mut self, value: &str) -> u32 {
        self.intern_string_with_origin(value, StringOrigin::UserLiteral)
    }

    pub(super) fn intern_string_with_origin(&mut self, value: &str, origin: StringOrigin) -> u32 {
        if let Some(offset) = self.strings.get(value) {
            return *offset;
        }
        let offset = align_to(self.next_data_offset, Layout::ALIGN)
            .expect("align_to: invalid alignment or overflow");
        self.next_data_offset = align_to(offset + 4 + value.len() as u32, Layout::ALIGN)
            .expect("align_to: invalid alignment or overflow");
        self.strings.insert(value.to_owned(), offset);
        if matches!(&origin, StringOrigin::Runtime(_)) {
            self.runtime_string_set.insert(value.to_owned());
        }
        self.string_data.push((offset, value.to_owned(), origin));
        offset
    }

    pub(super) fn string_value(&self, value: &str) -> u32 {
        *self
            .strings
            .get(value)
            .unwrap_or_else(|| panic!("string value `{value}` was not interned"))
            | ValueTag::STRING_TAG
    }

    pub(super) fn string_offset(&self, value: &str) -> u32 {
        *self
            .strings
            .get(value)
            .unwrap_or_else(|| panic!("string offset `{value}` was not interned"))
    }

    pub(super) fn emit_data_segments(&self, wat: &mut String) {
        let mut writer = WatModuleBuilder::new();
        for (offset, value, _origin) in &self.string_data {
            let mut bytes = Vec::new();
            bytes.extend_from_slice(&(value.len() as u32).to_le_bytes());
            bytes.extend_from_slice(value.as_bytes());
            writer.push_data_segment_escaped(*offset, &wat_bytes(&bytes));
        }
        wat.push_str(&writer.into_inner());
    }

    pub(super) fn string_len(&self, value: &str) -> u32 {
        value.len() as u32
    }

    /// Returns true if the given string was interned as a runtime-originated string.
    #[allow(dead_code)]
    pub(super) fn is_runtime_string(&self, value: &str) -> bool {
        self.runtime_string_set.contains(value)
    }
}
