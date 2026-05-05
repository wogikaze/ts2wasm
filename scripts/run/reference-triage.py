#!/usr/bin/env python3
"""Generate rich diagnostics for one reference test case.

Usage:
  mise run reference-triage -- test262 reference/test262/test/built-ins/Date/now.js
  mise run reference-triage -- --format json tsc reference/typescript/tests/cases/compiler/foo.ts
"""

from __future__ import annotations

import argparse
import importlib.util
import json
import os
import re
import subprocess
import sys
import tempfile
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Any

sys.path.insert(0, str(Path(__file__).parent.parent / "lib"))
from ts2wasm_binary import resolve_ts2wasm_binary

REPO_ROOT = Path(__file__).resolve().parents[2]
TS2WASM_BINARY = resolve_ts2wasm_binary()
REFERENCE_ROOT = Path(os.environ.get("TS2WASM_REFERENCE_ROOT", REPO_ROOT / "reference")).resolve()


def load_reference_coverage_module() -> Any:
    module_path = REPO_ROOT / "scripts" / "run" / "reference-coverage.py"
    spec = importlib.util.spec_from_file_location("reference_coverage", module_path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"failed to load {module_path}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


REFERENCE_COVERAGE = load_reference_coverage_module()
REFERENCE_COVERAGE._ensure_test262_runner()  # lazy init for test262_runner


@dataclass
class CommandResult:
    command: list[str]
    returncode: int
    stdout: str
    stderr: str
    timeout: bool


@dataclass
class DiagnosticInfo:
    code: str
    message: str
    span_start: int | None
    span_end: int | None
    line: int | None
    column: int | None
    feature_label: str
    error_type: str


@dataclass
class TriageReport:
    suite: str
    title: str
    path: str
    reproduction_command: str
    issue_class: str
    source_summary: dict[str, Any]
    diagnostic: DiagnosticInfo
    source_context: list[str]
    visible_symbols: list[dict[str, Any]]
    stack_trace: list[str]
    dump: dict[str, dict[str, Any]]
    oracle: dict[str, Any]
    duplicate_candidates: list[dict[str, str]]
    suggestions: list[str]
    rough_rust: str | None


def run_command(command: list[str], timeout_seconds: int = 12) -> CommandResult:
    env = dict(os.environ)
    env.setdefault("RUST_BACKTRACE", "1")
    try:
        result = subprocess.run(
            command,
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
            timeout=timeout_seconds,
            env=env,
        )
        return CommandResult(command, result.returncode, result.stdout, result.stderr, False)
    except subprocess.TimeoutExpired as error:
        return CommandResult(
            command,
            124,
            error.stdout or "",
            error.stderr or "",
            True,
        )


def repo_relative(path: Path) -> str:
    try:
        return path.resolve().relative_to(REPO_ROOT).as_posix()
    except ValueError:
        pass
    try:
        return f"reference/{path.resolve().relative_to(REFERENCE_ROOT).as_posix()}"
    except ValueError:
        return path.as_posix()


def resolve_input(suite: str, raw_path: str) -> Path:
    path = Path(raw_path)
    candidates = []
    if path.is_absolute():
        candidates.append(path)
    else:
        candidates.append(REPO_ROOT / path)
        candidates.append(REFERENCE_ROOT / raw_path.removeprefix("reference/"))
        suite_config = REFERENCE_COVERAGE.SUITE_METADATA.get(suite)
        if suite_config:
            candidates.append(suite_config["path"] / raw_path)

    for candidate in candidates:
        if candidate.is_file():
            return candidate.resolve()

    raise SystemExit(f"reference-triage: input file not found: {raw_path}")


def offset_to_line_col(source: str, offset: int | None) -> tuple[int | None, int | None]:
    if offset is None:
        return None, None
    offset = max(0, min(offset, len(source)))
    line = source.count("\n", 0, offset) + 1
    line_start = source.rfind("\n", 0, offset)
    column = offset + 1 if line_start == -1 else offset - line_start
    return line, column


def line_context(source: str, center_line: int | None, radius: int = 3) -> list[str]:
    lines = source.splitlines()
    if center_line is None:
        return lines[: min(len(lines), 8)]
    start = max(center_line - radius - 1, 0)
    end = min(center_line + radius, len(lines))
    width = len(str(end))
    return [f"{idx + 1:>{width}} | {lines[idx]}" for idx in range(start, end)]


def parse_test262_metadata(source: str) -> dict[str, Any]:
    match = re.search(r"/\*---(.*?)---\*/", source, re.S)
    if not match:
        return {}
    metadata: dict[str, Any] = {}
    for raw_line in match.group(1).splitlines():
        line = raw_line.strip()
        if not line or line.startswith("-"):
            continue
        if ":" in line:
            key, value = line.split(":", 1)
            metadata[key.strip()] = value.strip()
    return metadata


def source_summary(path: Path, suite: str, source: str) -> dict[str, Any]:
    non_empty = [line.strip() for line in source.splitlines() if line.strip()]
    first_code = next((line for line in non_empty if not line.startswith(("//", "/*", "*"))), "")
    summary: dict[str, Any] = {
        "suite": suite,
        "bytes": len(source.encode("utf-8")),
        "lines": len(source.splitlines()),
        "extension": path.suffix,
        "first_code_line": first_code[:160],
    }
    if suite == "test262":
        summary["test262_metadata"] = parse_test262_metadata(source)
    return summary


def parse_diagnostic(returncode: int, stderr: str, source: str, path: Path) -> DiagnosticInfo:
    if returncode == 0:
        return DiagnosticInfo(
            "BuildPass",
            "ts2wasm build succeeded",
            None,
            None,
            None,
            None,
            "build-pass",
            "pass",
        )

    diag_match = re.search(r"\[([A-Za-z0-9_]+)\]\s*(.*)", stderr)
    code = diag_match.group(1) if diag_match else "Unknown"
    message = diag_match.group(2).strip() if diag_match else stderr.strip().splitlines()[0][:240] if stderr.strip() else ""

    span_match = re.search(r" at ([0-9]+)\.\.([0-9]+)", stderr)
    span_start = int(span_match.group(1)) if span_match else None
    span_end = int(span_match.group(2)) if span_match else None
    line, column = offset_to_line_col(source, span_start)
    feature_label = REFERENCE_COVERAGE.feature_label(code, stderr, str(path))

    if code == "UnsupportedSyntax":
        error_type = "parser-or-frontend-unsupported"
    elif code in {
        "UnsupportedBuiltin",
        "UnsupportedDate",
        "UnsupportedRegExp",
        "UnsupportedModule",
        "UnsupportedEval",
        "UnsupportedTypeScriptSyntax",
        "UnsupportedRuntimeSubset",
    }:
        error_type = "unsupported-feature-boundary"
    elif code in {"UnresolvedName", "UnresolvedFunction"}:
        error_type = "resolver-symbol"
    elif code == "BackendIo":
        error_type = "backend-io"
    elif code == "InvariantViolation":
        error_type = "compiler-invariant"
    elif code == "Unknown":
        error_type = "unknown"
    elif code == "BuildPass":
        error_type = "pass"
    else:
        error_type = "compiler-diagnostic"

    return DiagnosticInfo(code, message, span_start, span_end, line, column, feature_label, error_type)


def collect_visible_symbols(source: str, before_offset: int | None) -> list[dict[str, Any]]:
    scan = source if before_offset is None else source[:before_offset]
    patterns = [
        ("function", re.compile(r"\bfunction\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*\(([^)]*)\)")),
        ("class", re.compile(r"\bclass\s+([A-Za-z_$][A-Za-z0-9_$]*)")),
        ("binding", re.compile(r"\b(?:let|const|var)\s+([A-Za-z_$][A-Za-z0-9_$]*)(?:\s*=\s*([^;\n]+))?")),
        ("import", re.compile(r"\bimport\s+(?:[^;\n]+?\s+from\s+)?['\"]([^'\"]+)['\"]")),
    ]
    symbols: list[dict[str, Any]] = []
    for kind, pattern in patterns:
        for match in pattern.finditer(scan):
            line, column = offset_to_line_col(source, match.start())
            item: dict[str, Any] = {
                "kind": kind,
                "name": match.group(1),
                "line": line,
                "column": column,
            }
            if kind == "function":
                item["params"] = match.group(2).strip()
            if kind == "binding" and match.group(2):
                item["initializer"] = match.group(2).strip()[:120]
            symbols.append(item)
    return sorted(symbols, key=lambda item: (item["line"] or 0, item["column"] or 0))[-25:]


def extract_stack_trace(stderr: str) -> list[str]:
    lines = stderr.splitlines()
    stack = []
    capture = False
    for line in lines:
        stripped = line.rstrip()
        if "stack backtrace:" in stripped.lower() or re.match(r"\s*[0-9]+:\s", stripped):
            capture = True
        if capture or re.search(r"\bat\s+[^ ]+", stripped):
            stack.append(stripped)
    return stack[:40]


def dump_phase(path: Path, phase: str, max_chars: int) -> dict[str, Any]:
    result = run_command(
        [str(TS2WASM_BINARY), "dump", phase, str(path)],
        timeout_seconds=12,
    )
    output = result.stdout if result.returncode == 0 else result.stderr
    return {
        "ok": result.returncode == 0,
        "returncode": result.returncode,
        "truncated": len(output) > max_chars,
        "output": output[:max_chars],
    }


def typescript_oracle(path: Path, source: str, diagnostic: DiagnosticInfo) -> dict[str, Any]:
    oracle_script = REPO_ROOT / "scripts" / "check" / "typescript-oracle.js"
    oracle = run_command(["node", str(oracle_script), str(path)], timeout_seconds=12)
    result: dict[str, Any] = {
        "ok": oracle.returncode == 0,
        "returncode": oracle.returncode,
    }
    if oracle.stdout.strip():
        try:
            result["typescript"] = json.loads(oracle.stdout)
        except json.JSONDecodeError:
            result["stdout"] = oracle.stdout[:3000]
    if oracle.stderr.strip():
        result["stderr"] = oracle.stderr[:3000]

    ast_script = r"""
const fs = require("fs");
const ts = require("typescript");
const path = process.argv[1];
const pos = Number(process.argv[2] || 0);
const source = fs.readFileSync(path, "utf8");
const sf = ts.createSourceFile(path, source, ts.ScriptTarget.Latest, true, path.endsWith(".ts") ? ts.ScriptKind.TS : ts.ScriptKind.JS);
function loc(node) {
  const p = sf.getLineAndCharacterOfPosition(node.getStart(sf));
  return { kind: ts.SyntaxKind[node.kind], text: node.getText(sf).slice(0, 120), line: p.line + 1, character: p.character + 1 };
}
const top = sf.statements.slice(0, 20).map(loc);
let pathToPos = [];
function visit(node) {
  if (node.getFullStart() <= pos && pos <= node.getEnd()) {
    pathToPos.push(loc(node));
    ts.forEachChild(node, visit);
  }
}
visit(sf);
process.stdout.write(JSON.stringify({topLevel: top, pathToPosition: pathToPos.slice(-12)}, null, 2));
"""
    if diagnostic.span_start is not None:
        ast = run_command(["node", "-e", ast_script, str(path), str(diagnostic.span_start)], timeout_seconds=12)
        if ast.returncode == 0 and ast.stdout:
            try:
                result["ast"] = json.loads(ast.stdout)
            except json.JSONDecodeError:
                result["ast_stdout"] = ast.stdout[:3000]
        elif ast.stderr:
            result["ast_error"] = ast.stderr[:1200]
    return result


def duplicate_candidates(title: str, rel_path: str, feature_label: str) -> list[dict[str, str]]:
    candidates = []
    issue_dirs = [(REPO_ROOT / "issues" / "open", "open"), (REPO_ROOT / "issues" / "done", "done")]
    title_terms = {term.lower() for term in re.findall(r"[A-Za-z0-9_+-]{4,}", title)}
    for directory, state in issue_dirs:
        if not directory.exists():
            continue
        for path in sorted(directory.glob("*.md")):
            text = path.read_text(encoding="utf-8", errors="replace")
            score = 0
            reasons = []
            if rel_path in text:
                score += 4
                reasons.append("same reference path")
            if feature_label and feature_label in text:
                score += 2
                reasons.append("same feature label")
            issue_title = ""
            match = re.search(r'^title:\s*"?(.+?)"?\s*$', text, re.M)
            if match:
                issue_title = match.group(1).strip().strip('"')
                overlap = title_terms & {term.lower() for term in re.findall(r"[A-Za-z0-9_+-]{4,}", issue_title)}
                if overlap:
                    score += min(3, len(overlap))
                    reasons.append("title overlap")
            if score >= 3:
                candidates.append({
                    "state": state,
                    "path": path.relative_to(REPO_ROOT).as_posix(),
                    "title": issue_title or path.stem,
                    "reason": ", ".join(reasons),
                })
    return candidates[:10]


def rough_rust_suggestion(diagnostic: DiagnosticInfo, source: str) -> str | None:
    text = f"{diagnostic.code} {diagnostic.message} {diagnostic.feature_label}".lower()
    if "class" in text:
        class_match = re.search(r"\bclass\s+([A-Za-z_$][A-Za-z0-9_$]*)", source)
        class_name = class_match.group(1) if class_match else "Example"
        return f"""// Rough sketch only: make class syntax observable before lowering full semantics.
// Candidate source class: {class_name}
#[derive(Debug, Clone, PartialEq)]
pub struct ClassDecl {{
    pub name: String,
    pub constructor: Option<FunctionDecl>,
    pub methods: Vec<MethodDecl>,
    pub span: Span,
}}

fn class_statement(&mut self) -> Result<Stmt, Diagnostic> {{
    let span = self.expect(TokenKind::Class)?;
    let name = self.expect_ident()?;
    self.expect(TokenKind::LeftBrace)?;
    let mut methods = Vec::new();
    while !self.consume(TokenKind::RightBrace) {{
        methods.push(self.class_method()?);
    }}
    Ok(Stmt::ClassDecl(ClassDecl {{ name, constructor: None, methods, span }}))
}}"""
    if diagnostic.code == "UnresolvedName":
        return """// Rough sketch only: make unresolved names inspectable at resolver failure.
if let Some(binding) = self.lookup_name(name) {
    return Ok(binding);
}
return Err(Diagnostic {
    code: DiagCode::UnresolvedName,
    message: format!("unresolved name `{name}`; visible bindings: {:?}", self.visible_names()),
    span,
});"""
    return None


def suggestions_for(diagnostic: DiagnosticInfo) -> list[str]:
    suggestions = []
    if diagnostic.error_type == "pass":
        suggestions.append("No compiler blocker was found by the build step; use reference-coverage for semantic parity evidence.")
        return suggestions
    if diagnostic.error_type == "parser-or-frontend-unsupported":
        suggestions.append("Start at lexer/parser support and add a minimal fixture for the exact source construct at the failing span.")
        suggestions.append("Use `dump --tokens` and the TypeScript AST path to decide whether this is tokenization, precedence, or statement dispatch.")
    if diagnostic.error_type == "resolver-symbol":
        suggestions.append("Check whether the missing name should be a local binding, function binding, builtin, import binding, or runtime global.")
        suggestions.append("Acceptance should assert both the formerly missing symbol and an adjacent negative case.")
    if diagnostic.feature_label in {"builtin-api", "array-builtin", "object-builtin", "string-builtin", "date", "function"}:
        suggestions.append("Classify this as runtime/API work unless the parser fails before builtin resolution.")
    if diagnostic.feature_label in {"import-export", "module-resolution", "module-system-amd"}:
        suggestions.append("Keep module graph behavior separate from parser syntax unless the diagnostic proves syntax is the blocker.")
    if not suggestions:
        suggestions.append("Create a child issue around this exact path and diagnostic before broadening the reference window.")
    return suggestions


def title_for(path: Path, diagnostic: DiagnosticInfo) -> str:
    stem = path.stem.replace("_", " ").replace("-", " ")
    if diagnostic.error_type == "pass":
        return f"Build pass: {stem}"
    feature = diagnostic.feature_label.replace("-", " ")
    return f"Triage {feature}: {stem}"


def prepare_triage_input(
    suite: str, path: Path, tmp_dir: Path
) -> tuple[Path, Path, str, str]:
    source = path.read_text(encoding="utf-8", errors="replace")
    if suite != "test262":
        return path, path, source, source

    metadata = REFERENCE_COVERAGE.test262_runner.parse_test262_metadata(source)
    REFERENCE_COVERAGE.test262_runner.HARNESS_DIR = REFERENCE_COVERAGE.test262_harness_dir_for(path)
    build_input = tmp_dir / "test262-triage-wasm-input.js"
    node_input = tmp_dir / "test262-triage-node-input.js"
    build_source = REFERENCE_COVERAGE.test262_runner.build_test262_source(
        path, source, metadata, target="wasm"
    )
    node_source = REFERENCE_COVERAGE.test262_runner.build_test262_source(
        path, source, metadata, target="node"
    )
    build_input.write_text(build_source, encoding="utf-8")
    node_input.write_text(node_source, encoding="utf-8")
    return build_input, node_input, source, build_source


def build_report(suite: str, path: Path, max_dump_chars: int) -> TriageReport:
    rel_path = repo_relative(path)
    reproduction_command = f"mise run reference-triage -- {suite} {rel_path}"

    with tempfile.TemporaryDirectory() as tmp_dir:
        tmp_path = Path(tmp_dir)
        build_input, oracle_input, source, diagnostic_source = prepare_triage_input(
            suite, path, tmp_path
        )
        out_wasm = Path(tmp_dir) / "out.wasm"
        build = run_command(
            [str(TS2WASM_BINARY), "build", str(build_input), "-o", str(out_wasm)],
            timeout_seconds=12,
        )
        diagnostic = parse_diagnostic(build.returncode, build.stderr, diagnostic_source, build_input)
        title = title_for(path, diagnostic)
        dumps = {
            "tokens": dump_phase(build_input, "--tokens", max_dump_chars),
            "ast": dump_phase(build_input, "--ast", max_dump_chars),
            "resolved": dump_phase(build_input, "--resolved", max_dump_chars),
        }
        if build.returncode != 0 and diagnostic.error_type in {"backend-io", "compiler-invariant"}:
            dumps["wat"] = dump_phase(build_input, "--wat", max_dump_chars)

        return TriageReport(
            suite=suite,
            title=title,
            path=rel_path,
            reproduction_command=reproduction_command,
            issue_class="none" if diagnostic.error_type == "pass" else "triage-needed",
            source_summary=source_summary(path, suite, source),
            diagnostic=diagnostic,
            source_context=line_context(diagnostic_source, diagnostic.line),
            visible_symbols=collect_visible_symbols(diagnostic_source, diagnostic.span_start),
            stack_trace=extract_stack_trace(build.stderr),
            dump=dumps,
            oracle=typescript_oracle(oracle_input, source, diagnostic),
            duplicate_candidates=duplicate_candidates(title, rel_path, diagnostic.feature_label),
            suggestions=suggestions_for(diagnostic),
            rough_rust=rough_rust_suggestion(diagnostic, diagnostic_source),
        )


def fenced(value: str, lang: str = "text") -> str:
    value = value.rstrip()
    if not value:
        value = "(empty)"
    return f"```{lang}\n{value}\n```"


def render_markdown(report: TriageReport) -> str:
    lines = [
        f"### Smart triage: {report.title}",
        "",
        f"- Issue class: `{report.issue_class}`",
        f"- Feature label: `{report.diagnostic.feature_label}`",
        f"- Diagnostic: `{report.diagnostic.code}` / `{report.diagnostic.error_type}`",
        f"- Path: `{report.path}`",
        "",
        "Reproduction:",
        "",
        fenced(report.reproduction_command, "sh"),
        "",
        "Source overview:",
        "",
        fenced(json.dumps(report.source_summary, indent=2, ensure_ascii=False), "json"),
        "",
        "Failure location:",
        "",
        fenced(json.dumps(asdict(report.diagnostic), indent=2, ensure_ascii=False), "json"),
        "",
        "Source context:",
        "",
        fenced("\n".join(report.source_context), "text"),
        "",
        "Visible symbols before failure:",
        "",
        fenced(json.dumps(report.visible_symbols, indent=2, ensure_ascii=False), "json"),
        "",
        "Duplicate candidates:",
        "",
        fenced(json.dumps(report.duplicate_candidates, indent=2, ensure_ascii=False), "json"),
        "",
        "Error-specific suggestions:",
        "",
    ]
    lines.extend(f"- {suggestion}" for suggestion in report.suggestions)
    if report.rough_rust:
        lines.extend(["", "Automatic repair sketch:", "", fenced(report.rough_rust, "rust")])

    lines.extend(["", "Compiler dumps:"])
    for name, dump in report.dump.items():
        lines.extend([
            "",
            f"#### {name}",
            "",
            f"- ok: `{dump['ok']}`",
            f"- truncated: `{dump['truncated']}`",
            "",
            fenced(dump["output"], "text"),
        ])

    lines.extend([
        "",
        "TypeScript/JavaScript oracle:",
        "",
        fenced(json.dumps(report.oracle, indent=2, ensure_ascii=False)[:8000], "json"),
    ])
    if report.stack_trace:
        lines.extend(["", "Stack trace:", "", fenced("\n".join(report.stack_trace), "text")])
    return "\n".join(lines)


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--format", choices=["markdown", "json"], default="markdown")
    parser.add_argument("--max-dump-chars", type=int, default=5000)
    parser.add_argument("suite", choices=sorted(REFERENCE_COVERAGE.SUITE_METADATA.keys()))
    parser.add_argument("path")
    return parser.parse_args(argv)


def main(argv: list[str]) -> int:
    args = parse_args(argv)
    path = resolve_input(args.suite, args.path)
    report = build_report(args.suite, path, args.max_dump_chars)
    if args.format == "json":
        print(json.dumps(asdict(report), indent=2, ensure_ascii=False))
    else:
        print(render_markdown(report))
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
