use std::time::Duration;
use std::time::Instant;

pub(crate) fn main() {
    println!("----------------------------");
    println!("Fibonacci Sequence Calculator (with u_0 = u_1 = 1)");
    println!("----------------------------");

    let test_values = [10, 20, 30, 40, 45];

    // Test de la solution itérative
    println!("\nIterative Solution:");
    for &n in &test_values {
        let duration = time_execution(fibonacci_iterative, n);
        println!("Time for n={}: {:?}", n, duration);
    }

    // Test de la solution récursive (on prend des valeurs plus petites, car l'algo est très peu efficace)
    println!("\nRecursive Solution:");
    for &n in &test_values {
        let duration = time_execution(fibonacci_recursive, n);
        println!("Time for n={}: {:?}", n, duration);
    }

    // Test logarithmic solution
    println!("\nLogarithmic Solution:");
    for &n in &test_values {
        let duration = time_execution(fibonacci_logarithmic, n);
        println!("Time for n={}: {:?}", n, duration);
    }

    // Test larger values with logarithmic solution only
    println!("\nLogarithmic Solution with Larger Values:");
    let large_values = [100]; //500, 1000]; // With 500 & 1000 values, we hit the u128 limit
    for &n in &large_values {
        let duration = time_execution(fibonacci_logarithmic, n);
        println!("Time for n={}: {:?}", n, duration);
    }
}


fn fibonacci_iterative(n: u64) -> u128 {
    if n == 0 || n == 1 {
        return 1;
    }

    let mut a = 1u128; // u_{n-2}
    let mut b = 1u128; // u_{n-1}
    let mut result = 0u128;

    for _ in 2..=n {
        result = a + b;
        a = b;
        b = result;
    }

    result
}


fn fibonacci_recursive(n: u64) -> u128 {
    if n == 0 || n == 1 {
        return 1;
    }

    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}

fn fibonacci_logarithmic(n: u64) -> u128 {
    if n == 0 || n == 1 {
        return 1;
    }

    // For computing the nth term with matrix exponentiation
    let mut matrix = [[1u128, 1u128], [1u128, 0u128]];
    let mut result = [[1u128, 0u128], [0u128, 1u128]]; // Identity matrix
    let mut power = n - 1;

    // Compute matrix^(n-1) using binary exponentiation
    while power > 0 {
        if power % 2 == 1 {
            result = matrix_multiply(result, matrix);
        }
        matrix = matrix_multiply(matrix, matrix);
        power /= 2;
    }

    // Apply the result matrix to the initial values [u_1, u_0] = [1, 1]
    result[0][0] * 1 + result[0][1] * 1
}

fn matrix_multiply(a: [[u128; 2]; 2], b: [[u128; 2]; 2]) -> [[u128; 2]; 2] {
    let mut result = [[0u128, 0u128], [0u128, 0u128]];

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    result
}

fn time_execution<F>(f: F, n: u64) -> Duration
where
    F: Fn(u64) -> u128,
{
    let start = Instant::now();
    f(n); //let result = f(n);
    let duration = start.elapsed();
    //println!("Result for n={}: {}", n, result);

    duration
}
