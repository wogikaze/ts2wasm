use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

#[derive(Parser)]
#[command(name = "ts2wasm")]
enum Command {
    /// Build a TypeScript source file to a WebAssembly binary
    Build {
        input: PathBuf,
        #[arg(short = 'o')]
        output: PathBuf,
        /// Path to emit capability manifest JSON (alias: --emit-capabilities)
        #[arg(long = "emit-manifest", alias = "emit-capabilities")]
        manifest: Option<PathBuf>,
        /// Reject node host imports
        #[arg(long)]
        host_deny: bool,
    },
    /// Check a TypeScript source file for parse errors
    Check { input: PathBuf },
    /// Run the LSP-style language server
    Server,
    /// Dump intermediate representations
    Dump {
        /// Dump token stream
        #[arg(long)]
        tokens: bool,
        /// Dump AST
        #[arg(long)]
        ast: bool,
        /// Dump name-resolved AST
        #[arg(long)]
        resolved: bool,
        /// Dump typed IR
        #[arg(long)]
        tir: bool,
        /// Dump optimized typed IR
        #[arg(long)]
        optimize: bool,
        /// Dump lowered IR (alias: --ir)
        #[arg(long)]
        lowered: bool,
        /// Dump WAT output
        #[arg(long)]
        wat: bool,
        /// Unparse the AST back to source
        #[arg(long)]
        unparse: bool,
        /// Optimization level (0-3, requires --optimize)
        #[arg(short = 'O')]
        opt: Option<u8>,
        /// Input source file
        input: PathBuf,
    },
}

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
    match Command::parse() {
        Command::Build {
            input,
            output,
            manifest,
            host_deny,
        } => {
            let result = if host_deny {
                ts2wasm_cli::build_file_with_host_deny(&input, &output, manifest.as_deref(), true)
            } else {
                ts2wasm_cli::build_file_with_options(&input, &output, manifest.as_deref())
            };
            match result {
                Ok(report) => {
                    for diag in &report.diagnostics {
                        eprintln!("warning: {diag}");
                    }
                    Ok(())
                }
                Err(e) => Err(e.to_string()),
            }
        }
        Command::Check { input } => {
            ts2wasm_cli::check_typescript_file(&input).map_err(|e| e.to_string())
        }
        Command::Server => ts2wasm_compiler::server::run_server(),
        Command::Dump {
            tokens,
            ast,
            resolved,
            tir,
            optimize,
            lowered,
            wat,
            unparse,
            opt,
            input,
        } => {
            let mut options = ts2wasm_cli::DumpOptions::default();

            if tokens {
                options.set_phase(ts2wasm_cli::DumpPhase::Tokens)?;
            } else if ast {
                options.set_phase(ts2wasm_cli::DumpPhase::Ast)?;
            } else if resolved {
                options.set_phase(ts2wasm_cli::DumpPhase::Resolved)?;
            } else if tir {
                options.set_phase(ts2wasm_cli::DumpPhase::TypedIr)?;
            } else if optimize {
                options.set_phase(ts2wasm_cli::DumpPhase::OptimizedIr)?;
            } else if lowered {
                options.set_phase(ts2wasm_cli::DumpPhase::Lowered)?;
            } else if wat {
                options.set_phase(ts2wasm_cli::DumpPhase::Wat)?;
            }

            if let Some(level) = opt {
                match level {
                    0 => options.set_optimization_level(ts2wasm_cli::OptimizationLevel::O0),
                    1 => options.set_optimization_level(ts2wasm_cli::OptimizationLevel::O1),
                    2 => options.set_optimization_level(ts2wasm_cli::OptimizationLevel::O2),
                    3 => options.set_optimization_level(ts2wasm_cli::OptimizationLevel::O3),
                    other => return Err(format!("invalid optimization level: {other}")),
                }
                if options.phase != ts2wasm_cli::DumpPhase::OptimizedIr {
                    return Err("-O levels require --optimize".to_owned());
                }
            }

            options.unparse = unparse;

            let output =
                ts2wasm_cli::dump_file_with_options(&input, options).map_err(|e| e.to_string())?;
            print!("{output}");
            Ok(())
        }
    }
}
