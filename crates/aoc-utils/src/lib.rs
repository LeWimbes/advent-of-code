//! Advent of Code utility library with trait-based solution framework.

mod solution;

// Re-export cache utilities and error types
pub use aoc_cache::{CacheError, get_answers, get_example_input_files, get_input_file};

// Re-export macros
pub use aoc_macros::{run, solution_tests};

// Export the Solution trait
pub use solution::Solution;
