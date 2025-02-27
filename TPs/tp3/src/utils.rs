// This file contains utility functions for the "Le compte est bon" game
use rand::Rng;
use crate::types::*;

/// Generates all possible pairs from a vector of integers
/// Returns a vector of index pairs (i, j) where i < j
pub fn generate_pairs(numbers: &[i32]) -> Vec<(usize, usize)> {
    let mut pairs = Vec::new();
    for i in 0..numbers.len() {
        for j in (i+1)..numbers.len() {
            pairs.push((i, j));
        }
    }
    pairs
}

/// Applies an operation to two numbers if it's valid according to game rules
/// Returns Some(result) if the operation is valid, None otherwise
pub fn apply_operation(a: i32, b: i32, op: Operation) -> Option<i32> {
    match op {
        Operation::Add => Some(a + b),
        Operation::Subtract => {
            // Only allow subtraction if result is positive
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
            // Only allow division if result is an integer
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

/// Generates a random set of 6 plaques according to game rules
pub fn random_plaques() -> Vec<i32> {
    let mut rng = rand::rng();
    let mut available = Vec::new();

    // Add the 20 plaques numbered from 1 to 10 (2 of each)
    for i in 1..=10 {
        available.push(i);
        available.push(i);
    }

    // Add the special plaques
    available.extend_from_slice(&[25, 25, 50, 50, 75, 75, 100, 100]);

    // Shuffle the available plaques
    for _ in 0..1000 {
        let i = rng.random_range(0..available.len());
        let j = rng.random_range(0..available.len());
        available.swap(i, j);
    }

    // Take the first 6
    available[0..6].to_vec()
}

/// Generates a random target between 100 and 999
pub fn random_target() -> i32 {
    let mut rng = rand::rng();
    rng.random_range(100..1000)
}