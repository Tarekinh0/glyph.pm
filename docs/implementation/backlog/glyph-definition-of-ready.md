# Definition of Ready (DoR) — Glyph V1

## Objectif

Garantir une entrée en implémentation **déterministe**, auditable et alignée ADR.

## Préconditions obligatoires

Un item est `READY` uniquement si:

1. `status` ∉ `{DRAFT, WAITING_INPUT, BLOCKED}`.
2. Toutes les dépendances sont `DONE`.
3. Tous les `human_inputs_required` sont `PROVIDED`.
4. `adr_ref`, `forbidden`, `acceptance_evidence` sont complets.
5. Les gates sont déterminés par la matrice ci-dessous (pas d'exception implicite).

## Matrice de gates déterministe

### Règle de base
- Toujours requis: `ORCHESTRATOR`, `QA`.

### Quand `DPO` est requis
Si `domains` contient au moins un de:
- `privacy`
- `identity`
- `logs`
- `ai_payload`
- `parsing_import`

### Quand `CISO` est requis
Si `domains` contient au moins un de:
- `auth_session`, `crypto`, `identity`, `security`, `memory`, `parsing_import`, `logs`, `network`, `infrastructure`, `ci_cd`, `dependencies`, `secrets`, `supply_chain`, `build_release`, `ai_payload`

### Quand `RELEASE` est requis
Si `domains` contient au moins un de:
- `ci_cd`, `dependencies`, `supply_chain`, `build_release`, `infrastructure`

## Algorithme (pseudo-code)

```text
required = {ORCHESTRATOR, QA}

if domains ∩ {privacy, identity, logs, ai_payload, parsing_import} != ∅:
  required += {DPO}

if domains ∩ {auth_session, crypto, identity, security, memory, parsing_import, logs, network,
              infrastructure, ci_cd, dependencies, secrets, supply_chain,
              build_release, ai_payload} != ∅:
  required += {CISO}

if domains ∩ {ci_cd, dependencies, supply_chain, build_release, infrastructure} != ∅:
  required += {RELEASE}

return required
```

## Contrôles de conformité DoR

- L'item est refusé si ses `gates_required` diffèrent du résultat de l'algorithme.
- L'item est refusé si un `forbidden` zero-data est absent.
- L'item est refusé si le périmètre IA ne mentionne pas `labels + version`.
- L'item est refusé si l'item introduit comptes/recovery/tracking/analytics/IDs persistants/access logs/tokens d'agrégation.
- L'item est refusé si un risque critique du registre n'a pas de contrôle.

## Critères de préparation documentaire

Avant passage `READY`:

1. Story créée dans `docs/implementation/sprints/GLYPH-XXXX/story.md`.
2. Risques mappés (`glyph-risk-register.md`).
3. Screening AIPD initial saisi (`../aipd/index.md`).
4. Plan de tests (privacy, sécurité, régression) défini.
