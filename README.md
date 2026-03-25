# ConsensusCouncilLLM

**Policy-governed admission, evidence, and attestation layer for AI-generated code changes**

> **Branch protection for AI-generated change**

## The Problem

84% of developers now use AI coding assistants (Knostic 2025), yet approximately half do not trust AI-generated code for production use (Checkmarx 2025). The governance gap — the absence of structured, attestable review for AI-generated changes — is the bottleneck.

AI coding tools produce useful diffs. The trust layer around those diffs is still weak. Teams lack a reusable way to answer: what review depth applies? What evidence was considered? Who became the final writer? What checks existed? What durable record can be replayed?

## What This Is

ConsensusCouncilLLM is a local-first, open-source governance layer that sits between AI code generation and merge. It provides:

- **Policy by change class** — different review depths for auth changes vs dependency upgrades
- **Structured evidence** — machine-readable records of proposals, critiques, and rationale
- **Deterministic adjudication** — one governed outcome, one final writer
- **Attestation export** — SLSA/in-toto-inspired attestation artifacts
- **Replay and verification** — reproducible decision records with verifier-backed consistency checks

## What This Is NOT

- Not a multi-agent framework or orchestration platform
- Not a hosted service or web dashboard
- Not a "models debate and vote" toy
- Not a replacement for coding agents — it governs the change they produce

Existing tools like Copilot and Cursor generate code. SLSA and in-toto gate artifacts. Neither side governs the change itself. ConsensusCouncilLLM sits in that gap.

## Current Status

This repository contains a runnable early prototype — not a production system:

- **Rust CLI** (`council run`, `council verify`)
- **JSON schemas** for policy, evidence graph, adjudication, attestation, and replay
- **3 replayable governed-change fixtures** (auth validation, dependency upgrade, security regression prevented)
- **Verifier** that checks artifact presence, schema validity, references, and digests
- **Local evidence bundle** with tamper-proof example

### Governed Flow

```
Task + Policy -> Proposals -> Critiques -> Adjudication -> Final Writer -> Attestation -> Verify
```

1. Load task and policy files
2. Replay structured proposals from fixtures
3. Apply deterministic adjudication (policy compliance, scope, test impact)
4. Designate one final writer
5. Emit: evidence_graph.json, adjudication.json, patch_attestation.json, replay_record.json, final_patch.diff
6. Run `council verify` to validate the artifact set

## Quick Start

```bash
cargo build --release
./target/release/council run --task fixtures/auth_validation_fix/task.json --policy policies/auth_security.json
./target/release/council verify demo/sample_run/
```

See [QUICKSTART.md](QUICKSTART.md) and [DEMO_RUNBOOK.md](DEMO_RUNBOOK.md) for details.

## Architecture

```
+--------------+     +---------------+     +----------------+
| Policy Layer |---->| Evidence      |---->| Adjudication   |
| (risk class) |     | (proposals,   |     | (deterministic |
|              |     |  critiques)   |     |  ranking)      |
+--------------+     +---------------+     +--------+-------+
                                                    |
                     +---------------+     +--------v-------+
                     | Replay Layer  |<----| Attestation    |
                     | (reproduce)   |     | (SLSA-inspired)|
                     +---------------+     +----------------+
                                                    |
                                           +--------v-------+
                                           | Verification   |
                                           | (PASS/FAIL)    |
                                           +----------------+
```

See [ARCHITECTURE.md](ARCHITECTURE.md) for the full layer model.

## Research Context

The design choice to use structured scoring rather than majority vote is supported by recent work showing that evidence-based adjudication outperforms frequency-based voting, especially when a minority proposal is correct but outvoted (cf. AgentAuditor, arXiv:2602.09341). Independent-proposal voting is more robust for tasks where diversity of approaches matters more than convergence (cf. ACL 2025 Findings, arXiv:2502.19130).

## Repo Map

- `src/` — Rust CLI prototype
- `schemas/` — JSON Schema definitions
- `policies/` — sample policy files
- `fixtures/` — replayable governed-change fixtures
- `demo/` — generated sample outputs
- `ARCHITECTURE.md` — layer model and design
- `QUICKSTART.md` — getting started
- `DEMO_RUNBOOK.md` — running the demos

## Roadmap

The next funded phase aims to:
1. Expand policy coverage beyond the current three fixtures
2. Add a comparative replay/eval pack
3. Generalize the patch path beyond replay-driven fixtures
4. Strengthen verifier coverage and release packaging
5. Publish as reusable commons-grade infrastructure

## Author

**Panagiotis Drakatos**

- 7 peer-reviewed publications at IEEE and Springer venues (including IEEE ICDE)
- Creator of [Adrestus](https://github.com/Adrestus-net/Adrestus) — L1 blockchain, ~300K LOC, solo
- 5-year Engineering Diploma + MSc Cybersecurity + 4 years PhD research
- [DBLP](https://dblp.org/pid/184/9146.html) | [ORCID](https://orcid.org/0000-0002-8738-1947) | [GitHub](https://github.com/PanagiotisDrakatos)

## License

EUPL-1.2 — see [LICENSE](LICENSE)

## Status

Early prototype — Governance layer exists in runnable form. Not production-ready. Contributions and feedback welcome.
