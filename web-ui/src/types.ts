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
