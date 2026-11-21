# Advent of Code

I've been enjoying [Advent of Code](https://adventofcode.com/) since 2020, so I decided to structure my solutions and share them here.

In 2020 and 2021, I solved the puzzles in Kotlin as it was my primary language at the time.
In 2022, I decided to start learning Rust using Advent of Code and have continued to use it ever since.

For now, only some of my solutions are available here, but I plan to add more in the future.
Maybe in Rust, maybe in Kotlin, maybe in another language.
We'll see!

## Repository Layout

Everything lives in a single Cargo workspace:

```text
.
├── 2023/day01/ … 2025/day12/  # One binary crate per puzzle, named e.g. `y2025-d05`
├── crates/
│   ├── aoc-utils/             # The `Solution` trait, re-exports the macros and cache
│   ├── aoc-macros/            # `run!` and `solution_tests!`
│   └── aoc-cache/             # Fetches and caches inputs, examples and answers
├── template/                  # Template used by `just new-day`
└── cache/                     # Puzzle data fetched from the website (not committed)
```

## Requirements

- [Rust](https://rustup.rs/) (stable)
- [just](https://github.com/casey/just) to run the commands below
- [prek](https://github.com/j178/prek) for the Git hooks
- [uv](https://docs.astral.sh/uv/), used by `just lint` and `just fix` to run [tombi](https://tombi-toml.github.io/tombi/) and [typos](https://github.com/crate-ci/typos)
- Firefox, logged in to adventofcode.com (see [Puzzle Data](#puzzle-data))

## Usage

The day and year default to today's puzzle, based on New York time since that's when puzzles unlock.
Both can be passed explicitly, e.g. `just test-day 5 2025`.
Any further arguments are passed on to cargo.

| Command                              | Description                                                              |
| ------------------------------------ | ------------------------------------------------------------------------ |
| `just new-day [day] [year]`          | Create a new day from the template                                       |
| `just run-day [day] [year]`          | Run a day's solution                                                     |
| `just test-day [day] [year]`         | Run a day's tests against the puzzle's examples                          |
| `just test-day-answers [day] [year]` | Also check the solution against my submitted answers                     |
| `just test` / `just test-answers`    | The same for every day                                                   |
| `just lint`                          | Check formatting, clippy, TOML files and typos without changing anything |
| `just fix`                           | Apply clippy, rustfmt and TOML formatting fixes                          |

Run `just` to list all commands.

## Writing a Solution

Each day implements the `Solution` trait from `aoc-utils`.
`process` parses the input once, and both parts work on the result:

```rust
use aoc_utils::Solution;

struct Day05;

impl Solution for Day05 {
    type Input<'a> = Vec<&'a str>;
    type Output1 = u64;
    type Output2 = u64;

    fn process(input: &str) -> Self::Input<'_> { /* … */ }
    fn part1(input: &Self::Input<'_>) -> Self::Output1 { /* … */ }
    fn part2(input: &Self::Input<'_>) -> Self::Output2 { /* … */ }
}

aoc_utils::run!(2025, 5, Day05);
```

`run!` generates the `main` function.
In the test module, `solution_tests!` generates [rstest](https://github.com/la10736/rstest) fixtures for example inputs and a `test_answers` test that is enabled by the `test-answers` feature:

```rust
aoc_utils::solution_tests!(2025, 5, Day05, examples = [1, 3]);
```

`examples` selects code blocks on the puzzle page by their position, starting at 1.
Here, `data1` is the first and `data2` the third code block.
All code blocks are cached in `cache/<year>/day<day>/examples/`, which makes it easy to find the right positions.

## Puzzle Data

The [FAQ prohibits sharing puzzle text and inputs](https://adventofcode.com/about#faq_copying), so none of it is committed.
Instead, `aoc-cache` fetches each day's input and puzzle page from the website and stores them in `cache/<year>/day<day>/`.
The example inputs and my submitted answers are read from the puzzle page.

Fetching happens at compile time, inside the macros.
Building a day for the first time, or opening it in an editor with rust-analyzer, downloads its data.
Authentication uses the session cookie from Firefox, so you need to be logged in there.
Each file is only fetched once.
The puzzle page is fetched again only when a test needs a code block or answer it doesn't contain yet, i.e. after solving a part.
All requests are at least one second apart.

## Development

Install the Git hooks once:

```sh
prek install
```

They check whitespace and line endings, formatting (rustfmt, tombi), typos, clippy with warnings denied, and run the tests.
`just prek` runs them on all files.
