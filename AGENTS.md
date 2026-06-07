# Glyph agent rules

Glyph is a zero-data / zero-knowledge personal finance infrastructure.

Non-negotiable rules:
- Do not weaken the Architecture Decision Records in the `docs/decisions/` folder, especially ADR-001, ADR-003, ADR-004, ADR-007, ADR-019, ADR-021, ADR-031, and ADR-032.
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

## Glyph Backlog Governance

Before taking any action, all agents MUST read the canonical backlog and roadmap to understand the current context and priorities. The source of truth is located at:
- `docs/implementation/backlog/glyph-v1-backlog.yaml`
- `docs/implementation/backlog/glyph-v1-roadmap.md`

## Multi-Agent Governance

Glyph uses a strict multi-agent governance model to ensure security, privacy, and quality.

### Agents
- **glyph-orchestrator**: Primary agent and arbiter. Manages the sprint lifecycle, creates stories, coordinates other agents, and resolves conflicts or rejections.
- **glyph-dpo**: Reviewer. Ensures GDPR compliance, privacy by design, and zero-data principles. Cannot modify code.
- **glyph-ciso**: Reviewer. Ensures security, threat modeling, and compliance with ADRs. Cannot modify code.
- **glyph-devsecops**: Implementer. Writes code, tests, and CI/CD workflows. Cannot modify ADRs.
- **glyph-qa**: Reviewer. Verifies tests, edge cases, and quality. Cannot modify code.
- **glyph-release**: Reviewer. Verifies CI/CD, SBOM, and supply chain security. Cannot modify code.

### Strict Sequential Workflow

The workflow is strictly sequential and file-based within the sprint folder (`docs/implementation/sprints/GLYPH-XXXX/`):

1. **Sprint Initialization**: `glyph-orchestrator` creates the sprint folder and writes `story.md`.
2. **Design**:
   - `glyph-dpo` writes `dpo-requirements.md`.
   - `glyph-ciso` writes `ciso-requirements.md`.
   - *If blocked, the sprint stops and `glyph-orchestrator` arbitrates.*
3. **Implementation**: `glyph-devsecops` implements the story (code, tests) and writes `dev-notes.md` (factual, technical).
4. **Review**:
   - `glyph-ciso` verifies the implementation and writes `ciso-review.md`.
   - `glyph-dpo` verifies the implementation and writes `dpo-review.md`.
5. **Validation**:
   - `glyph-qa` verifies tests and edge cases, then writes `qa-review.md`.
   - `glyph-release` verifies CI/CD and supply chain, then writes `release-review.md`.
6. **Closure**: `glyph-orchestrator` reviews all artifacts, resolves any remaining conflicts, and produces `closure.md` with the final verdict.

### Commands
- `/glyph-sprint`: Starts a full sprint cycle.
- `/glyph-gate`: Final gate before merging.
