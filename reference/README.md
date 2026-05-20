# Reference Corpus

`reference/` stores external corpora or symlinks used by reference coverage. It is not the source of project design truth.

## Commands

```bash
python scripts/manager.py reference-corpus verify
python scripts/manager.py reference-corpus write-lock
python scripts/manager.py link-reference
python scripts/manager.py reference-coverage test262 --jsonl
```

## Rules

- Keep locks and evidence commands with generated coverage data.
- Do not copy large external repositories into tracked docs.
- Use `docs/06-testing-and-coverage.md` and `docs/15-coverage-matrix.md` for current coverage policy.
