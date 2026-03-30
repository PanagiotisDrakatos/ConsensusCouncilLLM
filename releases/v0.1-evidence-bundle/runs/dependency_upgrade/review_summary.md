# Governed Change Review Summary

## Task
- Task: `dependency-upgrade-001`
- Title: Apply a narrow axios patch upgrade with lockfile sync
- Policy: `dependency_upgrade_v1`
- Fixture: `dependency_upgrade`
- Outcome: replayed patch candidate applied to fixed local worktree

## Decision
- Winning proposal: `proposal-b`
- Final writer: `agent:release`
- Ranked survivors: proposal-b, proposal-a

## Rejections
- `proposal-c` rejected: SCOPE_TOO_BROAD

## Patch path
- Applied worktree: `worktree`
- Patch diff: `final_patch.diff`

## Patch metadata
- Intended files: package.json, package-lock.json, tests/api-client.test.ts
- Human witness: pending
- Artifact set: `evidence_graph.json`, `adjudication.json`, `patch_attestation.json`, `review_summary.md`, `replay_record.json`, `final_patch.diff`, `worktree/`
