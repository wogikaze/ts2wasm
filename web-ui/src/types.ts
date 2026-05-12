export type TestStatus = 'pass' | 'mismatch' | 'runtime_error' | 'fail' | 'unsupported' | 'blocked';

export interface TestResult {
  id: string;
  name: string;
  status: TestStatus;
  suite: string;
  duration?: number;
  count?: number;
  error?: string;
  case?: string;
  target?: string;
  reason?: string;
  expected?: string;
  actual?: string;
  stderr?: string;
  source_code?: string;
  error_line?: number;
}

export interface TestResultsMetadata {
  schema_version?: number;
  generated_at?: string;
  generator?: string;
  record_mode?: 'jsonl' | 'aggregate' | string;
  total_records?: number;
  shown_records?: number;
  row_limit?: number;
  row_limit_per_suite?: number;
  truncated?: boolean;
  total_by_suite?: Record<string, number>;
  shown_by_suite?: Record<string, number>;
  summary_by_suite?: Record<string, TestSummary>;
  sources?: string[];
}

export interface CoverageData {
  total: number;
  implemented?: number;
  build_implemented: number;
  build_coverage_percent?: string;
  semantic_pass?: number;
  semantic_coverage_percent?: string;
  differential_pass?: number;
  negative_compile_pass?: number;
  conformance_pass?: number;
  unimplemented: number;
  future: number;
  byPriority: {
    p0: number;
    p1: number;
    p2: number;
    p3: number;
    future: number;
  };
  suites?: CoverageSuite[];
}

export interface HistoricalData {
  run_id: string;
  suite?: string;
  executed?: number;
  denominator?: number;
  timestamp: string;
  passed: number;
  failed: number;
  skipped: number;
  duration_ms?: number | null;
  compile_time?: number;
  runtime?: number;
}

export interface TestSummary {
  passed: number;
  mismatch: number;
  runtime_error: number;
  build_error: number;
  unsupported: number;
  blocked: number;
}

export interface CoverageSuite {
  suite: string;
  source: string;
  denominator: number;
  executed: number;
  build_coverage_percent?: string;
  semantic_coverage_percent?: string;
  build_pass: number;
  semantic_pass: number;
  executable_build_pass?: number;
  differential_pass?: number;
  negative_compile_pass?: number;
  negative_compile_unverified?: number;
  negative_compile_mismatch?: number;
  conformance_pass?: number;
  build_pass_by_detail?: Record<string, number>;
  unresolved_name_by_symbol?: Record<string, number>;
  harness_includes?: string[];
  fail: number;
  unsupported: number;
  blocked: number;
  skip_with_reason?: number;
}
