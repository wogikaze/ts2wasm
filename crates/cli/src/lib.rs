pub use ts2wasm_compiler::{
    BuildPipelineOptions, CompileReport, DumpOptions, DumpPhase, HirMirBuildMode,
    OptimizationLevel, build_file, build_file_with_host_deny, build_file_with_options,
    build_file_with_pipeline_options, check_typescript_file, collect_typescript_diagnostics,
    dump_file_with_options, parse_program,
};
