use super::emitter::WatEmitter;
use super::runtime_fn::RuntimeFn;

impl WatEmitter<'_> {
    /// Dispatch String and RegExp domain runtime functions.
    pub(super) fn emit_dispatch_string(&mut self, f: RuntimeFn, wat: &mut String) {
        match f {
            RuntimeFn::StringEqual => self.emit_string_equal(wat),
            RuntimeFn::Concat => self.emit_concat(wat),
            RuntimeFn::StringCharAt => self.emit_string_char_at(wat),
            RuntimeFn::StringAt => self.emit_string_at(wat),
            RuntimeFn::StringSubstring => self.emit_string_substring(wat),
            RuntimeFn::StringSubstr => self.emit_string_substr(wat),
            RuntimeFn::StringSlice => self.emit_string_slice(wat),
            RuntimeFn::StringIndexOf => self.emit_string_index_of(wat),
            RuntimeFn::StringLastIndexOf => self.emit_string_last_index_of(wat),
            RuntimeFn::StringLocaleCompare => self.emit_string_locale_compare(wat),
            RuntimeFn::StringIncludes => self.emit_string_includes(wat),
            RuntimeFn::StringPadStart => self.emit_string_pad_start(wat),
            RuntimeFn::StringPadEnd => self.emit_string_pad_end(wat),
            RuntimeFn::StringRepeat => self.emit_string_repeat(wat),
            RuntimeFn::StringSplit => self.emit_string_split(wat),
            RuntimeFn::StringTrim => self.emit_string_trim(wat),
            RuntimeFn::StringTrimStart => self.emit_string_trim_start(wat),
            RuntimeFn::StringTrimEnd => self.emit_string_trim_end(wat),
            RuntimeFn::StringStartsWith => self.emit_string_starts_with(wat),
            RuntimeFn::StringEndsWith => self.emit_string_ends_with(wat),
            RuntimeFn::StringMatch => self.emit_string_match(wat),
            RuntimeFn::StringSearch => self.emit_string_search(wat),
            RuntimeFn::StringToUpperCase => self.emit_string_to_upper_case(wat),
            RuntimeFn::StringToLowerCase => self.emit_string_to_lower_case(wat),
            RuntimeFn::StringCharCodeAt => self.emit_string_char_code_at(wat),
            RuntimeFn::StringCodePointAt => self.emit_string_at(wat),
            RuntimeFn::StringIsWellFormed => self.emit_string_is_well_formed(wat),
            RuntimeFn::StringToWellFormed => self.emit_string_to_well_formed(wat),
            RuntimeFn::StringFromCharCode => self.emit_string_from_char_code(wat),
            RuntimeFn::StringFromCodePoint => self.emit_string_from_code_point(wat),
            RuntimeFn::StringReplace => self.emit_string_replace(wat),
            RuntimeFn::StringReplaceAll => self.emit_string_replace_all(wat),
            RuntimeFn::RegexpMatchInner => self.emit_regexp_match_inner(wat),
            RuntimeFn::RegExpTest => self.emit_regexp_test(wat),
            RuntimeFn::RegExpMatch => self.emit_regexp_match(wat),
            RuntimeFn::RegExpSearch => self.emit_regexp_search(wat),
            _ => unreachable!("non-string RuntimeFn routed to string dispatch"),
        }
    }
}
