# Governed Change Review Summary

## Task
- Task: `auth-validation-001`
- Title: Harden OAuth callback redirect validation
- Policy: `auth_security_v1`
- Fixture: `auth_validation_fix`
- Outcome: replayed patch candidate applied to fixed local worktree

## Decision
- Winning proposal: `proposal-b`
- Final writer: `agent:security`
- Ranked survivors: proposal-b, proposal-a

## Rejections
- `proposal-c` rejected: SCOPE_TOO_BROAD

## Patch path
- Applied worktree: `worktree`
- Patch diff: `final_patch.diff`

## Patch metadata
- Intended files: src/auth/callback.ts, tests/auth/callback.test.ts
- Human witness: pending
- Artifact set: `evidence_graph.json`, `adjudication.json`, `patch_attestation.json`, `review_summary.md`, `replay_record.json`, `final_patch.diff`, `worktree/`
