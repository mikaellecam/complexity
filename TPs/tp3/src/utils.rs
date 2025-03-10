// Ce fichier contient des fonctions utilitaires pour le jeu
use rand::Rng;
use crate::types::*;

/// Génère toutes les paires possibles à partir d'un vecteur d'entiers
/// Retourne un vecteur de paires d'indices (i, j) où i < j
pub fn generate_pairs(numbers: &[i32]) -> Vec<(usize, usize)> {
    let mut pairs = Vec::new();
    for i in 0..numbers.len() {
        for j in (i+1)..numbers.len() {
            pairs.push((i, j));
        }
    }
    pairs
}

/// Applique une opération à deux nombres si elle est valide selon les règles du jeu
/// Retourne Some(résultat) si l'opération est valide, None sinon
pub fn apply_operation(a: i32, b: i32, op: Operation) -> Option<i32> {
    match op {
        Operation::Add => Some(a + b),
        Operation::Subtract => {
            // Autorise la soustraction seulement si le résultat est positif
            if a >= b {
                Some(a - b)
            } else if b > a {
                Some(b - a)
            } else {
                None
            }
        },
        Operation::Multiply => Some(a * b),
        Operation::Divide => {
            // Autorise la division seulement si le résultat est un entier
            if b != 0 && a % b == 0 {
                Some(a / b)
            } else if a != 0 && b % a == 0 {
                Some(b / a)
            } else {
                None
            }
        },
    }
}

/// Génère un ensemble aléatoire de 6 plaques selon les règles du jeu
pub fn random_plaques() -> Vec<i32> {
    let mut rng = rand::rng();
    let mut available = Vec::new();

    // Ajoute les 20 plaques numérotées de 1 à 10 (2 de chaque)
    for i in 1..=10 {
        available.push(i);
        available.push(i);
    }

    // Ajoute les plaques spéciales
    available.extend_from_slice(&[25, 25, 50, 50, 75, 75, 100, 100]);

    // Mélange les plaques disponibles
    for _ in 0..1000 {
        let i = rng.random_range(0..available.len());
        let j = rng.random_range(0..available.len());
        available.swap(i, j);
    }

    // Prend les 6 premières
    available[0..6].to_vec()
}

/// Génère une cible aléatoire entre 100 et 999
pub fn random_target() -> i32 {
    let mut rng = rand::rng();
    rng.random_range(100..1000)
}