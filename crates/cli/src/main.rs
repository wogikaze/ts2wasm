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
        [command, input] if command == "check" => {
            ts2wasm_cli::check_typescript_file(&PathBuf::from(input)).map_err(|e| e.to_string())
        }
        [command, rest @ ..] if command == "dump" => run_dump(rest),
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
        [command, input, flag, output, host_deny_flag]
            if command == "build"
                && flag == "-o"
                && host_deny_flag == "--host-deny" =>
        {
            ts2wasm_cli::build_file_with_host_deny(
                &PathBuf::from(input),
                &PathBuf::from(output),
                None,
                true,
            )
            .map_err(|e| e.to_string())
        }
        [command, input, flag, output, emit_flag, manifest, host_deny_flag]
            if command == "build"
                && flag == "-o"
                && matches!(emit_flag.as_str(), "--emit-manifest" | "--emit-capabilities")
                && host_deny_flag == "--host-deny" =>
        {
            ts2wasm_cli::build_file_with_host_deny(
                &PathBuf::from(input),
                &PathBuf::from(output),
                Some(&PathBuf::from(manifest)),
                true,
            )
            .map_err(|e| e.to_string())
        }
        _ => Err(
            "usage: ts2wasm build <input.ts> -o <output.wasm> [--emit-manifest <output.manifest.json>] [--host-deny]\n       ts2wasm check <input.ts>\n       ts2wasm dump [--tokens|--ast|--resolved|--tir|--lowered|--wat] [--unparse] <input.ts>\n(deprecated alias: --emit-capabilities <output.manifest.json>)"
                .to_owned(),
        ),
    }
}

fn run_dump(args: &[String]) -> Result<(), String> {
    let mut options = ts2wasm_cli::DumpOptions::default();
    let mut input = None;

    for arg in args {
        match arg.as_str() {
            "--tokens" => options.set_phase(ts2wasm_cli::DumpPhase::Tokens)?,
            "--ast" => options.set_phase(ts2wasm_cli::DumpPhase::Ast)?,
            "--resolved" => options.set_phase(ts2wasm_cli::DumpPhase::Resolved)?,
            "--tir" => options.set_phase(ts2wasm_cli::DumpPhase::TypedIr)?,
            "--lowered" | "--ir" => options.set_phase(ts2wasm_cli::DumpPhase::Lowered)?,
            "--wat" => options.set_phase(ts2wasm_cli::DumpPhase::Wat)?,
            "--unparse" => options.unparse = true,
            "--optimize" => {
                return Err(
                    "dump --optimize is not available yet; optimizer dump is tracked in issue 205"
                        .to_owned(),
                );
            }
            "-O0" | "-O1" | "-O2" | "-O3" => {
                return Err(format!(
                    "{arg} is only supported after optimizer dump is implemented"
                ));
            }
            _ if arg.starts_with('-') => return Err(format!("unknown dump option: {arg}")),
            _ => {
                if input.replace(PathBuf::from(arg)).is_some() {
                    return Err("dump accepts exactly one input file".to_owned());
                }
            }
        }
    }

    let input = input.ok_or_else(|| {
        "usage: ts2wasm dump [--tokens|--ast|--resolved|--tir|--lowered|--wat] [--unparse] <input.ts>"
            .to_owned()
    })?;
    let output = ts2wasm_cli::dump_file_with_options(&input, options).map_err(|e| e.to_string())?;
    print!("{output}");
    Ok(())
}
