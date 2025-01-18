use aoc_utils::get_input_file;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::token::Comma;
use syn::{parse_macro_input, LitBool, LitInt};

struct YearDayLit {
    year_lit: LitInt,
    day_lit: LitInt,
}

impl Parse for YearDayLit {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let year = input.parse()?;
        let _comma: Comma = input.parse()?;
        let day = input.parse()?;
        Ok(YearDayLit {
            year_lit: year,
            day_lit: day,
        })
    }
}

impl TryInto<(u32, u32)> for YearDayLit {
    type Error = String;

    fn try_into(self) -> Result<(u32, u32), Self::Error> {
        let year: u32 = match self.year_lit.base10_parse() {
            Ok(y) => y,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        let day: u32 = match self.day_lit.base10_parse() {
            Ok(d) => d,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        Ok((year, day))
    }
}

#[proc_macro]
pub fn include_input(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let year_day_lit = parse_macro_input!(input as YearDayLit);

    let (year, day) = match year_day_lit.try_into() {
        Ok(year_day) => year_day,
        Err(msg) => {
            return quote! {
                compile_error!(#msg)
            }
            .into();
        }
    };

    let file_path = match get_input_file(year, day) {
        Ok(path) => path,
        Err(e) => {
            let msg = e.to_string();
            return quote! {
                compile_error!(#msg)
            }
            .into();
        }
    };

    quote! {
        (#year, #day, include_str!(#file_path))
    }
    .into()
}

#[proc_macro]
pub fn test_answers(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let has_part2 = parse_macro_input!(input as LitBool).value;

    let part2_test = if has_part2 {
        quote! {
            assert!(answers.1.is_some(), "The answer for part 2 hasn't been submitted yet.");
            if let Some(answer) = answers.1 {
                assert_eq!(answer, part2(&data).to_string(), "Part 2 is incorrect.");
            }
        }
    } else {
        quote! {
            assert!(answers.1.is_none(), "An answer has been submitted for part 2.");
        }
    };

    quote! {
        #[rstest]
        fn test_answers() {
            let answers = aoc_utils::get_answers(INPUT.0, INPUT.1);
            assert!(answers.is_ok(), "{}", answers.unwrap_err());
            let answers = answers.unwrap();

            let data = process_input(INPUT.2);
            assert!(answers.0.is_some(), "The answer for part 1 hasn't been submitted yet.");
            if let Some(answer) = answers.0 {
                assert_eq!(answer, part1(&data).to_string(), "Part 1 is incorrect.");
            }
            #part2_test
        }
    }
    .into()
}
