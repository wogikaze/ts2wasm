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
  build_implemented: number;
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
  build_pass: number;
  semantic_pass: number;
  fail: number;
  unsupported: number;
  blocked: number;
}
