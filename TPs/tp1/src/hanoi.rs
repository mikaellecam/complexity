use std::time::Duration;
use std::time::Instant;

pub(crate) fn main() {
    let disk_numbers: Vec<f64> = vec![1.0, 2.0, 5.0, 10.0, 15.0, 25.0, 50.0];

    let results = hanoi_towers_benchmark(&disk_numbers);

    let seconds: Vec<f64> = results.iter().map(|d| d.as_secs_f64()).collect();

    for (i, &n) in disk_numbers.iter().enumerate() {
        println!("Disk number: {}, Time: {} seconds", n, seconds[i]);
    }
}

fn hanoi_towers_benchmark(array: &Vec<f64>) -> Vec<Duration> {
    let mut results = Vec::new();
    for &n in array {
        let now = Instant::now();

        hanoi_towers(n as i32, 1, 3, 2);

        let elapsed = now.elapsed();
        results.push(elapsed);
    }
    results
}

fn hanoi_towers(n: i32, source: i32, destination: i32, middle: i32) {
    if n == 1 {
        //println!("Move disk 1 from source {} to destination {}", source, destination);
        return;
    }
    hanoi_towers(n - 1, source, middle, destination);
    //println!("Move disk {} from source {} to destination {}", n, source, destination);
    hanoi_towers(n - 1, middle, destination, source);
}