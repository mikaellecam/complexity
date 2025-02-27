// Main entry point for the "Le compte est bon" solver
mod types;
mod utils;
mod solver;
mod game;
mod analysis;

use crate::game::Game;
use std::time::Instant;
use std::time::Duration;

// Function to calculate the number of sets examined in worst case for question 1
fn calculate_sets_examined(n: u32) -> u64 {
    // For n plaques, we can create n(n-1)/2 pairs
    // For each pair, we can apply up to 4 operations
    // This process continues recursively until we have only one number

    // Calculate factorial: n!
    fn factorial(n: u32) -> u64 {
        if n <= 1 {
            1
        } else {
            n as u64 * factorial(n - 1)
        }
    }

    // A simplified formula for the worst-case: 4^(n-1) * n! / 2^(n-1)
    let four_power = 4u64.pow(n - 1);
    let fact = factorial(n);
    let two_power = 2u64.pow(n - 1);

    four_power * fact / two_power
}

// Function to measure average execution time for question 3
fn measure_average_execution_time(iterations: usize) -> Duration {
    let mut total_duration = Duration::new(0, 0);
    let mut solved_count = 0;

    println!("Running {} iterations to measure average execution time...", iterations);

    for i in 0..iterations {
        let game = Game::new_random();

        let start = Instant::now();
        let solution = game.solve();
        let duration = start.elapsed();

        if solution.is_some() {
            solved_count += 1;
        }

        total_duration += duration;

        // Progress indicator
        if (i + 1) % 5 == 0 {
            println!("Completed {}/{} iterations", i + 1, iterations);
        }
    }

    println!("Solved {}/{} games", solved_count, iterations);
    total_duration / iterations as u32
}

fn main() {
    // Question 1: Calculate the number of sets examined by the algorithm in the worst case
    println!("\n----- Question 1: Number of Sets Examined in Worst Case -----");
    println!("n | Number of Sets Examined");
    println!("---------------------------");

    for n in 2..=7 {
        let sets = calculate_sets_examined(n);
        println!("{} | {}", n, sets);
    }

    // Special focus on n=6 (standard game)
    let sets_for_n6 = calculate_sets_examined(6);
    println!("\nFor the standard game with n=6:");
    println!("Number of sets examined in worst case: {}", sets_for_n6);

    // Question 2: Determine the complexity of the exploration algorithm
    println!("\n----- Question 2: Algorithm Complexity Analysis -----");
    println!("The algorithm for solving 'Le compte est bon' has exponential complexity.");
    println!("For n plaques, the worst-case complexity is approximately O(4^(n-1) * n!/(2^(n-1))).");

    println!("\nWhy is it exponential?");
    println!("1. For each step, we consider all possible pairs of numbers: O(n²)");
    println!("2. For each pair, we try up to 4 operations: O(4)");
    println!("3. Each operation creates a new set with n-1 numbers");
    println!("4. The process repeats recursively until we have only one number");
    println!("5. The recursion tree has a depth of n-1 and a high branching factor");

    println!("\nSimplified complexity class: O(n² × 4^n)");

    // Question 3: Determine the average execution time
    println!("\n----- Question 3: Average Execution Time -----");

    // Number of iterations for averaging (adjust as needed)
    let iterations = 30;

    // Measure average execution time
    let avg_duration = measure_average_execution_time(iterations);
    println!("\nAverage execution time over {} iterations: {:?}", iterations, avg_duration);

    // Optional: Run a simple example from the assignment for verification
    println!("\n----- Example from the assignment -----");
    let example = Game::new(vec![2, 10, 100], 120);
    println!("Target: {}", example.target);
    println!("Plaques: {:?}", example.plaques);

    match example.solve() {
        Some(solution) => println!("{}", solution),
        None => println!("No solution found!"),
    }
}


/*
Question bonus : Ce problème est-il NP-complet ?

Oui, le problème "Le compte est bon" est NP-complet. Voici l'analyse :

1. Le problème est dans NP :
   - Une solution proposée peut être vérifiée en temps polynomial
   - Pour vérifier une solution, il suffit de s'assurer que :
     a) Chaque plaque n'est utilisée qu'une seule fois
     b) Les opérations respectent les contraintes (pas de nombres négatifs, pas de divisions avec reste)
     c) Le résultat final correspond à la cible
   - Cette vérification s'effectue en O(n) où n est le nombre de plaques

2. Le problème est NP-difficile :
   - Le problème peut être vu comme une généralisation du problème de la somme de sous-ensembles
     (Subset Sum Problem) qui est NP-complet
   - Avec les quatre opérations, nous avons un cas spécial du problème de construction d'expressions
     arithmétiques, qui est également NP-complet
   - La difficulté augmente avec le nombre de plaques et d'opérations possibles

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

// This analysis helps explain why our algorithm has exponential complexity
// and why it's unlikely that we could find a significantly more efficient algorithm
// for the general case of this problem.