// This file provides theoretical analysis of the algorithm complexity

/// Calculates the theoretical number of sets examined in worst case
pub fn calculate_theoretical_complexity(n: u32) -> u64 {
    // For n plaques, we can create n(n-1)/2 pairs
    // For each pair, we can apply up to 4 operations
    // This process continues for n-1 steps (until we have 1 number)

    // Calculate factorial: n!
    fn factorial(n: u32) -> u64 {
        if n <= 1 {
            1
        } else {
            n as u64 * factorial(n - 1)
        }
    }

    // Calculate 4^(n-1) * n! / 2^(n-1)
    // This is a rough approximation of the worst-case complexity
    let four_power = 4u64.pow(n - 1);
    let fact = factorial(n);
    let two_power = 2u64.pow(n - 1);

    four_power * fact / two_power
}

/// Calculates binomial coefficient C(n,k)
fn binomial(n: u32, k: u32) -> u64 {
    if k > n {
        return 0;
    }

    let k = k.min(n - k);
    let mut result = 1;

    for i in 0..k {
        result = result * (n - i) as u64 / (i + 1) as u64;
    }

    result
}

/// Calculates a simplified worst-case complexity estimate
pub fn simplified_complexity(n: u32) -> u64 {
    // A simpler approximation: O(n² × 2^n)
    let n_squared = (n * n) as u64;
    let two_power_n = 1u64 << n;  // 2^n

    n_squared * two_power_n
}

/// Prints a complexity analysis for different values of n
pub fn print_complexity_analysis() {
    println!("Complexity Analysis:");
    println!("------------------");
    println!("n | Theoretical Operations | Simplified Estimate");
    println!("------------------");

    for n in 2..=6 {
        let theoretical = calculate_theoretical_complexity(n);
        let simplified = simplified_complexity(n);
        println!("{} | {:20} | {:20}", n, theoretical, simplified);
    }

    // For n=6 (the standard game):
    let n = 6;
    println!("\nFor the standard game with n={}:", n);
    println!("Number of sets examined in worst case: {}", calculate_theoretical_complexity(n));

    // Explaining the complexity class
    println!("\nAlgorithm complexity analysis:");
    println!("- The algorithm has exponential complexity");
    println!("- For each step, we consider O(n²) pairs with 4 operations");
    println!("- We make n-1 steps to reduce from n numbers to 1");
    println!("- Overall complexity: O(n² × 2^n)");
}