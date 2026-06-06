# Backlog canonique V1 — Glyph

Ce dossier contient la **source canonique** de planification V1 pour Glyph.

## Source de vérité

- Fichier canonique: `glyph-v1-backlog.yaml`
- Les autres fichiers Markdown sont des vues explicatives et des garde-fous opérationnels.
- En cas d'écart, le YAML fait foi.

## Objectif

Piloter les sprints par:

1. dépendances explicites,
2. gates de revue déterministes,
3. preuves d'acceptation,
4. invariants zero-data non négociables.

## Invariants non négociables (rappel)

- Interdits: comptes utilisateur, recovery email, tracking, analytics, identifiants persistants, access logs, bank aggregation tokens.
- Aucune persistance serveur des données financières utilisateur.
- Déchiffrement `.grist` et imports financiers: RAM only + no swap.
- IA: **payload = labels sanitizés + version** uniquement.
- Aucune erreur ne doit exposer des données financières brutes.
- Aucun `prod apply` sans approbation humaine explicite.

## Fichiers

- `glyph-v1-backlog.yaml`: backlog canonique (items, dépendances, gates, preuves).
- `glyph-v1-roadmap.md`: roadmap macro bottom-up.
- `glyph-human-inputs.md`: registre des entrées humaines manquantes/fournies.
- `glyph-risk-register.md`: registre des risques et contrôles.
- `glyph-definition-of-ready.md`: règles d'entrée en implémentation.
- `glyph-definition-of-done.md`: règles de clôture avec preuves.
- `glyph-sprint-template.md`: template sprint prêt à exécuter.
- `../aipd/index.md`: index de screening AIPD (`required? yes/no + justification`).

## Validation minimale attendue

- YAML valide.
- 100% des items avec `adr_ref`, `dependencies`, `gates_required`, `human_inputs_required`, `forbidden`, `acceptance_evidence`.
- Couverture ADR prioritaire vérifiable: `ADR-001/003/004/007/013/019/021/031/032`.
- Cohérence des références croisées (IDs, risques, inputs humains).
