// Ce fichier implémente la logique du jeu
use std::time::Instant;
use crate::types::*;
use crate::utils::*;
use crate::solver::*;

/// Représente une instance de jeu avec des plaques et une cible
pub struct Game {
    pub plaques: Vec<i32>,
    pub target: i32,
}

impl Game {
    /// Crée un nouveau jeu avec des plaques et une cible aléatoires
    pub fn new_random() -> Self {
        Self {
            plaques: random_plaques(),
            target: random_target(),
        }
    }

    /// Résout le jeu actuel
    /// Retourne la solution si trouvée
    pub fn solve(&self) -> Option<Solution> {
        let start = Instant::now();
        let solution = solve(&self.plaques, self.target);
        let duration = start.elapsed();

        println!("Recherche de solution terminée en {:?}", duration);

        solution
    }
}