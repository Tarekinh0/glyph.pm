# GLYPH-002 Dev Notes

## Fichiers modifiés
- `src/redaction.rs`
- `tests/middleware_test.rs`
- `docs/implementation/sprints/GLYPH-002/dev-notes.md`

## Choix techniques
- `redaction::redact` a reçu une couverture corpus-driven de type property/fuzz minimale, sans nouvelle dépendance.
- Les cas de test couvrent les formes synthétiques longues, multi-lignes et malformées via des payloads générés en mémoire.
- Le chemin 500 et le chemin `panic!` réutilisent un payload synthétique multi-lignes/long pour vérifier la redaction end-to-end.
- La blocklist CI n'a pas été modifiée: le périmètre actuel reste couvert par `tests/zero_data_scan.sh`.

## Comment tester
- `cargo test redact -- --nocapture`
- `cargo test test_panic_path_redacts_stderr_end_to_end -- --nocapture`
- `cargo test`
- `bash tests/zero_data_scan.sh .`
