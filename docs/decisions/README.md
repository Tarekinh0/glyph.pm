# Architecture Decision Records (ADR)

\---

**Classification des données**

☒ Externe  ☐ Interne  ☐ Confidentiel  ☐ Secret

\---

## Historique des révisions

|Version|Date|Rédacteur|Approbateur|Description|
|-|-|-|-|-|
|0.1|28/03/2025|Fondateur|Fondateur|Version initiale|
|1.0|Juin 2026|Fondateur|Fondateur|Refonte complète — réorganisation du matériel à l'application, ajout ADR-024 à ADR-031, paramètres cryptographiques précisés, limites honnêtes documentées|



\---



## Table des matières

1. Objectif
2. Rôles et responsabilités
3. Note sur les limites honnêtes de cette architecture
4. Couche matériel et démarrage sécurisé

   * [ADR-024 : Choix du matériel et politique d'hébergement physique](./ADR-024.md)
   * [ADR-025 : Boot Guard, fuses CPU et ancrage de confiance matériel](./ADR-025.md)
   * [ADR-026 : TPM 2.0 — politique EK, AIK et PCR extend-only](./ADR-026.md)
   * [ADR-027 : Secure Boot et chaîne de vérification du bootloader](./ADR-027.md)
   * [ADR-028 : Measured Boot — PCR 0-9 et golden values](./ADR-028.md)
5. Couche OS et intégrité d'exécution

   * [ADR-029 : IMA, politique verrouillée et PCR10](./ADR-029.md)
   * [ADR-003 : Stockage en RAM uniquement](./ADR-003.md)
   * [ADR-004 : Swap désactivé — garantie de non-écriture disque](./ADR-004.md)
   * [ADR-030 : Attestation continue — Keylime et heartbeat 30s](./ADR-030.md)
6. Couche infrastructure et isolation

   * [ADR-006 : Architecture hybride on-prem / Hetzner](./ADR-006.md)
   * [ADR-013 : Choix de LXC vs Docker](./ADR-013.md)
   * [ADR-005 : Cloudflare Tunnel vs IP fixe](./ADR-005.md)
   * [ADR-011 : Cloudflare Tunnel vs VPN pour l'exposition publique](./ADR-011.md)
   * [ADR-012 : Stratégie de PCA — bascule automatique](./ADR-012.md)
7. Couche IaC, CI/CD et provenance du code

   * [ADR-031 : Choix du toolchain IaC — Ansible + Terraform vs NixOS](./ADR-031.md)
   * [ADR-020 : Stratégie CI/CD — Forgejo, GitHub Actions, SAST/DAST](./ADR-020.md)
   * [ADR-032 : Provenance du binaire — SLSA, Sigstore, Rekor, SBOM](./ADR-032.md)
8. Couche cryptographie et gestion des clés

   * [ADR-008 : Chiffrement du fichier .grist — AES-256-GCM](./ADR-008.md)
   * [ADR-009 : Dérivation de clé — Argon2id](./ADR-009.md)
   * [ADR-010 : Absence de mécanisme de récupération de mot de passe](./ADR-010.md)
9. Couche architecture système et session

   * [ADR-001 : Choix de l'architecture zéro-knowledge](./ADR-001.md)
   * [ADR-002 : Choix de Grist comme moteur de visualisation](./ADR-002.md)
   * [ADR-019 : Routing Traefik — authentification et sessions Grist](./ADR-019.md)
10. Couche réseau et exposition

    * [ADR-014 : Politique de mise à jour](./ADR-014.md)
    * [ADR-021 : Absence de logs utilisateurs](./ADR-021.md)
11. Couche application et données

    * [ADR-007 : Catégorisation IA avec effacement immédiat du CSV](./ADR-007.md)
    * [ADR-015 : Choix du modèle IA de catégorisation](./ADR-015.md)
    * [ADR-016 : Périmètre exact des données envoyées à l'IA](./ADR-016.md)
    * [ADR-023 : Formats d'import acceptés](./ADR-023.md)
    * [ADR-022 : Refus de la connexion bancaire directe DSP2](./ADR-022.md)
12. Couche gouvernance et juridique

    * [ADR-017 : Licence open source — AGPLv3 vs MIT](./ADR-017.md)
    * [ADR-018 : Juridiction et hébergement des données de l'association](./ADR-018.md)
13. Références



\---



## 1\. Objectif

Ce document regroupe l'ensemble des Architecture Decision Records (ADR) du projet Glyph. Chaque ADR documente une décision d'architecture significative : son contexte, les alternatives considérées, la décision retenue, sa justification, et ses conséquences.

Ces documents sont publics et servent à trois fins : traçabilité interne des choix techniques, transparence externe vis-à-vis des utilisateurs et des institutions, et formalisation d'engagements architecturaux qui ont valeur de contrainte et non de simple préférence.


\---



## 2\. Rôles et responsabilités

|Tâche|Autorisé|Responsable|Consulté|Informé|
|-|-|-|-|-|
|Rédaction d'un ADR|Contributeurs|Fondateur|Communauté|Communauté|
|Approbation d'un ADR|Contributeurs|Fondateur|Communauté|Communauté|
|Modification d'un ADR accepté|Contributeurs|Fondateur|Communauté|Communauté|
|Révocation d'un ADR|Contributeurs|Fondateur|Communauté|Communauté|
|Publication sur glyph.pm/documentation|Contributeurs|Fondateur|—|Communauté|
|Signalement d'incohérence ADR / implémentation|Contributeurs|Fondateur|Communauté|Communauté|
|Proposition d'un nouvel ADR|Contributeurs|Fondateur|Communauté|Communauté|
|Versioning et archivage des révisions|Contributeurs|Fondateur|Communauté|Communauté|



\---



## 3\. Note sur les limites de cette architecture

Glyph affirme des garanties fortes sur la vie privée de ses utilisateurs. Ce document serait incomplet — et malhonnête — s'il ne documentait pas avec précision ce que ces garanties ne couvrent pas.

**La limite principale : absence de TEE.**

L'architecture décrite dans `Zero\_Data\_Architecture.md` section 6 décrit une chaîne d'attestation cryptographique idéale reposant sur des environnements d'exécution de confiance (SGX, TDX) et une attestation à distance. Glyph ne l'implémente pas en v1, pour des raisons documentées dans l'ADR-024.

La conséquence est directe et doit être formulée sans détour : **une personne disposant d'un accès physique au serveur on-prem pendant une session active peut intercepter les mots de passe utilisateurs en mémoire et lire les fichiers déchiffrés.** Ce n'est pas une vulnérabilité logicielle — c'est une limite structurelle de toute architecture sans TEE.

Ce risque est atténué, non supprimé, par trois mécanismes :

* L'hébergement physique chez le fondateur avec accès restreint, je change mes serrures tous les 4 matins, un peu de vie privée bordel (ADR-024).
* Le heartbeat de 30 secondes + l'immuabilité des logs au niveau du noyau : tout reboot, toute anomalie système est détectable en temps réel par n'importe quel utilisateur via l'endpoint public TEET.
* Keylime distant indépendant (ADR-030) : un vérificateur tiers, non corréllé et sans dépendance SSI au projet Glyph, surveille l'intégrité du serveur on-prem en continu.

Glyph reste une architecture zéro-knowledge côté réseau et côté disque. Elle n'est pas une architecture zéro-confiance côté accès physique. Cette distinction est fondamentale et communiquée clairement aux utilisateurs.

**L'attestation sans TEE est organisationnelle, pas cryptographique.**

Sans TEE, la correspondance entre le code publié et le code qui s'exécute en production repose sur la chaîne IaC → CI/CD et sur la TEET, pas sur une preuve cryptographique vérifiable à distance. SLSA, Sigstore, Rekor (ADR-032) et les mesures de journalisation publiques couvrent la provenance du binaire de la source au déploiement, mais ne prouvent pas cryptographiquement qu'un acteur malveillant n'a pas modifié la mémoire d'un processus en cours d'exécution en exploitant une vulnérabilité qui aurait pu bypass toutes ces mesures de prévention et de détection.

Ces limites sont documentées ici parce qu'un modèle de confiance crédible exige une honnêteté totale sur ce qu'il ne peut, à ce jour, garantir.



\---



