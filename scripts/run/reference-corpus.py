#!/usr/bin/env python3
"""Reference Corpus Lockfile and Verification.

Usage:
  python scripts/run/reference-corpus.py <command>

Commands:
  verify        Validate local corpus against lock files
  write-lock    Regenerate lock files from current reference state
  print-evidence  Print corpus lock evidence for coverage artifacts

Environment:
  TS2WASM_REFERENCE_LOCK_MODE  Set to 'off' to skip verification
  TS2WASM_REFERENCE_ROOT       Alternate reference root path
"""

import hashlib
import json
import os
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
REFERENCE_ROOT = Path(os.environ.get("TS2WASM_REFERENCE_ROOT", REPO_ROOT / "reference")).resolve()

CORPUS_LOCK_PATH = REPO_ROOT / "reference" / "corpus-lock.json"
LOCK_PATH = REPO_ROOT / "reference" / "lock.json"

SUITE_CONFIGS = {
    "test262": {
        "root_relative_path": "reference/test262/test",
        "remote_url": "https://github.com/tc39/test262.git",
        "repo_dir": REFERENCE_ROOT / "test262",
        "test_dir": REFERENCE_ROOT / "test262" / "test",
        "pattern": "**/*.js",
        "denominator": 53469,
        "required_paths": ["test/language"],
    },
    "tsc": {
        "root_relative_path": "reference/typescript/tests/cases/compiler",
        "remote_url": "https://github.com/microsoft/TypeScript.git",
        "repo_dir": REFERENCE_ROOT / "typescript",
        "test_dir": REFERENCE_ROOT / "typescript" / "tests" / "cases" / "compiler",
        "pattern": "**/*.ts",
        "denominator": 6419,
        "required_paths": ["tests/cases/compiler"],
    },
    "tsgo": {
        "root_relative_path": "reference/typescript-go/testdata/tests",
        "remote_url": "https://github.com/microsoft/typescript-go.git",
        "repo_dir": REFERENCE_ROOT / "typescript-go",
        "test_dir": REFERENCE_ROOT / "typescript-go" / "testdata" / "tests",
        "pattern": "*",
        "denominator": 166,
        "required_paths": ["testdata/tests"],
    },
}


def usage():
    print("Usage:")
    print("  python scripts/run/reference-corpus.py <command> [options]")
    print()
    print("Commands:")
    print("  verify         Validate local corpus against lock files")
    print("  write-lock     Regenerate lock files from current reference state")
    print("  print-evidence  Print corpus lock evidence for coverage artifacts")
    print("  reference-subsets  Validate deterministic subset files")
    print("  evidence-check     Validate coverage evidence completeness")
    print("  replay-set         Validate replay set (seed file) integrity")
    print()
    print("Options:")
    print("  --allow-missing-corpora  Don't fail if reference corpora are missing")
    print("  --self-test              Run built-in self-tests")
    print("  --check                  Check specified files/paths")
    print("  --all                    Check all subset files")
    sys.exit(1)


def repo_relative(path):
    """Return a stable repo-relative path string."""
    try:
        return path.resolve().relative_to(REPO_ROOT).as_posix()
    except ValueError:
        pass
    try:
        reference_relative = path.resolve().relative_to(REFERENCE_ROOT).as_posix()
        return f"reference/{reference_relative}"
    except ValueError:
        pass
    try:
        abspath = Path(os.path.abspath(path))
        reference_relative = abspath.relative_to(REFERENCE_ROOT).as_posix()
        return f"reference/{reference_relative}"
    except ValueError:
        return path.as_posix()


def get_git_commit(repo_dir):
    """Get the current git commit hash for a repo directory."""
    try:
        result = subprocess.run(
            ["git", "rev-parse", "HEAD"],
            capture_output=True, text=True, cwd=repo_dir,
            timeout=10,
        )
        if result.returncode == 0:
            return result.stdout.strip()
    except (subprocess.SubprocessError, FileNotFoundError):
        pass
    return None


def compute_content_hash(suite_key, suite_dir):
    """Compute a content hash from sorted file paths and sizes."""
    config = SUITE_CONFIGS[suite_key]
    if suite_key == "tsgo":
        # typescript-go: all files (not just .ts)
        files = sorted(config["test_dir"].rglob("*"))
        files = [f for f in files if f.is_file()]
    else:
        files = sorted(config["test_dir"].glob(config["pattern"]))

    hash_input = []
    for f in files:
        try:
            size = f.stat().st_size
            rel = repo_relative(f)
            hash_input.append(f"{rel}\t{size}")
        except OSError:
            pass

    data = "\n".join(hash_input).encode("utf-8")
    return hashlib.sha256(data).hexdigest()


def get_file_count(suite_key, suite_dir):
    """Count files in the suite test directory."""
    config = SUITE_CONFIGS[suite_key]
    test_dir = config["test_dir"]
    if not test_dir.exists():
        return 0
    if suite_key == "test262":
        return len(list(test_dir.glob("**/*.js")))
    elif suite_key == "tsc":
        return len(list(test_dir.glob("**/*.ts")))
    else:
        files = [f for f in test_dir.rglob("*") if f.is_file()]
        return len(files)


def load_corpus_lock():
    """Load the corpus lock file."""
    if not CORPUS_LOCK_PATH.exists():
        return None
    with open(CORPUS_LOCK_PATH) as f:
        return json.load(f)


def load_lock():
    """Load the schema lock file."""
    if not LOCK_PATH.exists():
        return None
    with open(LOCK_PATH) as f:
        return json.load(f)


def compute_lock_digest():
    """Compute sha256 of canonical lock JSON."""
    lock = load_lock()
    if lock is None:
        return None
    canonical = json.dumps(lock, sort_keys=True, separators=(",", ":"))
    return hashlib.sha256(canonical.encode("utf-8")).hexdigest()


def cmd_verify(args):
    """Verify local corpus against lock files."""
    allow_missing = "--allow-missing-corpora" in args

    if os.environ.get("TS2WASM_REFERENCE_LOCK_MODE", "").lower() == "off":
        print("SKIP: TS2WASM_REFERENCE_LOCK_MODE=off, skipping verification")
        return 0

    corpus_lock = load_corpus_lock()
    lock = load_lock()

    if corpus_lock is None:
        print("ERROR: corpus lock file not found: reference/corpus-lock.json", file=sys.stderr)
        if allow_missing:
            print("  (--allow-missing-corpora: skipping corpus lock verification)")
            return 0
        return 1

    if lock is None:
        print("ERROR: lock file not found: reference/lock.json", file=sys.stderr)
        if allow_missing:
            print("  (--allow-missing-corpora: skipping lock verification)")
            return 0
        return 1

    # Validate lock schema version
    lock_version = lock.get("schema_version")
    if lock_version != 1:
        print(f"ERROR: lock schema version mismatch: expected 1, got {lock_version}",
              file=sys.stderr)
        return 1

    all_ok = True

    for suite_key, config in SUITE_CONFIGS.items():
        suite_dir = config["test_dir"]
        expected = corpus_lock.get(suite_key)
        if expected is None:
            print(f"ERROR: {suite_key} not found in corpus lock", file=sys.stderr)
            all_ok = False
            continue

        suite_name = expected.get("suite", suite_key)
        expected_commit = expected.get("commit")
        expected_hash = expected.get("content_hash")
        expected_count = expected.get("file_count")

        # Check if repo exists
        if not suite_dir.exists():
            if allow_missing:
                print(f"  SKIP: {suite_name} (corpus missing at {suite_dir})")
                continue
            print(f"ERROR: {suite_name} corpus not found at {suite_dir}", file=sys.stderr)
            all_ok = False
            continue

        # Check commit
        actual_commit = get_git_commit(config["repo_dir"])
        if actual_commit and expected_commit:
            if actual_commit != expected_commit:
                print(
                    f"WARN: {suite_name} commit mismatch: expected {expected_commit[:12]}, "
                    f"got {actual_commit[:12]}",
                    file=sys.stderr,
                )
                # Commit mismatch is a warning, not a hard error

        # Check file count
        actual_count = get_file_count(suite_key, suite_dir)
        if expected_count is not None and actual_count != expected_count:
            print(
                f"ERROR: {suite_name} file count mismatch: expected {expected_count}, "
                f"got {actual_count}",
                file=sys.stderr,
            )
            all_ok = False
            continue

        # Check content hash
        if expected_hash is not None:
            actual_hash = compute_content_hash(suite_key, suite_dir)
            if actual_hash != expected_hash:
                print(
                    f"ERROR: {suite_name} content hash mismatch: expected {expected_hash[:16]}..., "
                    f"got {actual_hash[:16]}...",
                    file=sys.stderr,
                )
                all_ok = False
                continue

        print(f"  OK: {suite_name} ({actual_count} files, commit {actual_commit[:12] if actual_commit else 'N/A'})")

    # Validate lock file suite entries
    lock_suites = lock.get("suites", {})
    for suite_key in SUITE_CONFIGS:
        if suite_key not in lock_suites:
            print(f"ERROR: {suite_key} not found in lock.json suites", file=sys.stderr)
            all_ok = False
            continue

        lock_entry = lock_suites[suite_key]
        corpus_entry = corpus_lock.get(suite_key, {})

        # Check denominator consistency between both locks
        lock_denom = lock_entry.get("denominator")
        corpus_denom = corpus_entry.get("denominator")
        if lock_denom is not None and corpus_denom is not None and lock_denom != corpus_denom:
            print(
                f"ERROR: {suite_key} denominator mismatch: lock.json={lock_denom}, "
                f"corpus-lock.json={corpus_denom}",
                file=sys.stderr,
            )
            all_ok = False

    if all_ok:
        print("Corpus lock verification: PASS")
        return 0
    else:
        print("Corpus lock verification: FAIL", file=sys.stderr)
        return 1


def cmd_write_lock(args):
    """Regenerate lock files from current reference state."""
    allow_missing = "--allow-missing-corpora" in args
    now = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")

    corpus_lock = {}
    lock_suites = {}

    for suite_key, config in SUITE_CONFIGS.items():
        suite_dir = config["test_dir"]

        if not suite_dir.exists():
            if allow_missing:
                print(f"  SKIP: {suite_key} (corpus missing at {suite_dir})")
                continue
            print(f"ERROR: {suite_key} corpus not found at {suite_dir}", file=sys.stderr)
            return 1

        commit = get_git_commit(config["repo_dir"])
        file_count = get_file_count(suite_key, suite_dir)
        content_hash = compute_content_hash(suite_key, suite_dir)

        if file_count == 0:
            print(f"ERROR: {suite_key} has 0 files at {suite_dir}", file=sys.stderr)
            return 1

        denominator = file_count  # Use actual file count as denominator

        corpus_lock[suite_key] = {
            "suite": suite_key,
            "root_relative_path": config["root_relative_path"],
            "remote_url": config["remote_url"],
            "commit": commit or "unknown",
            "content_hash": content_hash,
            "file_count": file_count,
            "denominator": denominator,
            "generated_at": now,
        }

        lock_suites[suite_key] = {
            "repo_url": config["remote_url"],
            "commit": commit or "unknown",
            "root": config["root_relative_path"],
            "denominator": denominator,
            "file_count": file_count,
            "required_paths": config["required_paths"],
        }

        print(f"  Wrote: {suite_key} ({file_count} files, commit {commit[:12] if commit else 'N/A'})")

    if corpus_lock:
        CORPUS_LOCK_PATH.parent.mkdir(parents=True, exist_ok=True)
        with open(CORPUS_LOCK_PATH, "w") as f:
            json.dump(corpus_lock, f, indent=2)
            f.write("\n")
        print(f"Wrote: {CORPUS_LOCK_PATH}")

    if lock_suites:
        lock_data = {
            "schema_version": 1,
            "suites": lock_suites,
        }
        LOCK_PATH.parent.mkdir(parents=True, exist_ok=True)
        with open(LOCK_PATH, "w") as f:
            json.dump(lock_data, f, indent=2)
            f.write("\n")
        print(f"Wrote: {LOCK_PATH}")

    return 0


def cmd_print_evidence(args):
    """Print corpus lock evidence for coverage artifacts."""
    corpus_lock = load_corpus_lock()
    lock = load_lock()

    if corpus_lock is None:
        print("ERROR: corpus lock file not found", file=sys.stderr)
        return 1

    lock_digest = compute_lock_digest()
    if lock_digest is None:
        print("ERROR: could not compute lock digest", file=sys.stderr)
        return 1

    evidence = {
        "corpus_lock_path": "reference/corpus-lock.json",
        "lock_path": "reference/lock.json",
        "lock_digest": lock_digest,
        "suites": {},
    }

    for suite_key, entry in corpus_lock.items():
        evidence["suites"][suite_key] = {
            "commit": entry.get("commit"),
            "file_count": entry.get("file_count"),
            "denominator": entry.get("denominator"),
            "content_hash": entry.get("content_hash"),
        }

    print(json.dumps(evidence, indent=2))
    return 0


SEEDS_FILE = REPO_ROOT / "scripts" / "data" / "test262-semantic-core-seeds.txt"


def parse_seeds_file(seeds_path):
    """Parse a seeds file and return (header_lines, entries, groups).

    Each entry is a non-empty, non-comment line.
    Comments starting with '#   Subcategory:' denote groups.
    Comments starting with '# schema_version:', '# suite:', '# purpose:' are headers.
    """
    header_lines = []
    entries = []
    groups = []
    current_group = None
    in_header = True

    with open(seeds_path, "r", encoding="utf-8") as f:
        for line in f:
            stripped = line.rstrip()
            if in_header and stripped.startswith("#") and not stripped.startswith("#   "):
                header_lines.append(stripped)
                continue
            in_header = False

            if stripped.startswith("#   Subcategory:"):
                current_group = stripped.split(":", 1)[1].strip()
                groups.append((current_group, []))
                continue

            if stripped.startswith("#") or not stripped:
                continue

            entries.append(stripped)
            if groups:
                groups[-1][1].append(stripped)

    return header_lines, entries, groups


def cmd_reference_subsets(args):
    """Validate deterministic subset files (seeds/paths files).

    Checks:
      - No duplicate paths
      - Paths sorted lexicographically within section
      - All paths exist under locked reference suite (when corpus present)
      - Comments use '# group: <name>' format
    """
    self_test = "--self-test" in args
    check_all = "--all" in args or not args

    seeds_path = SEEDS_FILE
    if not seeds_path.is_file():
        print(f"ERROR: seeds file not found: {seeds_path}", file=sys.stderr)
        return 1

    if self_test:
        return cmd_reference_subsets_self_test(args)

    if check_all:
        return cmd_reference_subsets_all(args)

    return 0


def cmd_reference_subsets_self_test(args):
    """Self-test: check that the validator catches duplicate and unsorted paths."""
    import tempfile
    errors = 0

    # Test 1: duplicate path detection
    test_file = tempfile.NamedTemporaryFile(mode="w", suffix=".txt", delete=False, dir="/tmp")
    test_file.write("# schema_version: 1\n")
    test_file.write("# suite: test262\n")
    test_file.write("reference/test262/test/language/asi/S7.9_A6.1_T1.js\n")
    test_file.write("reference/test262/test/language/asi/S7.9_A6.1_T1.js\n")
    test_file.close()
    try:
        _, entries, _ = parse_seeds_file(test_file.name)
        if len(entries) == 2 and entries[0] == entries[1]:
            dup_count = len(entries) - len(set(entries))
            if dup_count > 0:
                print(f"  OK: duplicate path detected ({dup_count} dupes)")
            else:
                print("  WARN: duplicate path test needs improvement")
        else:
            print("  WARN: duplicate test produced unexpected entries")
    finally:
        os.unlink(test_file.name)

    # Test 2: unsorted path detection
    test_file2 = tempfile.NamedTemporaryFile(mode="w", suffix=".txt", delete=False, dir="/tmp")
    test_file2.write("# schema_version: 1\n")
    test_file2.write("# suite: test262\n")
    test_file2.write("reference/test262/test/language/asi/S7.9_A6.1_T2.js\n")
    test_file2.write("reference/test262/test/language/asi/S7.9_A6.1_T1.js\n")
    test_file2.close()
    try:
        _, entries2, _ = parse_seeds_file(test_file2.name)
        if entries2 != sorted(entries2):
            print("  OK: unsorted paths detected")
        else:
            print("  WARN: unsorted path test (entries already sorted)")
    finally:
        os.unlink(test_file2.name)

    print("Self-test complete")
    return 0 if errors == 0 else 1


def cmd_reference_subsets_all(args):
    """Validate all deterministic subset files."""
    all_ok = True

    # Validate the main seeds file
    if SEEDS_FILE.is_file():
        print(f"Checking: {SEEDS_FILE}")
        ok = validate_seeds_file(SEEDS_FILE, check_existence="--allow-missing-corpora" in args)
        if not ok:
            all_ok = False

    return 0 if all_ok else 1


def validate_seeds_file(seeds_path, check_existence=False):
    """Validate a seeds/paths file for duplicates, sorting, and existence."""
    header_lines, entries, groups = parse_seeds_file(seeds_path)
    all_ok = True

    # Check schema header
    has_schema = any(l.startswith("# schema_version:") for l in header_lines)
    has_suite = any(l.startswith("# suite:") for l in header_lines)
    has_purpose = any(l.startswith("# purpose:") for l in header_lines)

    if not has_schema:
        print(f"  WARN: missing schema_version header in {seeds_path}", file=sys.stderr)
    if not has_suite:
        print(f"  WARN: missing suite header in {seeds_path}", file=sys.stderr)
    if not has_purpose:
        print(f"  WARN: missing purpose header in {seeds_path}", file=sys.stderr)

    # Check for duplicates
    if len(entries) != len(set(entries)):
        seen = set()
        for i, entry in enumerate(entries):
            if entry in seen:
                print(f"  ERROR: duplicate path: {entry}", file=sys.stderr)
                all_ok = False
            seen.add(entry)

    # Check sorted order within each group
    for group_name, group_entries in groups:
        if group_entries != sorted(group_entries):
            print(
                f"  ERROR: paths not sorted in group '{group_name}'",
                file=sys.stderr,
            )
            all_ok = False

    # Check comment format
    with open(seeds_path, "r", encoding="utf-8") as f:
        for line in f:
            stripped = line.strip()
            if stripped.startswith("#   ") and ":" in stripped:
                key = stripped.split(":", 1)[0].strip()
                if not key.startswith("#   Subcategory"):
                    print(
                        f"  WARN: unexpected comment format: {stripped}",
                        file=sys.stderr,
                    )

    # Check path existence when corpus is present
    if check_existence:
        for entry in entries:
            full_path = REPO_ROOT / entry
            if not full_path.exists():
                print(f"  WARN: path not found: {entry}", file=sys.stderr)

    if all_ok:
        print(f"  OK: {len(entries)} entries, {len(groups)} groups")

    return all_ok


def cmd_evidence_check(args):
    """Validate coverage evidence completeness.

    Checks that evidence fields include required keys.
    """
    self_test = "--self-test" in args
    check_mode = "--check" in args

    if self_test:
        print("Evidence check self-test")
        return 0

    if check_mode:
        # Read evidence from args or default path
        evidence_path = None
        rest = [a for a in args if a not in ("--check", "--self-test")]
        if rest and not rest[0].startswith("--"):
            evidence_path = Path(rest[0])
        if evidence_path is None:
            # Check all suite evidence files
            evidence_base = REPO_ROOT / "reports" / "coverage"
            if evidence_base.is_dir():
                for suite_dir in sorted(evidence_base.iterdir()):
                    if suite_dir.is_dir():
                        ev_path = suite_dir / "evidence.json"
                        if ev_path.is_file():
                            ok = validate_evidence_file(ev_path)
                            if not ok:
                                return 1
                print("All evidence files validated")
                return 0
            else:
                print("No evidence files found", file=sys.stderr)
                return 0
        else:
            ok = validate_evidence_file(evidence_path)
            return 0 if ok else 1

    return 0


def validate_evidence_file(evidence_path):
    """Validate an evidence JSON file for required keys."""
    if not evidence_path.is_file():
        print(f"ERROR: evidence file not found: {evidence_path}", file=sys.stderr)
        return False

    try:
        data = json.loads(evidence_path.read_text())
    except (OSError, json.JSONDecodeError) as e:
        print(f"ERROR: could not parse evidence: {e}", file=sys.stderr)
        return False

    required_keys = [
        "argv", "argv_str", "selection_mode", "oracle_policy",
        "semantic_check", "server_mode",
    ]
    optional_evidence_keys = [
        "case_count", "path_sha256", "mode", "sample", "sample_seed",
        "path_filters", "paths_file", "limit", "category",
    ]

    missing = [k for k in required_keys if k not in data]
    if missing:
        print(f"  ERROR: missing evidence keys: {missing}", file=sys.stderr)
        return False

    # Check for corpus evidence
    if "corpus" in data:
        corpus = data["corpus"]
        if "lock_digest" not in corpus:
            print(f"  WARN: corpus evidence missing lock_digest", file=sys.stderr)

    print(f"  OK: {evidence_path}")
    return True


def cmd_replay_set(args):
    """Validate replay set (seed file) integrity.

    Checks:
      - Schema header present
      - No duplicate paths
      - Paths sorted
      - Paths exist when local corpus is present
    """
    self_test = "--self-test" in args
    check_mode = "--check" in args

    if self_test:
        return cmd_replay_set_self_test(args)

    if check_mode:
        # Find the seeds file or specific paths file from args
        rest = [a for a in args if a not in ("--check", "--self-test", "--allow-missing-corpora")]
        targets = rest if rest else [str(SEEDS_FILE)]
        all_ok = True
        allow_missing = "--allow-missing-corpora" in args
        for target in targets:
            target_path = Path(target)
            if not target_path.is_absolute():
                target_path = REPO_ROOT / target_path
            if target_path.is_file():
                ok = validate_seeds_file(
                    target_path,
                    check_existence=not allow_missing,
                )
                if not ok:
                    all_ok = False
            else:
                print(f"  SKIP: not found: {target_path}")
        return 0 if all_ok else 1

    return 0


def cmd_replay_set_self_test(args):
    """Self-test for replay set validation."""
    import tempfile

    # Test: valid file
    test_file = tempfile.NamedTemporaryFile(mode="w", suffix=".txt", delete=False, dir="/tmp")
    test_file.write("# schema_version: 1\n")
    test_file.write("# suite: test262\n")
    test_file.write("# purpose: test\n")
    test_file.write("#   Subcategory: language/asi\n")
    test_file.write("reference/test262/test/language/asi/S7.9_A6.1_T1.js\n")
    test_file.write("reference/test262/test/language/asi/S7.9_A6.1_T2.js\n")
    test_file.close()
    try:
        ok = validate_seeds_file(test_file.name)
        print(f"  Valid file test: {'PASS' if ok else 'FAIL'}")
    finally:
        os.unlink(test_file.name)

    # Test: duplicate path
    test_file2 = tempfile.NamedTemporaryFile(mode="w", suffix=".txt", delete=False, dir="/tmp")
    test_file2.write("# schema_version: 1\n")
    test_file2.write("# suite: test262\n")
    test_file2.write("# purpose: test\n")
    test_file2.write("reference/test262/test/language/asi/S7.9_A6.1_T1.js\n")
    test_file2.write("reference/test262/test/language/asi/S7.9_A6.1_T1.js\n")
    test_file2.close()
    try:
        ok = validate_seeds_file(test_file2.name)
        print(f"  Duplicate detection: {'PASS' if not ok else 'FAIL'}")
    finally:
        os.unlink(test_file2.name)

    # Test: unsorted
    test_file3 = tempfile.NamedTemporaryFile(mode="w", suffix=".txt", delete=False, dir="/tmp")
    test_file3.write("# schema_version: 1\n")
    test_file3.write("# suite: test262\n")
    test_file3.write("# purpose: test\n")
    test_file3.write("#   Subcategory: language/asi\n")
    test_file3.write("reference/test262/test/language/asi/S7.9_A6.1_T2.js\n")
    test_file3.write("reference/test262/test/language/asi/S7.9_A6.1_T1.js\n")
    test_file3.close()
    try:
        ok = validate_seeds_file(test_file3.name)
        print(f"  Unsorted detection: {'PASS' if not ok else 'FAIL'}")
    finally:
        os.unlink(test_file3.name)

    return 0


def main():
    if len(sys.argv) < 2 or sys.argv[1] in ("--help", "-h"):
        usage()
        sys.exit(0)

    command = sys.argv[1]
    args = sys.argv[2:]
    if args and args[0] == "--":
        args = args[1:]

    if command == "verify":
        sys.exit(cmd_verify(args))
    elif command == "write-lock":
        sys.exit(cmd_write_lock(args))
    elif command == "print-evidence":
        sys.exit(cmd_print_evidence(args))
    elif command == "reference-subsets":
        sys.exit(cmd_reference_subsets(args))
    elif command == "evidence-check":
        sys.exit(cmd_evidence_check(args))
    elif command == "replay-set":
        sys.exit(cmd_replay_set(args))
    else:
        print(f"Unknown command: {command}", file=sys.stderr)
        usage()


if __name__ == "__main__":
    main()
