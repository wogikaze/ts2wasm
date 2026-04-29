export interface TestResult {
  id: string;
  name: string;
  status: 'pass' | 'fail' | 'skip' | 'error';
  suite: string;
  duration?: number;
  error?: string;
}

export interface CoverageData {
  total: number;
  implemented: number;
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
  timestamp: string;
  passed: number;
  failed: number;
  skipped: number;
  compile_time: number;
  runtime: number;
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
