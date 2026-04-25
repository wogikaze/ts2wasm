# Start autonomous compiler development loop

Use this prompt to invoke the compiler-autonomy skill and begin the FSM-driven development cycle.

## Prompt

```
Start autonomous compiler development loop. Invoke the compiler-autonomy skill, read workflows/compiler_dev_fsm.md and state/current_task.json, then follow the FSM states (SYNC → TRIAGE → TASK_SELECT → PLAN → IMPLEMENT → VERIFY → RETRO).
```

## When to use

- When you want the agent to work through issues autonomously following the FSM contract
- When starting a new development cycle on the compiler
- When resuming work after a context switch

## What happens

1. Agent reads the FSM contract from `workflows/compiler_dev_fsm.md`
2. Agent checks current state from `state/current_task.json` and `state/project_state.json`
3. Agent follows the state machine: SYNC → TRIAGE → TASK_SELECT → PLAN → IMPLEMENT → VERIFY → RETRO
4. Agent updates state files and writes cycle reports to `reports/runs/`
