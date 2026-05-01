# OpenCode high-parallel wave planner

You are the parent planner.

Goal:
Create a high-parallel wave plan from `issues/open/*.md`.

Do not implement.
Do not edit source files.
Only inspect issues and repository structure.

Output JSON only:
{
  "wave": [
    {
      "worker_id": "w01",
      "issue_files": ["issues/open/....md"],
      "phase": "phase-b-parallel",
      "likely_files": ["..."],
      "allowed_files": ["..."],
      "forbidden_files": ["..."],
      "validation": ["..."],
      "risk": "low|medium|high",
      "parallel_safe": true,
      "reason": "..."
    }
  ],
  "deferred": [
    {
      "issue_file": "...",
      "reason": "conflicts with selected worker or requires contract phase"
    }
  ],
  "contract_first": [
    {
      "issue_file": "...",
      "reason": "must define shared API/type/ABI before fanout"
    }
  ]
}

Selection policy:
- Prefer file-disjoint issues.
- Prefer small implementation slices.
- Prefer test/fixture/docs/CLI/parser work that does not edit the same central files.
- Do not select two workers that likely touch the same high-conflict file.
- If an issue is large, split conceptually into Phase A/B/C in the plan.
- Max selected workers: ${PARALLELISM}.
- Treat these as high-conflict unless specifically assigned to a single worker:
  - Cargo.lock
  - issues/index.md
  - .agents/state/**
  - shared runtime ABI files
  - central emitter/resolver/lowering files
  - docs/current-state.md

Write the final plan to:
reports/runs/<run_id>/wave-<wave>/wave-plan.json

Do not write any other file.
