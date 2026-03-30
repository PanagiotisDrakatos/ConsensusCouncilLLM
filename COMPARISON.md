# Comparison

Updated: 2026-03-31

ConsensusCouncilLLM is not trying to replace AI coding tools or supply-chain provenance tools.

## Adjacent categories

### AI coding tools

Examples: Copilot, Cursor, Codeium

What they do well:
- generate or edit code
- speed up patch creation

What they do not provide:
- policy by change class
- structured governed review evidence
- one-final-writer discipline
- replayable governed artifact bundles

### Supply-chain provenance tools

Examples: SLSA, in-toto, Sigstore-adjacent provenance workflows

What they do well:
- gate build and release artifacts
- record provenance for produced artifacts

What they do not provide here:
- adjudication over competing code-change proposals
- governed merge-decision evidence for the patch itself

### Multi-agent orchestration frameworks

Examples: LangGraph, CrewAI, AutoGen, debate/selection panels

What they do well:
- coordinate agents
- compare outputs
- route tasks

What they do not provide here:
- policy-bound merge-trust artifacts
- stable replay records for governed code-change decisions

## ConsensusCouncilLLM's actual lane

ConsensusCouncilLLM sits after generation and before merge.

Its current prototype focuses on:
- policy admission
- structured review evidence
- deterministic adjudication
- attestation export
- replayable verification artifacts

## Related repo notes

For the longer reviewer-facing comparison, see:

- `POSITIONING_MATRIX_V1.md`
- `COMPARATIVE_EVAL_NOTE.md`
