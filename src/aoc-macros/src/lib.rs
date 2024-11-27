use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Lit};
use syn::punctuated::Punctuated;
use syn::token::Comma;

/// Marks a function as an Advent of Code solver
///
/// # Example
/// ```ignore
/// #[advent_of_code(2024, 1, 1)]
/// pub fn solve_level1(input: &[&str]) -> String {
///     // solution implementation
/// }
/// ```
#[proc_macro_attribute]
pub fn advent_of_code(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args with Punctuated::<Lit, Comma>::parse_terminated);
    let input_fn = parse_macro_input!(input as ItemFn);
    
    let (year, day, level) = parse_args(&args);
    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_block = &input_fn.block;
    
    let expanded = quote! {
        #[doc(hidden)]
        #fn_vis fn #fn_name(input: &[&str]) -> String #fn_block

        inventory::submit! {
            crate::solver::SolverInfo {
                year: #year,
                day: #day,
                level: #level,
                name: stringify!(#fn_name),
                func: #fn_name,
            }
        }
    };

    TokenStream::from(expanded)
}

fn parse_args(args: &Punctuated<Lit, Comma>) -> (u16, u8, u8) {
    if args.len() != 3 {
        panic!("advent_of_code attribute requires exactly 3 arguments: year, day, and level");
    }

    let year = match &args[0] {
        Lit::Int(lit) => lit.base10_parse::<u16>().unwrap(),
        _ => panic!("year must be a literal integer"),
    };

    let day = match &args[1] {
        Lit::Int(lit) => lit.base10_parse::<u8>().unwrap(),
        _ => panic!("day must be a literal integer"),
    };

    let level = match &args[2] {
        Lit::Int(lit) => lit.base10_parse::<u8>().unwrap(),
        _ => panic!("level must be a literal integer"),
    };

    (year, day, level)
} 