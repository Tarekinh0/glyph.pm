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

   * ADR-024 : Choix du matériel et politique d'hébergement physique
   * ADR-025 : Boot Guard, fuses CPU et ancrage de confiance matériel
   * ADR-026 : TPM 2.0 — politique EK, AIK et PCR extend-only
   * ADR-027 : Secure Boot et chaîne de vérification du bootloader
   * ADR-028 : Measured Boot — PCR 0-9 et golden values
5. Couche OS et intégrité d'exécution

   * ADR-029 : IMA, politique verrouillée et PCR10
   * ADR-003 : Stockage en RAM uniquement
   * ADR-004 : Swap désactivé — garantie de non-écriture disque
   * ADR-030 : Attestation continue — Keylime et heartbeat 30s
6. Couche infrastructure et isolation

   * ADR-006 : Architecture hybride on-prem / Hetzner
   * ADR-013 : Choix de LXC vs Docker
   * ADR-005 : Cloudflare Tunnel vs IP fixe
   * ADR-011 : Cloudflare Tunnel vs VPN pour l'exposition publique
   * ADR-012 : Stratégie de PCA — bascule automatique
7. Couche IaC, CI/CD et provenance du code

   * ADR-031 : Choix du toolchain IaC — Ansible + Terraform vs NixOS
   * ADR-020 : Stratégie CI/CD — Forgejo, GitHub Actions, SAST/DAST
   * ADR-032 : Provenance du binaire — SLSA, Sigstore, Rekor, SBOM
8. Couche cryptographie et gestion des clés

   * ADR-008 : Chiffrement du fichier .grist — AES-256-GCM
   * ADR-009 : Dérivation de clé — Argon2id
   * ADR-010 : Absence de mécanisme de récupération de mot de passe
9. Couche architecture système et session

   * ADR-001 : Choix de l'architecture zéro-knowledge
   * ADR-002 : Choix de Grist comme moteur de visualisation
   * ADR-019 : Routing Traefik — authentification et sessions Grist
10. Couche réseau et exposition

    * ADR-014 : Politique de mise à jour
    * ADR-021 : Absence de logs utilisateurs
11. Couche application et données

    * ADR-007 : Catégorisation IA avec effacement immédiat du CSV
    * ADR-015 : Choix du modèle IA de catégorisation
    * ADR-016 : Périmètre exact des données envoyées à l'IA
    * ADR-023 : Formats d'import acceptés
    * ADR-022 : Refus de la connexion bancaire directe DSP2
12. Couche gouvernance et juridique

    * ADR-017 : Licence open source — AGPLv3 vs MIT
    * ADR-018 : Juridiction et hébergement des données de l'association
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



## 4\. Couche matériel et démarrage sécurisé

### ADR-024 — Choix du matériel et politique d'hébergement physique

**Statut** : Accepté | **Date** : Juin 2026 | **Domaine** : Infrastructure, Sécurité physique

**Contexte**

Glyph opère sur un serveur dédié on-premise hébergé chez le fondateur, avec réplication sur Hetzner en cas d'indisponibilité (ADR-012). Le choix du matériel conditionne la chaîne de confiance depuis le silicium : support TPM 2.0, capacités de measured boot, et compatibilité avec Proxmox VE comme hyperviseur.

Le TEE (SGX/TDX) a été évalué et explicitement écarté pour la v1. Les raisons sont documentées dans la section Alternatives rejetées ci-dessous.

**Décision**

Serveur dédié on-premise avec les caractéristiques minimales suivantes :

* CPU x86-64 avec support Intel TXT ou AMD-V avec IOMMU
* TPM 2.0 physique (puce discrète, non firmware TPM)
* Secure Boot supporté au niveau UEFI
* RAM : 16 Go minimum (contrainte sans swap — ADR-003 et ADR-004)
* Disque système : SSD NVMe, chiffrement LUKS activé
* Accès physique : fondateur uniquement. Journal d'accès physique tenu.

Le serveur est hébergé au domicile du fondateur. La sécurité physique repose sur un accès restreint, un journal public (petite amie comprise), et un chihuahua dont la capacité de dissuasion est inversement proportionnelle à sa taille mais dont la vigilance est sans faille. Ce dernier point est documenté avec le sérieux qu'il mérite.

**Justification**

* Un serveur dédié on-premise élimine la couche hyperviseur d'un fournisseur cloud, vecteur d'accès non contrôlé dans les architectures mutualisées.
* Le TPM 2.0 physique est la racine de confiance matérielle pour la chaîne de measured boot. Un firmware TPM (fTPM) implémenté dans le CPU peut être compromis via mise à jour firmware — le TPM discret est plus robuste.
* L'hébergement chez le fondateur avec journal d'accès public est cohérent avec la politique de transparence radicale du projet. L'identité de la personne ayant accès physique est connue et publique.
* Le coût est inférieur à un serveur dédié Hetzner avec les mêmes garanties matérielles.

**Limite explicite — absence de TEE**

Glyph n'implémente pas de TEE en v1. La conséquence directe est documentée en section 3 : un accès physique pendant une session active permet la lecture de la mémoire déchiffrée. Cette décision est motivée par :

* Budget : les serveurs avec SGX/TDX activé et support opérateur pour l'attestation représentent un surcoût incompatible avec le budget.
* Cohérence de chaîne : le nœud de réplication Hetzner ne dispose pas de garanties TEE équivalentes sur les offres accessibles. Une chaîne d'attestation hétérogène serait plus complexe qu'une absence uniforme de TEE.

Le TEE est documenté comme objectif v2. Les ADR correspondants seront rédigés avant implémentation.

**Compromis assumés**

* L'accès physique pendant une session active est une surface d'attaque réelle, atténuée mais non supprimée.
* La dépendance à un site physique unique crée un risque de disponibilité compensé par la réplication Hetzner.

**Alternatives rejetées**

* Serveur dédié Hetzner uniquement : écarté. Le contrôle physique par l'organisation est un engagement de gouvernance documenté dans `Zero\_Data\_Architecture.md` section 6.4.
* VPS mutualisé : écarté. Accès hyperviseur du fournisseur incompatible avec l'architecture zéro-knowledge.
* SGX/TDX en v1 : écarté. Budget, compétences, cohérence de chaîne entre on-prem et Hetzner. Objectif v2.



\---



### ADR-025 — Boot Guard, fuses CPU et ancrage de confiance matériel

**Statut** : Accepté | **Date** : Juin 2026 | **Domaine** : Sécurité matérielle

**Contexte**

Intel Boot Guard est un mécanisme de vérification du BIOS par le CPU lui-même, via une clé cryptographique gravée dans les fuses OTP du processeur en usine. Il constitue l'ancrage de confiance matériel le plus bas de la chaîne : avant même que le BIOS s'exécute, le CPU vérifie que le firmware n'a pas été modifié. Cette vérification est indépendante du TPM et du Secure Boot.

**Décision**

* Boot Guard activé en mode Verified Boot ou Measured Boot selon les capacités du CPU retenu.
* Le BIOS/UEFI est mis à jour uniquement via le mécanisme officiel du fabricant, avec vérification du hash avant application. La version du firmware est pinned dans l'IaC et vérifiée à chaque déploiement.
* Les paramètres BIOS critiques sont documentés et leur état est vérifié au boot via TPM PCR1 (qui mesure la configuration NVRAM).

**Paramètres BIOS à vérifier dans l'IaC**

|Paramètre|Valeur attendue|PCR mesuré|
|-|-|-|
|Boot Guard|Activé, Verified Boot|Fuses CPU|
|Secure Boot|Activé|PCR7|
|TPM|Activé, TPM 2.0|PCR0|
|IOMMU (VT-d)|Activé|PCR0|
|USB boot|Désactivé|PCR1|
|Network boot|Désactivé|PCR1|
|Wake-on-LAN|Désactivé|PCR1|

**Vérification**

Un job CI/CD `verify-firmware-config` lit les PCR 0 et 1 via `tpm2\_pcrread` et les compare aux golden values publiées dans le dépôt. Toute divergence bloque le déploiement.

**Limite explicite**

Boot Guard ancre la confiance dans les fuses du CPU et dans la clé OEM du fabricant. Si le fabricant (Intel, AMD) est compromis ou contraint par une juridiction, cette racine de confiance peut être corrompue. Ce risque est hors de portée de toute mitigation purement technique.

**Alternatives rejetées**

* Pas de vérification firmware : écarté. Un BIOS compromis invalide l'intégralité de la chaîne de confiance au-dessus.
* fTPM à la place du TPM discret : écarté. Moins robuste qu'un TPM discret face à un patch microcode malveillant.



\---



### ADR-026 — TPM 2.0 — politique EK, AIK et PCR extend-only

**Statut** : Accepté | **Date** : Juin 2026 | **Domaine** : Sécurité, Cryptographie

**Contexte**

Le TPM 2.0 est la puce témoin de la chaîne de confiance. Il contient les Platform Configuration Registers (PCR), des registres extend-only qui accumulent les mesures de chaque composant du boot. L'Endorsement Key (EK) est gravée en usine et prouve que la puce est un vrai TPM physique. L'Attestation Identity Key (AIK) est générée localement et signe les quotes de mesure au quotidien.

**Décision**

**EK (Endorsement Key)**

Clé asymétrique gravée en usine par le fabricant du TPM. Utilisée uniquement pour prouver l'authenticité du TPM auprès d'un tiers. Elle ne signe jamais directement des données applicatives. Le certificat EK est vérifié contre le certificat racine du fabricant lors du provisionnement initial et enregistré dans l'IaC.

**AIK (Attestation Identity Key)**

Clé asymétrique générée localement sur le TPM, certifiée par l'EK. Utilisée pour signer les quotes PCR au quotidien (Keylime, ADR-030). Rotation annuelle ou lors de tout événement de sécurité.

**PCR (Platform Configuration Registers)**

Les PCR sont extend-only : une valeur PCR ne peut être qu'étendue, jamais réinitialisée sans reboot.

|PCR|Contenu|Responsable|
|-|-|-|
|PCR 0-7|Firmware, BIOS, Secure Boot|Firmware UEFI|
|PCR 8-9|Bootloader, kernel, initrd|GRUB|
|PCR 10|Mesures IMA runtime|Kernel Linux|
|PCR 11-23|Réservés usage applicatif futur|—|

**Vérification**

```bash
tpm2\_pcrread sha256:0,1,2,3,4,5,6,7,8,9,10

tpm2\_quote --key-context aik.ctx --pcr-list sha256:0,1,2,3,4,5,7,9,10 \\
  --message nonce.bin --signature quote.sig --pcr pcr.out
```

Ces commandes sont exécutées par Keylime (ADR-030) toutes les 30 secondes. Les résultats sont envoyés au verifier et publiés via TEET.

**Alternatives rejetées**

* fTPM : écarté (voir ADR-025).
* TPM 1.2 : écarté. SHA-1 obsolète, pas de SHA-256 natif.



\---



### ADR-027 — Secure Boot et chaîne de vérification du bootloader

**Statut** : Accepté | **Date** : Juin 2026 | **Domaine** : Sécurité, Infrastructure

**Contexte**

Secure Boot est le mécanisme par lequel l'UEFI vérifie la signature du bootloader avant de lui transférer l'exécution. Sur Linux, le Shim est un relais signé par Microsoft qui permet de valider GRUB via la Machine Owner Key (MOK).

**Décision**

* Secure Boot activé, mode Custom avec gestion des clés MOK.
* Shim signé Microsoft chargé en premier.
* GRUB, kernel et initrd signés avec la clé MOK de l'organisation. L'empreinte de la clé MOK est publiée dans l'IaC.
* Les variables NVRAM Secure Boot (PK, KEK, db, dbx) sont verrouillées après provisionnement.
* L'état Secure Boot est mesuré dans PCR7 par le firmware.

**Vérification**

```bash
mokutil --sb-state
# Attendu : SecureBoot enabled

sbverify --list /boot/vmlinuz-$(uname -r)

tpm2\_pcrread sha256:7
```

**Alternatives rejetées**

* Secure Boot désactivé : écarté. Ouvre le vecteur bootloader malveillant.
* Clés Microsoft uniquement sans MOK personnalisée : écarté. Dépendance aux décisions de révocation de Microsoft.



\---



### ADR-028 — Measured Boot — PCR 0-9 et golden values

**Statut** : Accepté | **Date** : Juin 2026 | **Domaine** : Sécurité, Infrastructure

**Contexte**

Le Measured Boot est le mécanisme par lequel chaque composant du démarrage mesure (hash SHA-256) le composant suivant et étend cette mesure dans un PCR du TPM. Si un composant est modifié, le PCR correspondant diverge des golden values — divergence détectable par Keylime (ADR-030).

**Décision**

**Golden values**

Les golden values sont les valeurs PCR de référence d'une machine saine après un démarrage propre. Elles sont :

* Générées lors du provisionnement initial après vérification manuelle.
* Publiées dans le dépôt Forgejo sous `infrastructure/tpm/golden-values.json`, signées avec Sigstore.
* Mises à jour explicitement et uniquement lors d'une mise à jour du firmware, du bootloader ou du kernel. Jamais automatiquement.
* Toute mise à jour des golden values est un commit tracé dans l'historique git et notifié publiquement.

**Procédure de mise à jour des golden values**

1. Planifier la fenêtre de maintenance (ADR-014).
2. Appliquer la mise à jour.
3. Redémarrer et lire les nouveaux PCR : `tpm2\_pcrread sha256:0,1,2,3,4,5,6,7,8,9`.
4. Vérifier manuellement que les changements PCR correspondent exactement aux composants mis à jour.
5. Committer les nouvelles golden values signées dans le dépôt.
6. Keylime reprend la vérification automatique dans les 30 secondes.

**Alternatives rejetées**

* Measured Boot sans golden values publiées : écarté. La mesure sans référence publique est une vérification interne non auditable.



\---



## 5\. Couche OS et intégrité d'exécution

### ADR-029 — IMA, politique verrouillée et PCR10

**Statut** : Accepté | **Date** : Juin 2026 | **Domaine** : Sécurité, OS

**Contexte**

Integrity Measurement Architecture (IMA) est un sous-système Linux qui mesure les fichiers au moment de leur exécution et étend ces mesures dans PCR10. Contrairement au Measured Boot qui couvre le démarrage, IMA couvre le runtime.

**Décision**

**Politique IMA**

```
# /etc/ima/ima-policy
dont\_measure fsmagic=0x9fa0
dont\_measure fsmagic=0x62656572
dont\_measure fsmagic=0x64626720
measure func=BPRM\_CHECK
measure func=FILE\_MMAP prot=EXEC
measure func=MODULE\_CHECK
measure func=FIRMWARE\_CHECK
```

**Verrouillage**

La politique IMA est verrouillée au boot via `ima\_policy=tcb` dans les paramètres kernel. Une fois verrouillée, elle ne peut être modifiée sans reboot — ce qui déclenche une divergence PCR9 détectable par Keylime. La politique mesure son propre fichier au chargement, garantissant que toute modification est tracée dans PCR10.

**auditd en mode immuable**

```bash
# /etc/audit/audit.rules — dernière ligne obligatoire
-e 2
```

Le flag `-e 2` rend les règles auditd immuables jusqu'au prochain reboot.

**Vérification**

```bash
tpm2\_pcrread sha256:10

cat /sys/kernel/security/ima/ascii\_runtime\_measurements | head -20

auditctl -s | grep enabled
# Attendu : enabled 2
```

**Alternatives rejetées**

* IMA sans verrouillage de politique : écarté. Une politique modifiable au runtime annule la valeur de PCR10.
* dm-verity sur le système de fichiers root : envisagé, écarté en v1 pour complexité opérationnelle. Objectif v2.



\---



### ADR-003 — Stockage en RAM uniquement (/dev/shm)

**Statut** : Accepté | **Date** : Mars 2026 | **Domaine** : Sécurité, Architecture système

**Contexte**

L'ADR-001 établit que les données utilisateurs ne doivent jamais être stockées de manière persistante sur nos serveurs. Il reste à définir comment les données sont manipulées pendant la session active.

**Décision**

Les fichiers .grist déchiffrés sont montés exclusivement dans `/dev/shm`. Ils ne touchent jamais le disque.

**Paramètres d'implémentation obligatoires**

Le service Auth appelle `mlockall(MCL\_CURRENT | MCL\_FUTURE)` au démarrage. En cas d'échec, le service refuse de démarrer — pas de fallback silencieux.

Les buffers contenant des données déchiffrées ou des matériaux cryptographiques appellent `madvise(ptr, len, MADV\_DONTDUMP)` après allocation pour les exclure des core dumps.

En Rust, utiliser `zeroize::Zeroize` sur tout type contenant des données de session. Le trait `Drop` doit appeler `zeroize()` explicitement.

En Go, utiliser `golang.org/x/sys/unix.Mlock()` sur les slices sensibles. Zéroiser avec `copy(slice, make(\[]byte, len(slice)))` avant libération.

**Vérification**

```bash
mount | grep shm
# Attendu : tmpfs on /dev/shm type tmpfs (rw,nosuid,nodev)

cat /proc/$(pgrep glyph-auth)/status | grep VmLck
# Attendu : valeur non nulle

find / -name "\*.grist" 2>/dev/null | grep -v /dev/shm
# Attendu : aucun résultat
```

**Compromis assumés**

* La RAM disponible est la limite stricte du nombre de sessions simultanées. Un pic de charge provoque le refus de nouvelles sessions plutôt qu'une dégradation silencieuse.
* En cas de crash, les sessions actives non fermées proprement sont perdues.

**Alternatives rejetées**

* Stockage dans `/tmp` : peut être swappé selon la configuration.
* Chiffrement du fichier temporaire sur disque : déplace le problème sans le résoudre.



\---



### ADR-004 — Swap désactivé — garantie de non-écriture disque

**Statut** : Accepté | **Date** : Mars 2026 | **Domaine** : Sécurité, Infrastructure

**Contexte**

L'ADR-003 établit que les fichiers déchiffrés sont en `/dev/shm`. Cette garantie est insuffisante si le kernel peut swapper les pages mémoire vers le disque sous pression mémoire.

**Décision**

Le swap est désactivé sur l'ensemble des nœuds via les mécanismes suivants, tous configurés dans l'IaC Ansible :

```bash
# /etc/sysctl.d/99-glyph-security.conf
vm.swappiness=0
vm.overcommit\_memory=2
kernel.dmesg\_restrict=1
```

```bash
# /etc/systemd/system/glyph-auth.service
\[Service]
LimitCORE=0
MemorySwapMax=0
```

**Vecteurs à neutraliser — tous obligatoires**

|Vecteur|Mitigation|Vérification|
|-|-|-|
|Swap|`swapoff -a` + `vm.swappiness=0`|`swapon --show` vide|
|zswap|`zswap.enabled=0` dans cmdline kernel|`cat /sys/module/zswap/parameters/enabled` retourne N|
|zram|Ne pas charger le module `zram`|`lsmod \| grep zram` vide|
|Core dumps OS|`ulimit -c 0` dans `/etc/security/limits.conf`|`ulimit -c` retourne 0|
|Core dumps processus|`LimitCORE=0` dans le service systemd|Vérifié dans `/proc/PID/limits`|
|Crash dumps (kdump)|Service `kdump` désactivé|`systemctl is-enabled kdump` retourne disabled|
|Hibernation|`systemctl mask hibernate.target`|`systemctl is-enabled hibernate` retourne masked|

**Job CI/CD `verify-no-swap`**

```bash
#!/bin/bash
set -e
SWAP=$(swapon --show --noheadings)
ZSWAP=$(cat /sys/module/zswap/parameters/enabled 2>/dev/null || echo "N")
ZRAM=$(lsmod | grep -c zram || true)

\[ -z "$SWAP" ]      || (echo "FAIL: swap actif" \&\& exit 1)
\[ "$ZSWAP" = "N" ]  || (echo "FAIL: zswap actif" \&\& exit 1)
\[ "$ZRAM" = "0" ]   || (echo "FAIL: zram chargé" \&\& exit 1)
echo "PASS: aucun vecteur swap détecté"
```

**Compromis assumés**

Sans swap, la RAM disponible est la seule ressource mémoire. Le dimensionnement (16 Go minimum, ADR-024) intègre cette contrainte.

**Alternatives rejetées**

* Swap chiffré : protège contre un accès physique au disque froid, pas contre une lecture mémoire à chaud.



\---



### ADR-030 — Attestation continue — Keylime et heartbeat 30s

**Statut** : Accepté | **Date** : Juin 2026 | **Domaine** : Sécurité, Infrastructure

**Contexte**

Les ADR-025 à ADR-029 définissent ce qui est mesuré. Il reste à définir comment ces mesures sont vérifiées en continu pendant l'exploitation.

**Décision**

**Keylime local**

Un agent Keylime est déployé sur le serveur on-prem. Il interroge le TPM toutes les 30 secondes, génère une quote signée par l'AIK, et compare les PCR aux golden values.

En cas de divergence PCR :

1. Alerte immédiate publiée via TEET (`/health/attestation`).
2. Blocage des nouvelles sessions.
3. Les sessions actives sont terminées proprement.
4. Reboot automatique après 60 secondes si la divergence persiste.

**Keylime distant — vérificateur tiers indépendant**

Un second vérificateur Keylime est déployé sur une instance Hetzner indépendante du serveur de production. Il reçoit les quotes du serveur on-prem et vérifie indépendamment contre les golden values publiées dans le dépôt Forgejo. Ses résultats sont publiés publiquement via TEET.

Ce vérificateur tiers garantit que l'opérateur ne peut pas altérer silencieusement le vérificateur local.

**Heartbeat 30s — endpoint public**

L'endpoint `/health/heartbeat` expose en temps réel :

* Timestamp du dernier heartbeat
* État de l'attestation PCR (PASS / FAIL / DEGRADED)
* Valeurs PCR actuelles (hash uniquement)
* Nombre de sessions actives (sans identifiant)
* Résultat de `swapon --show`
* Uptime du service Auth

Toute interruption supérieure à 90 secondes est détectable par n'importe quel utilisateur.

**Limite explicite**

Keylime atteste l'intégrité des composants mesurés. Il ne détecte pas une manipulation de la mémoire d'un processus en cours d'exécution par une personne avec accès physique pendant une session active. Cette limite est documentée en section 3.

**Alternatives rejetées**

* Vérification manuelle périodique : intervalle insuffisant pour détecter une compromission pendant une session.
* Keylime local uniquement sans vérificateur tiers : l'opérateur pourrait altérer le vérificateur local.



\---



## 6\. Couche infrastructure et isolation

### ADR-006 — Architecture hybride on-prem / Hetzner

**Statut** : Accepté | **Date** : Mars 2026 | **Domaine** : Infrastructure, Disponibilité

**Contexte**

Glyph repose sur un serveur on-prem hébergé chez le fondateur. Ce choix garantit le contrôle physique mais crée un risque de disponibilité. Une infrastructure de réplication est nécessaire.

**Décision**

Architecture hybride : serveur on-prem comme nœud principal, serveur dédié Hetzner comme nœud de réplication et de failover. Le nœud Hetzner est dimensionné identiquement au nœud on-prem pour les paramètres critiques. La réplication est chiffrée en transit via WireGuard (ADR-011).

**Justification**

* Le nœud Hetzner est un nœud de réplication, pas le nœud principal. Le contrôle physique du nœud principal par l'organisation est maintenu.
* Hetzner est une infrastructure dédiée hébergée en Allemagne, soumis au RGPD européen.

**Compromis assumés**

* Le nœud Hetzner est sous contrôle physique de Hetzner. L'accès hyperviseur Hetzner constitue une surface d'attaque résiduelle sur le nœud de failover.
* Les sessions actives lors d'un failover sont interrompues.

**Alternatives rejetées**

* Nœud Hetzner uniquement : perd le contrôle physique, incompatible avec `Zero\_Data\_Architecture.md` section 6.4.
* Multi-cloud : juridictions multiples, coûts incompatibles, souveraineté diluée.



\---



### ADR-013 — Choix de LXC vs Docker pour l'isolation des services

**Statut** : Accepté | **Date** : Mars 2026 | **Domaine** : Infrastructure, Sécurité

**Contexte**

Les services Glyph (Auth, Grist, Traefik, catégorisation IA) doivent être isolés. Deux technologies sont disponibles sur Proxmox VE : LXC et Docker.

**Décision**

LXC est retenu pour l'isolation des services sur Proxmox VE.

**Paramètres LXC obligatoires pour tous les conteneurs de production**

```yaml
lxc\_security:
  unprivileged: true
  apparmor: true
  seccomp: true
  capabilities\_drop:
    - NET\_RAW
    - SYS\_ADMIN
    - SYS\_PTRACE
  memory\_limit: "{{ container\_memory\_limit }}"
  swap: 0
  no\_new\_privs: true
```

Un profil seccomp restrictif est appliqué à tous les conteneurs. La liste des syscalls autorisés est versionnée dans l'IaC sous `infrastructure/lxc/seccomp/glyph-default.json`.

**Frontières LXC infranchissables**

* Le conteneur `glyph-auth` ne communique avec `glyph-grist` que via l'API Grist documentée. Aucune communication directe filesystem.
* Le conteneur `glyph-ai` reçoit uniquement des libellés texte via une API REST interne. Aucun montage de volume partagé avec `glyph-auth` ou `glyph-grist`.
* Seul Traefik expose des ports publics.

**Justification**

* LXC sur Proxmox VE est le modèle d'isolation natif de la stack — pas de démon Docker supplémentaire.
* Les conteneurs LXC non privilégiés font tourner les processus en tant qu'utilisateurs mappés, limitant l'impact d'une compromission.
* La surface d'attaque de Docker (démon root, registry, layers) est plus large que LXC.

**Alternatives rejetées**

* Docker : démon root, surface d'attaque plus large.
* VMs complètes par service : overhead mémoire incompatible avec l'ADR-003.
* Isolation par processus systemd uniquement : isolation insuffisante.



\---



### ADR-005 — Choix de Cloudflare Tunnel vs IP fixe

**Statut** : Accepté | **Date** : Mars 2026 | **Domaine** : Infrastructure réseau, Sécurité

**Contexte**

Le serveur on-prem est hébergé chez le fondateur, derrière une connexion internet résidentielle sans IP fixe garantie. Il faut exposer le service publiquement sans exposer l'adresse IP physique.

**Décision**

Cloudflare Tunnel est retenu pour l'exposition publique du nœud on-prem. Le tunnel est initié par le serveur (connexion sortante) — aucun port entrant n'est ouvert.

**Justification**

* Masque l'adresse IP physique du serveur.
* Zéro port entrant ouvert — surface d'attaque réseau directe nulle.
* Protection DDoS Cloudflare sans coût supplémentaire.
* Fonctionne indépendamment de l'IP dynamique résidentielle.

**Compromis assumés**

* Cloudflare voit le trafic entre son edge et le serveur. TLS end-to-end activé dans l'IaC.
* Dépendance à la disponibilité Cloudflare pour le nœud on-prem. Atténué par le failover Hetzner (ADR-012).

**Alternatives rejetées**

* IP fixe résidentielle : non disponible de manière fiable, expose l'adresse physique du fondateur.
* WireGuard avec IP Hetzner comme relais : plus complexe, avantages de souveraineté limités à cette échelle.



\---



### ADR-011 — Cloudflare Tunnel vs VPN pour l'exposition publique

**Statut** : Accepté | **Date** : Mars 2026 | **Domaine** : Infrastructure réseau

**Contexte**

L'ADR-005 traite de l'exposition du nœud on-prem. L'ADR-011 traite de la politique d'exposition globale et du protocole pour les communications inter-nœuds.

**Décision**

* Exposition publique : Cloudflare Tunnel sur le nœud on-prem (ADR-005), IP fixe dédiée sur le nœud Hetzner.
* Communications inter-nœuds (réplication, failover) : WireGuard en mesh privé. Le trafic de réplication ne transite jamais par Cloudflare.

**Alternatives rejetées**

* OpenVPN : plus lourd, moins performant que WireGuard.
* Réplication via Cloudflare : expose le trafic de réplication à un tiers.



\---



### ADR-012 — Stratégie de PCA — bascule automatique on-prem vers Hetzner

**Statut** : Accepté | **Date** : Mars 2026 | **Domaine** : Disponibilité, Infrastructure

**Contexte**

En cas d'indisponibilité du nœud on-prem, le service doit basculer automatiquement sur le nœud Hetzner.

**Décision**

La bascule est déclenchée par l'absence de heartbeat pendant 90 secondes consécutives.

**Procédure de bascule**

1. Le vérificateur Keylime distant (Hetzner) détecte l'absence de heartbeat.
2. Le DNS Cloudflare bascule vers l'IP du nœud Hetzner.
3. Le nœud Hetzner active l'accès public.
4. Le fondateur est notifié.

**Retour en production on-prem**

La bascule retour est manuelle et requiert une validation explicite du fondateur après vérification de l'intégrité du nœud on-prem (attestation PCR complète, golden values vérifiées).

**Compromis assumés**

* Les sessions actives lors de la bascule sont interrompues.
* Le nœud Hetzner a une posture de sécurité légèrement inférieure (pas de contrôle physique). Ce compromis est documenté et communiqué via TEET.



\---



## 7\. Couche IaC, CI/CD et provenance du code

### ADR-031 — Choix du toolchain IaC — Ansible + Terraform vs NixOS

**Statut** : Accepté | **Date** : Juin 2026 | **Domaine** : Infrastructure, Opérations

**Contexte**

L'infrastructure Glyph doit être décrite sous forme de code pour garantir reproductibilité, traçabilité et vérifiabilité. Deux approches ont été évaluées : NixOS et la combinaison Ansible + Terraform.

**Décision**

Ansible + Terraform sont retenus pour la v1. NixOS est documenté comme objectif v2.

* **Terraform** : provisionnement des ressources cloud (nœud Hetzner, DNS Cloudflare, réseau WireGuard).
* **Ansible** : configuration des nœuds (OS hardening, LXC, services applicatifs, paramètres de sécurité).
* **Forgejo** : source de vérité de l'IaC. Tout changement passe par un merge request.

**Justification**

* Ansible et Terraform sont maîtrisés par une base de contributeurs beaucoup plus large que NixOS. La barrière d'entrée pour les futurs mainteneurs est significativement plus basse.
* Glyph est à un stade de lancement avec une équipe réduite (1). La priorité est une infrastructure opérationnelle et sécurisée, pas l'exploration de nouveaux paradigmes de configuration.
* Ansible est suffisant pour implémenter toutes les contraintes définies dans les ADR-025 à ADR-030.

**Limite explicite — NixOS absent en v1**

NixOS aurait offert une reproductibilité de builds plus forte : le même fichier de configuration produit bit-à-bit le même système, rendant la correspondance entre IaC et production vérifiable cryptographiquement. Cette propriété est précieuse pour le modèle d'architecture zero-trust.

Elle est sacrifiée en v1 pour des raisons de ressources humaines — pas de ressources pour former les mainteneurs actuels (1) et gérer la migration. Ce compromis est documenté honnêtement. NixOS sera reconsidéré en v2 conjointement avec l'introduction du TEE.

**Structure du dépôt IaC**

```
infrastructure/
  terraform/
    hetzner/
    cloudflare/
    wireguard/
  ansible/
    roles/
      os-hardening/
      lxc/
      tpm/
      services/
    inventory/
      onprem.yml
      hetzner.yml
  tpm/
    golden-values.json
```

**Alternatives rejetées**

* NixOS en v1 : pas de ressources pour la migration. Objectif v2.
* Scripts Bash versionnés : non déclaratifs, difficiles à auditer.
* Puppet / Chef : overhead disproportionné pour cette taille d'infrastructure.



\---



### ADR-020 — Stratégie CI/CD — Forgejo, GitHub Actions, SAST/DAST

**Statut** : Accepté | **Date** : Mars 2026 | **Domaine** : Sécurité, Développement, Gouvernance

**Contexte**

La chaîne CI/CD doit garantir que chaque modification du code est analysée avant d'atteindre la production. Implémenter une chaîne CI/CD robuste en full self-hosted représente un effort de maintenance disproportionné pour une équipe de la taille de Glyph.

**Décision**

Forgejo auto-hébergé est la source de vérité du code. Un miroir automatique pousse les commits vers GitHub. GitHub Actions exécute les pipelines de sécurité.

**Pipeline CI/CD obligatoire**

|Job|Outil|Bloquant|
|-|-|-|
|SAST Rust/Go|CodeQL + Semgrep|Oui|
|Scan dépendances|Dependabot|Oui (CVE critiques)|
|DAST interface web|OWASP ZAP|Oui|
|Tests unitaires|cargo test / go test|Oui|
|Vérification swap post-deploy|Script bash (ADR-004)|Oui|
|Vérification PCR golden values|tpm2-tools|Oui|
|Vérification paramètres Argon2id|Lecture constantes compilées|Oui|
|Génération SBOM|syft|Non — publication|
|Signature Sigstore|cosign|Oui|

Les correctifs de sécurité urgents peuvent être déployés sans attendre la CI en cas de crise documentée.

**Justification**

* Forgejo comme source de vérité garantit que la gouvernance reste sous contrôle de l'association indépendamment de GitHub.
* GitHub Actions est mature, gratuit pour les dépôts publics, et le code source de Glyph est public par nature — le faire transiter par des runners GitHub ne compromet pas la souveraineté des données utilisateurs.

**Compromis assumés**

* Dépendance à GitHub pour la CI/CD. En cas d'indisponibilité, migration vers Forgejo Actions ou Woodpecker CI.
* Les résultats de la CI sont publics sur GitHub, y compris les vulnérabilités détectées non encore corrigées.

**Alternatives rejetées**

* Full self-hosted Forgejo Actions : charge de maintenance des runners incompatible avec la taille de l'équipe.
* GitHub comme source de vérité : rejeté (ADR-018).



\---



### ADR-032 — Provenance du binaire — SLSA, Sigstore, Rekor, SBOM

**Statut** : Accepté | **Date** : Juin 2026 | **Domaine** : Sécurité, Chaîne d'approvisionnement

**Contexte**

L'IaC et le CI/CD garantissent que seul le code mergé sur Forgejo atteint la production. Cette garantie est organisationnelle. SLSA, Sigstore et Rekor ajoutent une couche cryptographique sur la provenance du binaire.

**Décision**

**SLSA niveau 2 en v1, niveau 3 objectif v2**

SLSA niveau 2 exige un build sur plateforme hébergée avec provenance signée et disponible pour vérification.

**Sigstore / cosign — signature keyless**

```bash
cosign sign-blob --bundle glyph-auth.bundle ./glyph-auth-linux-amd64

cosign verify-blob --bundle glyph-auth.bundle \\
  --certificate-identity https://github.com/Tarekinh0/glyph.pm/.github/workflows/release.yml@refs/heads/main \\
  --certificate-oidc-issuer https://token.actions.githubusercontent.com \\
  ./glyph-auth-linux-amd64
```

**Rekor**

Chaque signature est publiée dans Rekor, le log immuable Merkle tree de Sigstore. Vérifiable par n'importe qui, indépendamment de l'opérateur.

**SBOM**

Un SBOM au format SPDX est généré via `syft` à chaque release et publié sous `releases/sbom/`.

**Ce que cela garantit**

Un utilisateur peut vérifier que le binaire a été produit par le pipeline CI public depuis un commit précis. Toute modification du binaire après signature est détectable.

**Ce que cela ne garantit pas**

Sans TEE, la correspondance entre le binaire signé et le processus en mémoire reste une garantie organisationnelle, pas cryptographique. Documenté en section 3.

**Alternatives rejetées**

* Signature GPG manuelle : clé privée à gérer, pas de log public immuable.
* Pas de signature : incompatible avec la politique de vérifiabilité directe.



\---



## 8\. Couche cryptographie et gestion des clés

### ADR-008 — Chiffrement du fichier .grist — AES-256-GCM

**Statut** : Accepté | **Date** : Mars 2026 | **Domaine** : Cryptographie, Sécurité

**Contexte**

Le fichier .grist doit être chiffré avec un algorithme garantissant simultanément confidentialité et intégrité.

**Décision**

AES-256-GCM est retenu.

**Paramètres obligatoires — encodés comme constantes non configurables**

|Paramètre|Valeur|Justification|
|-|-|-|
|Algorithme|AES-256-GCM|ANSSI recommandé, accélération AES-NI|
|Taille de clé|256 bits|Recommandation ANSSI|
|Tag d'authentification|128 bits|Valeur maximale GCM, recommandation NIST|
|Nonce|96 bits aléatoires (OsRng)|Recommandation NIST SP 800-38D|
|Génération du nonce|CSPRNG, nouveau à chaque chiffrement|Interdit : compteur, nonce fixe, nonce dérivé|
|AAD|`version\_schema \|\| session\_uuid`|Lie le blob à sa version de schéma|
|Limite bytes par clé|64 Go maximum|Rotation obligatoire au-delà|

Ces paramètres sont des constantes dans `crypto::cipher::AES256GCM\_PARAMS`. Aucun n'est configurable à l'exécution.

**Implémentation Rust obligatoire**

```rust
use aes\_gcm::{Aes256Gcm, Key, Nonce};
use aes\_gcm::aead::{Aead, NewAead};
use rand::rngs::OsRng;
use rand::RngCore;

const KEY\_SIZE: usize = 32;
const NONCE\_SIZE: usize = 12;

pub fn encrypt(key: \&\[u8; KEY\_SIZE], plaintext: \&\[u8], aad: \&\[u8]) -> Result<Vec<u8>, CryptoError> {
    let mut nonce\_bytes = \[0u8; NONCE\_SIZE];
    OsRng.fill\_bytes(\&mut nonce\_bytes);
    let nonce = Nonce::from\_slice(\&nonce\_bytes);
    let cipher = Aes256Gcm::new(Key::from\_slice(key));
    let ciphertext = cipher.encrypt(nonce, aes\_gcm::aead::Payload { msg: plaintext, aad })
        .map\_err(|\_| CryptoError::EncryptionFailed)?;
    // Format de sortie : nonce (12 octets) || ciphertext || tag (16 octets)
    Ok(\[nonce\_bytes.as\_ref(), ciphertext.as\_ref()].concat())
}
```

**Alternatives rejetées**

* ChaCha20-Poly1305 : solide, mais AES-NI disponible sur tous les CPU cibles et ANSSI cite AES-256-GCM en premier.
* AES-128-GCM : marge de sécurité inférieure sans gain de performance significatif.
* AES-256-CBC + HMAC-SHA256 : construction plus complexe et plus risquée qu'AEAD.



\---



### ADR-009 — Dérivation de clé — Argon2id

**Statut** : Accepté | **Date** : Mars 2026 | **Domaine** : Cryptographie, Sécurité

**Contexte**

Le mot de passe utilisateur ne peut pas être utilisé directement comme clé AES-256. Une KDF est nécessaire.

**Décision**

Argon2id est retenu avec les paramètres suivants, encodés comme constantes dans `crypto::kdf::ARGON2\_PARAMS`.

**Paramètres obligatoires**

|Paramètre|Valeur|Justification|
|-|-|-|
|Variant|Argon2id|Hybride data-dependent + data-independent, recommandé OWASP|
|Version|0x13 (v1.3)|Version courante|
|`m` mémoire|65536 KiB (64 MiB)|Recommandation OWASP 2023, résistance GPU|
|`t` itérations|3|OWASP minimum avec m=64 MiB|
|`p` parallélisme|4|Adapté au CPU cible|
|Longueur de sortie|32 octets|Taille de clé AES-256|
|Salt|16 octets aléatoires (OsRng)|Stocké en clair avec le blob|

Toute modification de ces paramètres nécessite un nouvel ADR supersédant celui-ci.

**Implémentation Rust obligatoire**

```rust
use argon2::{Argon2, Params, Version, Algorithm};
use rand::rngs::OsRng;
use rand::RngCore;

const ARGON2\_M\_COST: u32 = 65536;
const ARGON2\_T\_COST: u32 = 3;
const ARGON2\_P\_COST: u32 = 4;
const ARGON2\_OUTPUT\_LEN: usize = 32;
const ARGON2\_SALT\_LEN: usize = 16;

pub fn derive\_key(password: \&\[u8]) -> Result<(\[u8; ARGON2\_OUTPUT\_LEN], \[u8; ARGON2\_SALT\_LEN]), KdfError> {
    let mut salt = \[0u8; ARGON2\_SALT\_LEN];
    OsRng.fill\_bytes(\&mut salt);
    let params = Params::new(ARGON2\_M\_COST, ARGON2\_T\_COST, ARGON2\_P\_COST, Some(ARGON2\_OUTPUT\_LEN))
        .map\_err(|\_| KdfError::InvalidParams)?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut key = \[0u8; ARGON2\_OUTPUT\_LEN];
    argon2.hash\_password\_into(password, \&salt, \&mut key)
        .map\_err(|\_| KdfError::DerivationFailed)?;
    Ok((key, salt))
}
```

**Job CI/CD de vérification des paramètres**

```bash
#!/bin/bash
BINARY="./target/release/glyph-auth"
M=$(strings $BINARY | grep -oP 'ARGON2\_M\_COST=\\K\\d+')
T=$(strings $BINARY | grep -oP 'ARGON2\_T\_COST=\\K\\d+')
P=$(strings $BINARY | grep -oP 'ARGON2\_P\_COST=\\K\\d+')

\[ "$M" = "65536" ] || (echo "FAIL: m=$M attendu 65536" \&\& exit 1)
\[ "$T" = "3" ]     || (echo "FAIL: t=$T attendu 3" \&\& exit 1)
\[ "$P" = "4" ]     || (echo "FAIL: p=$P attendu 4" \&\& exit 1)
echo "PASS: paramètres Argon2id conformes à ADR-009"
```

**Justification**

* Vainqueur du Password Hashing Competition 2015, recommandé par OWASP, NIST SP 800-63B et l'ANSSI.
* m=64 MiB force chaque tentative d'attaque à allouer 64 MiB de RAM. Sur GPU, une RTX 4090 (24 Go VRAM) ne peut tester que \~375 mots de passe simultanément.

**Compromis assumés**

* 64 MiB de RAM alloués à chaque ouverture de session. Sur 32 Go sans swap, cela limite à \~500 sessions simultanées.
* La dérivation prend \~300ms sur le CPU cible.

**Alternatives rejetées**

* bcrypt : pas de paramètre mémoire, résistance GPU inférieure.
* scrypt : Argon2id supérieur sur tous les critères pertinents.
* PBKDF2 : pas de paramètre mémoire, résistance GPU très faible.



\---



### ADR-010 — Absence de mécanisme de récupération de mot de passe

**Statut** : Accepté | **Date** : Mars 2026 | **Domaine** : Architecture, Vie privée

**Contexte**

Tout mécanisme de récupération de mot de passe implique un tiers qui connaît quelque chose sur l'utilisateur. Un système qui affirme ne rien savoir de ses utilisateurs mais qui peut envoyer un email de récupération se contredit structurellement.

**Décision**

Glyph ne propose pas et ne proposera pas de mécanisme de récupération de mot de passe.

Un utilisateur qui perd son mot de passe sans sauvegarde de son fichier .grist perd définitivement l'accès à ses données. Ce n'est pas une limitation technique non résolue — c'est une conséquence délibérée du modèle.

**Recommandations communiquées aux utilisateurs**

* Sauvegarder le fichier .grist chiffré sur deux supports distincts.
* Utiliser un gestionnaire de mots de passe pour stocker le mot de passe Glyph.
* Le fichier .grist sans le mot de passe est inutilisable. Le mot de passe sans le fichier .grist est inutile. Les deux sont nécessaires et suffisants.

**Justification**

La cohérence du modèle est un actif de confiance. "Nous ne savons rien de vous, sauf quand vous oubliez votre mot de passe" n'est pas une promesse crédible.

**Alternatives rejetées**

* Récupération par email : requiert de connaître l'email de l'utilisateur, incompatible avec ADR-001.
* Clé de secours chiffrée chez l'opérateur : l'opérateur détient quelque chose qui permet de reconstruire l'accès.
* Récupération sociale (N-of-M) : élégant cryptographiquement, complexité UX disproportionnée au stade de lancement. Objectif v2 potentiel.



\---



## 9\. Couche architecture système et session

### ADR-001 — Choix de l'architecture zéro-knowledge

**Statut** : Accepté | **Date** : Mars 2026 | **Domaine** : Architecture système, Sécurité, Vie privée

**Contexte**

Glyph traite des données financières parmi les plus sensibles qui soient. Trois modèles étaient envisageables : stockage centralisé classique (Finary, Bankin'), chiffrement E2E avec stockage serveur (ProtonMail, Bitwarden), et architecture zéro-knowledge stateless.

**Décision**

Adoption de l'architecture zéro-knowledge stateless. L'utilisateur conserve son fichier chiffré localement. À chaque session il apporte son fichier, travaille en RAM, et repart avec son fichier rechiffré. Le serveur n'a aucune mémoire entre les sessions.

**Justification**

* Un serveur qui ne contient pas de données ne peut pas les perdre. Ce principe élimine une surface d'attaque entière.
* La menace principale dans l'industrie n'est pas l'attaquant externe mais l'erreur de configuration interne, la compromission d'un accès légitime, et la contrainte légale forçant la divulgation.
* Elle remplace "faites-nous confiance" par "vérifiez par vous-même" — garantie structurelle, indépendante des acteurs.
* Coût d'infrastructure bas et stable, indépendant du volume de données.

**Compromis assumés**

* L'utilisateur gère son fichier chiffré entre les sessions.
* Pas de synchronisation automatique entre appareils en mode local.
* Les libellés bancaires uniquement transitent de manière éphémère vers le service d'IA pour catégorisation.
* La récupération de compte est impossible sans fichier et mot de passe (ADR-010).

**Limite explicitée**

L'architecture zéro-knowledge est garantie côté réseau et côté disque. Elle ne constitue pas une garantie zéro-confiance côté accès physique au serveur en l'absence de TEE. Documenté en section 3.

**Alternatives rejetées**

* Stockage centralisé classique : incompatible avec la mission.
* Chiffrement E2E seul : maintient une dépendance serveur et génère des obligations de gestion des clés.
* Architecture hybride avec opt-in : deux classes de garanties diluent le message.



\---



### ADR-002 — Choix de Grist comme moteur de visualisation

**Statut** : Accepté | **Date** : Mars 2026 | **Domaine** : Architecture système, Expérience utilisateur

**Contexte**

Glyph a besoin d'un moteur de visualisation et d'interaction avec les données patrimoniales.

**Décision**

Adoption de Grist en self-hosted. Il charge les fichiers utilisateurs depuis la RAM de session conformément à l'ADR-001 et ne stocke aucune donnée de manière permanente dans le flux standard.

**Justification**

* Combine la familiarité d'un tableur avec des capacités d'application métier : vues multiples, formules, formulaires, graphiques.
* Widgets custom HTML/JS embarquables pour les visualisations avancées.
* Licence Apache 2.0, entièrement self-hostable, sans dépendance au Cloud Act.
* La vraie valeur ajoutée de Glyph réside dans le template — tables, formules, vues adaptées à la fiscalité française.

**Compromis assumés**

* Le routing dynamique de sessions nécessite un ForwardAuth via Traefik (ADR-019).
* Les graphiques natifs sont limités pour des visualisations avancées.

**Alternatives rejetées**

* Interface React custom : effort disproportionné pour une équipe réduite.
* Google Sheets, Airtable, Notion : propriétaires, Cloud Act, incompatibles avec ADR-001.



\---



### ADR-019 — Routing Traefik — authentification et sessions Grist

**Statut** : Accepté | **Date** : Mars 2026 | **Domaine** : Architecture système, Sécurité

**Contexte**

Grist n'a pas été conçu pour le modèle zéro-knowledge de Glyph. Il faut un mécanisme qui intercède entre l'utilisateur et Grist pour valider la session, monter le fichier .grist en RAM, et démonter cet espace à la fermeture.

**Décision**

Traefik ForwardAuth intercepte chaque requête vers Grist et la valide auprès du service Auth. L'URL Grist est opaque pour l'utilisateur. Sticky routing par cookie — une session reste ancrée sur la même instance Grist.

**Paramètres de session obligatoires**

|Paramètre|Valeur|Justification|
|-|-|-|
|Durée max de session|4 heures|Limite la fenêtre d'exposition|
|Timeout d'inactivité|30 minutes|Fermeture automatique|
|Token de session|UUID v4, HttpOnly, Secure, SameSite=Strict|Résistance CSRF|
|Rotation du token|À chaque validation ForwardAuth|Limite la fenêtre de vol de session|

**Vérification**

```bash
curl -s https://glyph.pm/app | grep -oP 'grist/\[a-zA-Z0-9/\_-]+' | grep -v 'api/'
# Attendu : aucun document\_id visible

curl -I https://glyph.pm/app | grep Set-Cookie
# Attendu : HttpOnly; Secure; SameSite=Strict
```

**Alternatives rejetées**

* URL avec document\_id visible : surface d'attaque IDOR.
* Authentification Grist uniquement : ne peut pas valider les tokens de session Glyph.
* Un container Grist par utilisateur : 200-300 MB au repos, incompatible avec ADR-003.



\---



## 10\. Couche réseau et exposition

### ADR-014 — Politique de mise à jour

**Statut** : Accepté | **Date** : Mars 2026 | **Domaine** : Opérations, Disponibilité

**Décision**

* CVE CVSS ≥ 9.0 : appliquées dans les 24 heures, fenêtre notifiée 2 heures à l'avance via le status page public.
* CVE CVSS 7.0-8.9 : appliquées dans les 72 heures, fenêtre notifiée 24 heures à l'avance.
* Mises à jour courantes : fenêtre hebdomadaire, dimanche 02h00-04h00 UTC.
* Toute mise à jour modifiant un composant mesuré (firmware, kernel, GRUB) déclenche une mise à jour des golden values (ADR-028).

**Compromis assumés**

* Les sessions actives lors d'une fenêtre de maintenance sont interrompues proprement.
* La fenêtre 02h00-04h00 UTC correspond à 03h00-05h00 heure de Paris.



\---



### ADR-021 — Absence de logs utilisateurs

**Statut** : Accepté | **Date** : Mars 2026 | **Domaine** : Vie privée, Sécurité, Opérations

**Contexte**

Les logs applicatifs constituent, par accumulation, un profil comportemental de chaque utilisateur.

**Décision**

Glyph ne conserve aucun log associable à un utilisateur identifiable.

**Configuration Traefik obligatoire**

```yaml
log:
  level: ERROR
accessLog: false
```

Les logs d'erreur système sont conservés 24 heures puis supprimés automatiquement.

**Métriques agrégées publiées via TEET — uniquement**

|Métrique|Format|
|-|-|
|Sessions actives|Entier, sans identifiant|
|Volume trafic par heure|Octets agrégés|
|Codes de réponse HTTP|Compteurs par code|
|Latence p50/p95/p99|Millisecondes|

**Vérification**

```bash
docker exec traefik cat /etc/traefik/traefik.yml | grep -A2 accessLog
# Attendu : accessLog: false

find /var/log -name "\*access\*" -newer /var/log/dpkg.log 2>/dev/null
# Attendu : aucun résultat
```

**Justification**

* Un log contenant IP + timestamp + endpoint est une donnée personnelle au sens du RGPD. Ne pas le créer est plus propre que le créer puis le protéger.
* L'absence de logs est vérifiable dans la configuration Traefik — cohérent avec la politique de transparence radicale.

**Alternatives rejetées**

* Logs complets avec anonymisation des IPs : l'anonymisation est réversible dans certaines conditions.
* Logs chiffrés accessibles uniquement en cas d'incident : crée une infrastructure de gestion des clés qui contredit la simplicité de l'architecture.



\---



## 11\. Couche application et données

### ADR-007 — Catégorisation IA avec effacement immédiat du CSV

**Statut** : Accepté | **Date** : Mars 2026 | **Domaine** : Architecture, Vie privée

**Contexte**

La catégorisation automatique des transactions bancaires nécessite l'envoi de données vers un service IA. Il faut définir exactement ce qui est envoyé et garantir que les données ne persistent pas.

**Décision**

Seuls les libellés de transactions sont envoyés au service de catégorisation. Le fichier CSV est effacé de la mémoire immédiatement après extraction des libellés, avant tout envoi.

**Séquence obligatoire**

1. Parse du CSV en mémoire uniquement.
2. Extraction des libellés dans une structure distincte.
3. Zeroing de la structure contenant le CSV complet.
4. Envoi des libellés au service IA.
5. Réception des catégories.
6. Zeroing de la liste des libellés.

**Schéma exact de la requête envoyée au service IA**

```json
{
  "labels": \["FRANPRIX", "SNCF PARIS LYON", "EDF FACTURE"],
  "version": "1.0"
}
```

Ce schéma est la seule structure autorisée. Tout autre champ constitue une violation de l'ADR-016.

**Test d'intégration obligatoire**

```rust
#\[test]
fn test\_ai\_payload\_labels\_only() {
    let csv = "date,libelle,montant\\n2026-01-15,FRANPRIX,12.50\\n2026-01-16,SNCF,45.00";
    let payload = build\_ai\_payload(csv).unwrap();
    let s = serde\_json::to\_string(\&payload).unwrap();
    assert!(s.contains("FRANPRIX"));
    assert!(!s.contains("12.50"));
    assert!(!s.contains("2026-01-15"));
}
```

**Alternatives rejetées**

* Envoi de la transaction complète : expose des données financières précises à un tiers.
* Catégorisation locale uniquement : objectif v2, requiert un modèle embarqué de qualité suffisante.



\---



### ADR-015 — Choix du modèle IA de catégorisation — génération de règles vs catégorisation directe

**Statut** : Accepté | **Date** : Mars 2026 | **Domaine** : Architecture, Vie privée

**Décision**

Génération de règles est retenue. Le modèle IA génère des règles de correspondance (libellé → catégorie) stockées localement dans le fichier .grist. La catégorisation des nouvelles transactions est effectuée localement, sans appel à l'IA.

L'IA est appelée uniquement lors de l'import initial et lors de la détection de libellés non couverts par les règles existantes.

**Justification**

* Minimise le nombre de libellés envoyés : une fois les règles générées, les transactions suivantes sont catégorisées localement.
* Les règles appartiennent à l'utilisateur et sont stockées dans son fichier .grist chiffré.
* Résilience : si le service IA est indisponible, les règles existantes continuent de fonctionner.

**Alternatives rejetées**

* Catégorisation directe à chaque transaction : volume de données envoyées plus élevé, dépendance permanente au service IA.



\---



### ADR-016 — Périmètre exact des données envoyées à l'IA

**Statut** : Accepté | **Date** : Mars 2026 | **Domaine** : Vie privée, Architecture

**Décision**

**Données autorisées**

|Champ|Autorisé|
|-|-|
|Libellé brut de transaction|✓|
|Libellé normalisé|✓|
|Version du schéma|✓|

**Données explicitement interdites**

|Champ|Interdit|
|-|-|
|Montant de la transaction|Absolu|
|Date de la transaction|Absolu|
|Solde du compte|Absolu|
|Numéro de compte|Absolu|
|Nom du titulaire|Absolu|
|IBAN|Absolu|
|Nom de l'établissement bancaire|Absolu|
|Identifiant de session|Absolu|

**Gestion des libellés contenant des montants intégrés**

Certains libellés contiennent des montants dans leur texte ("VIREMENT 500EUR LOYER"). Preprocessing obligatoire avant envoi :

```rust
fn preprocess\_label(label: \&str) -> String {
    let re = Regex::new(r"\\b\\d\[\\d\\s,.']\*\\s\*(EUR|€|CHF|USD|GBP)?\\b").unwrap();
    re.replace\_all(label, "\[MONTANT]").to\_string()
}

#\[test]
fn test\_preprocess\_strips\_amounts() {
    assert\_eq!(preprocess\_label("VIREMENT 500EUR LOYER"), "VIREMENT \[MONTANT] LOYER");
    assert\_eq!(preprocess\_label("SNCF PARIS 45.90"), "SNCF PARIS \[MONTANT]");
    assert\_eq!(preprocess\_label("FRANPRIX"), "FRANPRIX");
}
```

Le libellé préprocessé ne doit contenir aucune séquence numérique supérieure à 2 chiffres consécutifs.

**Vérification contractuelle**

Ce périmètre est contractualisé avec le fournisseur du service IA. Le contrat précise que le fournisseur s'interdit de stocker les requêtes, d'entraîner ses modèles sur les libellés reçus, et de les transmettre à des tiers. Le contrat est publié sur glyph.pm/documentation.



\---



### ADR-023 — Formats d'import acceptés — CSV, Excel, OFX, QIF

**Statut** : Accepté | **Date** : Mars 2026 | **Domaine** : Périmètre produit, Expérience utilisateur

**Décision**

Glyph supporte : CSV, Excel (XLS/XLSX), OFX, QIF. Un parseur spécifique est maintenu pour chaque banque française majeure.

**Contraintes d'implémentation obligatoires**

* Tous les parseurs opèrent en mémoire uniquement. Aucune écriture sur disque.
* Taille maximale d'un fichier importé : 10 Mo.
* Tout fichier importé est zéroïsé après extraction des données.
* Les parseurs échouent sur des fichiers malformés sans exposer le contenu dans les messages d'erreur.

**Alternatives rejetées**

* Parseur CSV générique universel : les variations entre banques produisent trop d'erreurs.
* CAMT.053 : peu exposé dans les interfaces retail françaises actuelles.
* OCR de relevés PDF : fiabilité insuffisante sans vérification manuelle.



\---



### ADR-022 — Refus de la connexion bancaire directe DSP2

**Statut** : Accepté | **Date** : Mars 2026 | **Domaine** : Périmètre produit, Sécurité

**Décision**

Glyph ne propose pas et ne proposera pas de connexion bancaire directe via DSP2 ou tout mécanisme d'agrégation automatique nécessitant de stocker des credentials bancaires ou des tokens d'accès.

**Justification**

* La connexion DSP2 repose sur des tokens d'accès qui doivent être stockés quelque part — incompatible avec ADR-001.
* Les agrégateurs DSP2 sont des tiers supplémentaires dans la chaîne de confiance.
* L'import manuel est présenté comme une feature de sécurité : l'utilisateur contrôle exactement ce qu'il partage et quand.
* L'agrément DSP2 représente une charge réglementaire incompatible avec le modèle associatif.

**Compromis assumés**

* La synchronisation automatique est le principal avantage concurrentiel des services existants. Ce segment est explicitement hors périmètre.

**Alternatives rejetées**

* DSP2 via agrégateur tiers : tiers supplémentaire + stockage de tokens.
* Option opt-in : deux niveaux de garanties dans le même produit diluent le message.



\---



## 12\. Couche gouvernance et juridique

### ADR-017 — Licence open source — AGPLv3 vs MIT

**Statut** : Accepté | **Date** : Mars 2026 | **Domaine** : Gouvernance, Juridique

**Décision**

AGPLv3 est retenue.

**Justification**

* L'AGPLv3 impose que tout dérivé de Glyph exposé comme service réseau reste open source. Un acteur commercial ne peut pas créer un fork propriétaire.
* Cohérente avec la mission : construire un outil qui ne peut pas être retourné contre les utilisateurs par un repreneur commercial.
* Compatible avec Grist (Apache 2.0) et les autres dépendances.

**Alternatives rejetées**

* MIT : permet à n'importe quel acteur commercial de créer un fork propriétaire.
* GPL v3 sans clause Affero : ne couvre pas le cas du service réseau.
* BSL : pas une licence open source au sens OSI.



\---



### ADR-018 — Juridiction et hébergement des données de l'association

**Statut** : Accepté | **Date** : Mars 2026 | **Domaine** : Gouvernance, Juridique

**Décision**

* L'association Glyph est constituée sous le régime de la loi 1901 (association française à but non lucratif).
* Les données de l'association sont hébergées exclusivement en France ou dans un État membre de l'Union Européenne.
* Les communications internes sensibles utilisent des outils auto-hébergés ou des services souverains européens (Proton Mail, Proton Drive).
* Forgejo auto-hébergé est la source de vérité du code. GitHub est utilisé pour la CI/CD et la visibilité, pas comme source de vérité.

**Justification**

* La loi 1901 offre un cadre juridique stable, non lucratif, reconnu par les institutions françaises.
* L'hébergement EU limite l'exposition au Cloud Act américain pour les données sensibles de l'organisation.

**Alternatives rejetées**

* Association de droit suisse : neutralité juridique intéressante, complexité de constitution incompatible avec les ressources actuelles.
* GitHub comme source de vérité : juridiction américaine, Cloud Act.



\---



## 13\. Références

### Documents internes Glyph

* Manifeste Glyph v1.0 — glyph.pm/manifeste
* Zero\_Data\_Architecture.md — dépôt Forgejo
* Politique de confidentialité Glyph v1.0 — glyph.pm/privacy
* Politique de responsible disclosure — glyph.pm/security
* CONTRIBUTING.md — glyph.pm/docs/contributing
* Runbook opérationnel — glyph.pm/docs/runbook

### Standards et référentiels

* RGPD Art. 25 — Protection des données dès la conception
* RGPD Art. 32 — Sécurité du traitement
* ANSSI — Guide de sécurité des architectures sans état
* ANSSI — Référentiel cryptographique (AES-256-GCM, Argon2id)
* ANSSI — EBIOS Risk Manager
* OWASP Top 10
* OWASP ASVS (Application Security Verification Standard)
* OWASP Password Storage Cheat Sheet (Argon2id m=65536, t=3, p=4)
* ISO/IEC 27001
* NIST SP 800-63B — Digital Identity Guidelines
* NIST SP 800-38D — Recommandations AES-GCM
* Password Hashing Competition — Argon2id (vainqueur 2015)
* SLSA — slsa.dev
* Sigstore — sigstore.dev

### Licences et gouvernance open source

* GNU Affero General Public License v3 — gnu.org/licenses/agpl-3.0
* Open Source Initiative — opensource.org
* Grist — Licence Apache 2.0 — github.com/gristlabs/grist-core
* Forgejo — Licence MIT — forgejo.org

### Services et infrastructure

* Cloudflare Tunnel — developers.cloudflare.com/cloudflare-one/connections/connect-networks
* Hetzner Cloud — hetzner.com
* Proxmox VE — proxmox.com
* Proton Drive — proton.me/drive
* WireGuard — wireguard.com
* Ansible — ansible.com
* Terraform — terraform.io

### Outils de sécurité

* CodeQL — codeql.github.com
* Semgrep — semgrep.dev
* OWASP ZAP — zaproxy.org
* Dependabot — docs.github.com/dependabot
* tpm2-tools — github.com/tpm2-software/tpm2-tools
* Keylime — github.com/keylime/keylime
* cosign (Sigstore) — github.com/sigstore/cosign
* syft (SBOM) — github.com/anchore/syft



\---



*Toutes les révisions de ce document sont conservées et accessibles publiquement sur glyph.pm/documentation.*

*Version 1.0 — Juin 2026*

