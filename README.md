Architecture, organisation, structure minimale d'un projet (racine)

| Dossier                        | Proposition / Exemples                                                                   |
| ------------------------------ | ---------------------------------------------------------------------------------------- |
| apps/                          | Applications liées à notre projet                                                        |
| apps/{app}/bin/                | Différents points d'entrées d'une app                                                    |
| apps/{app}/src/                | Fichier source, code métier                                                              |
| apps/{app}/`app.{ext}`         | app.ts, app.rs, app.php, ...                                                             |
| apps/{app}/`{pollution-files}` | ex: .gitignore, .eslint, rome.json, clippy.toml, phpstan.neon, ...                       |
|                                |                                                                                          |
| assets/                        | Ressources statiques partagées des applications                                          |
| assets/{dir}/{file}            | ex: audio/file.mp3, img/file.png, svg/icon.svg, ...                                      |
|                                |                                                                                          |
| audits/                        | Audits, pentests, review, rapports de sécurité                                           |
| audits/{timestamp}/{file}      |                                                                                          |
|                                |                                                                                          |
| config/                        | Configurations des applications                                                          |
| config/shared                  | Contient les fichiers de configurations applicatifs partagées (ex: certificats ssl, ...) |
| config/{app}/{app}.{ext}       | Configuration principale d'une application                                               |
| config/{app}/{name}.{ext}      | Configurations secondaires d'une application                                             |
|                                |                                                                                          |
| database/                      | Base de données des applications                                                         |
|                                | ex: extensions, enums, migrations, seeders, fixtures, ...                                |
|                                |                                                                                          |
| docs/                          | Documentation du projet, ADR, Qualité de code, Capture d'écrans                          |
| docs/apps/{app}/README.md      | Documentation d'une application                                                          |
|                                |                                                                                          |
| env/                           | Variables d'environnement des applications                                               |
| env/{app}/{env-file}           | ex: editor/.env, editor/.env.local, mail/.env.vault                                      |
|                                |                                                                                          |
| examples/                      | Des fichiers d'exemples                                                                  |
|                                |                                                                                          |
| i18n                           | Internationalisation, locales                                                            |
|                                |                                                                                          |
| infra/                         | Containérisation, orchestrateur, CI/CD, configurations toolings, monitoring, ...         |
| infra/{tool}/{conf-file}       | ex: docker, k8s, ansible, puppet                                                         |
|                                |                                                                                          |
| packages/                      | Code partagé par nos applications                                                        |
| packages/patches/{module}      | Correctifs de certaines dépendances externes                                             |
| packages/third-party/{module}  | Dépendances externes (non versionnées)?                                                  |
| packages/{manager}/{module}    | Les modules internes en fonction de l'écosystème (npm, composer, crates, gems, ...)      |
|                                |                                                                                          |
| supports                       | Communications, discussions                                                              |
|                                |                                                                                          |
| tests/                         | Les tests fonctionnels des applications                                                  |
| tests/{app}/...                |                                                                                          |
|                                |                                                                                          |
| tmp/                           | Les fichiers temporaires générés par les applications                                    |
| tmp/{dir}/...                  | ex: des logs, des sessions (php), ...                                                    |
|                                |                                                                                          |
| tools/                         | Outils internes pour améliorer la dx                                                     |
| tools/{dir}                    | ex: ./tools/check-license, ./tools/generate-docs, ./tools/vscode, ...                    |
|                                |                                                                                          |
| www/                           | Sites webs                                                                               |
| www/{site}                     |                                                                                          |
