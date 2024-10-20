use clap::Parser;
use num_bigint::BigUint;
use num_traits::One;
use std::time::Duration;
use std::time::Instant;

#[derive(Parser)]
struct Cli {
    /// How many iterations steps to take (optional)
    steps: Option<usize>,
}

fn main() {
    // Parse the command line arguments for the number of steps
    let cli: Cli = Cli::parse();

    // Calculate the fibonacci sequence
    let (result, iterations, elapsed) = match cli.steps {
        Some(steps) => calculate_fib_steps(steps),
        None => calculate_fib_timed(Duration::from_secs(1)),
    };

    // Print the result
    println!("Result: {}", result);
    println!("Iterations: {}", iterations);
    println!("Elapsed: {:.2?}", elapsed);
}

fn calculate_fib_steps(iterations: usize) -> (BigUint, usize, Duration) {
    // Start a timer
    let now = Instant::now();

    // Initialize the first two values in the sequence
    let mut f0 = BigUint::ZERO;
    let mut f1 = BigUint::one();

    for _ in 1..iterations {
        // Find the next value in the sequence
        let n = f0 + &f1;

        // Update the values
        f0 = f1;
        f1 = n;
    }

    // Stop the timer
    let elapsed = now.elapsed();

    (f1, iterations, elapsed)
}

fn calculate_fib_timed(duration: Duration) -> (BigUint, usize, Duration) {
    let mut iterations = 0;

    // Start a timer
    let start = Instant::now();

    // Initialize the first two values in the sequence
    let mut f0 = BigUint::ZERO;
    let mut f1 = BigUint::one();

    while start.elapsed() < duration {
        // Find the next value in the sequence
        let n = f0 + &f1;

        // Update the values
        f0 = f1;
        f1 = n;
        iterations += 1;
    }

    (f1, iterations, start.elapsed())
}
