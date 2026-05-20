# ts2wasm Site

VitePress wrapper for published docs/dashboard content.

## Commands

```bash
npm install
npm run dev
npm run build
npm run serve
```

## Relationship to root docs

Root `docs/` is the canonical documentation source. `site/docs/**` is publication content and should not silently diverge from root docs.

## Dashboard

The dashboard landing page is `site/docs/dashboard/index.md`. Build dashboard assets with `python scripts/build-dashboard-site.py` or `cd web-ui && npm run build:dashboard` as appropriate.
