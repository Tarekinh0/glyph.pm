# Index AIPD / DPIA screening — Backlog V1

## Méthode

- Champ `required?`: `yes` ou `no`.
- `yes` si l'item touche le traitement de données personnelles financières, la logique d'identification/session, les logs, l'IA, ou les transferts/risques de réidentification.
- `no` si l'item est strictement gouvernance/processus sans traitement de données utilisateur.
- Toute entrée `yes` doit déboucher sur une fiche AIPD détaillée lors du sprint concerné.

## Screening backlog

| Item | required? | Justification |
|---|---|---|
| GLYPH-000 | no | Gouvernance agents/outillage, pas de traitement utilisateur. |
| GLYPH-001 | no | Story documentaire de backlog canonique. |
| GLYPH-002 | yes | Contrôles zero-data et détection de patterns d'identification. |
| GLYPH-003 | yes | Données déchiffrées en RAM, risque de persistance. |
| GLYPH-004 | yes | Contrôle swap/dumps lié à confidentialité des données en mémoire. |
| GLYPH-005 | yes | Zeroing/mlock sur données sensibles en session. |
| GLYPH-006 | yes | Cycle de session Grist avec fichiers utilisateur éphémères. |
| GLYPH-007 | yes | Envoi IA de libellés (même minimisé). |
| GLYPH-008 | yes | Chiffrement des données utilisateur (.grist). |
| GLYPH-009 | yes | Dérivation de clé utilisateur (mot de passe). |
| GLYPH-010 | yes | Gestion d'accès utilisateur sans récupération. |
| GLYPH-011 | yes | Parsing d'imports financiers et gestion erreurs. |
| GLYPH-012 | yes | Session/token et métadonnées de connexion. |
| GLYPH-013 | yes | Isolation runtime de services traitant données sensibles. |
| GLYPH-014 | yes | Exposition réseau et trafic lié sessions utilisateur. |
| GLYPH-015 | yes | Stratégie IA impactant volume de données envoyées. |
| GLYPH-016 | yes | Sanitization de libellés et prévention fuite motifs sensibles. |
| GLYPH-017 | yes | Télémétrie publique, risque de réidentification indirecte. |
| GLYPH-018 | no | Gouvernance repo/miroir CI sans données utilisateur. |
| GLYPH-019 | yes | ForwardAuth/session, cookies et contrôle accès. |
| GLYPH-020 | no | Pipeline CI sécurité (processus), sans données utilisateur réelles. |
| GLYPH-021 | yes | Politique d'absence de logs utilisateur. |
| GLYPH-022 | no | Secrets management process, pas de données utilisateur. |
| GLYPH-023 | no | Gouvernance dépendances/SLA CVE. |
| GLYPH-024 | yes | Capacité mémoire appliquée à sessions et données déchiffrées. |
| GLYPH-025 | yes | Définition TEE pour opérations sur données sensibles. |
| GLYPH-026 | yes | Attestation continue sur environnement de traitement. |
| GLYPH-027 | no | Runbooks incidents/documentation de gouvernance. |
| GLYPH-028 | no | Gouvernance PCR/golden values (infrastructure). |
| GLYPH-029 | yes | Politique backup: démontrer exclusion des données utilisateur. |
| GLYPH-030 | yes | Heartbeat/failover potentiellement corrélables aux sessions. |
| GLYPH-031 | no | IaC framework/process sans données utilisateur. |
| GLYPH-032 | no | Provenance/artefacts build supply-chain. |
| GLYPH-BETA-001 | yes | Dry-run beta sur flux opérationnels liés aux sessions. |
| GLYPH-BETA-002 | yes | Threat model privacy/sécurité orienté données personnelles. |
| GLYPH-BETA-003 | no | Pack documentaire de preuves publiques. |
| GLYPH-BETA-004 | yes | Parsing IA/import même sur jeux synthétiques. |
| GLYPH-BETA-005 | yes | Tests charge en présence de données en mémoire. |
| GLYPH-BETA-006 | yes | Validation TEET et risque de métadonnées. |
| GLYPH-BETA-007 | yes | Régression end-to-end sur parcours données sensibles. |
| GLYPH-BETA-008 | no | Hardening release/artifacts. |
| GLYPH-BETA-009 | no | Décision de gouvernance go/no-go. |
| GLYPH-BETA-010 | yes | Validation finale readiness incluant invariants privacy/runtime. |

## Statut global

- Screening initial: **complété** pour l'ensemble des items du backlog V1.
- Items `required? yes`: AIPD détaillée à produire dans chaque sprint concerné.
