use std::time::Instant;

pub fn main(){
    let n = 100; // Prime numbers under 100
    let start_small = Instant::now();
    let primes = sieve_of_eratosthenes(n);
    let duration_small = start_small.elapsed();

    println!("\nFound {} prime numbers up to {} in {:?}",
             primes.len(), n, duration_small);

    let large_n = 1_000_000;
    let start = Instant::now();
    let large_primes = sieve_of_eratosthenes(large_n);
    let duration = start.elapsed();

    println!("\nFound {} prime numbers up to {} in {:?}",
             large_primes.len(), large_n, duration);
}


fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    if n <= 1 {
        return Vec::new();
    }

    let mut is_marked = vec![false; n + 1];

    is_marked[1] = true;

    let mut primes = Vec::new();

    for p in 2..=n {
        if !is_marked[p] {
            primes.push(p);

            let mut multiple = p * p;
            while multiple <= n {
                is_marked[multiple] = true;
                multiple += p;
            }
        }
    }
    primes
}

/*fn analyze_complexity(n: usize) -> String {
    let complexity = format!(
        "Time Complexity: O(N log log N) where N = {}\n\
         This is because each number is visited approximately log log N times on average.\n\
         \n\
         Space Complexity: O(N) for the boolean array of size N.\n\
         \n\
         For N = {}, the algorithm performs significantly fewer operations than N²,\n\
         making it more efficient than trial division for finding all primes up to N.",
        n, n
    );
    complexity
}*/
