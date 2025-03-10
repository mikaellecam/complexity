mod types;
mod utils;
mod solver;
mod game;

use crate::game::Game;
use std::time::Instant;
use std::time::Duration;

// Calcule le nombre de sets examinés dans le pire cas pour la question 1
fn calculate_sets_examined(n: u32) -> u64 {
    // Pour n plaques, nous pouvons créer n(n-1)/2 paires
    // Pour chaque paire, nous pouvons appliquer jusqu'à 4 opérations
    // Ce processus continue récursivement jusqu'à ce qu'il ne reste qu'un seul nombre

    // Calcul factoriel: n!
    fn factorial(n: u32) -> u64 {
        if n <= 1 {
            1
        } else {
            n as u64 * factorial(n - 1)
        }
    }

    // Une formule simplifiée pour le pire cas: 4^(n-1) * n! / 2^(n-1)
    let four_power = 4u64.pow(n - 1);
    let fact = factorial(n);
    let two_power = 2u64.pow(n - 1);

    four_power * fact / two_power
}

// Fonction pour mesurer le temps d'exécution moyen pour la question 3
fn measure_average_execution_time(iterations: usize) -> Duration {
    let mut total_duration = Duration::new(0, 0);
    let mut solved_count = 0;

    println!("Exécution de {} itérations pour mesurer le temps d'exécution moyen...", iterations);

    for i in 0..iterations {
        let game = Game::new_random();

        let start = Instant::now();
        let solution = game.solve();
        let duration = start.elapsed();

        if solution.is_some() {
            solved_count += 1;
        }

        total_duration += duration;

        // Indicateur de progression
        if (i + 1) % 5 == 0 {
            println!("Complété {}/{} itérations", i + 1, iterations);
        }
    }

    println!("Résolu {}/{} jeux", solved_count, iterations);
    total_duration / iterations as u32
}

fn main() {
    // Question 1: Calculer le nombre de sets examinés par l'algorithme dans le pire cas
    println!("\n----- Question 1: Nombre de Sets Examinés dans le Pire Cas -----");
    println!("n | Nombre de Sets Examinés");
    println!("---------------------------");

    for n in 2..=7 {
        let sets = calculate_sets_examined(n);
        println!("{} | {}", n, sets);
    }

    // Focus spécial sur n=6 (jeu standard)
    let sets_for_n6 = calculate_sets_examined(6);
    println!("\nPour le jeu standard avec n=6:");
    println!("Nombre de sets examinés dans le pire cas: {}", sets_for_n6);

    // Question 2: Déterminer la complexité de l'algorithme d'exploration
    println!("\n----- Question 2: Analyse de la Complexité de l'Algorithme -----");
    println!("L'algorithme pour résoudre 'Le compte est bon' a une complexité exponentielle.");
    println!("Pour n plaques, la complexité dans le pire cas est approximativement O(4^(n-1) * n!/(2^(n-1))).");

    println!("\nPourquoi est-ce exponentiel?");
    println!("1. Pour chaque étape, nous considérons toutes les paires possibles de nombres: O(n²)");
    println!("2. Pour chaque paire, nous essayons jusqu'à 4 opérations: O(4)");
    println!("3. Chaque opération crée un nouvel ensemble avec n-1 nombres");
    println!("4. Le processus se répète récursivement jusqu'à ce qu'il ne reste qu'un seul nombre");
    println!("5. L'arbre de récursion a une profondeur de n-1 et un facteur de branchement élevé");

    println!("\nClasse de complexité simplifiée: O(n² × 4^n)");

    // Question 3: Déterminer le temps d'exécution moyen
    println!("\n----- Question 3: Temps d'Exécution Moyen -----");

    // Nombre d'itérations pour la moyenne (ajuster selon les besoins)
    let iterations = 30;

    // Mesurer le temps d'exécution moyens
    let avg_duration = measure_average_execution_time(iterations);
    println!("\nTemps d'exécution moyen sur {} itérations: {:?}", iterations, avg_duration);
}


/*
Question bonus : Ce problème est-il NP-complet ?

Oui, le problème "Le compte est bon" est NP-complet.

1. Le problème est dans NP :
   - Une solution proposée peut être vérifiée en temps polynomial
   - Pour vérifier une solution, il suffit de s'assurer que :
     a) Chaque plaque n'est utilisée qu'une seule fois
     b) Les opérations respectent les contraintes (pas de nombres négatifs, pas de divisions avec reste)
     c) Le résultat final correspond à la cible
   - Cette vérification s'effectue en O(n) où n est le nombre de plaques.

2. Le problème est NP-difficile :
   - Le problème peut être vu comme une généralisation du problème de la somme de sous-ensembles
     (Subset Sum Problem) qui est NP-complet
   - Avec les quatre opérations, nous avons un cas spécial du problème de construction d'expressions
     arithmétiques, qui est également NP-complet
   - La difficulté augmente avec le nombre de plaques et d'opérations possibles.

3. Preuve informelle de NP-difficulté :
   - Notre algorithme a une complexité exponentielle dans le pire cas
   - L'espace de recherche croît exponentiellement avec le nombre de plaques
   - Aucun algorithme polynomial connu ne peut résoudre ce problème dans le cas général

4. Comparaison avec d'autres problèmes NP-complets :
   - Comme pour le problème du voyageur de commerce ou le problème du sac à dos,
     nous ne pouvons pas faire mieux qu'une exploration exhaustive optimisée
   - Les heuristiques peuvent améliorer les performances dans la pratique, mais
     n'éliminent pas la complexité exponentielle dans le pire cas

Conclusion :
Le problème "Le compte est bon" satisfait les critères de NP-complétude :
il est à la fois dans NP et NP-difficile, ce qui confirme qu'il est NP-complet.
*/