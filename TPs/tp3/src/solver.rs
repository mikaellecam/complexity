// Ce fichier implémente l'algorithme d'exploration récursive pour résoudre le jeu
use crate::types::*;
use crate::utils::*;

/// Tente de trouver une solution pour atteindre la cible avec les nombres donnés
/// Retourne Some(Solution) si une solution est trouvée, None sinon
pub fn solve(numbers: &[i32], target: i32) -> Option<Solution> {
    // Vérifie d'abord si la cible est déjà dans les nombres
    if numbers.contains(&target) {
        return Some(Solution {
            steps: Vec::new(),
            target,
        });
    }
    // Pour stocker les étapes de la solution
    let mut steps = Vec::new();
    // Commence l'exploration
    if explore(&numbers.to_vec(), target, &mut steps) {
        Some(Solution { steps, target })
    } else {
        None
    }
}

/// Explore récursivement toutes les opérations possibles pour atteindre la cible
/// Retourne true si une solution est trouvée, false sinon
/// Le vecteur steps contiendra la solution si trouvée
fn explore(numbers: &Vec<i32>, target: i32, steps: &mut Vec<CalculationStep>) -> bool {
    // Cas de base : S'il n'y a qu'un seul nombre et qu'il est égal à la cible, nous avons trouvé une solution
    if numbers.len() == 1 && numbers[0] == target {
        return true;
    }
    // Génère toutes les paires d'indices
    let pairs = generate_pairs(&numbers);
    // Essaie toutes les paires de nombres
    for (i, j) in pairs {
        let a = numbers[i];
        let b = numbers[j];
        // Essaie toutes les opérations
        for &op in &[Operation::Add, Operation::Subtract, Operation::Multiply, Operation::Divide] {
            // Applique l'opération si valide
            if let Some(result) = apply_operation(a, b, op) {
                // Crée un nouvel ensemble de nombres avec le résultat remplaçant les opérandes
                let mut new_numbers = numbers.clone();
                new_numbers.remove(j); // Supprime d'abord l'indice le plus grand
                new_numbers.remove(i); // Puis l'indice le plus petit
                new_numbers.push(result);
                // Enregistre cette étape
                let step = CalculationStep {
                    left: a,
                    right: b,
                    operation: op,
                    result,
                };
                steps.push(step);
                // Vérifie si nous avons atteint la cible
                if result == target || explore(&new_numbers, target, steps) {
                    return true;
                }
                // Revient en arrière si ce chemin ne mène pas à une solution
                steps.pop();
            }
        }
    }
    // Aucune solution trouvée avec les nombres actuels
    false
}