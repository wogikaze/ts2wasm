import { useState } from 'react'
import { Play, CheckCircle, XCircle, AlertCircle, SkipForward, BarChart3, History, Download, Search, Filter } from 'lucide-react'
import { useTestData, useCoverageData, useHistoricalData } from './hooks/useData'
import './index.css'

function App() {
  const [activeTab, setActiveTab] = useState<'tests' | 'coverage' | 'history'>('tests')
  const [searchQuery, setSearchQuery] = useState('')
  const [statusFilter, setStatusFilter] = useState<'all' | 'pass' | 'fail' | 'skip'>('all')

  // Load real data
  const { tests, summary, loading: testsLoading, error: testsError } = useTestData()
  const { coverage, loading: coverageLoading, error: coverageError } = useCoverageData()
  const { history, loading: historyLoading, error: historyError } = useHistoricalData()

  const filteredTests = tests.filter(test => {
    const matchesSearch = test.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
                         test.suite.toLowerCase().includes(searchQuery.toLowerCase())
    const matchesStatus = statusFilter === 'all' || test.status === statusFilter
    return matchesSearch && matchesStatus
  })

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'pass': return <CheckCircle className="w-5 h-5 text-green-500" />
      case 'fail': return <XCircle className="w-5 h-5 text-red-500" />
      case 'skip': return <SkipForward className="w-5 h-5 text-yellow-500" />
      default: return <AlertCircle className="w-5 h-5 text-gray-500" />
    }
  }

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'pass': return 'bg-green-500/10 text-green-500 border-green-500/20'
      case 'fail': return 'bg-red-500/10 text-red-500 border-red-500/20'
      case 'skip': return 'bg-yellow-500/10 text-yellow-500 border-yellow-500/20'
      default: return 'bg-gray-500/10 text-gray-500 border-gray-500/20'
    }
  }

  return (
    <div className="min-h-screen bg-gray-900 text-gray-100">
      {/* Header */}
      <header className="border-b border-gray-800 bg-gray-900/50 backdrop-blur-sm sticky top-0 z-10">
        <div className="max-w-7xl mx-auto px-4 py-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <Play className="w-8 h-8 text-purple-500" />
              <h1 className="text-2xl font-bold">ts2wasm Test Reporter</h1>
            </div>
            <div className="flex items-center gap-4">
              <button className="flex items-center gap-2 px-4 py-2 bg-purple-600 hover:bg-purple-700 rounded-lg transition-colors">
                <Download className="w-4 h-4" />
                Export
              </button>
            </div>
          </div>
        </div>
      </header>

      {/* Navigation */}
      <nav className="border-b border-gray-800 bg-gray-900">
        <div className="max-w-7xl mx-auto px-4">
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
      <main className="max-w-7xl mx-auto px-4 py-6">
        {activeTab === 'tests' && (
          <div>
            {testsLoading ? (
              <div className="flex items-center justify-center py-12">
                <div className="text-gray-400">Loading test results...</div>
              </div>
            ) : testsError ? (
              <div className="bg-red-500/10 border border-red-500/20 rounded-lg p-4 text-red-500">
                Error loading test results: {testsError}
              </div>
            ) : (
              <>
            {/* Summary Cards */}
            <div className="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
              <div className="bg-gray-800 rounded-lg p-4 border border-gray-700">
                <div className="flex items-center justify-between">
                  <span className="text-gray-400">Total</span>
                  <span className="text-2xl font-bold">{summary.total}</span>
                </div>
              </div>
              <div className="bg-gray-800 rounded-lg p-4 border border-gray-700">
                <div className="flex items-center justify-between">
                  <span className="text-gray-400">Passed</span>
                  <span className="text-2xl font-bold text-green-500">
                    {summary.passed}
                  </span>
                </div>
              </div>
              <div className="bg-gray-800 rounded-lg p-4 border border-gray-700">
                <div className="flex items-center justify-between">
                  <span className="text-gray-400">Failed</span>
                  <span className="text-2xl font-bold text-red-500">
                    {summary.failed}
                  </span>
                </div>
              </div>
              <div className="bg-gray-800 rounded-lg p-4 border border-gray-700">
                <div className="flex items-center justify-between">
                  <span className="text-gray-400">Skipped</span>
                  <span className="text-2xl font-bold text-yellow-500">
                    {summary.skipped}
                  </span>
                </div>
              </div>
            </div>

            {/* Filters */}
            <div className="flex gap-4 mb-6">
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
              <div className="flex items-center gap-2">
                <Filter className="w-4 h-4 text-gray-400" />
                <select
                  value={statusFilter}
                  onChange={(e) => setStatusFilter(e.target.value as any)}
                  className="px-4 py-2 bg-gray-800 border border-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-purple-500"
                >
                  <option value="all">All Status</option>
                  <option value="pass">Pass</option>
                  <option value="fail">Fail</option>
                  <option value="skip">Skip</option>
                </select>
              </div>
            </div>

            {/* Test List */}
            <div className="bg-gray-800 rounded-lg border border-gray-700 overflow-hidden">
              <table className="w-full">
                <thead className="bg-gray-800/50">
                  <tr>
                    <th className="px-4 py-3 text-left text-sm font-medium text-gray-400">Status</th>
                    <th className="px-4 py-3 text-left text-sm font-medium text-gray-400">Test Name</th>
                    <th className="px-4 py-3 text-left text-sm font-medium text-gray-400">Suite</th>
                    <th className="px-4 py-3 text-left text-sm font-medium text-gray-400">Duration</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-gray-700">
                  {filteredTests.map((test) => (
                    <tr key={test.id} className="hover:bg-gray-700/50 transition-colors">
                      <td className="px-4 py-3">
                        <div className="flex items-center gap-2">
                          {getStatusIcon(test.status)}
                          <span className={`px-2 py-1 text-xs font-medium rounded border ${getStatusColor(test.status)}`}>
                            {test.status.toUpperCase()}
                          </span>
                        </div>
                      </td>
                      <td className="px-4 py-3 font-medium">{test.name}</td>
                      <td className="px-4 py-3 text-gray-400">{test.suite}</td>
                      <td className="px-4 py-3 text-gray-400">{test.duration}ms</td>
                    </tr>
                  ))}
                </tbody>
              </table>
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
            <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
              <div className="bg-gray-800 rounded-lg p-6 border border-gray-700">
                <h3 className="text-lg font-semibold mb-4">Implementation Status</h3>
                <div className="space-y-4">
                  <div>
                    <div className="flex justify-between mb-2">
                      <span className="text-gray-400">Implemented</span>
                      <span className="font-medium">{coverage.implemented}/{coverage.total}</span>
                    </div>
                    <div className="w-full bg-gray-700 rounded-full h-2">
                      <div
                        className="bg-green-500 h-2 rounded-full transition-all"
                        style={{ width: `${coverage.total > 0 ? (coverage.implemented / coverage.total) * 100 : 0}%` }}
                      />
                    </div>
                  </div>
                  <div>
                    <div className="flex justify-between mb-2">
                      <span className="text-gray-400">Unimplemented</span>
                      <span className="font-medium">{coverage.unimplemented}/{coverage.total}</span>
                    </div>
                    <div className="w-full bg-gray-700 rounded-full h-2">
                      <div
                        className="bg-red-500 h-2 rounded-full transition-all"
                        style={{ width: `${coverage.total > 0 ? (coverage.unimplemented / coverage.total) * 100 : 0}%` }}
                      />
                    </div>
                  </div>
                  <div>
                    <div className="flex justify-between mb-2">
                      <span className="text-gray-400">Future</span>
                      <span className="font-medium">{coverage.future}/{coverage.total}</span>
                    </div>
                    <div className="w-full bg-gray-700 rounded-full h-2">
                      <div
                        className="bg-yellow-500 h-2 rounded-full transition-all"
                        style={{ width: `${coverage.total > 0 ? (coverage.future / coverage.total) * 100 : 0}%` }}
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
            <div className="bg-gray-800 rounded-lg border border-gray-700 overflow-hidden">
              <table className="w-full">
                <thead className="bg-gray-800/50">
                  <tr>
                    <th className="px-4 py-3 text-left text-sm font-medium text-gray-400">Run ID</th>
                    <th className="px-4 py-3 text-left text-sm font-medium text-gray-400">Timestamp</th>
                    <th className="px-4 py-3 text-left text-sm font-medium text-gray-400">Passed</th>
                    <th className="px-4 py-3 text-left text-sm font-medium text-gray-400">Failed</th>
                    <th className="px-4 py-3 text-left text-sm font-medium text-gray-400">Skipped</th>
                    <th className="px-4 py-3 text-left text-sm font-medium text-gray-400">Compile Time</th>
                    <th className="px-4 py-3 text-left text-sm font-medium text-gray-400">Runtime</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-gray-700">
                  {history.map((run) => (
                    <tr key={run.run_id} className="hover:bg-gray-700/50 transition-colors">
                      <td className="px-4 py-3 font-medium">{run.run_id}</td>
                      <td className="px-4 py-3 text-gray-400">{new Date(run.timestamp).toLocaleString()}</td>
                      <td className="px-4 py-3 text-green-500">{run.passed}</td>
                      <td className="px-4 py-3 text-red-500">{run.failed}</td>
                      <td className="px-4 py-3 text-yellow-500">{run.skipped}</td>
                      <td className="px-4 py-3 text-gray-400">{run.compile_time}s</td>
                      <td className="px-4 py-3 text-gray-400">{run.runtime}s</td>
                    </tr>
                  ))}
                </tbody>
              </table>
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
