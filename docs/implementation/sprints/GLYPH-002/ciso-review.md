# CISO Review - GLYPH-002

## 1. Surface d'attaque
- `stderr/stdout` lors des `panic!` Axum et des réponses HTTP 500.
- Workflow GitHub Actions `anti-regression.yml`.
- Scanner anti-dépendances sur `Cargo.toml` et les sources `.rs`.

## 2. Assets protégés
- Données financières et PII en mémoire.
- Invariant zero-data / ADR-001.
- Absence de logs utilisateur / ADR-021.
- Secrets CI/CD.

## 3. Threat model
- Fuite de données sensibles dans les logs lors d'un `panic!` ou d'un 500.
- Introduction de crates de persistance/tracking.
- Exposition accidentelle de secrets via la CI.

## 4. Exigences sécurité bloquantes
- Le hook panic global écrit une ligne technique générique sur `stderr` sans payload brut.
- Le middleware 5xx remplace systématiquement le corps par `[REDACTED]`.
- Le test e2e Axum sur `/panic` capture `stderr` via le fd 2 sur Unix et prouve l'absence de fuite des données synthétiques.
- Les durcissements QA sont intégrés: tests corpus-driven sur `redaction::redact` avec chaînes longues, multi-lignes et payloads malformés.
- Le workflow CI reste en `permissions: read-all` et ne contient aucun secret en clair.
- Le linter bloque les crates DB/tracking visées par la story et la blocklist CI reste en place.
- Aucune régression observée sur ADR-001 / ADR-021.

## 5. Tests obligatoires
- `cargo test` passe.
- `tests/zero_data_scan.sh` passe et rejette bien un `Cargo.toml` factice avec crate DB/tracking.
- `tests/middleware_test.rs` couvre la redaction 500, la réponse panic redacted et le chemin e2e réel `/panic` avec capture `stderr`.
- Les tests utilisent uniquement des données synthétiques.

## 6. Risques résiduels
- Le blocklist scanner reste borné à une liste finie de crates.

## 7. Verdict
PASS
