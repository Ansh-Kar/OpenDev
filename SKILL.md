---
name: opendev-cli
description: Use the `orx` CLI to orchestrate autonomous multi-agent dev teams, allocate isolated Git worktrees, supervise detached test suites, and manage DAG task execution.
---

# OpenDev CLI (`orx`)

`orx up` runs the local multi-agent software engineering workspace backed by OmniRoute.

## Dev Team Skills & Modules

Load focused persona modules with `orx skill <name>`:

- **`orx-architect`**: PRD ingestion, interface design, `_locked` Blackboard schemas, DAG wave decomposition.
- **`orx-backend`**: Backend APIs, database transactions, service development in isolated Git worktrees.
- **`orx-frontend`**: Accessible UI components, state management, and client integration in isolated worktrees.
- **`orx-qa`**: Unit, integration, and E2E test suites with automated detached supervisor verification.
- **`orx-reviewer`**: Security auditing (OWASP top 10), performance analysis, and PR changelog generation.
- **`orx-git`**: Git worktree lifecycle, 3-way diffing, branch promotion, and commit tracking.

## Core Commands

```sh
orx up                   # Launch the OpenDev workspace dashboard (127.0.0.1:4791)
orx projects             # List registered software repositories
orx runs <projectId>     # List verification runs and test execution logs
orx logs <runId>         # Stream live test logs from detached supervisor
```
