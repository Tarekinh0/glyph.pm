# Glyph V1 Roadmap (macro)

## Principe directeur

Roadmap **bottom-up**: on verrouille d'abord les garanties structurelles (zero-data, RAM-only, no-swap, no-logs, supply-chain), puis on enchaîne vers la phase beta.

## Non-negotiables

1. Aucune persistance serveur de données financières utilisateur (ADR-001).
2. Données déchiffrées et imports uniquement en RAM (ADR-003) avec swap désactivé (ADR-004).
3. Aucune création de logs utilisateur / identifiants persistants (ADR-021).
4. Routing session via ForwardAuth et contrôles de session stricts (ADR-019).
5. IA limitée à `labels sanitizés + version`, sans champs sensibles (ADR-007/016).
6. IaC + CI/CD + provenance vérifiable (ADR-031/032).
7. Aucun `prod apply` sans approbation humaine explicite.

## Ordre stratégique

### Phase 0 — Gouvernance
- GLYPH-000 (DONE)
- GLYPH-001 (DONE)

### Phase 1 — Invariants sécurité/vie privée
- GLYPH-002 → GLYPH-012

### Phase 2 — Infrastructure de confiance
- GLYPH-013 → GLYPH-021

### Phase 3 — CI/CD, secrets, dépendances, provenance
- GLYPH-022 → GLYPH-032

### Phase 4 — Programme Beta
- GLYPH-BETA-001 → GLYPH-BETA-010

## Macro dependency chain

`GLYPH-000`
→ `GLYPH-001`
→ (`GLYPH-002`, `GLYPH-003`, `GLYPH-008`, `GLYPH-009`, `GLYPH-010`, `GLYPH-018`)
→ (`GLYPH-004`, `GLYPH-005`, `GLYPH-011`, `GLYPH-012`, `GLYPH-013`, `GLYPH-014`)
→ (`GLYPH-006`, `GLYPH-007`, `GLYPH-015`, `GLYPH-016`, `GLYPH-019`, `GLYPH-021`)
→ (`GLYPH-017`, `GLYPH-020`, `GLYPH-023`, `GLYPH-031`)
→ (`GLYPH-022`, `GLYPH-024`, `GLYPH-029`, `GLYPH-030`, `GLYPH-032`)
→ (`GLYPH-025`, `GLYPH-026`, `GLYPH-028`) *si inputs humains fournis*
→ `GLYPH-BETA-001`
→ `GLYPH-BETA-002`
→ `GLYPH-BETA-003`
→ (`GLYPH-BETA-004`, `GLYPH-BETA-005`, `GLYPH-BETA-006`)
→ `GLYPH-BETA-007`
→ `GLYPH-BETA-008`
→ `GLYPH-BETA-009` *(décision humaine)*
→ `GLYPH-BETA-010`.

## Items WAITING_INPUT (à date)

- GLYPH-006 (version pin Grist).
- GLYPH-015 (référence modèle IA).
- GLYPH-016 (jeu de regex sanitization IA).
- GLYPH-020 (approbateur humain prod apply).
- GLYPH-025, GLYPH-026, GLYPH-028 (scope hardware trust / attestation).
- GLYPH-BETA-001, GLYPH-BETA-009, GLYPH-BETA-010 (fenêtre beta + go/no-go).

## Règle d'exécution

Un item ne passe en `IN_PROGRESS` que si:

- toutes ses dépendances sont `DONE`,
- les inputs humains requis sont `PROVIDED`,
- ses gates déterministes sont pré-identifiés (DoR),
- ses preuves attendues sont explicitement listées (DoD).
