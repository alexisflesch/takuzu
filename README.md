# Takuzu — Application Angular + Générateur Rust 🇫🇷

## Jouer en ligne

[Takuzu](https://takuzu.alexisfles.ch)

## Description ✨
Takuzu est une application web pour jouer au Takuzu (jeu binaire). L'UI est une application Angular et les grilles jouables sont générées par un générateur écrit en Rust par Claude Opus 4.5 (dans `grid-generator/`). Le format JSON produit est compatible avec l'app et utilise :
- `grid`: matrice de cases (-1 = case vide, 0 ou 1 = valeur connue)
- `solution`: matrice complète avec la solution (0/1)
- `id`: identifiant de la grille

Exemple d'URL partageable : `/play?size=6&difficulty=1&index=23` (charge une grille précise).

---

## Prérequis 🛠️
- Node.js et npm (utilisez la version compatible avec Angular 14, p.ex. Node 16+)
- Angular CLI (optionnel pour le dev) : `npm i -g @angular/cli`
- Rust toolchain (pour le générateur) : `rustup` / `cargo`
- (Optionnel) nginx pour servir la version production

---

## Installation (développement) 🚀
1. Installer les dépendances frontend :

```bash
npm install
```

2. Lancer le serveur de développement :

```bash
npm start
# ouvrir http://localhost:4200/
```

---

## Générer des grilles (Rust) 🧩
1. Installer Rust si nécessaire :

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

2. Compiler le générateur :

```bash
cd grid-generator
cargo build --release
```

3. Générer des grilles (exemple) :

```bash
./target/release/takuzu-gen --size 6 --count 50 --difficulty 1 -o ./grids/
```
Le générateur écrira des fichiers `takuzu_${size}x${size}_d${difficulty}.json` dans `grid-generator/grids/`.

4. Copier les grilles dans l'app Angular (pour servir depuis `assets`) :

```bash
cp grid-generator/grids/*.json src/assets/grids/
```

Note : le format JSON utilise `-1` pour les cases vides afin d'être directement utilisable par l'application.

---

## Build production & déploiement 📦
1. Construire l'app Angular :

```bash
npm run build
# artefacts dans dist/takuzu-angular/
```

---

## Licence 🔐

- **Application Angular** (`src/` et `dist/`) : **GNU GPL v3 (ou ultérieure)**. Voir `src/LICENSE-GPL-3.0.txt`.
- **Générateur de grilles** (`grid-generator/`) : **MIT License**. Voir `grid-generator/LICENSE`.
