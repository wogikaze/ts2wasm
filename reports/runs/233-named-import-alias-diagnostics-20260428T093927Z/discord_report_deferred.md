# Deferred Discord Report

Run ID: `233-named-import-alias-diagnostics-20260428T093927Z`
Issue: `233`
Status: PROGRESS
Report: `reports/runs/233-named-import-alias-diagnostics-20260428T093927Z/cycle_report.md`

## Reason

`scripts/manager discord-report --run-id 233-named-import-alias-diagnostics-20260428T093927Z` failed because `DISCORD_WEBHOOK_URL` was not configured in the environment or `.env`.

```text
エラー: DISCORD_WEBHOOK_URL が環境変数または .env に設定されていません
```
