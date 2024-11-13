use clap::Parser;

mod fibonacci_generator;

fn main() {
    let generator = fibonacci_generator::FibonacciGenerator::parse();
    generator.generate();
}
