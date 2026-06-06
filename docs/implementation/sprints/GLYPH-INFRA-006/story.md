# GLYPH-INFRA-006: Terraform module LXC test pour services Glyph

## Objectif
Créer les modules Terraform permettant de provisionner les LXC de test pour les services Glyph (glyph-auth, glyph-grist, glyph-traefik, glyph-ai et glyph-teet) sur Proxmox.

## Périmètre
- Création du module Terraform `lxc-service` dans `infrastructure/terraform/proxmox/modules/lxc-service`.
- Création de l'environnement de test dans `infrastructure/terraform/proxmox/envs/test`.
- Définition des variables typées pour l'environnement de test (endpoint API Proxmox, nom du node, storage pool, bridge réseau, template LXC).
- Configuration des conteneurs LXC non privilégiés.
- Désactivation du swap (swap = 0) pour respecter l'ADR-004.

## Hors périmètre
- Déploiement en production (`terraform apply prod` interdit).
- Configuration interne des services (Ansible, etc.).
- Création du template LXC de base (supposé existant).

## ADR impactés
- **ADR-013** : Choix de LXC vs Docker pour l'isolation des services.
- **ADR-031** : Choix du toolchain IaC — Ansible + Terraform vs NixOS.
- **ADR-003** : Stockage en RAM uniquement.
- **ADR-004** : Swap désactivé — garantie de non-écriture disque.

## Critères d'acceptation
- Le code Terraform est formaté (`terraform fmt`).
- Le code Terraform est valide (`terraform validate`).
- Un `terraform plan` pour l'environnement de test s'exécute sans erreur et ne cible aucune ressource de production.
- Aucun secret n'est commité dans le dépôt (utilisation de variables d'environnement ou de fichiers `.tfvars` ignorés).
- Les conteneurs LXC générés sont non privilégiés (`unprivileged = true`).
- Le swap est strictement désactivé (`swap = 0`).

## Tests attendus
- Exécution de `terraform fmt -check`.
- Exécution de `terraform validate`.
- Exécution de `terraform plan` avec des variables mockées pour valider la syntaxe et la structure.
