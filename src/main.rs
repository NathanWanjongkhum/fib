use clap::Parser;
use num_bigint::BigUint;
use num_traits::One;

#[derive(Parser)]
struct Cli {
    /// How many iterations steps to take
    steps: usize,
}

fn main() {
    // Parse the command line arguments for the number of steps
    let steps = Cli::parse().steps;

    // Start a timer
    use std::time::Instant;
    let now = Instant::now();

    // Calculate the fibonacci sequence
    let result = calculate_fib(steps);

    // Stop the timer
    let elapsed = now.elapsed();

    // Print the result
    println!("f_{:?} = {}", steps, result);
    println!("Elapsed: {:.2?}", elapsed);
}

fn calculate_fib(n: usize) -> BigUint {
    // Initialize the first two values in the sequence
    let mut f0 = BigUint::ZERO;
    let mut f1 = BigUint::one();

    for _ in 1..n {
        // Find the next value in the sequence
        let n = f0 + &f1;

        // Update the values
        f0 = f1;
        f1 = n;
    }

    f1
}
