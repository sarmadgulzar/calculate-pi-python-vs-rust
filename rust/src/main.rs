use std::time::Instant;

const N: u64 = 100_000_000;

fn calculate_pi(n_terms: u64) -> f64 {
    let numerator = 4.0;
    let mut denominator = 1.0;
    let mut operation = 1.0;
    let mut pi = 0.0;
    for _ in 0..n_terms {
        pi += operation * (numerator / denominator);
        denominator += 2.0;
        operation *= -1.0;
    }
    pi
}

fn main() {
    let start = Instant::now();
    let pi = calculate_pi(N);
    let duration = start.elapsed();
    println!("π = {}", pi);
    println!("Time elapsed: {:.2} seconds!", duration.as_secs_f64());
}
