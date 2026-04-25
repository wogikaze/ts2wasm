use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [command, input, flag, output] if command == "build" && flag == "-o" => {
            ts2wasm_cli::build_file_with_options(
                &PathBuf::from(input),
                &PathBuf::from(output),
                None,
            )
            .map_err(|e| e.to_string())
        }
        [command, input, flag, output, emit_flag, manifest]
            if command == "build"
                && flag == "-o"
                && matches!(emit_flag.as_str(), "--emit-manifest" | "--emit-capabilities") =>
        {
            ts2wasm_cli::build_file_with_options(
                &PathBuf::from(input),
                &PathBuf::from(output),
                Some(&PathBuf::from(manifest)),
            )
            .map_err(|e| e.to_string())
        }
        _ => Err(
            "usage: ts2wasm build <input.ts> -o <output.wasm> [--emit-manifest <output.manifest.json>]\n(deprecated alias: --emit-capabilities <output.manifest.json>)"
                .to_owned(),
        ),
    }
}
