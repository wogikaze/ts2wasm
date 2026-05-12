use super::emitter::WatEmitter;
use super::runtime_fn::RuntimeFn;

impl WatEmitter<'_> {
    /// Dispatch Host, Module, Encoding, Symbol, and Iterator domain runtime functions.
    pub(super) fn emit_dispatch_host(&mut self, f: RuntimeFn, wat: &mut String) {
        match f {
            RuntimeFn::FsReadFileSync => self.emit_fs_read_file_sync(wat),
            RuntimeFn::FsWriteFileSync => self.emit_fs_write_file_sync(wat),
            RuntimeFn::FsAppendFileSync => self.emit_fs_append_file_sync(wat),
            RuntimeFn::ProcessArgv => self.emit_process_argv(wat),
            RuntimeFn::ProcessEnv => self.emit_process_env(wat),
            RuntimeFn::ProcessExit => self.emit_process_exit(wat),
            RuntimeFn::PathJoin => self.emit_path_join(wat),
            RuntimeFn::PathResolve => self.emit_path_resolve(wat),
            RuntimeFn::PathBasename => self.emit_path_basename(wat),
            RuntimeFn::PathDirname => self.emit_path_dirname(wat),
            RuntimeFn::CryptoRandomBytes => self.emit_crypto_random_bytes(wat),
            RuntimeFn::Dollar262Global => self.emit_dollar_262_global(wat),
            RuntimeFn::Dollar262Eval => self.emit_dollar_262_eval(wat),
            RuntimeFn::ModuleRequire => self.emit_module_require(wat),
            RuntimeFn::ModuleExportsSet => self.emit_module_exports_set(wat),
            RuntimeFn::ModuleExportsAssign => self.emit_module_exports_assign(wat),
            RuntimeFn::EncodeURI => self.emit_encode_uri(wat),
            RuntimeFn::DecodeURI => self.emit_decode_uri(wat),
            RuntimeFn::Escape => self.emit_escape(wat),
            RuntimeFn::Unescape => self.emit_unescape(wat),
            RuntimeFn::SymbolNew => self.emit_symbol_new(wat),
            RuntimeFn::SymbolFor => self.emit_symbol_for(wat),
            RuntimeFn::SymbolKeyFor => self.emit_symbol_key_for(wat),
            RuntimeFn::SymbolToPrimitive => self.emit_symbol_to_primitive(wat),
            RuntimeFn::SymbolToStringTag => self.emit_symbol_to_string_tag(wat),
            RuntimeFn::SymbolHasInstance => self.emit_symbol_has_instance(wat),
            RuntimeFn::GetIterator => self.emit_get_iterator(wat),
            RuntimeFn::IteratorNext => self.emit_iterator_next(wat),
            _ => unreachable!("non-host RuntimeFn routed to host dispatch"),
        }
    }
}
