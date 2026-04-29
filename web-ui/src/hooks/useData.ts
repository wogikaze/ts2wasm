import { useState, useEffect } from 'react'
import type { TestResult, CoverageData, HistoricalData } from '../types'

const DEFAULT_LIVE_POLL_MS = 2000
const MIN_LIVE_POLL_MS = 250

type LiveStatus = 'static' | 'connecting' | 'connected' | 'error'

function liveModeEnabled() {
  if (typeof window === 'undefined') return false
  const value = new URLSearchParams(window.location.search).get('live')
  return value === '1' || value === 'true'
}

function livePollIntervalMs() {
  if (typeof window === 'undefined') return DEFAULT_LIVE_POLL_MS
  const value = Number(new URLSearchParams(window.location.search).get('liveIntervalMs'))
  return Number.isFinite(value) && value >= MIN_LIVE_POLL_MS ? value : DEFAULT_LIVE_POLL_MS
}

export function useTestData() {
  const [tests, setTests] = useState<TestResult[]>([])
  const [summary, setSummary] = useState({ total: 0, passed: 0, failed: 0, skipped: 0 })
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [liveStatus, setLiveStatus] = useState<LiveStatus>('static')
  const [lastUpdated, setLastUpdated] = useState<string | null>(null)

  useEffect(() => {
    let cancelled = false
    const liveMode = liveModeEnabled()
    const intervalMs = livePollIntervalMs()

    async function loadData(mode: 'static' | 'live') {
      try {
        if (mode === 'live') setLiveStatus(current => current === 'connected' ? current : 'connecting')
        const cacheBust = mode === 'live' ? `?t=${Date.now()}` : ''
        const response = await fetch(`/data/test-results.json${cacheBust}`, { cache: 'no-store' })
        if (!response.ok) throw new Error('Failed to load test results')
        const data = await response.json()
        if (cancelled) return
        setTests(data.tests || [])
        setSummary(data.summary || { total: 0, passed: 0, failed: 0, skipped: 0 })
        setError(null)
        setLastUpdated(new Date().toISOString())
        setLiveStatus(mode === 'live' ? 'connected' : 'static')
      } catch (err) {
        if (cancelled) return
        setError(err instanceof Error ? err.message : 'Unknown error')
        setLiveStatus(mode === 'live' ? 'error' : 'static')
      } finally {
        if (!cancelled) setLoading(false)
      }
    }

    loadData(liveMode ? 'live' : 'static')
    if (!liveMode) {
      return () => {
        cancelled = true
      }
    }

    const interval = window.setInterval(() => loadData('live'), intervalMs)
    return () => {
      cancelled = true
      window.clearInterval(interval)
    }
  }, [])

  return { tests, summary, loading, error, liveStatus, lastUpdated, liveMode: liveModeEnabled() }
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
