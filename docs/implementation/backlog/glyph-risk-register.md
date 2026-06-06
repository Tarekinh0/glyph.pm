# Glyph V1 — Risk Register

## Échelle

- Probabilité: Low / Medium / High
- Impact: Medium / High / Critical
- Statut: Open / Mitigating / Monitoring / Closed

## Registre

| Risk ID | Risque | Probabilité | Impact | Contrôles obligatoires | Owner gate | Items liés | Statut |
|---|---|---|---|---|---|---|---|
| R-001 | Optimisation locale orchestrator qui contourne la chaîne backlog/gates | Medium | High | YAML canonique obligatoire; DoR déterministe; QA check cohérence dépendances | ORCHESTRATOR + QA | GLYPH-001/002 | Mitigating |
| R-002 | `prod apply` exécuté trop tôt | Medium | Critical | Politique no-prod-apply; approbation humaine explicite; gate Release bloquant | RELEASE + CISO | GLYPH-020/031/032/BETA-008/009 | Mitigating |
| R-003 | Persistance involontaire de `.grist` ou imports | Medium | Critical | RAM-only /dev/shm; no-swap; tests de fuite disque; wipe mémoire | CISO + QA | GLYPH-003/004/005/006/024 | Open |
| R-004 | Logs/telemetry contenant identifiants | Medium | High | `accessLog:false`; métriques agrégées TEET; tests négatifs logs | DPO + CISO | GLYPH-021/017/BETA-006 | Open |
| R-005 | Payload IA trop large (montants/dates/IBAN/session...) | Medium | Critical | Contrat labels+version; sanitization regex; tests négatifs motifs sensibles | DPO + CISO + QA | GLYPH-007/016/BETA-004 | Open |
| R-006 | Modification ADR pour forcer une implémentation | Low | High | Interdiction explicite de modifier ADR pendant implémentation; signalement d'écart | ORCHESTRATOR + CISO + DPO | GLYPH-001 (gouvernance) | Monitoring |
| R-007 | Inputs humains inventés par agent | Medium | High | Registre `glyph-human-inputs.md`; statut WAITING_INPUT strict; preuve de source | ORCHESTRATOR + QA | GLYPH-025/026/028/BETA-009/010 | Open |

## Contrôles transverses

1. Chaque story doit pointer vers ce registre et référencer les risques applicables.
2. Tout risque `Critical` impose revue CISO + DPO avant fermeture.
3. Si un risque touche CI/CD/dépendances/supply-chain, gate Release devient obligatoire.
4. Les preuves de mitigation sont archivées dans `docs/implementation/sprints/GLYPH-XXXX/`.
