# Deferred Discord Report

Run id: `233-static-module-ir-binding-20260428T094954Z`
Command: `scripts/manager discord-report --run-id 233-static-module-ir-binding-20260428T094954Z`
Result: deferred

The report was not sent because no Discord webhook was configured in this worktree environment.

```text
DISCORD_WEBHOOK_URL is not set in the environment or .env.
```

No webhook URL, token, or secret value was present in the command output.

## Payload Summary

```text
PROGRESS issue=233
branch=agent/233-static-module-ir-binding-20260428T094954Z
implementation_commit=43c0de6bfa4b309399f39b43f1b7daefa19512ed
report=reports/runs/233-static-module-ir-binding-20260428T094954Z/cycle_report.md
validation=cargo fmt, package gates, CLI module shard, static entry builds, issue/agent health, full nextest, repo smoke all passed
merge_request=yes
```
