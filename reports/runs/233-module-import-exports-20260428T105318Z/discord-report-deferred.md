# Discord Report Deferred

Run id: `233-module-import-exports-20260428T105318Z`
Issue: `233`
Branch: `agent/233-module-import-exports-20260428T105318Z`

Command:

```text
scripts/manager discord-report --run-id 233-module-import-exports-20260428T105318Z
```

Result:

```text
exit: 1
DISCORD_WEBHOOK_URL is not configured in the environment or .env.
```

Payload summary:

```text
PROGRESS issue=233
branch=agent/233-module-import-exports-20260428T105318Z
report=reports/runs/233-module-import-exports-20260428T105318Z/cycle_report.md
validation=cargo fmt, ir/backend/compiler/cli module tests, static entry/alias/shadow builds, issue-health, agent-state all passed
merge_request=yes
```
