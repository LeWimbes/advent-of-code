//! Macros for the `Solution` trait from `aoc-utils`.

use aoc_cache::{get_example_input_files, get_input_file};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::token::Comma;
use syn::{Ident, LitInt, parse_macro_input};

/// Parsed macro input: year, day, `SolutionType`
struct YearDaySolution {
    year: u32,
    day: u32,
    solution_type: Ident,
}

impl Parse for YearDaySolution {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let year_lit: LitInt = input.parse()?;
        let _comma1: Comma = input.parse()?;
        let day_lit: LitInt = input.parse()?;
        let _comma2: Comma = input.parse()?;
        let solution_type: Ident = input.parse()?;

        Ok(YearDaySolution {
            year: year_lit.base10_parse()?,
            day: day_lit.base10_parse()?,
            solution_type,
        })
    }
}

fn compile_error(msg: impl std::fmt::Display) -> TokenStream {
    let msg = msg.to_string();
    quote! { compile_error!(#msg) }
}

/// Generate a main function that runs the solution and prints both parts.
///
/// # Example
///
/// ```ignore
/// struct Day01;
/// impl Solution for Day01 { /* ... */ }
///
/// aoc_utils::run!(2015, 1, Day01);
/// ```
#[proc_macro]
pub fn run(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let YearDaySolution {
        year,
        day,
        solution_type,
    } = parse_macro_input!(input as YearDaySolution);

    let file_path = match get_input_file(year, day) {
        Ok(path) => path,
        Err(e) => return compile_error(&e).into(),
    };

    quote! {
        fn main() {
            const INPUT: &str = include_str!(#file_path);
            let (part1, part2) = <#solution_type as ::aoc_utils::Solution>::solve(INPUT);
            println!("Part1: {part1}");
            println!("Part2: {part2}");
        }
    }
    .into()
}

/// Generate rstest fixtures for example inputs and an answer regression test.
///
/// Creates fixtures named `data1`, `data2`, etc. that return `<T as Solution>::Input`.
/// Also generates a `test_answers` test (gated by `#[cfg(feature = "test-answers")]`)
/// that validates solutions against submitted answers.
///
/// # Example
///
/// ```ignore
/// #[cfg(test)]
/// mod tests {
///     use super::*;
///
///     aoc_utils::solution_tests!(2015, 1, Day01);
///
///     #[rstest]
///     fn part1_example(data1: <Day01 as Solution>::Input) {
///         assert_eq!(Day01::part1(&data1), 42);
///     }
/// }
/// ```
#[proc_macro]
pub fn solution_tests(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let YearDaySolution {
        year,
        day,
        solution_type,
    } = parse_macro_input!(input as YearDaySolution);

    let file_path = match get_input_file(year, day) {
        Ok(path) => path,
        Err(e) => return compile_error(&e).into(),
    };

    let example_files = match get_example_input_files(year, day) {
        Ok(files) => files,
        Err(e) => return compile_error(&e).into(),
    };

    let fixtures = example_files.iter().enumerate().map(|(idx, file_path)| {
        let fixture_name = format_ident!("data{}", idx + 1);
        quote! {
            #[::rstest::fixture]
            fn #fixture_name() -> <#solution_type as ::aoc_utils::Solution>::Input<'static> {
                const INPUT: &str = include_str!(#file_path);
                <#solution_type as ::aoc_utils::Solution>::process(INPUT)
            }
        }
    });

    let has_part2 = (year <= 2024 && day != 25) || (year >= 2025 && day != 12);

    let part2_test = if has_part2 {
        quote! {
            assert!(answers.1.is_some(), "The answer for part 2 hasn't been submitted yet.");
            if let Some(answer) = answers.1 {
                let result = <#solution_type as ::aoc_utils::Solution>::part2(&data);
                assert_eq!(answer, ::std::string::ToString::to_string(&result), "Part 2 is incorrect.");
            }
        }
    } else {
        quote! {
            assert!(answers.1.is_none(), "An answer has been submitted for part 2, but the last day shouldn't have one.");
        }
    };

    quote! {
        #(#fixtures)*

        #[cfg(feature = "test-answers")]
        #[::rstest::rstest]
        fn test_answers() {
            let answers = ::aoc_utils::get_answers(#year, #day);
            assert!(answers.is_ok(), "{}", answers.unwrap_err());
            let answers = answers.unwrap();

            const INPUT: &str = include_str!(#file_path);
            let data = <#solution_type as ::aoc_utils::Solution>::process(INPUT);

            assert!(answers.0.is_some(), "The answer for part 1 hasn't been submitted yet.");
            if let Some(answer) = answers.0 {
                let result = <#solution_type as ::aoc_utils::Solution>::part1(&data);
                assert_eq!(answer, ::std::string::ToString::to_string(&result), "Part 1 is incorrect.");
            }

            #part2_test
        }
    }
    .into()
}
