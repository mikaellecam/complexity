// This file implements the game logic for "Le compte est bon"
use std::time::Instant;
use crate::types::*;
use crate::utils::*;
use crate::solver::*;

/// Represents a game instance with plaques and a target
pub struct Game {
    pub plaques: Vec<i32>,
    pub target: i32,
}

impl Game {
    /// Creates a new game with random plaques and target
    pub fn new_random() -> Self {
        Self {
            plaques: random_plaques(),
            target: random_target(),
        }
    }

    /// Creates a new game with specific plaques and target
    pub fn new(plaques: Vec<i32>, target: i32) -> Self {
        Self { plaques, target }
    }

    /// Solves the current game
    /// Returns the solution if found
    pub fn solve(&self) -> Option<Solution> {
        let start = Instant::now();
        let solution = solve(&self.plaques, self.target);
        let duration = start.elapsed();

        println!("Solution search completed in {:?}", duration);

        solution
    }

    /// Analyzes the algorithm by solving musolveltiple random games
    /// Useful for measuring average solution time
    pub fn analyze_algorithm(num_tests: usize) {
        let mut total_duration = std::time::Duration::new(0, 0);
        let mut solved_count = 0;

        for i in 0..num_tests {
            let game = Game::new_random();
            println!("Test {}/{}: Target = {}, Plaques = {:?}", i+1, num_tests, game.target, game.plaques);

            let start = Instant::now();
            let solution = game.solve();
            let duration = start.elapsed();

            if solution.is_some() {
                solved_count += 1;
                println!("Solution found in {:?}", duration);
                println!("{}", solution.unwrap());
            } else {
                println!("No solution found in {:?}", duration);
            }

            total_duration += duration;
            println!();
        }

        let avg_duration = total_duration / num_tests as u32;
        println!("Solved {}/{} games", solved_count, num_tests);
        println!("Average solution time: {:?}", avg_duration);
    }

    /// Analyzes algorithm complexity by measuring solution time for different numbers of plaques
    pub fn analyze_complexity() {
        for n in 2..=6 {
            let mut total_duration = std::time::Duration::new(0, 0);
            let num_tests = 5;

            for _ in 0..num_tests {
                let plaques = random_plaques()[0..n].to_vec();
                let target = random_target();

                let game = Game::new(plaques.clone(), target);
                println!("Testing with {} plaques: Target = {}, Plaques = {:?}", n, target, plaques);

                let start = Instant::now();
                let _ = game.solve();
                let duration = start.elapsed();

                total_duration += duration;
            }

            let avg_duration = total_duration / num_tests as u32;
            println!("Average time for {} plaques: {:?}", n, avg_duration);
        }
    }
}