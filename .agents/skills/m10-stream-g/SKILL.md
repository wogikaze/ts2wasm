# Stream G: Test Infrastructure & Coverage Tracking

## Goal
Enhance test runners and coverage dashboards to track M7-M10 progress through test262, TypeScript, and typescript-go reference corpuses.

## Scope (1-2 hour window)

Implement:
1. **Comprehensive test262 runner** with category breakdown
2. **Differential test reporter** (wasm output vs Node output)
3. **Coverage matrix auto-update** for new features
4. **Test status schema integration** (pass/fail/unsupported/blocked)
5. **Performance baseline tracking** (optional: wasm size, execution time)
6. **Regression gate** (prevent coverage decrease)

## Implementation strategy

### Phase 1: Test status schema (10 min)

Integrate `crates/shared/src/test_status.rs`:
```rust
#[derive(Serialize)]
pub struct TestRecord {
    pub suite: String,        // "test262", "tsc", "tsgo"
    pub case: String,         // filename or test ID
    pub target: String,       // "wasm-iwasm", "wasm-node-host"
    pub status: TestStatus,   // pass, fail, unsupported, blocked, skip-with-reason
    pub expected: Option<String>,
    pub actual: Option<String>,
    pub reason: Option<String>,
    pub tracking: Option<String>,  // issue ID, feature label
}

#[derive(Serialize)]
pub enum TestStatus {
    Pass,
    Fail,
    Unsupported { reason: String },
    Blocked { condition: String },
    SkipWithReason { reason: String },
}
```

### Phase 2: Enhanced test262 runner (20 min)

Create `scripts/test262_runner.sh`:
1. Category filter: `language/expressions`, `language/statements`, `builtins/Math`, etc.
2. For each .js file:
   a. Compile with ts2wasm
   b. Run with iwasm
   c. Capture stdout/stderr/exit code
   d. Compare with Node.js reference
3. Generate TestRecord for each case
4. Aggregate by DiagCode (UnsupportedSyntax, UnresolvedName, etc.)

Output: `test262-results.jsonl` (one record per line)

### Phase 3: Differential reporter (20 min)

Create `scripts/test_differential_reporter.sh`:
1. Read test262-results.jsonl
2. Group by status: pass, fail, unsupported, blocked
3. For fail: show diffs (expected stdout vs actual)
4. For unsupported: extract DiagCode and reason
5. Generate HTML report: `test262-report.html`
6. Summary table: category × status

Format (markdown):
```
| Category | Pass | Fail | Unsupported | Blocked |
|----------|------|------|-------------|---------|
| Expressions | 45 | 2 | 12 | 0 |
| Statements | 32 | 1 | 5 | 0 |
| Builtins | 8 | 0 | 42 | 0 |
```

### Phase 4: Coverage matrix enhancement (15 min)

Update `docs/16-coverage-matrix.md`:
1. Add section: `## Test suites overview`
2. Link to test262-report.html
3. Show trending data (if multiple runs available)
4. Add breakdown by:
   - Syntax category (expressions, statements, functions, classes)
   - Feature category (builtins, scoping, semantics, control flow)
5. Highlight blockers (features blocking many tests)

Example:
```markdown
## Test262 breakdown by category

- Expressions: 45/92 executed (49%)
  - Arithmetic: 8/8 pass
  - String: 5/8 pass, 3 unsupported (template literals)
  - Function calls: 12/20 pass, 5 fail (method call binding), 3 unsupported
```

### Phase 5: Regression gate (10 min)

Create `scripts/test_regression_gate.sh`:
1. Compare current results vs baseline (stored in repo or PR comment)
2. Check: `pass` count not decreased
3. Check: `fail` count not increased (fail = regression)
4. Check: `unsupported` count not increased (unsupported = new blocker)
5. Fail CI if any check fails
6. Exception: allow increase if issue ID provided

Example output:
```
✓ pass: 45 → 48 (+3)
✓ fail: 2 → 2 (no change)
✓ unsupported: 12 → 11 (-1, removed blocker)
All gates passed
```

### Phase 6: Performance baseline (10 min, optional)

Track per build:
1. Wasm file size (bytes)
2. Compilation time (seconds)
3. Execution time (first 10 fixtures, iwasm)
4. Memory usage (peak resident set)

Store: `benchmark-results.json` (append-only)
Visualize: simple CSV with run date, metrics

### Phase 7: CI integration (10 min)

Update `.github/workflows/reference-coverage.yml`:
1. Add test262_runner.sh step
2. Capture test262-results.jsonl as artifact
3. Generate report and upload
4. Run regression_gate.sh
5. Comment on PR with summary:
   ```
   ✓ Reference coverage test suite passed
   - test262: 45 pass, 2 fail, 12 unsupported
   - TypeScript: 120 pass, 8 fail, 150 unsupported
   - No regressions detected
   ```

### Phase 8: Tests (10 min)

Fixtures for test runner itself:
1. `test-infrastructure/pass-fixture.ts`: should compile and pass
2. `test-infrastructure/fail-fixture.ts`: should compile but fail (wrong output)
3. `test-infrastructure/unsupported-fixture.ts`: should have UnsupportedSyntax

Validate:
- Test runner classifies each correctly
- Report generation produces valid JSON
- Regression gate detects (simulated) regressions

## Output

**Commits**:
1. `tests: add test262 runner script (category-based filtering)`
2. `tests: add differential test reporter (pass/fail/unsupported breakdown)`
3. `tests: add regression gate for coverage tracking`
4. `docs: enhance coverage matrix with test suite breakdown`
5. `ci: integrate test262 runner into reference-coverage workflow`
6. `tests: add test infrastructure validation tests`

**Scripts added**:
- `scripts/test262_runner.sh` (compiles & runs ~1000 test262 files)
- `scripts/test_differential_reporter.sh` (generates report)
- `scripts/test_regression_gate.sh` (validates no regression)
- `scripts/benchmark_tracker.sh` (optional: performance baseline)

**Outputs generated**:
- `test262-results.jsonl` (one TestRecord per file)
- `test262-report.html` (human-readable summary)
- `benchmark-results.json` (performance tracking)

**DiagCode impact**:
- Categorized by feature: expressions, statements, functions, classes, builtins
- Top blockers visible in report (e.g., "spread operator blocks 45 tests")

**Coverage matrix delta**:
- Dramatic improvement in visibility (from single number to detailed breakdown)
- Able to trace which feature unblocks which tests

## Validation before commit

```bash
cargo fmt --all --check
# Run test infrastructure tests
cargo test -q --test test_infrastructure
# Generate test262 report (sample: first 50 files)
scripts/test262_runner.sh --sample 50 > /tmp/sample-results.jsonl
scripts/test_differential_reporter.sh < /tmp/sample-results.jsonl > /tmp/sample-report.html
# Check for HTML validity
grep -q "<table" /tmp/sample-report.html && echo "✓ report generated"
```

## Gatekeeper checklist

✓ Test262 runner handles all file types (strict, non-strict, etc.)
✓ Differential reporter distinguishes parsing errors from runtime differences
✓ Regression gate prevents accidental coverage decrease
✓ Performance tracking doesn't slow down tests significantly
✓ CI job completes in reasonable time (~5 min for full test262)
✓ Report clearly shows which features are blockers
✓ Test results reproducible across runs

## Design decisions

1. **Test262 scope**: Sample-based initially (first N files per category) for speed
2. **Performance tracking**: Optional (doesn't block M10 gate)
3. **Report format**: HTML table (easy to view in GitHub) + JSON (machine-readable)
4. **Regression threshold**: No regression tolerated (binary gate)
5. **Exception handling**: Provide issue ID to allow intentional regression

## Future enhancements (not in scope)

- Per-test DiagCode tracking (why did test fail?)
- Visual chart of coverage over time
- Breakdown by TypeScript version (if testing multiple versions)
- Integration with external metrics (test262 official status)
- Automated issue creation for top blockers

## References

- Test status schema: `crates/shared/src/test_status.rs`
- Existing coverage scripts: `scripts/reference_coverage.sh`, `scripts/update_coverage_matrix.sh`
- CI workflow: `.github/workflows/reference-coverage.yml`
- Reference corpus: `reference/test262`, `reference/TypeScript/tests/cases`
