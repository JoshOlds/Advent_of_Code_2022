use std::time::Instant;

pub mod input_helpers;

/// Helper function to measure execution time (in microseconds) and print to screen
/// The supplied function is intended to accept a bool that specifies whether this iteration of the function should print or not
pub fn measure_execution_time_us(func: fn(bool), iterations: u32) {
    println!("Measuring {} iterations...", iterations);
    let start = Instant::now();
    for _x in 0..iterations - 1{
        func(false);
    }
    func(true);
    let elapsed = (Instant::now() - start) / iterations;
    println!("Elapsed time microseconds (over {} iterations): {}",iterations, elapsed.as_micros());
}

/// Helper function to measure execution time (in nanoseconds) and print to screen
/// The supplied function is intended to accept a bool that specifies whether this iteration of the function should print or not
pub fn measure_execution_time_ns(func: fn(bool), iterations: u32) {
    println!("Measuring {} iterations...", iterations);
    let start = Instant::now();
    for _x in 0..iterations - 1{
        func(false);
    }
    func(true);
    let elapsed = (Instant::now() - start) / iterations;
    println!("Elapsed time nanoseconds (over {} iterations): {}",iterations, elapsed.as_nanos());
}