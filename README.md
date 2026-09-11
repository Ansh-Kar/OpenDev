<div align="center">

<h1>OpenDev</h1>

**The Autonomous Multi-Agent Software Engineering Platform powered by OmniRoute.**

<p>Orchestrate specialized AI developer teams (Architect, Backend, Frontend, QA, Security Reviewer) working in parallel Git worktrees with continuous detached test supervision and token-compressed inference.</p>

</div>

## Key Architectural Highlights

| Subsystem | What OpenDev Delivers |
|---|---|
| **Git Worktree Isolation** | Gives every subagent task an isolated, dedicated Git worktree on disk (`~/.cache/openresearch/worktrees/`) with zero branch or lockfile collisions. |
| **Detached OS File-Locked Supervisors** | Long-running test runners (`cargo test`, `npm test`, `pytest`) run in background processes protected by kernel `fd-lock` advisory locks. Survives UI and gateway reloads. |
| **OmniRoute Gateway Integration** | Connects to OmniRoute's 356+ AI providers and 150+ free tiers, with RTK + Caveman token compression (~89% savings) and model capability tag routing (`plan`, `code`, `vision`, `research`, `chat`). |
| **Shared Blackboard & A2A Sync** | Enforces locked interface contracts (`_locked: ["api_spec", "schema"]`) and enables bounded cross-agent clarifying questions (`@ask <task-id>`). |
| **Continuous Judge Loop** | Test exit codes and compiler errors are fed directly to the judge loop, automatically re-queueing flawed tasks with compiler diagnostics. |
| **MCP Safety Plan-Gate** | Stdio MCP bridge intercepts destructive shell commands (`rm -rf`, `git push --force`) and requires human authorization in the UI. |

---

## The Autonomous Engineering Loop

1. **Architect Ingestion**: Architect agent ingests the PRD/feature request, surveys the codebase AST, defines locked API/DB schemas, and emits a DAG task plan.
2. **Parallel Worktree Execution**: Backend and Frontend agents develop in isolated Git worktrees against the locked Blackboard schemas simultaneously.
3. **Detached Test Supervision**: QA agents run integration suites supervised by background detached processes.
4. **Judge & Review**: The judge verifies that all tests pass green, and the Security Reviewer synthesizes a consolidated Pull Request with inline diffs for human 1-click merge.

---

## Getting Started

```bash
# Start OmniRoute AI Gateway
cd omniroute && pnpm dev

# Launch OpenDev Dashboard & Daemon
orx up
```

OpenDev opens the local workspace dashboard at `http://127.0.0.1:4791`.
