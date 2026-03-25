# ConsensusCouncilLLM Integration Contracts v1

## Purpose

This document describes the first embed path for ConsensusCouncilLLM.

The repository does **not** yet ship live integrations. It does ship a stable enough thin slice to define:
- what gets passed into `council run`
- what artifact set comes out
- what `council verify` can gate

## Local Hook Contract

Use this when an external tool has already produced candidate proposals and critiques.

### Inputs

- `task.json`
- `policy.json`
- fixture-scoped `proposals.json`
- fixture-scoped `critiques.json`
- optional `checks.json`
- fixture manifest defining repo baseline and replayed patch locations

### Command

```bash
council run --task fixtures/<fixture>/task.json --policy fixtures/<fixture>/policy.json --out demo/<run>
```

### Outputs

- `evidence_graph.json`
- `adjudication.json`
- `patch_attestation.json`
- `review_summary.md`
- `replay_record.json`
- `final_patch.diff`
- `worktree/`

### Gate

```bash
council verify --run demo/<run>
```

`PASS` means:
- required artifacts exist
- JSON artifacts match schemas
- references resolve
- available digests match
- attested intended files exist inside the generated worktree

## CI Contract

This is the thinnest truthful CI shape for today.

```yaml
steps:
  - run: cargo run -- run --task fixtures/auth_validation_fix/task.json --policy fixtures/auth_validation_fix/policy.json --out demo/ci_run
  - run: cargo run -- verify --run demo/ci_run
  - upload: demo/ci_run/
```

Current status:
- documented
- runnable locally
- not yet published as a GitHub Action or reusable CI package

## MCP-Style Adapter Sketch

An MCP wrapper could later expose:
- `council_run(task_path, policy_path, out_dir)`
- `council_verify(run_dir)`

The wrapper would remain thin:
- file orchestration only
- no hidden adjudication logic outside the CLI
- no live provider routing implied by the adapter itself

Current status:
- interface shape documented
- not implemented in this pass

## Non-Goals

- no hosted control plane
- no PR bot
- no merge automation
- no dashboard workflow
- no claim of production-ready live integrations
