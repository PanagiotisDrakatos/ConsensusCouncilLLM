# Roadmap

## Funded Phase (6 months — NLnet NGI Zero Commons)

### Milestone 1: Policy + Schema Baseline (Months 1-2)

- Clean up and document the policy schema
- Create an example policy pack for multiple change-risk classes (auth/security, dependency upgrade, refactoring, new feature)
- Document all artifact schemas with field-level descriptions
- Publish schema validation tests

### Milestone 2: Adjudication + Attestation Prototype (Months 3-4)

- Strengthen the adjudication flow with multi-dimensional scoring
- Add patch-path binding for at least two governed change types
- Improve attestation export with clearer provenance fields
- Strengthen verification rules (cross-reference checks, digest validation)
- Add tamper-detection examples for each fixture type

### Milestone 3: Governed Demo Pack + Verifier + Release (Months 5-6)

- Publish a multi-fixture governed demo pack (5+ change classes)
- Create a replay/eval runbook for external reviewers
- Harden verifier with comprehensive PASS/FAIL reason codes
- Release packaging (install docs, quickstart, checksums)
- Final documentation review and cleanup

## Post-Grant Vision (12-month horizon)

### v0.2 — Integration Layer
- Git hook integration (pre-merge governance gate)
- CI/CD integration contracts (GitHub Actions, GitLab CI)
- MCP server wrapper for agentic tool consumption

### v0.3 — Broader Evaluation
- Comparative eval harness: governed vs ungoverned baselines
- Community fixture contributions
- Policy template library for common change patterns

### v1.0 — Commons Release
- Stable CLI with semantic versioning
- Published crate on crates.io
- Comprehensive documentation site
- Integration guides for major forges (GitHub, GitLab, Gitea)

## Non-Goals (Preserved)

These remain explicitly out of scope:
- Hosted service or SaaS
- Web dashboard or IDE plugin
- General-purpose agent orchestration
- Autonomous merge authority
- Provider-specific lock-in

## Guiding Principle

> The next lift comes from verifiability, not from adding more agent magic.
