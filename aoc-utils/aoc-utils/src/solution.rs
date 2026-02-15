use std::fmt::Display;

/// A solution for a single Advent of Code day.
///
/// This trait defines the contract that all `AoC` solutions must follow.
pub trait Solution {
    /// The processed input type that both parts work with.
    type Input<'a>;

    /// The output type for part 1. Must implement `Display` for answer comparison.
    type Output1: Display;

    /// The output type for part 2. Must implement `Display` for answer comparison.
    type Output2: Display;

    /// Process the raw input string into the working format.
    fn process(input: &str) -> Self::Input<'_>;

    /// Solve part 1 of the puzzle.
    fn part1(input: &Self::Input<'_>) -> Self::Output1;

    /// Solve part 2 of the puzzle.
    fn part2(input: &Self::Input<'_>) -> Self::Output2;

    /// Run both parts and return their results.
    #[must_use]
    fn solve(input: &str) -> (Self::Output1, Self::Output2) {
        let data = Self::process(input);
        (Self::part1(&data), Self::part2(&data))
    }
}
