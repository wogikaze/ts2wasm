# ts2wasm Documentation Site

This is a VitePress-based documentation site for the ts2wasm project that provides a browsable interface for:

- **Documentation**: All design docs from `docs/`
- **Issues**: Issue tracker with ready queue and done issues
- **Fixtures**: Test fixture browser with code viewing
- **Coverage**: Test coverage results and statistics

## Setup

First-time setup:

```bash
cd site
npm install
```

## Usage

### Generate content only

```bash
# From project root
mise run gen-site
# or
python scripts/gen-site.py
```

### Build the site

```bash
# From project root
mise run build-site
# or
python scripts/gen-site.py && cd site && npm run build
```

### Serve locally for development

```bash
# From project root
mise run serve-site
# or
cd site && npm run dev
```

The site will be available at `http://localhost:5173/site/`

## Automatic Updates

The site is automatically updated after running test suites:

- `mise run test262` automatically generates site content after test completion
- Test results are saved to `artifacts/coverage/results/` for display in the coverage section

## Git Ignore

The following are gitignored (generated artifacts):

- `site/node_modules/` - npm dependencies
- `site/docs/.vitepress/cache/` - VitePress build cache
- `site/docs/.vitepress/dist/` - Built site output
- `artifacts/coverage/results/` - Test result JSON files

## Site Structure

```
site/
├── docs/                    # VitePress content directory
│   ├── .vitepress/         # VitePress config
│   ├── index.md           # Home page
│   ├── docs/              # Documentation pages
│   ├── issues/            # Issue listing pages
│   ├── fixtures/          # Test fixture pages
│   └── coverage/          # Coverage report pages
├── node_modules/          # Dependencies (gitignored)
└── package.json           # npm configuration
```

## Customization

To customize the site appearance or navigation, edit:

- `site/docs/.vitepress/config.ts` - VitePress configuration
- `scripts/gen-site.py` - Content generation logic
