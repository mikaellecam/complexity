# TPs d'Algorithmique en Rust

Ce dépôt contient une collection de TPs d'algorithmique implémentés en Rust et Python.

## Prérequis

### Rust et Cargo
Si vous n'avez pas encore installé Rust et Cargo, suivez ces étapes :

1. Visitez [rustup.rs](https://rustup.rs/) ou exécutez la commande suivante :
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Suivez les instructions d'installation (généralement l'option 1 par défaut)

3. Une fois l'installation terminée, redémarrez votre terminal ou exécutez :
   ```bash
   source $HOME/.cargo/env
   ```

4. Vérifiez que Rust et Cargo sont correctement installés :
   ```bash
   rustc --version
   cargo --version
   ```

### Python (pour les graphiques)
Pour exécuter les visualisations graphiques, vous aurez besoin de Python :

1. Installez Python (3.x)

2. Installez les dépendances Python nécessaires :
   ```bash
   pip install matplotlib numpy
   ```

## TP 1 : Hanoi, Fibonacci, et Crible d'Ératosthène

### Compilation et Exécution

Pour compiler et exécuter le programme :

```bash
# Exécuter avec une option spécifique
cargo run <option>
```

### Options disponibles

- `1` : Exécuter le programme des Tours de Hanoï
- `2` : Exécuter le programme de Fibonacci récursif
- `3` : Exécuter le programme du Crible d'Ératosthène
- `help` : Afficher l'aide

Exemples :
```bash
# Exécuter les Tours de Hanoï
cargo run 1

# Exécuter le programme Fibonacci
cargo run 2
```

## TP 2 : Algorithmes de Tri

Le TP 2 concerne l'analyse de performance des algorithmes de tri (sélection, insertion, bulles, fusion).

### Exécution

```bash
cargo run
```

### Visualisation des Résultats

Après l'exécution du programme Rust, un fichier de sortie `sorting_performance.png` sera généré.

## TP 3 : "Le compte est bon"

Le TP 3 implémente un solveur pour le jeu "Le compte est bon".

### Exécution

```bash
cargo run
```

Le programme effectuera :
1. Une analyse du nombre de sets examinés dans le pire cas
2. Une analyse de la complexité de l'algorithme
3. Une mesure du temps d'exécution moyen
4. Un exemple de résolution


### Remarque Performance

Pour améliorer la vitesse d'exécution des programmes, il est possible d'ajouter un paramètre dans la commande :

```bash
cargo run --release
```

Ceci permet de faire une compilation optimisée pour la performance et de réduire le temps d'exécution.

Pour effectuer le rapport, nous n'avons pas utilisé cette commande.