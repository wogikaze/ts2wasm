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
            ts2wasm_cli::build_file(&PathBuf::from(input), &PathBuf::from(output))
        }
        _ => Err("usage: ts2wasm build <input.ts> -o <output.wasm>".to_owned()),
    }
}
