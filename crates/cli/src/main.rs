use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::Parser;
use ts2wasm_frontend::{DiagCode, Diagnostic};

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
        /// Reject specified host imports (e.g., "node")
        #[arg(long = "host-deny", value_name = "HOST")]
        host_deny: Option<String>,
        /// Explain unsupported diagnostics in detail (tracking issue, fixture
        /// path, workaround, next crate to implement)
        #[arg(long)]
        explain_unsupported: bool,
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
            explain_unsupported,
        } => {
            let result = if let Some(ref host) = host_deny {
                ts2wasm_cli::build_file_with_host_deny(
                    &input,
                    &output,
                    manifest.as_deref(),
                    Some(host.as_str()),
                )
            } else {
                ts2wasm_cli::build_file_with_options(&input, &output, manifest.as_deref())
            };
            match result {
                Ok(report) => {
                    for diag in &report.diagnostics {
                        if explain_unsupported {
                            explain_unsupported_diagnostic(diag, &input);
                        } else {
                            eprintln!("warning: {diag}");
                        }
                    }
                    Ok(())
                }
                Err(e) => {
                    if explain_unsupported {
                        explain_unsupported_diagnostic(&e, &input);
                    }
                    Err(e.to_string())
                }
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

/// Print detailed explanation for unsupported diagnostics.
///
/// When `--explain-unsupported` is passed, this function prints the diagnostic
/// code breakdown, tracking issue reference (extracted from the message), the
/// input fixture path, and a suggestion for which crate to implement next.
fn explain_unsupported_diagnostic(diag: &Diagnostic, input: &Path) {
    let display = diag.display_code();
    let is_unsupported = matches!(
        display,
        DiagCode::UnsupportedSyntax
            | DiagCode::UnsupportedBuiltin
            | DiagCode::UnsupportedDate
            | DiagCode::UnsupportedRegExp
            | DiagCode::UnsupportedModule
            | DiagCode::UnsupportedEval
            | DiagCode::UnsupportedTypeScriptSyntax
            | DiagCode::UnsupportedRuntimeSubset
    );

    if !is_unsupported {
        return;
    }

    // Extract tracking issue reference from the message
    let tracking = diag
        .message
        .split_whitespace()
        .find(|word| word.starts_with("issue-") || word.starts_with("issue:"))
        .or_else(|| {
            diag.message
                .split_whitespace()
                .find(|word| word.contains("issue-"))
        })
        .map(|s| s.trim_end_matches(|c: char| !c.is_alphanumeric() && c != '-'))
        .unwrap_or("none");

    // Map display code to next crate suggestion
    let next_crate = match display {
        DiagCode::UnsupportedTypeScriptSyntax => "frontend (parser/TS erasure)",
        DiagCode::UnsupportedSyntax => "frontend (parser) or ir (lowering)",
        DiagCode::UnsupportedBuiltin => "runtime (builtin stubs)",
        DiagCode::UnsupportedRuntimeSubset => "runtime or ir (lowering)",
        DiagCode::UnsupportedDate => "runtime (Date implementation)",
        DiagCode::UnsupportedRegExp => "runtime (RegExp implementation)",
        DiagCode::UnsupportedModule => "compiler (module graph) or runtime",
        DiagCode::UnsupportedEval => "reference/triage (eval strategy)",
        _ => "unknown",
    };

    eprintln!("\n── explain-unsupported ──────────────────────────────");
    eprintln!("  code:       {:?}", display);
    eprintln!("  fixture:    {}", input.display());
    eprintln!("  tracking:   {}", tracking);
    eprintln!("  message:    {}", diag.message);
    eprintln!("  next crate: {}", next_crate);
    eprintln!("─────────────────────────────────────────────────\n");
}
