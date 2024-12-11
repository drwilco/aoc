use std::iter::{once, repeat_n};

use proc_macro::{TokenStream, TokenTree};

/// # Panics
/// * If the first argument is empty
/// * If the first argument only has a single token
/// * If the second argument is not a number
#[proc_macro]
pub fn repeat_methods(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();
    let mut first_argument = tokens
        .by_ref()
        .take_while(|token| !matches!(token, TokenTree::Punct(p) if p.as_char() == ','));
    let Some(call_site) = first_argument.next() else {
        panic!("Expected at least one argument");
    };
    let method_chain = first_argument.collect::<TokenStream>();
    assert!(!method_chain.is_empty(), "Expected at least one token behind the call site");
    let count = tokens.collect::<TokenStream>();
    let count: usize = syn::parse::<syn::LitInt>(count)
        .and_then(|lit| lit.base10_parse())
        .expect("Expected a number");
    let repeated_method_chain = repeat_n(method_chain.into_iter(), count).flatten();
    once(call_site)
        .chain(repeated_method_chain)
        .collect::<TokenStream>()
}
