# CISO Requirements - GLYPH-002

## 1. Surface d'attaque
- **Logs d'application (stderr/stdout)** : Fuite potentielle de données en mémoire lors de crashs (`panic!`) ou d'erreurs HTTP 500.
- **Pipeline CI/CD (GitHub Actions)** : Injection de dépendances malveillantes ou non conformes (bases de données, trackers), fuite de secrets dans les logs de CI.
- **Dépendances Rust (Cargo)** : Introduction de crates violant l'architecture stateless (ex: ORM, drivers DB).

## 2. Assets protégés
- **Données financières et PII en mémoire** : Doivent rester strictement éphémères et invisibles (ADR-001).
- **Intégrité de l'architecture Zero-Data** : Prévention de l'ajout de toute capacité de persistance ou de tracking.
- **Secrets d'infrastructure** : Clés, tokens, et variables d'environnement utilisés par la CI.

## 3. Threat model
- **STRIDE - Information Disclosure** : Un attaquant ou un opérateur système accède aux logs d'erreur (stderr) et y trouve des données financières ou PII suite à un `panic!` de l'application.
- **STRIDE - Information Disclosure (CI)** : Un développeur logge accidentellement des secrets ou des données sensibles dans les outputs de GitHub Actions.
- **STRIDE - Tampering / Elevation of Privilege** : Un développeur ou un attaquant (via supply chain) introduit une dépendance de base de données (`sqlx`, `diesel`) pour exfiltrer ou persister des données, brisant l'ADR-001.

## 4. Exigences sécurité bloquantes
- **Middleware Axum (Gestion des `panic!` et erreurs 500)** :
  - Le middleware doit intercepter les `panic!` sans faire crasher le thread de manière incontrôlée.
  - **Séparation Nature/Donnée** : Le log d'erreur DOIT conserver la *nature* de l'erreur (ex: `ParseIntError`, `IndexOutOfBounds`, stacktrace technique générique) pour permettre le debug système.
  - **Redaction (Censure)** : Toute *donnée* métier (valeurs de variables, payloads, IBAN, montants) DOIT être remplacée par `[REDACTED]`. Aucun log utilisateur n'est toléré (ADR-021).
- **Sécurité du Workflow GitHub Actions** :
  - **No Sensitive Logs** : La CI ne doit jamais logger de variables d'environnement sensibles. Utiliser `::add-mask::` pour masquer dynamiquement toute valeur sensible si manipulée.
  - **No Plaintext Secrets** : Interdiction stricte de hardcoder des secrets dans les fichiers `.yml`. Utiliser exclusivement les GitHub Secrets (`${{ secrets.XXX }}`).
  - **Permissions minimales** : Le workflow doit utiliser `permissions: read-all` par défaut, et n'élever les droits que si strictement nécessaire.
- **Scanner de dépendances (Anti-Régression)** :
  - Le linter CI doit parser `Cargo.toml` et échouer immédiatement si des crates de persistance (`sqlx`, `diesel`, `postgres`, `mongodb`, `redis`, `sqlite`, etc.) ou de télémétrie intrusive sont détectées (Respect ADR-001).

## 5. Tests obligatoires
- **Test unitaire de Redaction** : Provoquer un `panic!` intentionnel avec un payload contenant des données synthétiques (ex: faux IBAN) et vérifier par assertion que la sortie standard/erreur contient la nature de l'erreur ET la mention `[REDACTED]`, sans la donnée initiale.
- **Test CI Linter** : Vérifier que le script de CI échoue correctement lorsqu'on lui fournit un `Cargo.toml` factice contenant `sqlx`.

## 6. Risques résiduels
- Faux négatifs dans le middleware de rédaction (une donnée non reconnue comme sensible pourrait échapper au filtre `[REDACTED]`).
- Contournement du linter CI par l'utilisation de crates de persistance non listées dans la blocklist.

## 7. Verdict
PASS_WITH_CONDITIONS