# Definition of Done (DoD) — Glyph V1

## Objectif

Un item est `DONE` uniquement avec preuves vérifiables de conformité privacy/sécurité/qualité.

## Preuves minimales obligatoires

1. `story.md` finalisée (périmètre, ADR, tests attendus).
2. Revues archivées:
   - DPO: PASS (si gate requis)
   - CISO: PASS (si gate requis)
   - QA: PASS
   - Release: PASS (si gate requis)
3. `closure.md` avec verdict explicite.
4. Résultats de tests/validations attachés.
5. Pour toute surface critique (parsing/import, crypto/KDF, sanitization IA, logs/erreurs): preuves explicites de tests edge-cases + fuzz/property-based.

## Domaines de contrôle sécurité obligatoires

Chaque item applicable documente les résultats pour:

- Auth/session (tokens, timeout, rotation, ForwardAuth)
- Crypto (AES-256-GCM, Argon2id, constantes)
- Parsing/import (sanitization, rejets, robustesse)
- Mémoire (RAM-only, no-swap, zeroing, anti-dump)
- Logs (aucun log utilisateur, métriques agrégées uniquement)
- Réseau (surface exposée, segmentation, failover)
- CI/CD (jobs bloquants, traçabilité)
- Dépendances (inventaire, CVE, policies)
- Secrets (scan et redaction)
- Supply-chain (SBOM, signature, provenance)

## Contrôles négatifs zero-data (obligatoires)

Les tests doivent échouer si l'un des éléments suivants apparaît:

- comptes utilisateur
- email/password recovery
- tracking/analytics
- identifiants persistants
- access logs utilisateur
- bank aggregation tokens

## Contrat IA (obligatoire)

Preuve explicite que:

1. Payload autorisé = `labels` + `version` uniquement.
2. Les motifs/champs sensibles sont interdits et neutralisés avant envoi.
3. Les erreurs IA n'exposent jamais de données financières brutes.

## Règles de qualité additionnelles

- Aucune donnée utilisateur réelle en tests/docs/fixtures.
- Les surfaces critiques (parsing/import, crypto/KDF, IA sanitization) exigent des tests de cas limites (malformed, oversize, erreurs redacted) et des tests fuzz/property-based avec invariants documentés.
- Aucune divergence avec ADR contournée; toute divergence est signalée.
- Aucun secret ou credential dans le code/documentation.
- Aucun `prod apply` sans approbation humaine explicite traçable.
