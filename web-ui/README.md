# ts2wasm Web UI

Interactive web UI for displaying test results, coverage, and historical data - inspired by Playwright's test reporter.

## Features

- **Test Results**: Interactive test result browser with filtering and search
- **Coverage Visualization**: Implementation status, priority breakdown, and suite charts
- **Historical Comparison**: Track test runs over time with result deltas, regression flags, and performance trends
- **Real-time Updates**: Live updates during test runs (planned)
- **Export Functionality**: Export the active view as JSON or CSV, or use browser print/PDF

## Tech Stack

- **Framework**: Vite + React + TypeScript
- **Styling**: Tailwind CSS
- **Icons**: Lucide React
- **Charts**: Recharts
- **Data Loading**: JSON-based API

## Development

### Setup

```bash
cd web-ui
npm install
```

### Development Server

```bash
npm run dev
```

The UI will be available at `http://localhost:5173`

### Build for Production

```bash
npm run build
```

The built files will be in `dist/`

### Data Generation

Generate web UI data from test results:

```bash
mise run web-ui-data
```

This creates JSON files in `web-ui/public/data/`:
- `test-results.json`: Test results with summary
- `coverage.json`: Coverage implementation status
- `history.json`: Historical test run data
- `metadata.json`: Generation metadata

By default the generator reads checked-in coverage artifacts from
`artifacts/coverage/results/*.json`. It can include per-case test records from a
JSONL file produced by the test runner:

```bash
mise run web-ui-data -- --test-jsonl reports/runs/<run-id>/test262-results.jsonl
```

## Usage

### Local Development

1. Generate data: `mise run web-ui-data`
2. Start dev server: `cd web-ui && npm run dev`
3. Open browser to `http://localhost:5173`

### Production Deployment

1. Generate data: `mise run web-ui-data`
2. Build: `cd web-ui && npm run build`
3. Deploy `dist/` directory to your web server

### Integration with Test Runs

The web UI can be integrated into test workflows by:

1. Running tests with JSON output
2. Generating web UI data after test completion
3. Serving the UI or deploying to static hosting

The reference coverage and test262 runners can also refresh the data files as
part of a local run:

```bash
mise run reference-coverage -- test262 --limit 50 --web-ui
mise run test262 -- --sample 50 --web-ui
```

These commands write the same `web-ui/public/data/*.json` files used by the UI.

### Export

The header export buttons operate on the active tab:

- `JSON` downloads the current test, coverage, or history payload.
- `CSV` downloads the active tab's tabular rows.
- `PDF` opens the browser print flow so the current view can be saved as PDF.

## Future Enhancements

- [ ] Real-time WebSocket updates during test runs
- [ ] Dark/light theme toggle
- [ ] CI/CD integration
- [ ] Authentication/authorization
- [ ] Mobile-responsive design improvements
