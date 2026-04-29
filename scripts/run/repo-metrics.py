#!/usr/bin/env python3
"""Repository line, byte, and content-kind metrics.

Usage:
  mise run repo-metrics -- [options]

Options:
  --root <path>              Repo root or subdir (default: .)
  --mode <auto|git|walk>     File discovery mode (default: auto)
  --suffix-mode <all|last>   Extension grouping mode (default: all)
  --max-bytes <n>            Skip text counting above this size (default: 5000000, 0 disables)
  --binary <skip|bytes>      Skip binaries or count only file size (default: skip)
  --csv <path>               Write flattened CSV
  --json <path>              Write structured JSON
"""

from __future__ import annotations

import argparse
import csv
import json
import re
import subprocess
import sys
from dataclasses import dataclass, field
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]

MODE_CHOICES = ("auto", "git", "walk")
SUFFIX_MODE_CHOICES = ("last", "all")
BINARY_MODE_CHOICES = ("skip", "bytes")
AREA_KINDS = ("top_level_docs_tests", "source_tree", "other")
LINE_KINDS = ("blank", "source", "doc_comment", "document", "test", "comment", "other")

TOP_LEVEL_DOC_TEST_DIRS = {"tests", "tutorials", "doc", "examples"}
SOURCE_EXTS = {
    ".c",
    ".cpp",
    ".css",
    ".h",
    ".hpp",
    ".html",
    ".java",
    ".js",
    ".jsx",
    ".mjs",
    ".mts",
    ".nepl",
    ".py",
    ".rb",
    ".rs",
    ".sh",
    ".sql",
    ".ts",
    ".tsx",
    ".wat",
    ".wast",
    ".wasm",
    ".yaml",
    ".yml",
}
MARKDOWN_EXTS = {".md", ".n.md"}

DOCTEST_META_RE = re.compile(r"^\s*(stdin|argv|stdout|stderr|ret|diag_id|diag_ids|diag_span|diag_spans)\s*:\s*(.*?)\s*$")
DOCTEST_RE = re.compile(r"^\s*neplg2:test(?:\[[^\]]+\])?\s*$")
DOCTEST_FENCE_OPEN_RE = re.compile(r"^\s*```neplg2\s*$")
DOCTEST_FENCE_CLOSE_RE = re.compile(r"^\s*```\s*$")
NEPL_DOC_RE = re.compile(r"^\s*//:(\|)?\s?(.*)$")
RUST_DOC_RE = re.compile(r"^\s*(///|//!)")
RUST_COMMENT_RE = re.compile(r"^\s*//")
RUST_CFG_TEST_RE = re.compile(r"^\s*#\[\s*cfg\s*\(\s*test\s*\)\s*\]")
RUST_TEST_ATTR_RE = re.compile(r"^\s*#\[(?:test|tokio::test|wasm_bindgen_test)\b")
RUST_FN_RE = re.compile(r"^\s*(?:pub\s+)?(?:async\s+)?fn\b")


def verify_script_repo_root() -> None:
    if not (REPO_ROOT / "README.md").is_file() and not (REPO_ROOT / ".git").exists():
        raise RuntimeError(f"script repo root could not be verified: {REPO_ROOT}")


@dataclass
class TextLine:
    text: str
    raw_bytes: int


@dataclass
class FileStats:
    lines: int = 0
    chars: int = 0
    bytes: int = 0
    blank: int = 0
    source: int = 0
    doc_comment: int = 0
    document: int = 0
    test: int = 0
    comment: int = 0
    other: int = 0
    test_cases: int = 0
    kind_chars: dict[str, int] = field(default_factory=lambda: {kind: 0 for kind in LINE_KINDS})
    kind_bytes: dict[str, int] = field(default_factory=lambda: {kind: 0 for kind in LINE_KINDS})


@dataclass
class BucketStats(FileStats):
    files: int = 0


@dataclass
class SimpleStats:
    files: int = 0
    lines: int = 0
    chars: int = 0
    bytes: int = 0
    test_cases: int = 0


def run(cmd: list[str], cwd: Path) -> bytes:
    proc = subprocess.run(cmd, cwd=cwd, stdin=subprocess.DEVNULL, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if proc.returncode != 0:
        err = proc.stderr.decode("utf-8", errors="replace").strip()
        raise RuntimeError(err or f"Command failed: {' '.join(cmd)}")
    return proc.stdout


def is_git_repo(path: Path) -> bool:
    try:
        run(["git", "rev-parse", "--is-inside-work-tree"], path)
        return True
    except RuntimeError:
        return False


def git_root(path: Path) -> Path:
    return Path(run(["git", "rev-parse", "--show-toplevel"], path).decode("utf-8").strip())


def list_git_tracked_and_unignored(root: Path) -> list[str]:
    out = run(["git", "ls-files", "-z", "--cached", "--others", "--exclude-standard"], root)
    return [value.strip() for value in out.decode("utf-8", errors="replace").split("\0") if value.strip()]


def list_files_walk(root: Path) -> list[str]:
    out: list[str] = []
    stack = [root]
    while stack:
        current = stack.pop()
        for entry in current.iterdir():
            if entry.is_dir():
                if entry.name == ".git":
                    continue
                stack.append(entry)
            elif entry.is_file():
                out.append(entry.relative_to(root).as_posix())
    out.sort()
    return out


def is_probably_binary(path: Path, sample_size: int = 8192) -> bool:
    try:
        return b"\0" in path.read_bytes()[:sample_size]
    except OSError:
        return True


def suffix(rel_path: str) -> str:
    name = rel_path.rsplit("/", 1)[-1]
    idx = name.rfind(".")
    return name[idx:].lower() if idx >= 0 else ""


def ext_key(rel_path: str, suffix_mode: str) -> str:
    name = rel_path.rsplit("/", 1)[-1]
    if suffix_mode == "all":
        idx = name.find(".")
        return name[idx:].lower() if idx >= 0 else "(no_ext)"
    return suffix(rel_path) or "(no_ext)"


def classify_area(rel_path: str) -> str:
    parts = [part for part in rel_path.split("/") if part]
    if not parts:
        return "other"
    if parts[0] in TOP_LEVEL_DOC_TEST_DIRS:
        return "top_level_docs_tests"
    if parts[0] == "stdlib" or "src" in parts:
        return "source_tree"
    return "other"


def is_test_path(rel_path: str) -> bool:
    return "tests" in rel_path.split("/")


def increment_kind(stats: FileStats, kind: str, amount: int = 1) -> None:
    setattr(stats, kind, getattr(stats, kind) + amount)


def add_line(stats: FileStats, kind: str, line: TextLine) -> None:
    stats.lines += 1
    stats.chars += len(line.text)
    stats.bytes += line.raw_bytes
    if line.text.strip() == "":
        stats.blank += 1
        stats.kind_chars["blank"] += len(line.text)
        stats.kind_bytes["blank"] += line.raw_bytes
        return
    increment_kind(stats, kind)
    stats.kind_chars[kind] += len(line.text)
    stats.kind_bytes[kind] += line.raw_bytes


def split_raw_lines(raw: bytes) -> list[bytes]:
    matches = re.findall(rb"[^\r\n]*(?:\r\n|\r|\n|$)", raw)
    if matches and matches[-1] == b"":
        matches.pop()
    return matches


def read_text_lines(path: Path, max_bytes: int | None) -> list[TextLine]:
    raw = path.read_bytes()
    if max_bytes is not None and max_bytes > 0 and len(raw) > max_bytes:
        raise RuntimeError(f"file too large ({len(raw)} bytes) > maxBytes")
    return [
        TextLine(raw_line.decode("utf-8", errors="replace"), len(raw_line))
        for raw_line in split_raw_lines(raw)
    ]


def strip_newline(text: str) -> str:
    return re.sub(r"[\r\n]+$", "", text)


def classify_markdown_lines(lines: list[TextLine]) -> FileStats:
    stats = FileStats()
    state = "document"

    for line in lines:
        stripped = strip_newline(line.text)
        if state == "document":
            if DOCTEST_RE.match(stripped):
                add_line(stats, "test", line)
                stats.test_cases += 1
                state = "await_fence"
            else:
                add_line(stats, "document", line)
            continue

        if state == "await_fence":
            if DOCTEST_META_RE.match(stripped):
                add_line(stats, "test", line)
            elif DOCTEST_FENCE_OPEN_RE.match(stripped):
                add_line(stats, "test", line)
                state = "in_fence"
            else:
                add_line(stats, "document", line)
                state = "document"
            continue

        add_line(stats, "test", line)
        if DOCTEST_FENCE_CLOSE_RE.match(stripped):
            state = "document"

    return stats


def classify_nepl_lines(rel_path: str, lines: list[TextLine]) -> FileStats:
    stats = FileStats()
    test_file = is_test_path(rel_path)
    state = "document_comment"

    for line in lines:
        stripped = strip_newline(line.text)
        match = NEPL_DOC_RE.match(stripped)
        if match:
            doc_text = f"{'|' if match.group(1) else ''}{match.group(2) or ''}"
            if state == "document_comment":
                if DOCTEST_RE.match(doc_text):
                    add_line(stats, "test", line)
                    stats.test_cases += 1
                    state = "await_fence"
                else:
                    add_line(stats, "doc_comment", line)
            elif state == "await_fence":
                if DOCTEST_META_RE.match(doc_text):
                    add_line(stats, "test", line)
                elif DOCTEST_FENCE_OPEN_RE.match(doc_text):
                    add_line(stats, "test", line)
                    state = "in_fence"
                else:
                    add_line(stats, "doc_comment", line)
                    state = "document_comment"
            else:
                add_line(stats, "test", line)
                if DOCTEST_FENCE_CLOSE_RE.match(doc_text):
                    state = "document_comment"
            continue

        if line.text.strip() == "":
            add_line(stats, "other", line)
        elif test_file:
            add_line(stats, "test", line)
        elif stripped.lstrip().startswith("//"):
            add_line(stats, "comment", line)
        else:
            add_line(stats, "source", line)

    return stats


def classify_rust_lines(rel_path: str, lines: list[TextLine]) -> FileStats:
    stats = FileStats()
    test_file = is_test_path(rel_path)
    brace_depth = 0
    test_region_ends: list[int] = []
    pending_cfg_test = False
    pending_test_attr = False

    for line in lines:
        stripped = strip_newline(line.text)
        logical = stripped.strip()
        in_test_region = test_file or len(test_region_ends) > 0
        is_cfg_test = RUST_CFG_TEST_RE.match(stripped) is not None
        is_test_attr = RUST_TEST_ATTR_RE.match(stripped) is not None
        is_doc = RUST_DOC_RE.match(stripped) is not None

        if logical == "":
            add_line(stats, "other", line)
        elif is_cfg_test or is_test_attr:
            add_line(stats, "test", line)
            if is_cfg_test:
                pending_cfg_test = True
            if is_test_attr:
                pending_test_attr = True
                stats.test_cases += 1
        elif is_doc:
            add_line(stats, "doc_comment", line)
        elif pending_cfg_test or pending_test_attr or in_test_region:
            add_line(stats, "test", line)
        elif RUST_COMMENT_RE.match(stripped):
            add_line(stats, "comment", line)
        else:
            add_line(stats, "source", line)

        depth_before = brace_depth
        opens = stripped.count("{")
        closes = stripped.count("}")

        if pending_cfg_test and logical != "" and not is_cfg_test:
            if "{" in stripped:
                test_region_ends.append(depth_before)
                pending_cfg_test = False
            elif stripped.endswith(";"):
                pending_cfg_test = False

        if pending_test_attr and logical != "" and not is_test_attr:
            if RUST_FN_RE.match(stripped) and "{" in stripped:
                test_region_ends.append(depth_before)
                pending_test_attr = False
            elif not stripped.startswith("#[") and "{" in stripped:
                test_region_ends.append(depth_before)
                pending_test_attr = False
            elif stripped.endswith(";"):
                pending_test_attr = False

        brace_depth += opens - closes
        while test_region_ends and brace_depth <= test_region_ends[-1]:
            test_region_ends.pop()

    return stats


def classify_generic_lines(rel_path: str, lines: list[TextLine]) -> FileStats:
    stats = FileStats()
    key = ext_key(rel_path, "all")
    test_file = is_test_path(rel_path)
    is_markdown = key in MARKDOWN_EXTS or suffix(rel_path) == ".md"
    is_source = suffix(rel_path) in SOURCE_EXTS

    for line in lines:
        if line.text.strip() == "":
            add_line(stats, "other", line)
        elif is_markdown:
            add_line(stats, "document", line)
        elif test_file:
            add_line(stats, "test", line)
        elif is_source:
            add_line(stats, "source", line)
        else:
            add_line(stats, "other", line)

    return stats


def measure_text_file(rel_path: str, abs_path: Path, max_bytes: int | None) -> FileStats:
    lines = read_text_lines(abs_path, max_bytes)
    key = ext_key(rel_path, "all")
    file_suffix = suffix(rel_path)
    if key == ".n.md" or file_suffix == ".md":
        return classify_markdown_lines(lines)
    if file_suffix == ".nepl":
        return classify_nepl_lines(rel_path, lines)
    if file_suffix == ".rs":
        return classify_rust_lines(rel_path, lines)
    return classify_generic_lines(rel_path, lines)


def accumulate_bucket(dest: BucketStats, src: FileStats) -> None:
    dest.lines += src.lines
    dest.chars += src.chars
    dest.bytes += src.bytes
    dest.blank += src.blank
    dest.source += src.source
    dest.doc_comment += src.doc_comment
    dest.document += src.document
    dest.test += src.test
    dest.comment += src.comment
    dest.other += src.other
    dest.test_cases += src.test_cases


def sort_buckets(entries):
    return sorted(entries, key=lambda item: (-item[1].bytes, -item[1].lines, -item[1].files, item[0]))


def format_num(value: int) -> str:
    return f"{value:,}"


def calc_widths(headers: list[str], data: list[list[str]]) -> list[int]:
    widths = [len(header) for header in headers]
    for row in data:
        for idx, cell in enumerate(row):
            widths[idx] = max(widths[idx], len(cell))
    return widths


def format_row(row: list[str], widths: list[int]) -> str:
    return "  ".join(cell.ljust(widths[idx]) if idx == 0 else cell.rjust(widths[idx]) for idx, cell in enumerate(row))


def print_bucket_table(title: str, key_name: str, stats: dict[str, BucketStats]) -> None:
    rows = sort_buckets(stats.items())
    headers = [
        key_name,
        "files",
        "lines",
        "chars",
        "bytes",
        "blank",
        "source",
        "doc_comment",
        "document",
        "test",
        "comment",
        "other",
        "test_cases",
    ]
    data = [
        [
            key,
            format_num(stat.files),
            format_num(stat.lines),
            format_num(stat.chars),
            format_num(stat.bytes),
            format_num(stat.blank),
            format_num(stat.source),
            format_num(stat.doc_comment),
            format_num(stat.document),
            format_num(stat.test),
            format_num(stat.comment),
            format_num(stat.other),
            format_num(stat.test_cases),
        ]
        for key, stat in rows
    ]
    widths = calc_widths(headers, data)
    print(title)
    print(format_row(headers, widths))
    print(format_row(["-" * len(header) for header in headers], widths))
    for row in data:
        print(format_row(row, widths))

    total = BucketStats()
    for stat in stats.values():
        total.files += stat.files
        accumulate_bucket(total, stat)

    print()
    print(
        format_row(
            [
                "TOTAL",
                format_num(total.files),
                format_num(total.lines),
                format_num(total.chars),
                format_num(total.bytes),
                format_num(total.blank),
                format_num(total.source),
                format_num(total.doc_comment),
                format_num(total.document),
                format_num(total.test),
                format_num(total.comment),
                format_num(total.other),
                format_num(total.test_cases),
            ],
            widths,
        )
    )


def print_simple_table(title: str, key_name: str, stats: dict[str, SimpleStats]) -> None:
    rows = sort_buckets(stats.items())
    headers = [key_name, "files", "lines", "chars", "bytes", "test_cases"]
    data = [
        [
            key,
            format_num(stat.files),
            format_num(stat.lines),
            format_num(stat.chars),
            format_num(stat.bytes),
            format_num(stat.test_cases),
        ]
        for key, stat in rows
    ]
    widths = calc_widths(headers, data)
    print(title)
    print(format_row(headers, widths))
    print(format_row(["-" * len(header) for header in headers], widths))
    for row in data:
        print(format_row(row, widths))

    total = SimpleStats()
    for stat in stats.values():
        total.files += stat.files
        total.lines += stat.lines
        total.chars += stat.chars
        total.bytes += stat.bytes
        total.test_cases += stat.test_cases

    print()
    print(
        format_row(
            [
                "TOTAL",
                format_num(total.files),
                format_num(total.lines),
                format_num(total.chars),
                format_num(total.bytes),
                format_num(total.test_cases),
            ],
            widths,
        )
    )


def write_csv(path: Path, ext_stats: dict[str, BucketStats], area_stats: dict[str, BucketStats], kind_stats: dict[str, SimpleStats]) -> None:
    with path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(
            [
                "section",
                "name",
                "files",
                "lines",
                "chars",
                "bytes",
                "blank",
                "source",
                "doc_comment",
                "document",
                "test",
                "comment",
                "other",
                "test_cases",
            ]
        )
        for section, table in (("extension", ext_stats), ("area", area_stats)):
            for name, stat in sorted(table.items()):
                writer.writerow(
                    [
                        section,
                        name,
                        stat.files,
                        stat.lines,
                        stat.chars,
                        stat.bytes,
                        stat.blank,
                        stat.source,
                        stat.doc_comment,
                        stat.document,
                        stat.test,
                        stat.comment,
                        stat.other,
                        stat.test_cases,
                    ]
                )
        for name, stat in sorted(kind_stats.items()):
            writer.writerow(
                [
                    "content_kind",
                    name,
                    stat.files,
                    stat.lines,
                    stat.chars,
                    stat.bytes,
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                    stat.test_cases,
                ]
            )


def bucket_payload(name: str, stat: BucketStats) -> dict[str, int | str]:
    return {
        "name": name,
        "files": stat.files,
        "lines": stat.lines,
        "chars": stat.chars,
        "bytes": stat.bytes,
        "blank": stat.blank,
        "source": stat.source,
        "doc_comment": stat.doc_comment,
        "document": stat.document,
        "test": stat.test,
        "comment": stat.comment,
        "other": stat.other,
        "testCases": stat.test_cases,
    }


def write_json(
    path: Path,
    ext_stats: dict[str, BucketStats],
    area_stats: dict[str, BucketStats],
    kind_stats: dict[str, SimpleStats],
    skipped: list[dict[str, str]],
) -> None:
    payload = {
        "byExtension": [bucket_payload(name, stat) for name, stat in sorted(ext_stats.items())],
        "byArea": [bucket_payload(name, stat) for name, stat in sorted(area_stats.items())],
        "byContentKind": [
            {
                "name": name,
                "files": stat.files,
                "lines": stat.lines,
                "chars": stat.chars,
                "bytes": stat.bytes,
                "testCases": stat.test_cases,
            }
            for name, stat in sorted(kind_stats.items())
        ],
        "skipped": skipped,
    }
    path.write_text(f"{json.dumps(payload, indent=2)}\n", encoding="utf-8")


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        prog="mise run repo-metrics --",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        description="Repository line, byte, and content-kind metrics.",
    )
    parser.add_argument("--root", default=".", help="Repo root or subdir (default: .)")
    parser.add_argument("--mode", choices=MODE_CHOICES, default="auto")
    parser.add_argument("--suffix-mode", choices=SUFFIX_MODE_CHOICES, default="all")
    parser.add_argument("--max-bytes", type=int, default=5_000_000)
    parser.add_argument("--binary", choices=BINARY_MODE_CHOICES, default="skip")
    parser.add_argument("--csv")
    parser.add_argument("--json")
    return parser.parse_args(argv)


def ensure_bucket(stats: dict[str, BucketStats], key: str) -> BucketStats:
    if key not in stats:
        stats[key] = BucketStats()
    return stats[key]


def main(argv: list[str]) -> int:
    verify_script_repo_root()
    args = parse_args(argv)
    root = Path(args.root).resolve()
    use_git = False

    if args.mode in ("auto", "git"):
        use_git = is_git_repo(root)
        if args.mode == "git" and not use_git:
            print("ERROR: --mode git but not inside a Git repository.", file=sys.stderr)
            return 2

    if use_git:
        root = git_root(root)
        rel_paths = list_git_tracked_and_unignored(root)
    else:
        rel_paths = list_files_walk(root)

    ext_stats: dict[str, BucketStats] = {}
    area_stats: dict[str, BucketStats] = {}
    kind_stats: dict[str, SimpleStats] = {}
    skipped: list[dict[str, str]] = []
    max_bytes = None if args.max_bytes == 0 else args.max_bytes

    for rel_path in rel_paths:
        abs_path = (root / rel_path).resolve()
        try:
            stat = abs_path.stat()
        except OSError:
            skipped.append({"path": rel_path, "reason": "unreadable"})
            continue
        if not abs_path.is_file():
            continue

        ext = ext_key(rel_path, args.suffix_mode)
        area = classify_area(rel_path)

        if is_probably_binary(abs_path):
            if args.binary == "skip":
                skipped.append({"path": rel_path, "reason": "binary"})
                continue
            ext_bucket = ensure_bucket(ext_stats, ext)
            area_bucket = ensure_bucket(area_stats, area)
            ext_bucket.files += 1
            ext_bucket.bytes += stat.st_size
            area_bucket.files += 1
            area_bucket.bytes += stat.st_size
            continue

        try:
            measured = measure_text_file(rel_path, abs_path, max_bytes)
        except (OSError, UnicodeError, RuntimeError) as error:
            msg = str(error)
            skipped.append({"path": rel_path, "reason": "too_large" if "too large" in msg else "unreadable"})
            continue

        ext_bucket = ensure_bucket(ext_stats, ext)
        area_bucket = ensure_bucket(area_stats, area)
        ext_bucket.files += 1
        area_bucket.files += 1
        accumulate_bucket(ext_bucket, measured)
        accumulate_bucket(area_bucket, measured)

        for kind in LINE_KINDS:
            lines = getattr(measured, kind)
            if lines <= 0:
                continue
            if kind not in kind_stats:
                kind_stats[kind] = SimpleStats()
            bucket = kind_stats[kind]
            bucket.files += 1
            bucket.lines += lines
            bucket.chars += measured.kind_chars[kind]
            bucket.bytes += measured.kind_bytes[kind]
            bucket.test_cases += measured.test_cases if kind == "test" else 0

    print_bucket_table("By Extension", "ext", ext_stats)
    print()
    print_bucket_table("By Area", "area", area_stats)
    print()
    print_simple_table("By Content Kind", "kind", kind_stats)

    if skipped:
        print()
        print(f"Skipped files: {len(skipped)} (showing up to 20)")
        for item in skipped[:20]:
            print(f"  - {item['path']} [{item['reason']}]")
        if len(skipped) > 20:
            print("  ...")

    if args.csv:
        write_csv(Path(args.csv), ext_stats, area_stats, kind_stats)
    if args.json:
        write_json(Path(args.json), ext_stats, area_stats, kind_stats, skipped)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
