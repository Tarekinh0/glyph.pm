# Revue CISO - GLYPH-000

### 1. Surface d'attaque
- Fichiers de configuration de l'assistant IA (`.opencode/agents/`, `.opencode/commands/`).
- Fichiers de gouvernance et de contexte (`AGENTS.md`, `docs/implementation/templates/`).
- Outils d'édition et d'exécution système (outils `write`, `edit`, `bash` exposés aux agents).

### 2. Assets protégés
- **Intégrité du code source** : Prévenir l'introduction de code malveillant ou non testé par des agents non autorisés.
- **Intégrité des décisions d'architecture (ADR)** : Garantir que les fondations de sécurité (Zero-Data, RAM-only, pas de logs) ne peuvent pas être altérées discrètement.
- **Gouvernance** : Assurer que le processus de validation (Gatekeeping) reste impartial et inviolable.

### 3. Threat model
- **Menace** : Élévation de privilèges ou "Confused Deputy" via les agents IA.
- **Scénario 1** : Un agent de revue (CISO, DPO, QA) est manipulé (ex: prompt injection) pour modifier le code source de l'application et introduire une vulnérabilité ou une fuite de données.
- **Scénario 2** : L'agent DevSecOps modifie un ADR (ex: ADR-019 sur les logs) pour affaiblir une exigence de sécurité afin de faciliter l'implémentation d'une feature.
- **Impact** : Compromission silencieuse de l'architecture Zero-Data, perte de confiance dans le modèle de sécurité, violation des règles fondamentales de Glyph.

### 4. Exigences sécurité bloquantes
- **Moindre privilège (Reviewers)** : Les agents `glyph-dpo`, `glyph-ciso`, `glyph-qa` et `glyph-release` **DOIVENT** avoir des permissions strictes en lecture seule sur le code source. Les outils de modification (`write`, `edit`) doivent leur être interdits sur les répertoires de code.
- **Protection des ADRs** : L'agent `glyph-devsecops` **DOIT** avoir une interdiction stricte de modifier les fichiers dans `docs/architecture/decisions/` et le fichier `Zero_Data_Architecture.md`.
- **Restriction Bash** : Si l'outil `bash` est accordé aux agents, il doit être configuré pour empêcher le contournement des restrictions de fichiers (ex: empêcher l'utilisation de `sed`, `echo >`, ou `rm` sur les fichiers protégés).
- **Contexte immuable** : Le fichier `AGENTS.md` doit explicitement lister les règles non-négociables (Zero-Data, pas de logs, RAM-only) pour que chaque agent ait ce contexte injecté dans son prompt système.

### 5. Tests obligatoires
- **Audit de configuration** : Vérification statique des fichiers `.opencode/agents/*.json` pour valider la présence des restrictions d'outils et de chemins de fichiers.
- **Test de non-régression (Reviewers)** : Simuler une demande à l'agent CISO ou QA de modifier un fichier source (`.ts`, `.go`) et vérifier que l'action est techniquement bloquée par le moteur OpenCode.
- **Test de non-régression (DevSecOps)** : Simuler une demande à l'agent DevSecOps de modifier un ADR et vérifier que l'action est bloquée.

### 6. Risques résiduels
- Faille dans le moteur de permissions d'OpenCode permettant à un agent de contourner les restrictions de fichiers.
- Utilisation de scripts externes ou d'outils tiers via `bash` pour modifier des fichiers indirectement si l'isolation de l'environnement d'exécution n'est pas parfaite.

### 7. Verdict
**PASS**