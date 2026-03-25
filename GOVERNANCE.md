# Project Governance

## Governance Model

ConsensusCouncilLLM uses a Founding Council model designed for long-term commons stewardship.

### Founding Council (3 seats)

| Seat | Role | Current Holder | Term |
|------|------|---------------|------|
| 1 — Project Lead (BDFL) | Technical direction, release authority, tiebreaker | Panagiotis Drakatos | Founding term (18 months), then elected |
| 2 — Governance Advisor | Policy schema review, threat model validation, standards alignment | Open — seeking EU digital governance researcher | Advisory (18 months) |
| 3 — Commons Steward | Licensing compliance, community health, fork resilience | Open — seeking FSFE/EFF/open-source governance expert | Advisory (18 months) |

### Why 3 seats?

A solo-maintainer project that aims to become commons infrastructure must demonstrate credible stewardship beyond one person. The council model:
- Reduces key-person risk for funders and adopters
- Brings independent review for policy schema evolution
- Ensures licensing and governance decisions have external oversight
- Signals anti-capture commitment

### Council seat offers

Seats 2 and 3 are offered positions, not confirmed appointments. The project will actively seek qualified candidates from European digital governance research and open-source advocacy communities during the funded phase. Candidates must be non-conflicted (no commercial interest in competing tools) and willing to provide quarterly review input.

### After 18 months

After the founding term:
- All three seats transition to merit-based election by active contributors
- The BDFL role becomes an elected maintainer role with the same technical authority
- Election process: nomination by any contributor with 3+ merged contributions, simple majority vote among active contributors (defined as at least 1 contribution in the prior 6 months)

## Decision-Making Processes

### Schema evolution
1. Propose schema change as a GitHub issue with rationale and migration impact
2. 14-day comment period
3. Council reviews; any council member can request revision
4. Merge requires: Project Lead approval + no council objection

### Threat model updates
1. New threat identified → issue filed with severity assessment
2. Council notified for High/Critical severity
3. Mitigation merged by Project Lead; council reviews within 7 days
4. Quarterly threat model review by full council

### Policy file additions
1. New policy class proposed with example fixture
2. Must pass verifier with all existing fixtures
3. Project Lead merges; Governance Advisor reviews alignment with standards

## Fork Resilience

This project is designed to survive maintainer departure:
- All schemas, policies, and fixtures are file-based and self-contained
- The verifier operates on local artifacts with no external dependencies
- No account, API key, or hosted service is required to run the full pipeline
- The MIT license permits unrestricted forking and redistribution
- Documentation is sufficient for an independent developer to build, test, and extend the project

If the founding maintainer becomes unavailable:
1. Any contributor can fork and continue development
2. Council members (if appointed) can coordinate community transition
3. All governance records, schemas, and decision logs are in the public repository

## Code of Conduct

Contributors are expected to engage respectfully and constructively. Governance discussions should focus on technical merit and commons benefit. The Project Lead moderates disputes; the Commons Steward (when appointed) serves as escalation path.

## Contact

Governance questions: open a GitHub issue with the `governance` label.
