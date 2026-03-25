# EU Regulatory Impact

How ConsensusCouncilLLM supports compliance with EU digital regulation.

## AI Act (Regulation 2024/1689)

The EU AI Act imposes documentation and traceability obligations on providers of general-purpose AI systems. For organisations using AI coding tools in production, the governance layer addresses specific compliance requirements:

### Article 50 — Transparency obligations

AI-generated outputs used in professional contexts require documentation of what the AI system produced and how it was reviewed. ConsensusCouncilLLM provides:

| Obligation | How addressed |
|-----------|---------------|
| Document AI-generated outputs | Evidence graph records all proposals and critiques |
| Record review process | Adjudication result captures scoring dimensions and winner selection |
| Identify AI involvement | Attestation explicitly names AI agents as proposal authors |
| Enable human oversight | One-final-writer discipline ensures human-reviewable merge decision |

### Article 51 — Technical documentation

Providers must maintain technical documentation enabling assessment of AI system compliance. The governance layer produces:

- Machine-readable policy files (what review depth was required)
- Structured evidence graphs (what was considered)
- Replay records (reproducible decision audit trail)
- Verification reports (automated consistency checks)

### Article 14 — Human oversight

High-risk AI systems require human oversight mechanisms. While most code generation tools are not classified as high-risk, organisations applying precautionary governance benefit from:

- Policy-driven review depth (risk-proportionate oversight)
- Human witness fields in attestation records
- Verifier as independent trust anchor (rejection overrides AI adjudication)

## GDPR (Regulation 2016/679)

### Article 25 — Data protection by design

ConsensusCouncilLLM is local-first by design:

- No personal data is transmitted to external services during governance operations
- All artifacts are stored locally as files
- No account, API key, or cloud service required
- No telemetry or usage tracking

### Article 30 — Records of processing activities

For organisations that process code containing personal data references, the attestation layer provides auditable records of what AI tools processed the code and what governance was applied.

## Digital Services Act (Regulation 2022/2065)

For platforms using AI-generated code in production services:

- Attestation records support transparency reporting requirements
- Governance artifacts demonstrate due diligence in AI-assisted development
- Replay capability enables regulatory audit on request

## Regulatory Enablement Matrix

| Regulation | Article | Requirement | ConsensusCouncilLLM coverage |
|-----------|---------|-------------|------------------------------|
| AI Act | Art. 50 | Transparency for AI outputs | Evidence graph + attestation |
| AI Act | Art. 51 | Technical documentation | Policy + replay + verification |
| AI Act | Art. 14 | Human oversight | One-final-writer + witness fields |
| GDPR | Art. 25 | Privacy by design | Local-first, no cloud dependency |
| GDPR | Art. 30 | Processing records | Attestation as audit trail |
| DSA | Art. 15 | Transparency reporting | Governance artifacts for due diligence |

## Scope and limitations

This project does not claim to make adopters "AI Act compliant" by itself. Compliance is a broader organisational responsibility. The governance layer provides one concrete, machine-readable building block that supports traceability and documentation obligations — particularly for open-source teams without the resources for proprietary compliance tooling.
