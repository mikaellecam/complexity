// This file implements the recursive exploration algorithm for solving the game
use crate::types::*;
use crate::utils::*;

/// Attempts to find a solution for reaching the target with the given numbers
/// Returns Some(Solution) if a solution is found, None otherwise
pub fn solve(numbers: &[i32], target: i32) -> Option<Solution> {
    // First check if the target is already in the numbers
    if numbers.contains(&target) {
        return Some(Solution {
            steps: Vec::new(),
            target,
        });
    }

    // For storing the solution steps
    let mut steps = Vec::new();

    // Start the exploration
    if explore(&numbers.to_vec(), target, &mut steps) {
        Some(Solution { steps, target })
    } else {
        None
    }
}

/// Recursively explores all possible operations to reach the target
/// Returns true if a solution is found, false otherwise
/// The steps vector will contain the solution if found
fn explore(numbers: &Vec<i32>, target: i32, steps: &mut Vec<CalculationStep>) -> bool {
    // Base case: If there's only one number and it equals the target, we've found a solution
    if numbers.len() == 1 && numbers[0] == target {
        return true;
    }

    // Generate all pairs of indices
    let pairs = generate_pairs(&numbers);

    // Try all pairs of numbers
    for (i, j) in pairs {
        let a = numbers[i];
        let b = numbers[j];

        // Try all operations
        for &op in &[Operation::Add, Operation::Subtract, Operation::Multiply, Operation::Divide] {
            // Apply the operation if valid
            if let Some(result) = apply_operation(a, b, op) {
                // Create a new set of numbers with the result replacing the operands
                let mut new_numbers = numbers.clone();
                new_numbers.remove(j); // Remove the larger index first
                new_numbers.remove(i); // Then the smaller index
                new_numbers.push(result);

                // Record this step
                let step = CalculationStep {
                    left: a,
                    right: b,
                    operation: op,
                    result,
                };
                steps.push(step);

                // Check if we've reached the target
                if result == target || explore(&new_numbers, target, steps) {
                    return true;
                }

                // Backtrack if this path doesn't lead to a solution
                steps.pop();
            }
        }
    }

    // No solution found with the current numbers
    false
}