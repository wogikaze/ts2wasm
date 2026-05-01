#!/usr/bin/env python3
"""Set up dev-loop state for a worktree."""
import json, sys, os

worktree_dir = sys.argv[1]
issue_num = sys.argv[2]
issue_title = sys.argv[3]
issue_path = sys.argv[4]
issue_area = sys.argv[5]
acceptance = sys.argv[6].split("|")
fast_cmds = sys.argv[7].split("|") if sys.argv[7] else ["mise run fmt", "mise run nextest"]
full_cmds = sys.argv[8].split("|") if sys.argv[8] else ["mise run gate"]

now = "2026-05-01T23:00:00Z"

# File affinity groups for parallel dispatch.
# Each group maps to disjoint areas of the repo.
# Issues in different groups can be assigned to parallel children safely.
SCOPE_GROUPS = {
    "frontend/parser": {
        "allowed_files": ["crates/frontend/src/parser/"],
        "forbidden_files": ["docs/", "crates/ir/src/", "crates/cli/src/backend/"]
    },
    "frontend/semantics": {
        "allowed_files": ["crates/frontend/src/", "crates/ir/src/"],
        "forbidden_files": ["docs/", "crates/backend-wasm/", "crates/cli/src/backend/"]
    },
    "ir/lowering": {
        "allowed_files": [
            "crates/ir/src/resolved.rs", "crates/ir/src/lowered.rs",
            "crates/ir/src/builtin_resolver.rs"
        ],
        "forbidden_files": ["docs/", "crates/frontend/src/", "crates/cli/src/backend/"]
    },
    "runtime/semantics": {
        "allowed_files": [
            "crates/ir/src/builtin_resolver.rs", "crates/ir/src/lowered.rs",
            "crates/cli/src/backend/expr_emit.rs", "crates/cli/src/backend/runtime_builder.rs",
            "fixtures/"
        ],
        "forbidden_files": ["docs/"]
    },
    "runtime/builtins": {
        "allowed_files": ["crates/runtime-abi/src/", "crates/cli/src/backend/runtime_builder.rs", "fixtures/"],
        "forbidden_files": ["docs/"]
    },
    "backend/wasm": {
        "allowed_files": ["crates/backend-wasm/src/"],
        "forbidden_files": ["docs/", "crates/frontend/src/", "crates/ir/src/"]
    },
    "cli/orchestration": {
        "allowed_files": ["crates/cli/src/", "crates/compiler/src/"],
        "forbidden_files": ["docs/"]
    },
    "test/fixtures": {
        "allowed_files": ["fixtures/", "crates/cli/tests/"],
        "forbidden_files": ["docs/", "crates/frontend/src/", "crates/backend-wasm/"]
    },
    "meta/issues": {
        "allowed_files": ["issues/", "docs/"],
        "forbidden_files": ["crates/", "fixtures/"]
    },
}

scope = SCOPE_GROUPS.get(issue_area, {"allowed_files": [], "forbidden_files": ["docs/"]})

task_id = f"agent-{issue_num}-{os.path.basename(worktree_dir)}"

project_state = {
    "version": 1,
    "fsm": "TASK_SELECT",
    "active_task_id": task_id,
    "updated_at": now,
    "milestone_id": None,
    "run_id": None,
    "plan_path": None,
    "verify_fast_streak_fails": 0
}

current_task = {
    "id": task_id,
    "title": issue_title,
    "status": "selected",
    "issue_path": issue_path,
    "scope": scope,
    "acceptance": acceptance,
    "commands": {"fast": fast_cmds, "full": full_cmds},
    "risk": ["see issue for details"],
    "notes": f"Started in worktree {worktree_dir}"
}

state_dir = os.path.join(worktree_dir, ".agents/state")
os.makedirs(state_dir, exist_ok=True)

with open(os.path.join(state_dir, "project_state.json"), "w") as f:
    json.dump(project_state, f, indent=2)
    f.write("\n")

with open(os.path.join(state_dir, "current_task.json"), "w") as f:
    json.dump(current_task, f, indent=2)
    f.write("\n")

# Set up shared cargo target directory
cargo_dir = os.path.join(worktree_dir, ".cargo")
os.makedirs(cargo_dir, exist_ok=True)
cargo_config = os.path.join(cargo_dir, "config.toml")
parent_target = os.path.join(os.path.dirname(worktree_dir), "target")
# Only write if worktree is NOT the parent repo
if worktree_dir != os.path.dirname(cargo_dir):
    with open(cargo_config, "w") as f:
        f.write("[build]\n")
        f.write(f'target-dir = "{parent_target}"\n')
        f.write('rustflags = ["-C", "link-arg=-fuse-ld=mold"]\n')

print(f"Setup OK: {worktree_dir}")
print(f"  FSM: TASK_SELECT, task: {task_id}")
print(f"  Scope: {scope}")
print(f"  Scope group: {issue_area}")
print(f"  Acceptance: {acceptance}")
print(f"  Shared target: {parent_target}")
