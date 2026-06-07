# DPO Requirements - GLYPH-002

## 1. Résumé de la story
Mise en place de contrôles anti-régression "zero-data" (Privacy by Design) via une CI bloquant l'ajout de bases de données ou d'outils de tracking, et un middleware Axum censurant les fuites de données financières ou personnelles dans les logs d'erreurs (panic/HTTP 500).

## 2. Données traitées
- **En production** : Les données traitées en mémoire (transitoires) incluent des données financières et potentiellement identifiantes. Conformément à l'ADR-001, aucune donnée ne doit être persistée.
- **En test** : Uniquement des données 100% synthétiques. L'utilisation de données réelles est strictement interdite.

## 3. Finalité
Garantir techniquement l'architecture "zero-data" et prévenir toute fuite de données (Data Leakage Prevention) dans les logs d'infrastructure.

## 4. Base de minimisation
- **Définition stricte des données à censurer par le middleware Axum** :
  - **Données financières** : Montants (chiffres avec ou sans devise), dates de transaction, IBAN, BIC, numéros de carte bancaire, soldes de compte, identifiants d'établissement bancaire.
  - **Données identifiantes (PII)** : Noms, prénoms, adresses, emails, numéros de téléphone, identifiants de session, tokens d'agrégation bancaire, adresses IP.
- **Respect de l'ADR-021** : Aucun log utilisateur n'est autorisé. Les logs d'erreurs (stderr) interceptés lors d'un `panic!` ou d'une erreur 500 ne doivent contenir que des traces techniques génériques. Toute PII ou donnée financière doit être remplacée par `[REDACTED]`.
- **Respect de l'ADR-001** : L'architecture doit rester stateless et zero-knowledge. Le scanner CI est une mesure de sécurité essentielle pour empêcher l'ajout de dépendances de persistance.

## 5. Risques droits et libertés
- Fuite de données financières ou personnelles hautement sensibles dans les logs d'infrastructure (stderr/stdout) en cas de crash de l'application.
- Introduction de dépendances permettant le tracking ou la persistance des données, violant ainsi la promesse fondamentale de Glyph.

## 6. Points bloquants
- **Interdiction absolue d'utiliser des données réelles** (vrais relevés bancaires, vrais IBAN, vraies transactions) dans les tests GitHub Actions ou en local.
- Le middleware de rédaction doit être global et s'appliquer à toutes les routes Axum sans exception.
- Le scanner CI doit bloquer toute Pull Request introduisant des crates de bases de données (ex: `sqlx`, `diesel`, `postgres`, `mongodb`, `redis`, `sqlite`) ou d'analytics/tracking.

## 7. Tests privacy exigés
- Tests unitaires prouvant que le middleware intercepte un `panic!` ou une erreur contenant des fausses PII (ex: faux IBAN, faux montant, faux nom) et que la sortie ne contient que `[REDACTED]`.
- Preuve que les tests utilisent exclusivement des données synthétiques évidentes (ex: IBAN factices de test, noms comme "John Doe").

## 8. Verdict
PASS_WITH_CONDITIONS