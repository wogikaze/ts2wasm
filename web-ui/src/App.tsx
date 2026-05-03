import { Fragment, useEffect, useMemo, useState } from 'react'
import { Play, CheckCircle, XCircle, AlertCircle, AlertTriangle, SkipForward, BarChart3, History, Download, Search, Filter, Moon, Sun, ChevronDown, ChevronRight } from 'lucide-react'
import {
  Bar,
  BarChart,
  CartesianGrid,
  Cell,
  Legend,
  Line,
  LineChart,
  Pie,
  PieChart,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from 'recharts'
import { useTestData, useCoverageData, useHistoricalData } from './hooks/useData'
import type { CoverageData, HistoricalData, TestResult } from './types'
import './index.css'

const PERF_REGRESSION_THRESHOLD = 0.2
const THEME_STORAGE_KEY = 'ts2wasm-web-ui-theme'

const coverageColors = ['#22c55e', '#ef4444', '#eab308']
const priorityColors = {
  P0: '#ef4444',
  P1: '#f97316',
  P2: '#eab308',
  P3: '#3b82f6',
  Future: '#6b7280',
}

interface RunDeltas {
  passed: number | null
  failed: number | null
  skipped: number | null
  compile_time: number | null
  runtime: number | null
}

interface TrendRun extends HistoricalData {
  displayTime: string
  trendLabel: string
  deltas: RunDeltas
  regressionReasons: string[]
}

type ThemePreference = 'dark' | 'light'
type StatusFilter = 'all' | TestResult['status']

function getInitialTheme(): ThemePreference {
  if (typeof window === 'undefined') return 'dark'
  const storedTheme = window.localStorage.getItem(THEME_STORAGE_KEY)
  if (storedTheme === 'dark' || storedTheme === 'light') return storedTheme
  return window.matchMedia('(prefers-color-scheme: light)').matches ? 'light' : 'dark'
}

function formatPercent(value: number) {
  if (!Number.isFinite(value)) return '0.0%'
  return `${value.toFixed(1)}%`
}

function clampPercent(value: number) {
  if (!Number.isFinite(value)) return 0
  return Math.max(0, Math.min(100, value))
}

function percentOf(value: number, total: number) {
  return total > 0 ? clampPercent((value / total) * 100) : 0
}

function formatDuration(value?: number) {
  return Number.isFinite(value) ? `${value}ms` : '-'
}

function cleanDetail(value?: string | number) {
  if (value === undefined || value === null) return ''
  return String(value).trim()
}

function hasTestDetails(test: TestResult) {
  return Boolean(
    test.case || test.target || test.reason || test.error || test.expected ||
    test.actual || test.stderr || test.source_code || test.error_line !== undefined
  )
}

function DetailBlock({ title, value, tone = 'default' }: { title: string; value?: string | number; tone?: 'default' | 'error' | 'code' }) {
  const text = cleanDetail(value)
  if (!text) return null
  const toneClass = tone === 'error'
    ? 'border-red-500/30 bg-red-500/5 text-red-100'
    : tone === 'code'
      ? 'border-gray-700 bg-gray-950 text-gray-100'
      : 'border-gray-700 bg-gray-900/70 text-gray-100'

  return (
    <section className="min-w-0">
      <h4 className="mb-2 text-xs font-semibold uppercase tracking-wide text-gray-400">{title}</h4>
      <pre className={`max-h-[32rem] overflow-auto whitespace-pre-wrap break-words rounded-md border p-3 font-mono text-xs leading-5 ${toneClass}`}>
        {text}
      </pre>
    </section>
  )
}

function formatLastUpdated(value: string | null) {
  return value ? new Date(value).toLocaleTimeString() : '-'
}

function chartNumber(value: unknown) {
  const numeric = Number(value)
  return Number.isFinite(numeric) ? numeric : 0
}

function formatDelta(value: number | null, unit = '') {
  if (value === null) return '-'
  if (value === 0) return '0'
  const prefix = value > 0 ? '+' : ''
  return `${prefix}${value}${unit}`
}

function formatDurationDelta(value: number | null) {
  if (value === null) return '-'
  if (value === 0) return '0s'
  const prefix = value > 0 ? '+' : ''
  return `${prefix}${value.toFixed(2)}s`
}

function worsenedByThreshold(current: number, previous: number) {
  return previous > 0 && (current - previous) / previous > PERF_REGRESSION_THRESHOLD
}

function buildTrendRuns(history: HistoricalData[]): TrendRun[] {
  const chronological = [...history].sort((a, b) => {
    const timeDelta = new Date(a.timestamp).getTime() - new Date(b.timestamp).getTime()
    return timeDelta || a.run_id.localeCompare(b.run_id)
  })

  return chronological.map((run, index) => {
    const previous = chronological[index - 1]
    const deltas: RunDeltas = previous
      ? {
          passed: run.passed - previous.passed,
          failed: run.failed - previous.failed,
          skipped: run.skipped - previous.skipped,
          compile_time: run.compile_time - previous.compile_time,
          runtime: run.runtime - previous.runtime,
        }
      : {
          passed: null,
          failed: null,
          skipped: null,
          compile_time: null,
          runtime: null,
        }

    const regressionReasons = [
      previous && run.failed > previous.failed ? 'failed increased' : null,
      previous && run.passed < previous.passed ? 'passed dropped' : null,
      previous && worsenedByThreshold(run.compile_time, previous.compile_time) ? 'compile time +20%' : null,
      previous && worsenedByThreshold(run.runtime, previous.runtime) ? 'runtime +20%' : null,
    ].filter((reason): reason is string => Boolean(reason))

    return {
      ...run,
      displayTime: new Date(run.timestamp).toLocaleString(),
      trendLabel: `${run.run_id}`,
      deltas,
      regressionReasons,
    }
  })
}

function csvCell(value: unknown) {
  const text = String(value ?? '')
  return /[",\n\r]/.test(text) ? `"${text.replaceAll('"', '""')}"` : text
}

function toCsv(rows: Array<Record<string, unknown>>) {
  if (rows.length === 0) return ''
  const headers = Object.keys(rows[0])
  const lines = [
    headers.map(csvCell).join(','),
    ...rows.map(row => headers.map(header => csvCell(row[header])).join(',')),
  ]
  return `${lines.join('\n')}\n`
}

function downloadText(filename: string, mimeType: string, content: string) {
  const blob = new Blob([content], { type: mimeType })
  const url = URL.createObjectURL(blob)
  const link = document.createElement('a')
  link.href = url
  link.download = filename
  link.click()
  URL.revokeObjectURL(url)
}

function coverageCsvRows(coverage: CoverageData) {
  const suites = coverage.suites?.map(suite => ({
    kind: 'suite',
    name: suite.suite,
    total: suite.denominator,
    implemented: suite.build_pass,
    unimplemented: suite.unsupported,
    future: 0,
    failed: suite.fail,
    blocked: suite.blocked,
    semantic_pass: suite.semantic_pass,
  })) ?? []

  return [
    {
      kind: 'summary',
      name: 'all',
      total: coverage.total,
      implemented: coverage.implemented,
      unimplemented: coverage.unimplemented,
      future: coverage.future,
      failed: 0,
      blocked: 0,
      semantic_pass: 0,
    },
    ...suites,
  ]
}

function App() {
  const [activeTab, setActiveTab] = useState<'tests' | 'coverage' | 'history'>('tests')
  const [searchQuery, setSearchQuery] = useState('')
  const [statusFilter, setStatusFilter] = useState<StatusFilter>('all')
  const [suiteFilter, setSuiteFilter] = useState('all')
  const [expandedTestId, setExpandedTestId] = useState<string | null>(null)
  const [theme, setTheme] = useState<ThemePreference>(getInitialTheme)

  // Load real data
  const {
    tests,
    summary,
    metadata: testMetadata,
    loading: testsLoading,
    error: testsError,
    liveStatus,
    lastUpdated,
    liveMode,
  } = useTestData()
  const { coverage, loading: coverageLoading, error: coverageError } = useCoverageData()
  const { history, loading: historyLoading, error: historyError } = useHistoricalData()

  const filteredTests = tests.filter(test => {
    const haystack = [test.name, test.suite, test.case, test.reason, test.error, test.stderr].filter(Boolean).join(' ').toLowerCase()
    const matchesSearch = haystack.includes(searchQuery.toLowerCase())
    const matchesStatus = statusFilter === 'all' || test.status === statusFilter
    const matchesSuite = suiteFilter === 'all' || test.suite === suiteFilter
    return matchesSearch && matchesStatus && matchesSuite
  })

  const suiteOptions = useMemo(() => (
    Array.from(new Set(tests.map(test => test.suite))).sort()
  ), [tests])

  const coverageStatusData = useMemo(() => [
    { name: 'Implemented', value: coverage.implemented },
    { name: 'Unimplemented', value: coverage.unimplemented },
    { name: 'Future', value: coverage.future },
  ].filter(item => item.value > 0), [coverage])

  const priorityData = useMemo(() => [
    { name: 'P0', value: coverage.byPriority.p0, fill: priorityColors.P0 },
    { name: 'P1', value: coverage.byPriority.p1, fill: priorityColors.P1 },
    { name: 'P2', value: coverage.byPriority.p2, fill: priorityColors.P2 },
    { name: 'P3', value: coverage.byPriority.p3, fill: priorityColors.P3 },
    { name: 'Future', value: coverage.byPriority.future, fill: priorityColors.Future },
  ], [coverage])

  const coverageTotalMismatch = coverage.total > 0 &&
    coverage.implemented + coverage.unimplemented + coverage.future !== coverage.total

  const suiteCoverageData = useMemo(() => (coverage.suites || []).map(suite => ({
    suite: suite.suite,
    executed: suite.executed,
    build_pass: suite.build_pass,
    semantic_pass: suite.semantic_pass,
    failed: suite.fail,
    blocked: suite.blocked,
    unsupported: suite.unsupported,
    denominator: suite.denominator,
    buildRate: suite.denominator > 0 ? clampPercent((suite.build_pass / suite.denominator) * 100) : 0,
    semanticRate: suite.denominator > 0 ? clampPercent((suite.semantic_pass / suite.denominator) * 100) : 0,
  })), [coverage])

  const trendRuns = useMemo(() => buildTrendRuns(history), [history])
  const historyRows = useMemo(() => [...trendRuns].reverse(), [trendRuns])
  const latestTrend = trendRuns[trendRuns.length - 1]

  const currentExportName = `ts2wasm-${activeTab}`
  const nextTheme = theme === 'dark' ? 'light' : 'dark'

  useEffect(() => {
    window.localStorage.setItem(THEME_STORAGE_KEY, theme)
    document.documentElement.dataset.theme = theme
    document.documentElement.style.colorScheme = theme
  }, [theme])

  const exportJson = () => {
    const payload = activeTab === 'tests'
      ? { summary, tests: filteredTests }
      : activeTab === 'coverage'
        ? coverage
        : { history: historyRows }
    downloadText(`${currentExportName}.json`, 'application/json', `${JSON.stringify(payload, null, 2)}\n`)
  }

  const exportCsv = () => {
    const rows = activeTab === 'tests'
      ? filteredTests.map((test: TestResult) => ({
          id: test.id,
          suite: test.suite,
          name: test.name,
          status: test.status,
          duration: test.duration ?? '',
          case: test.case ?? '',
          target: test.target ?? '',
          reason: test.reason ?? '',
          expected: test.expected ?? '',
          actual: test.actual ?? '',
          error: test.error ?? '',
          stderr: test.stderr ?? '',
          source_code: test.source_code ?? '',
        }))
      : activeTab === 'coverage'
        ? coverageCsvRows(coverage)
        : historyRows.map(run => ({
            run_id: run.run_id,
            timestamp: run.timestamp,
            passed: run.passed,
            failed: run.failed,
            skipped: run.skipped,
            compile_time: run.compile_time,
            runtime: run.runtime,
            passed_delta: run.deltas.passed ?? '',
            failed_delta: run.deltas.failed ?? '',
            skipped_delta: run.deltas.skipped ?? '',
            regression: run.regressionReasons.join('; '),
          }))
    downloadText(`${currentExportName}.csv`, 'text/csv', toCsv(rows))
  }

  const exportPdf = () => {
    window.print()
  }

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'pass': return <CheckCircle className="w-5 h-5 text-green-500" />
      case 'mismatch': return <XCircle className="w-5 h-5 text-orange-500" />
      case 'runtime_error': return <AlertTriangle className="w-5 h-5 text-red-500" />
      case 'fail': return <XCircle className="w-5 h-5 text-red-500" />
      case 'unsupported': return <SkipForward className="w-5 h-5 text-yellow-500" />
      case 'blocked': return <AlertCircle className="w-5 h-5 text-gray-500" />
      default: return <AlertCircle className="w-5 h-5 text-gray-500" />
    }
  }

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'pass': return 'bg-green-500/10 text-green-500 border-green-500/20'
      case 'mismatch': return 'bg-orange-500/10 text-orange-500 border-orange-500/20'
      case 'runtime_error': return 'bg-red-500/10 text-red-500 border-red-500/20'
      case 'fail': return 'bg-red-700/10 text-red-400 border-red-700/20'
      case 'unsupported': return 'bg-yellow-500/10 text-yellow-500 border-yellow-500/20'
      case 'blocked': return 'bg-gray-500/10 text-gray-500 border-gray-500/20'
      default: return 'bg-gray-500/10 text-gray-500 border-gray-500/20'
    }
  }

  return (
    <div className={`theme-${theme} min-h-screen bg-gray-900 text-gray-100`}>
      {/* Header */}
      <header className="border-b border-gray-800 bg-gray-900/50 backdrop-blur-sm sticky top-0 z-10">
        <div className="max-w-[1680px] mx-auto px-4 sm:px-6 py-3">
          <div className="flex flex-col gap-3 md:flex-row md:items-center md:justify-between">
            <div className="flex items-center gap-3 min-w-0">
              <Play className="w-8 h-8 text-purple-500 shrink-0" />
              <h1 className="text-xl sm:text-2xl font-bold truncate">ts2wasm Test Reporter</h1>
            </div>
            <div className="flex flex-wrap items-center justify-end gap-2">
              <button
                type="button"
                onClick={() => setTheme(nextTheme)}
                aria-pressed={theme === 'light'}
                title={`Switch to ${nextTheme} theme`}
                className="flex items-center gap-2 px-3 py-2 bg-gray-800 hover:bg-gray-700 border border-gray-700 rounded-lg transition-colors"
              >
                {theme === 'dark' ? <Sun className="w-4 h-4" /> : <Moon className="w-4 h-4" />}
                {theme === 'dark' ? 'Light' : 'Dark'}
              </button>
              <button
                onClick={exportJson}
                className="flex items-center gap-2 px-3 py-2 bg-purple-600 hover:bg-purple-700 rounded-lg transition-colors"
              >
                <Download className="w-4 h-4" />
                JSON
              </button>
              <button
                onClick={exportCsv}
                className="flex items-center gap-2 px-3 py-2 bg-gray-800 hover:bg-gray-700 border border-gray-700 rounded-lg transition-colors"
              >
                <Download className="w-4 h-4" />
                CSV
              </button>
              <button
                onClick={exportPdf}
                className="flex items-center gap-2 px-3 py-2 bg-gray-800 hover:bg-gray-700 border border-gray-700 rounded-lg transition-colors"
              >
                <Download className="w-4 h-4" />
                PDF
              </button>
            </div>
          </div>
        </div>
      </header>

      {/* Navigation */}
      <nav className="border-b border-gray-800 bg-gray-900">
        <div className="max-w-[1680px] mx-auto px-4 sm:px-6">
          <div className="flex gap-1">
            <button
              onClick={() => setActiveTab('tests')}
              className={`px-4 py-3 font-medium transition-colors border-b-2 ${
                activeTab === 'tests'
                  ? 'border-purple-500 text-purple-400'
                  : 'border-transparent text-gray-400 hover:text-gray-200'
              }`}
            >
              Test Results
            </button>
            <button
              onClick={() => setActiveTab('coverage')}
              className={`px-4 py-3 font-medium transition-colors border-b-2 ${
                activeTab === 'coverage'
                  ? 'border-purple-500 text-purple-400'
                  : 'border-transparent text-gray-400 hover:text-gray-200'
              }`}
            >
              <div className="flex items-center gap-2">
                <BarChart3 className="w-4 h-4" />
                Coverage
              </div>
            </button>
            <button
              onClick={() => setActiveTab('history')}
              className={`px-4 py-3 font-medium transition-colors border-b-2 ${
                activeTab === 'history'
                  ? 'border-purple-500 text-purple-400'
                  : 'border-transparent text-gray-400 hover:text-gray-200'
              }`}
            >
              <div className="flex items-center gap-2">
                <History className="w-4 h-4" />
                History
              </div>
            </button>
          </div>
        </div>
      </nav>

      {/* Main Content */}
      <main className="max-w-[1680px] mx-auto px-4 sm:px-6 py-6">
        {activeTab === 'tests' && (
          <div>
            {testsLoading ? (
              <div className="flex items-center justify-center py-12">
                <div className="text-gray-400">Loading test results...</div>
              </div>
            ) : testsError && (!liveMode || tests.length === 0) ? (
              <div className="bg-red-500/10 border border-red-500/20 rounded-lg p-4 text-red-500">
                Error loading test results: {testsError}
              </div>
            ) : (
              <>
            {liveMode ? (
              <div className={`mb-5 rounded-lg border px-4 py-3 text-sm ${
                liveStatus === 'error'
                  ? 'border-red-500/30 bg-red-500/10 text-red-300'
                  : 'border-green-500/30 bg-green-500/10 text-green-300'
              }`}>
                Live mode: {liveStatus}. Last refresh: {formatLastUpdated(lastUpdated)}.
              </div>
            ) : null}

            {testMetadata ? (
              <div className="mb-5 flex flex-wrap items-center gap-3 rounded-lg border border-gray-700 bg-gray-800 px-4 py-3 text-sm text-gray-300">
                <span>Data source: <span className="font-semibold text-gray-100">{testMetadata.record_mode ?? 'unknown'}</span></span>
                <span>Showing <span className="font-semibold text-gray-100">{(testMetadata.shown_records ?? tests.length).toLocaleString()}</span> of <span className="font-semibold text-gray-100">{(testMetadata.total_records ?? tests.length).toLocaleString()}</span> records</span>
                {testMetadata.truncated ? <span className="text-yellow-400">table is capped for browser performance</span> : null}
                {testMetadata.generated_at ? <span className="text-gray-400">generated {new Date(testMetadata.generated_at).toLocaleString()}</span> : null}
              </div>
            ) : null}

            {/* Summary Cards */}
            <div className="grid grid-cols-2 xl:grid-cols-6 gap-3 mb-5">
              <div className="bg-gray-800 rounded-lg p-4 border border-gray-700">
                <div className="flex items-center justify-between gap-3">
                  <span className="text-gray-400">Total</span>
                  <span className="text-2xl font-bold">{
                    summary.passed + summary.mismatch + summary.runtime_error + summary.build_error + summary.unsupported + summary.blocked
                  }</span>
                </div>
              </div>
              <div className="bg-gray-800 rounded-lg p-4 border border-gray-700">
                <div className="flex items-center justify-between gap-3">
                  <span className="text-gray-400">Passed</span>
                  <span className="text-2xl font-bold text-green-500">{summary.passed}</span>
                </div>
              </div>
              <div className="bg-gray-800 rounded-lg p-4 border border-gray-700">
                <div className="flex items-center justify-between gap-3">
                  <span className="text-gray-400">Mismatch</span>
                  <span className="text-2xl font-bold text-orange-500">{summary.mismatch}</span>
                </div>
              </div>
              <div className="bg-gray-800 rounded-lg p-4 border border-gray-700">
                <div className="flex items-center justify-between gap-3">
                  <span className="text-gray-400">Runtime</span>
                  <span className="text-2xl font-bold text-red-500">{summary.runtime_error}</span>
                </div>
              </div>
              <div className="bg-gray-800 rounded-lg p-4 border border-gray-700">
                <div className="flex items-center justify-between gap-3">
                  <span className="text-gray-400">Build Error</span>
                  <span className="text-2xl font-bold text-red-400">{summary.build_error}</span>
                </div>
              </div>
              <div className="bg-gray-800 rounded-lg p-4 border border-gray-700">
                <div className="flex items-center justify-between gap-3">
                  <span className="text-gray-400">Unsupported</span>
                  <span className="text-2xl font-bold text-yellow-500">{summary.unsupported}</span>
                </div>
              </div>
            </div>

            {/* Filters */}
            <div className="flex flex-col xl:flex-row gap-3 mb-5">
              <div className="flex-1 relative">
                <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" />
                <input
                  type="text"
                  placeholder="Search tests..."
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  className="w-full pl-10 pr-4 py-2 bg-gray-800 border border-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-purple-500"
                />
              </div>
              <div className="flex flex-wrap items-center gap-2">
                <Filter className="w-4 h-4 text-gray-400" />
                  <select
                    value={statusFilter}
                    onChange={(e) => setStatusFilter(e.target.value as StatusFilter)}
                    className="min-w-44 px-4 py-2 bg-gray-800 border border-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-purple-500"
                  >
                    <option value="all">All Status</option>
                    <option value="pass">Pass</option>
                    <option value="mismatch">Mismatch</option>
                    <option value="runtime_error">Runtime Error</option>
                    <option value="fail">Build Error</option>
                    <option value="unsupported">Unsupported</option>
                    <option value="blocked">Blocked</option>
                  </select>
                <select
                  value={suiteFilter}
                  onChange={(e) => setSuiteFilter(e.target.value)}
                  className="min-w-36 px-4 py-2 bg-gray-800 border border-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-purple-500"
                >
                  <option value="all">All Suites</option>
                  {suiteOptions.map(suite => (
                    <option key={suite} value={suite}>{suite}</option>
                  ))}
                </select>
                <span className="text-sm text-gray-400">
                  Showing {filteredTests.length.toLocaleString()} of {tests.length.toLocaleString()}
                </span>
              </div>
            </div>

            {/* Test List */}
            <div className="overflow-hidden rounded-lg border border-gray-700 bg-gray-800">
              <div className="overflow-x-auto">
                <table className="w-full min-w-[1120px] table-fixed">
                  <thead className="bg-gray-800/50">
                    <tr>
                      <th className="w-40 px-4 py-3 text-left text-sm font-medium text-gray-400">Status</th>
                      <th className="px-4 py-3 text-left text-sm font-medium text-gray-400">Test Name / Reason</th>
                      <th className="w-36 px-4 py-3 text-left text-sm font-medium text-gray-400">Suite</th>
                      <th className="w-28 px-4 py-3 text-right text-sm font-medium text-gray-400">Duration</th>
                      <th className="w-32 px-4 py-3 text-right text-sm font-medium text-gray-400">Details</th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-gray-700">
                    {filteredTests.length === 0 ? (
                      <tr>
                        <td colSpan={5} className="px-4 py-10 text-center text-gray-400">
                          No tests match the current filters.
                        </td>
                      </tr>
                    ) : filteredTests.map((test) => {
                      const expanded = expandedTestId === test.id
                      const canExpand = hasTestDetails(test)
                      const primaryReason = test.reason || test.error
                      return (
                        <Fragment key={test.id}>
                          <tr className="hover:bg-gray-700/50 transition-colors">
                            <td className="px-4 py-3 align-top">
                              <div className="flex items-center gap-2">
                                {getStatusIcon(test.status)}
                                <span className={`px-2 py-1 text-xs font-medium rounded border ${getStatusColor(test.status)}`}>
                                  {test.status.toUpperCase()}
                                </span>
                              </div>
                            </td>
                            <td className="px-4 py-3 align-top font-medium">
                              <div className="truncate text-gray-100" title={test.name}>{test.name}</div>
                              {primaryReason ? (
                                <div className="mt-1 line-clamp-2 text-sm font-semibold text-red-400" title={primaryReason}>
                                  {primaryReason}
                                </div>
                              ) : null}
                              {test.case && test.case !== test.name ? (
                                <div className="mt-1 truncate text-xs text-gray-500" title={test.case}>{test.case}</div>
                              ) : null}
                            </td>
                            <td className="px-4 py-3 align-top text-gray-400">{test.suite}</td>
                            <td className="px-4 py-3 align-top text-right text-gray-400">{formatDuration(test.duration)}</td>
                            <td className="px-4 py-3 align-top text-right">
                              <button
                                type="button"
                                disabled={!canExpand}
                                onClick={() => setExpandedTestId(expanded ? null : test.id)}
                                className="inline-flex items-center gap-1 rounded-md border border-gray-700 bg-gray-900 px-3 py-2 text-sm font-medium text-gray-100 transition-colors hover:bg-gray-700 disabled:cursor-not-allowed disabled:opacity-40"
                              >
                                {expanded ? <ChevronDown className="h-4 w-4" /> : <ChevronRight className="h-4 w-4" />}
                                Details
                              </button>
                            </td>
                          </tr>
                          {expanded ? (
                            <tr className="bg-gray-900/40">
                              <td colSpan={5} className="px-4 py-4">
                                <div className="space-y-4">
                                  <div className="flex flex-wrap gap-2 text-xs text-gray-300">
                                    {test.target ? <span className="rounded border border-gray-700 bg-gray-800 px-2 py-1">target: {test.target}</span> : null}
                                    {test.error_line !== undefined ? <span className="rounded border border-gray-700 bg-gray-800 px-2 py-1">error line: {test.error_line}</span> : null}
                                    {test.case ? <span className="max-w-full truncate rounded border border-gray-700 bg-gray-800 px-2 py-1" title={test.case}>case: {test.case}</span> : null}
                                  </div>
                                  <DetailBlock title={test.error ? 'Build Error / Reason' : 'Reason'} value={test.reason || test.error} tone="error" />
                                  <div className="grid grid-cols-1 gap-4 xl:grid-cols-2">
                                    <DetailBlock title="Expected" value={test.expected} />
                                    <DetailBlock title="Actual" value={test.actual} />
                                  </div>
                                  <DetailBlock title="Stderr" value={test.stderr} tone="error" />
                                  <DetailBlock title="Test Code" value={test.source_code} tone="code" />
                                </div>
                              </td>
                            </tr>
                          ) : null}
                        </Fragment>
                      )
                    })}
                  </tbody>
                </table>
              </div>
            </div>
            </>
            )}
          </div>
        )}

        {activeTab === 'coverage' && (
          <div>
            {coverageLoading ? (
              <div className="flex items-center justify-center py-12">
                <div className="text-gray-400">Loading coverage data...</div>
              </div>
            ) : coverageError ? (
              <div className="bg-red-500/10 border border-red-500/20 rounded-lg p-4 text-red-500">
                Error loading coverage data: {coverageError}
              </div>
            ) : (
              <>
            <div className="grid grid-cols-1 xl:grid-cols-[minmax(0,1.2fr)_minmax(360px,0.8fr)] gap-5 mb-5">
              <div className="bg-gray-800 rounded-lg p-6 border border-gray-700">
                <div className="mb-5 flex flex-wrap items-start justify-between gap-3">
                  <div>
                    <h3 className="text-lg font-semibold">Implementation Status</h3>
                    {coverageTotalMismatch ? (
                      <p className="mt-1 text-sm text-yellow-500">
                        Category totals exceed the denominator; bars are capped at 100%.
                      </p>
                    ) : null}
                  </div>
                  <div className="text-right text-sm text-gray-400">
                    Total <span className="font-semibold text-gray-100">{coverage.total.toLocaleString()}</span>
                  </div>
                </div>
                <div className="space-y-4">
                  <div>
                    <div className="flex justify-between mb-2">
                      <span className="text-gray-400">Implemented</span>
                      <span className="font-medium">{coverage.implemented.toLocaleString()}/{coverage.total.toLocaleString()} ({formatPercent(percentOf(coverage.implemented, coverage.total))})</span>
                    </div>
                    <div className="w-full bg-gray-700 rounded-full h-2">
                      <div
                        className="bg-green-500 h-2 rounded-full transition-all"
                        style={{ width: `${percentOf(coverage.implemented, coverage.total)}%` }}
                      />
                    </div>
                  </div>
                  <div>
                    <div className="flex justify-between mb-2">
                      <span className="text-gray-400">Unimplemented</span>
                      <span className="font-medium">{coverage.unimplemented.toLocaleString()}/{coverage.total.toLocaleString()} ({formatPercent(percentOf(coverage.unimplemented, coverage.total))})</span>
                    </div>
                    <div className="w-full bg-gray-700 rounded-full h-2">
                      <div
                        className="bg-red-500 h-2 rounded-full transition-all"
                        style={{ width: `${percentOf(coverage.unimplemented, coverage.total)}%` }}
                      />
                    </div>
                  </div>
                  <div>
                    <div className="flex justify-between mb-2">
                      <span className="text-gray-400">Future</span>
                      <span className="font-medium">{coverage.future.toLocaleString()}/{coverage.total.toLocaleString()} ({formatPercent(percentOf(coverage.future, coverage.total))})</span>
                    </div>
                    <div className="w-full bg-gray-700 rounded-full h-2">
                      <div
                        className="bg-yellow-500 h-2 rounded-full transition-all"
                        style={{ width: `${percentOf(coverage.future, coverage.total)}%` }}
                      />
                    </div>
                  </div>
                </div>
              </div>

              <div className="bg-gray-800 rounded-lg p-6 border border-gray-700">
                <h3 className="text-lg font-semibold mb-4">Priority Breakdown</h3>
                <div className="space-y-3">
                  <div className="flex items-center justify-between">
                    <span className="text-gray-400">P0 (Critical)</span>
                    <span className="px-2 py-1 bg-red-500/10 text-red-500 rounded text-sm font-medium">{coverage.byPriority.p0}</span>
                  </div>
                  <div className="flex items-center justify-between">
                    <span className="text-gray-400">P1 (High)</span>
                    <span className="px-2 py-1 bg-orange-500/10 text-orange-500 rounded text-sm font-medium">{coverage.byPriority.p1}</span>
                  </div>
                  <div className="flex items-center justify-between">
                    <span className="text-gray-400">P2 (Medium)</span>
                    <span className="px-2 py-1 bg-yellow-500/10 text-yellow-500 rounded text-sm font-medium">{coverage.byPriority.p2}</span>
                  </div>
                  <div className="flex items-center justify-between">
                    <span className="text-gray-400">P3 (Low)</span>
                    <span className="px-2 py-1 bg-blue-500/10 text-blue-500 rounded text-sm font-medium">{coverage.byPriority.p3}</span>
                  </div>
                  <div className="flex items-center justify-between">
                    <span className="text-gray-400">Future</span>
                    <span className="px-2 py-1 bg-gray-500/10 text-gray-500 rounded text-sm font-medium">{coverage.byPriority.future}</span>
                  </div>
                </div>
              </div>
            </div>

            <div className="grid grid-cols-1 xl:grid-cols-3 gap-5 mb-5">
              <div className="bg-gray-800 rounded-lg p-6 border border-gray-700 xl:col-span-1">
                <h3 className="text-lg font-semibold mb-4">Coverage Mix</h3>
                <div className="h-72">
                  <ResponsiveContainer width="100%" height="100%">
                    <PieChart>
                      <Pie
                        data={coverageStatusData}
                        dataKey="value"
                        nameKey="name"
                        innerRadius={58}
                        outerRadius={92}
                        paddingAngle={2}
                      >
                        {coverageStatusData.map((entry, index) => (
                          <Cell key={entry.name} fill={coverageColors[index % coverageColors.length]} />
                        ))}
                      </Pie>
                      <Tooltip formatter={(value) => [chartNumber(value).toLocaleString(), 'cases']} />
                      <Legend />
                    </PieChart>
                  </ResponsiveContainer>
                </div>
              </div>

              <div className="bg-gray-800 rounded-lg p-6 border border-gray-700 xl:col-span-2">
                <h3 className="text-lg font-semibold mb-4">Suite Coverage</h3>
                <div className="h-72">
                  <ResponsiveContainer width="100%" height="100%">
                    <BarChart data={suiteCoverageData} margin={{ top: 8, right: 16, bottom: 8, left: 0 }}>
                      <CartesianGrid strokeDasharray="3 3" stroke="#374151" />
                      <XAxis dataKey="suite" stroke="#9ca3af" />
                      <YAxis stroke="#9ca3af" domain={[0, 100]} tickFormatter={(value) => `${value}%`} />
                      <Tooltip formatter={(value) => [formatPercent(chartNumber(value)), 'coverage']} />
                      <Legend />
                      <Bar dataKey="buildRate" name="Build" fill="#3b82f6" radius={[4, 4, 0, 0]} />
                      <Bar dataKey="semanticRate" name="Semantic" fill="#22c55e" radius={[4, 4, 0, 0]} />
                    </BarChart>
                  </ResponsiveContainer>
                </div>
              </div>
            </div>

            <div className="bg-gray-800 rounded-lg p-6 border border-gray-700">
              <h3 className="text-lg font-semibold mb-4">Priority Chart</h3>
              <div className="h-72">
                <ResponsiveContainer width="100%" height="100%">
                  <BarChart data={priorityData} margin={{ top: 8, right: 16, bottom: 8, left: 8 }}>
                    <CartesianGrid strokeDasharray="3 3" stroke="#374151" />
                    <XAxis dataKey="name" stroke="#9ca3af" />
                    <YAxis stroke="#9ca3af" />
                    <Tooltip formatter={(value) => [chartNumber(value).toLocaleString(), 'items']} />
                    <Bar dataKey="value" name="Open items" radius={[4, 4, 0, 0]}>
                      {priorityData.map((entry) => (
                        <Cell key={entry.name} fill={entry.fill} />
                      ))}
                    </Bar>
                  </BarChart>
                </ResponsiveContainer>
              </div>
            </div>
            </>
            )}
          </div>
        )}

        {activeTab === 'history' && (
          <div>
            {historyLoading ? (
              <div className="flex items-center justify-center py-12">
                <div className="text-gray-400">Loading historical data...</div>
              </div>
            ) : historyError ? (
              <div className="bg-red-500/10 border border-red-500/20 rounded-lg p-4 text-red-500">
                Error loading historical data: {historyError}
              </div>
            ) : (
              <>
            <div className="grid grid-cols-1 lg:grid-cols-3 gap-4 mb-5">
              <div className="bg-gray-800 rounded-lg p-4 border border-gray-700">
                <div className="text-sm text-gray-400 mb-1">Latest Run</div>
                <div className="truncate text-xl font-semibold" title={latestTrend?.run_id}>{latestTrend?.run_id ?? '-'}</div>
              </div>
              <div className="bg-gray-800 rounded-lg p-4 border border-gray-700">
                <div className="text-sm text-gray-400 mb-1">Pass Delta</div>
                <div className={latestTrend?.deltas.passed !== null && latestTrend?.deltas.passed !== undefined && latestTrend.deltas.passed < 0 ? 'text-xl font-semibold text-red-500' : 'text-xl font-semibold text-green-500'}>
                  {formatDelta(latestTrend?.deltas.passed ?? null)}
                </div>
              </div>
              <div className="bg-gray-800 rounded-lg p-4 border border-gray-700">
                <div className="text-sm text-gray-400 mb-1">Regression Flags</div>
                <div className={latestTrend?.regressionReasons.length ? 'flex items-center gap-2 text-red-400' : 'text-green-500'}>
                  {latestTrend?.regressionReasons.length ? (
                    <>
                      <AlertTriangle className="w-5 h-5 shrink-0" />
                      <span>{latestTrend.regressionReasons.join(', ')}</span>
                    </>
                  ) : (
                    'none'
                  )}
                </div>
              </div>
            </div>

            <div className="grid grid-cols-1 xl:grid-cols-2 gap-5 mb-5">
              <div className="bg-gray-800 rounded-lg p-6 border border-gray-700">
                <h3 className="text-lg font-semibold mb-4">Result Trend</h3>
                <div className="h-72">
                  <ResponsiveContainer width="100%" height="100%">
                    <LineChart data={trendRuns} margin={{ top: 8, right: 16, bottom: 8, left: 8 }}>
                      <CartesianGrid strokeDasharray="3 3" stroke="#374151" />
                      <XAxis dataKey="trendLabel" stroke="#9ca3af" />
                      <YAxis stroke="#9ca3af" />
                      <Tooltip />
                      <Legend />
                      <Line type="monotone" dataKey="passed" name="Passed" stroke="#22c55e" strokeWidth={2} dot={false} />
                      <Line type="monotone" dataKey="failed" name="Failed" stroke="#ef4444" strokeWidth={2} dot={false} />
                      <Line type="monotone" dataKey="skipped" name="Skipped" stroke="#eab308" strokeWidth={2} dot={false} />
                    </LineChart>
                  </ResponsiveContainer>
                </div>
              </div>

              <div className="bg-gray-800 rounded-lg p-6 border border-gray-700">
                <h3 className="text-lg font-semibold mb-4">Performance Trend</h3>
                <div className="h-72">
                  <ResponsiveContainer width="100%" height="100%">
                    <LineChart data={trendRuns} margin={{ top: 8, right: 16, bottom: 8, left: 8 }}>
                      <CartesianGrid strokeDasharray="3 3" stroke="#374151" />
                      <XAxis dataKey="trendLabel" stroke="#9ca3af" />
                      <YAxis stroke="#9ca3af" />
                      <Tooltip formatter={(value) => [`${chartNumber(value)}s`, 'duration']} />
                      <Legend />
                      <Line type="monotone" dataKey="compile_time" name="Compile" stroke="#3b82f6" strokeWidth={2} dot={false} />
                      <Line type="monotone" dataKey="runtime" name="Runtime" stroke="#f97316" strokeWidth={2} dot={false} />
                    </LineChart>
                  </ResponsiveContainer>
                </div>
              </div>
            </div>

            <div className="bg-gray-800 rounded-lg border border-gray-700 overflow-hidden">
              <div className="overflow-x-auto">
              <table className="w-full min-w-[1120px]">
                <thead className="bg-gray-800/50">
                  <tr>
                    <th className="w-44 px-4 py-3 text-left text-sm font-medium text-gray-400">Run ID</th>
                    <th className="w-44 px-4 py-3 text-left text-sm font-medium text-gray-400">Timestamp</th>
                    <th className="w-24 px-4 py-3 text-right text-sm font-medium text-gray-400">Passed</th>
                    <th className="w-24 px-4 py-3 text-right text-sm font-medium text-gray-400">Failed</th>
                    <th className="w-24 px-4 py-3 text-right text-sm font-medium text-gray-400">Skipped</th>
                    <th className="w-28 px-4 py-3 text-right text-sm font-medium text-gray-400">Compile</th>
                    <th className="w-28 px-4 py-3 text-right text-sm font-medium text-gray-400">Runtime</th>
                    <th className="px-4 py-3 text-left text-sm font-medium text-gray-400">Delta</th>
                    <th className="w-64 px-4 py-3 text-left text-sm font-medium text-gray-400">Regression</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-gray-700">
                  {historyRows.map((run) => (
                    <tr key={`${run.run_id}-${run.timestamp}`} className="hover:bg-gray-700/50 transition-colors">
                      <td className="px-4 py-3 font-medium">
                        <div className="max-w-40 truncate" title={run.run_id}>{run.run_id}</div>
                      </td>
                      <td className="px-4 py-3 text-gray-400">{run.displayTime}</td>
                      <td className="px-4 py-3 text-right text-green-500">{run.passed.toLocaleString()}</td>
                      <td className="px-4 py-3 text-right text-red-500">{run.failed.toLocaleString()}</td>
                      <td className="px-4 py-3 text-right text-yellow-500">{run.skipped.toLocaleString()}</td>
                      <td className="px-4 py-3 text-right text-gray-400">{run.compile_time}s</td>
                      <td className="px-4 py-3 text-right text-gray-400">{run.runtime}s</td>
                      <td className="px-4 py-3 text-gray-400">
                        <div className="flex flex-wrap gap-2 text-xs">
                          <span className={run.deltas.passed !== null && run.deltas.passed < 0 ? 'text-red-400' : 'text-green-400'}>
                            pass {formatDelta(run.deltas.passed)}
                          </span>
                          <span className={run.deltas.failed !== null && run.deltas.failed > 0 ? 'text-red-400' : 'text-green-400'}>
                            fail {formatDelta(run.deltas.failed)}
                          </span>
                          <span>skip {formatDelta(run.deltas.skipped)}</span>
                          <span>compile {formatDurationDelta(run.deltas.compile_time)}</span>
                          <span>runtime {formatDurationDelta(run.deltas.runtime)}</span>
                        </div>
                      </td>
                      <td className="px-4 py-3">
                        {run.regressionReasons.length ? (
                          <span className="inline-flex max-w-60 items-center gap-1 rounded border border-red-500/20 bg-red-500/10 px-2 py-1 text-xs text-red-400">
                            <AlertTriangle className="w-3 h-3 shrink-0" />
                            <span className="truncate" title={run.regressionReasons.join(', ')}>
                              {run.regressionReasons.join(', ')}
                            </span>
                          </span>
                        ) : (
                          <span className="px-2 py-1 rounded border border-green-500/20 bg-green-500/10 text-xs text-green-400">stable</span>
                        )}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
              </div>
            </div>
            </>
            )}
          </div>
        )}
      </main>
    </div>
  )
}

export default App
