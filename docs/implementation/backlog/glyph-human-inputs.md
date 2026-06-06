# Glyph V1 — Registre des human inputs

Statuts autorisés: `TODO`, `PROVIDED`, `NOT_NEEDED`, `DECISION_REQUIRED`.

## Tableau canonique

| Clé | Domaine | Description | Statut | Valeur/Orientation actuelle | Source | Impact backlog |
|---|---|---|---|---|---|---|
| `infra.target_topology_v1` | infra | Topologie d'exécution V1 | PROVIDED | On-prem principal + failover Hetzner | ADR-012 | GLYPH-003/013/024/030/BETA-005 |
| `infra.iac_toolchain_v1` | infra | Toolchain IaC officielle | PROVIDED | Ansible + Terraform | ADR-031 | GLYPH-004/031 |
| `infra.prod_apply_operator` | infra | Qui peut approuver un apply prod | DECISION_REQUIRED | Nominalement orchestrator + release + CISO | Politique backlog V1 | GLYPH-020 |
| `hardware.tee_scope_v1` | hardware trust | Scope TEE en V1 (oui/non, périmètre) | DECISION_REQUIRED | À arbitrer explicitement | ADR-030 (limites) | GLYPH-025/026 |
| `hardware.attestation_provider` | hardware trust | Mécanisme d'attestation retenu | PROVIDED | Keylime local + distant | ADR-030 | GLYPH-025/028 |
| `network.public_exposure_model` | network | Exposition publique contrôlée | PROVIDED | Cloudflare Tunnel + WireGuard interne | ADR-005/011 | GLYPH-014 |
| `network.failover_policy` | network | Politique failover | PROVIDED | Trigger heartbeat > 90s | ADR-012/030 | GLYPH-014/030 |
| `grist.version_pin` | grist | Version Grist cible V1 | TODO | Pinning à définir dans IaC | N/A | GLYPH-006 |
| `grist.plugin_policy` | grist | Politique widgets/plugins autorisés | TODO | Liste blanche à définir | N/A | GLYPH-006 |
| `backend.language_split` | backend | Répartition Rust/Go et ownership | TODO | Décision technique à formaliser | N/A | GLYPH-011 |
| `backend.error_redaction_policy` | backend | Politique anti-fuite dans erreurs | PROVIDED | Interdiction brute finance dans erreurs | Règles DevSecOps | GLYPH-002/011 |
| `ai.provider_region` | AI | Région d'hébergement IA | PROVIDED | France | README | GLYPH-007 |
| `ai.model_reference` | AI | Référence modèle IA versionnée | TODO | À figer avant GLYPH-015 | N/A | GLYPH-015 |
| `ai.payload_schema_version` | AI | Version de schéma payload IA | PROVIDED | `1.0` | ADR-007 | GLYPH-007 |
| `ai.sanitization_regex_set` | AI | Regex de sanitization des libellés | TODO | À compléter et valider CISO/DPO | ADR-016 | GLYPH-016 |
| `ai.full_transaction_payload` | AI | Envoi transaction complète à l'IA | NOT_NEEDED | Interdit par ADR-016 | ADR-016 | Contrôle négatif |
| `release.go_no_go_approver` | release | Décideur final Go/No-Go beta | DECISION_REQUIRED | À nommer explicitement | Gouvernance sprint | GLYPH-BETA-009 |
| `release.beta_window` | release | Fenêtre temporelle beta | DECISION_REQUIRED | À valider | Gouvernance sprint | GLYPH-BETA-001/009/010 |
| `release.prod_apply_policy` | release | Politique prod apply | PROVIDED | No prod apply sans approbation humaine explicite | `glyph-v1-backlog.yaml` | GLYPH-BETA-008 |

## Règles

1. Un item avec input `TODO` ou `DECISION_REQUIRED` reste en `WAITING_INPUT`.
2. Aucun input humain ne peut être inventé par un agent.
3. Toute mise à jour `PROVIDED` doit référencer une source (ADR, story, décision signée).
