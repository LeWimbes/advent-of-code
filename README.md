# Advent of Code

I've been enjoying [Advent of Code](https://adventofcode.com/) since 2020, so I decided to structure my solutions and share them here.

In 2020 and 2021, I solved the puzzles in Kotlin as it was my primary language at the time. In 2022, I decided to start learning Rust using Advent of Code and have continued to use it ever since.

To keep the code clean and avoid boilerplate, I created a small utility library called `aoc-utils` that standardizes the main part of the code using a trait. It provides macros to generate the main function and test fixtures for running solutions.

To avoid including parts of the puzzles in this repository (the [FAQ explicitly prohibits sharing puzzle text and inputs](https://adventofcode.com/about#faq_copying)), `aoc-utils` fetches relevant data from the Advent of Code website and caches it in `cache` directories. It authenticates using your Firefox session cookie, so a logged-in Firefox browser is required. Sometimes automatic parsing of the example inputs from the puzzle description doesn't work perfectly. In these cases, I've marked the affected tests with a comment and adjusted the cache manually.

For now, only some of my solutions are available here, but I plan to add more in the future. Maybe in Rust, maybe in Kotlin, maybe in another language. We'll see!
