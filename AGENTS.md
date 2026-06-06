# Glyph agent rules

Glyph is a zero-data / zero-knowledge personal finance infrastructure.

Non-negotiable rules:
- Do not weaken Documentation/Architecture-Decision-Records.md or Documentation/Zero_Data_Architecture.md.
- Do not introduce user accounts, email recovery, persistent user identifiers, analytics, tracking, access logs, or bank aggregation tokens.
- User financial data must not be persisted server-side.
- Decrypted `.grist` files and imported financial files must only exist in memory-backed storage.
- No feature is complete without privacy, security, and regression tests.
- Any change affecting cryptography, session handling, logging, import parsing, AI payloads, CI/CD, or infrastructure must go through DPO and CISO review.

ADR anchors:
- ADR-001: stateless zero-knowledge architecture.
- ADR-003/004: RAM-only and no swap.
- ADR-007/015/016: AI categorization must send labels only.
- ADR-008/009: AES-256-GCM and Argon2id constants.
- ADR-010: no password recovery.
- ADR-019/021: Traefik ForwardAuth and no user logs.
- ADR-020/032: CI/CD, SAST/DAST, SBOM, Sigstore, SLSA.

Never commit secrets, production credentials, private keys, user data, real bank statements, or real transaction samples.

## Multi-Agent Governance

Glyph uses a strict multi-agent governance model to ensure security, privacy, and quality.

### Agents
- **glyph-orchestrator**: Primary agent. Manages the sprint lifecycle, creates stories, and coordinates other agents.
- **glyph-dpo**: Reviewer. Ensures GDPR compliance, privacy by design, and zero-data principles. Cannot modify code.
- **glyph-ciso**: Reviewer. Ensures security, threat modeling, and compliance with ADRs. Cannot modify code.
- **glyph-devsecops**: Implementer. Writes code, tests, and CI/CD workflows. Cannot modify ADRs.
- **glyph-qa**: Reviewer. Verifies tests, edge cases, and quality. Cannot modify code.
- **glyph-release**: Reviewer. Verifies CI/CD, SBOM, and supply chain security. Cannot modify code.

### Workflow
1. **Sprint Initialization**: `glyph-orchestrator` creates a story in `docs/implementation/sprints/GLYPH-XXXX/story.md`.
2. **Design Review**: `glyph-dpo` and `glyph-ciso` review the story. If blocked, the sprint stops.
3. **Implementation**: `glyph-devsecops` implements the story.
4. **Code Review**: `glyph-ciso`, `glyph-dpo`, `glyph-qa`, and `glyph-release` review the implementation.
5. **Closure**: `glyph-orchestrator` produces `closure.md` with the final verdict.

### Commands
- `/glyph-sprint`: Starts a full sprint cycle.
- `/glyph-gate`: Final gate before merging.
