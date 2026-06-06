# Sprint template — Backlog canonique V1

## 1) Métadonnées

- Sprint: `GLYPH-XXXX`
- Item backlog: `GLYPH-XXX` ou `GLYPH-BETA-XXX`
- Statut initial item: `READY`
- Dépendances satisfaites: `yes/no`
- Inputs humains requis: `list + statut`

## 2) Périmètre validé

- Story: `docs/implementation/sprints/GLYPH-XXXX/story.md`
- Hors périmètre explicite:
  - Pas de modification ADR
  - Pas de prod apply
  - Pas de fonctionnalités interdites zero-data

## 3) Gates déterministes (DoR)

- Domains déclarés: `[...]`
- Gates attendus (algorithme DoR): `[...]`
- Gates effectivement assignés: `[...]`
- Écart: `none / détail`

## 4) Risques

- Risk IDs: `R-...`
- Contrôles associés:
  - contrôle 1
  - contrôle 2

## 5) Plan de tests

- Privacy tests:
- Security tests:
- Regression tests:
- Tests négatifs zero-data:
- Tests IA labels-only/sanitization:
- Tests conditionnels supply-chain (si applicable):

## 6) Preuves attendues (DoD)

- Revues DPO/CISO/QA/Release (selon gates)
- Rapports de tests
- Artefacts SBOM/signature/provenance (si applicable)
- `closure.md` final

## 7) Exécution

- DevSecOps implémentation: `done/in progress`
- QA review: `PASS/FAIL/N/A`
- DPO review: `PASS/BLOCKED/N/A`
- CISO review: `PASS/BLOCKED/N/A`
- Release review: `PASS/FAIL/N/A`

## 8) Closure

- Verdict final: `DONE/BLOCKED/REWORK`
- Liens preuves:
  - `story.md`
  - `dpo-review.md`
  - `ciso-review.md`
  - `qa-review.md`
  - `release-review.md`
  - `closure.md`
