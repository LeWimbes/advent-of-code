//! Macros for the `Solution` trait from `aoc-utils`.

use aoc_cache::{get_example_files, get_input_file, is_last_day};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Ident, LitInt, Token, bracketed, parse_macro_input};

/// Parsed macro input: year, day, `SolutionType`
struct YearDaySolution {
    year: u32,
    day: u32,
    solution_type: Ident,
}

impl Parse for YearDaySolution {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let year_lit: LitInt = input.parse()?;
        input.parse::<Token![,]>()?;
        let day_lit: LitInt = input.parse()?;
        input.parse::<Token![,]>()?;
        let solution_type: Ident = input.parse()?;

        Ok(YearDaySolution {
            year: year_lit.base10_parse()?,
            day: day_lit.base10_parse()?,
            solution_type,
        })
    }
}

/// Parsed macro input: year, day, `SolutionType`, and optionally `examples = [...]`
struct SolutionTestsInput {
    year_day_solution: YearDaySolution,
    examples: Vec<usize>,
    examples_span: Span,
}

impl Parse for SolutionTestsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let year_day_solution = input.parse()?;
        let mut examples = Vec::new();
        let mut examples_span = Span::call_site();

        if input.parse::<Option<Token![,]>>()?.is_some() && !input.is_empty() {
            let key: Ident = input.parse()?;
            if key != "examples" {
                return Err(syn::Error::new(key.span(), "expected `examples = [...]`"));
            }
            input.parse::<Token![=]>()?;

            let content;
            let brackets = bracketed!(content in input);
            examples_span = brackets.span.join();
            for lit in Punctuated::<LitInt, Token![,]>::parse_terminated(&content)? {
                let index: usize = lit.base10_parse()?;
                if index == 0 {
                    return Err(syn::Error::new(lit.span(), "example indices start at 1"));
                }
                examples.push(index);
            }
            input.parse::<Option<Token![,]>>()?;
        }

        Ok(SolutionTestsInput {
            year_day_solution,
            examples,
            examples_span,
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
            let data = <#solution_type as ::aoc_utils::Solution>::process(INPUT);
            println!("Part1: {}", <#solution_type as ::aoc_utils::Solution>::part1(&data));
            println!("Part2: {}", <#solution_type as ::aoc_utils::Solution>::part2(&data));
        }
    }
    .into()
}

/// Generate rstest fixtures for example inputs and an answer regression test.
///
/// `examples = [...]` selects code blocks from the puzzle page by their position (starting at 1),
/// and creates fixtures named `data1`, `data2`, etc. for them, in the given order.
/// They return `<T as Solution>::Input`.
/// All code blocks are cached in `cache/<year>/day<day>/examples/`,
/// which helps finding the right positions.
///
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
///     // `data1` is the 2nd and `data2` the 5th code block on the puzzle page
///     aoc_utils::solution_tests!(2015, 1, Day01, examples = [2, 5]);
///
///     #[rstest]
///     fn part1_example(data1: <Day01 as Solution>::Input) {
///         assert_eq!(Day01::part1(&data1), 42);
///     }
/// }
/// ```
#[proc_macro]
pub fn solution_tests(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let SolutionTestsInput {
        year_day_solution:
            YearDaySolution {
                year,
                day,
                solution_type,
            },
        examples,
        examples_span,
    } = parse_macro_input!(input as SolutionTestsInput);

    let file_path = match get_input_file(year, day) {
        Ok(path) => path,
        Err(e) => return compile_error(&e).into(),
    };

    let example_files = if examples.is_empty() {
        Vec::new()
    } else {
        match get_example_files(year, day, &examples) {
            Ok(files) => files,
            Err(e) => return syn::Error::new(examples_span, e).to_compile_error().into(),
        }
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

    let part2_test = if is_last_day(year, day) {
        quote! {
            assert!(answers.1.is_none(), "An answer has been submitted for part 2, but the last day shouldn't have one.");
        }
    } else {
        quote! {
            assert!(answers.1.is_some(), "The answer for part 2 hasn't been submitted yet.");
            if let Some(answer) = answers.1 {
                let result = <#solution_type as ::aoc_utils::Solution>::part2(&data);
                assert_eq!(answer, ::std::string::ToString::to_string(&result), "Part 2 is incorrect.");
            }
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
