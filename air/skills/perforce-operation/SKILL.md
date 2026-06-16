---
name: perforce-operation
description: This project uses version control provided by Perforce, and controlled files are read-only by default. Perforce workflows should be used when performing file operations. The agent should follow secure Perforce operations, detect workspace limitations, and avoid editing failures due to locked or unopened files.
---

# Perforce Operation

## Core Principles

1. If a modification fails due to read-only locking, the p4 read-only lock should be considered as the cause.
2. No operations that may affect the server are allowed, including but not limited to: submit. These operations should
   be performed by the user, the agent can only provide advices.

## Default Workflow

Use this sequence when making code changes in a Perforce workspace:

1. Verify workspace context (`p4 info`, `p4 client`).
2. Inspect file state (`p4 opened`, `p4 fstat`, `p4 have` as needed).
3. Open target files for edit (`p4 edit <file>`).
4. Apply and validate code changes.
5. Review changes (`p4 diff`, `p4 opened -c <cl>`).
6. Resolve conflicts if present (`p4 resolve` + verify).

## Changelist Hygiene

- When creating a cl, a change description or target description should be included.
- Do not revert user-owned pending work without explicit confirmation.
- For risky refactors, prefer shelving early for review.

## Agent Behavior Rules

- When Perforce is detected, prefer Perforce commands over Git-centric assumptions.
- Explicitly mention checkout/open-for-edit steps in your plan before file modification.
- If constraints block progress (no workspace, auth failure, lock/conflict), stop and ask for direction.
- Preserve user trust: explain Perforce state and proposed actions before destructive steps.

