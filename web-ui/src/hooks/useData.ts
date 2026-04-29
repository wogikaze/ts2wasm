import { useState, useEffect } from 'react'
import type { TestResult, CoverageData, HistoricalData } from '../types'

export function useTestData() {
  const [tests, setTests] = useState<TestResult[]>([])
  const [summary, setSummary] = useState({ total: 0, passed: 0, failed: 0, skipped: 0 })
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    async function loadData() {
      try {
        const response = await fetch('/data/test-results.json')
        if (!response.ok) throw new Error('Failed to load test results')
        const data = await response.json()
        setTests(data.tests || [])
        setSummary(data.summary || { total: 0, passed: 0, failed: 0, skipped: 0 })
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Unknown error')
      } finally {
        setLoading(false)
      }
    }

    loadData()
  }, [])

  return { tests, summary, loading, error }
}

export function useCoverageData() {
  const [coverage, setCoverage] = useState<CoverageData>({
    total: 0,
    implemented: 0,
    unimplemented: 0,
    future: 0,
    byPriority: { p0: 0, p1: 0, p2: 0, p3: 0, future: 0 }
  })
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    async function loadData() {
      try {
        const response = await fetch('/data/coverage.json')
        if (!response.ok) throw new Error('Failed to load coverage data')
        const data = await response.json()
        setCoverage(data)
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Unknown error')
      } finally {
        setLoading(false)
      }
    }

    loadData()
  }, [])

  return { coverage, loading, error }
}

export function useHistoricalData() {
  const [history, setHistory] = useState<HistoricalData[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    async function loadData() {
      try {
        const response = await fetch('/data/history.json')
        if (!response.ok) throw new Error('Failed to load historical data')
        const data = await response.json()
        setHistory(data || [])
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Unknown error')
      } finally {
        setLoading(false)
      }
    }

    loadData()
  }, [])

  return { history, loading, error }
}
