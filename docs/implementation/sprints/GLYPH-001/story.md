# GLYPH-001: Backlog canonique V1 cohérent et exécutable

## Objectif
Construire le **backlog canonique versionné** de Glyph V1 pour piloter les sprints par dépendances/gates/preuves, et non par prompts isolés.

## Story verticale minimale retenue
Mettre en place un socle documentaire unique (`docs/implementation/backlog/`) qui rend l’orchestration macro fiable : backlog YAML, roadmap, inputs humains, risk register, Definition of Ready, Definition of Done, template sprint et index AIPD.

## Périmètre
- [ ] Créer le dossier `docs/implementation/backlog/` et les documents de gouvernance backlog V1.
- [ ] Créer `glyph-v1-backlog.yaml` complet, cohérent et dépendance-driven.
- [ ] Créer `glyph-v1-roadmap.md` avec chaîne macro et non-négociables.
- [ ] Créer `glyph-human-inputs.md` avec statuts `TODO/PROVIDED/NOT_NEEDED/DECISION_REQUIRED`.
- [ ] Créer `glyph-risk-register.md` avec risques transverses et contrôles.
- [ ] Créer `glyph-definition-of-ready.md` (DoR) et `glyph-definition-of-done.md` (DoD).
- [ ] Créer un template sprint backlog (`glyph-sprint-template.md`) et un README backlog.
- [ ] Créer `docs/implementation/aipd/index.md` avec screening RGPD/AIPD par item.
- [ ] Intégrer explicitement les exigences bloquantes DPO/CISO de gouvernance documentaire.

## Hors périmètre
- Implémentation applicative, infra, CI/CD ou déploiement runtime.
- Modification des ADR.
- Mise en production (`prod apply`).
- Introduction de comptes, recovery email, tracking, analytics, logs utilisateurs, identifiants persistants.

## ADR impactés
- **Tous ADR** (référencement backlog), avec priorité:
  - **ADR-001** zero-knowledge stateless
  - **ADR-003** RAM-only
  - **ADR-004** no-swap
  - **ADR-007/015/016** IA labels-only
  - **ADR-013** isolation LXC
  - **ADR-019** ForwardAuth/session
  - **ADR-021** no user logs
  - **ADR-031** IaC Ansible/Terraform
  - **ADR-032** provenance/SBOM/Sigstore/SLSA

## Exigences bloquantes intégrées (Design)
- [ ] DoD explicite: interdiction logs utilisateurs, identifiants persistants, tracking, analytics, cookies non nécessaires.
- [ ] Contrat IA explicite: **autorisé = labels + version** uniquement.
- [ ] Contrat IA explicite: **interdit = montant, date, solde, IBAN, numéro de compte, titulaire, banque, session_id**.
- [ ] Contrôle IA explicite: **sanitization obligatoire des libellés** avant envoi IA (suppression/masquage des motifs sensibles dans le texte lui-même, conformément ADR-016).
- [ ] Hygiène documentaire: données réelles/secrets interdits dans exemples/tests/fixtures/docs.
- [ ] Screening AIPD: statut `oui/non + justification` obligatoire par item.

## Critères d'acceptation
- [ ] Chaque item du backlog a au moins un `adr_ref`.
- [ ] Couverture ADR prioritaire **100%** vérifiable: `{ADR-001, ADR-003, ADR-004, ADR-007, ADR-013, ADR-019, ADR-021, ADR-031, ADR-032}` est couverte par au moins un item, avec dépendances explicites.
- [ ] Chaque item du backlog a `dependencies`, `gates_required`, `forbidden`, `acceptance_evidence`.
- [ ] Le backlog n’est pas purement fonctionnel: risques et contrôles sont explicites.
- [ ] La DoR inclut une matrice de gates **déterministe et testable** (quand DPO/CISO/QA/Release est obligatoire selon la surface de risque).
- [ ] La DoR bloque les items `DRAFT/WAITING_INPUT/BLOCKED`.
- [ ] La DoD impose les preuves de revues DPO/CISO/QA (+ Release si applicable).
- [ ] La DoD inclut explicitement les domaines de contrôle sécurité: auth/session, crypto, parsing/import, mémoire RAM-only/no-swap, logs, réseau, CI/CD, dépendances, secrets, supply-chain.
- [ ] Le registre des risques est actionnable (risk, controls, status).
- [ ] L’index AIPD contient une structure de qualification exploitable.

## Tests attendus
- [ ] Test structure YAML: items complets avec champs obligatoires.
- [ ] Test couverture ADR: 100% des items avec ADR refs.
- [ ] Test couverture ADR prioritaire: présence des 9 ADR prioritaires dans le backlog.
- [ ] Test gates: présence DPO/CISO/QA conforme à la surface de risque des items.
- [ ] Test matrice de gates déterministe: mêmes conditions => mêmes gates requises.
- [ ] Test de non-régression zero-data (documentaire): un item « compte utilisateur » ou « access logs » doit être détecté comme non conforme.
- [ ] Tests négatifs zero-data supplémentaires: détection de `email recovery`, `tracking`, `analytics`, `persistent identifiers`, `bank aggregation tokens`.
- [ ] Tests cas limites parsing/import: fichiers malformés (colonnes invalides, encodage invalide, tronqués) et fichiers oversize avec rejet explicite.
- [ ] Test erreurs redacted (obligatoire): aucune erreur ne doit exposer montants, dates, IBAN, numéros de compte ou payload brut.
- [ ] Tests négatifs IA contenu: libellés contenant montants/dates/IBAN-like/bank names/session-like tokens doivent être sanitizés avant payload final.
- [ ] Test preuve ADR-016: payload IA final contient uniquement `labels sanitizés + version`.
- [ ] Tests fuzz/property-based parsing: entrées aléatoires/adversariales pour garantir `no panic`, rejet contrôlé et absence de données financières brutes dans logs/erreurs.
- [ ] Tests property-based crypto/KDF (exigences backlog): invariants chiffrement/déchiffrement et invariants Argon2id (stabilité paramètres, sorties attendues).
- [ ] Tests fuzz sanitization IA: motifs atypiques (Unicode, homoglyphes, concaténations, ponctuation/espaces exotiques) neutralisés avant envoi.
- [ ] Test AIPD: index avec screening `requis?` + justification.
- [ ] Test data hygiene docs: aucune donnée réelle ni secret.
- [ ] Test conditionnel supply-chain: tout item touchant CI/CD, dépendances, build, SBOM, signature, provenance requiert gate Release et preuves associées.
- [ ] Préparation artefacts de clôture: bundle de preuves QA (edge-cases + fuzz/property + validation YAML) prêt à être référencé dans `closure.md` par Orchestrator après QA PASS.

## Entrées humaines attendues
- Granularité cible des sprints.
- Conventions de nommage `GLYPH-*`.

## Deliverables cibles
- `docs/implementation/backlog/README.md`
- `docs/implementation/backlog/glyph-v1-backlog.yaml`
- `docs/implementation/backlog/glyph-v1-roadmap.md`
- `docs/implementation/backlog/glyph-human-inputs.md`
- `docs/implementation/backlog/glyph-risk-register.md`
- `docs/implementation/backlog/glyph-definition-of-ready.md`
- `docs/implementation/backlog/glyph-definition-of-done.md`
- `docs/implementation/backlog/glyph-sprint-template.md`
- `docs/implementation/aipd/index.md`

## Forbidden
- [ ] Backlog purement fonctionnel sans risques/contrôles.
- [ ] Valeurs d’inputs humains inventées par agent.
- [ ] Tout `prod apply`.

## Agents requis
- Orchestrator
- DPO
- CISO
- DevSecOps (implémentation documentaire)
- QA
- Release (si CI/CD, dépendances, build, SBOM, release touchés)
