# ts2wasm Web UI

React/Vite dashboard for coverage and test-result data.

## Data files

The app reads from `public/data/` or the published dashboard data path:

- `test-results.json`
- `coverage.json`
- `history.json`

Type contracts live in `src/types.ts`.

## Commands

```bash
npm install
npm run dev
npm run build
npm run build:dashboard
npm run lint
```

## Live mode

In dev mode live polling is enabled. In production, append `?live=1` or `?live=true`. Use `?liveIntervalMs=1000` or larger to tune polling.

## Project docs

- Dashboard reporting contract: `../docs/18-web-ui-reporting.md`
- Coverage policy: `../docs/15-coverage-matrix.md`
