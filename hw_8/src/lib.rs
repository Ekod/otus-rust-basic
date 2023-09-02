use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::token::Comma;

#[proc_macro]
pub fn proc_fn_runner(input: TokenStream) -> TokenStream {
    let funcs: Vec<_> = input
        .into_iter()
        .map(|fn_name| fn_name.to_string().replace('\"', ""))
        .filter(|fn_name| fn_name.to_string().len() % 2 == 0)
        .map(|fn_name| Ident::new(&fn_name, Span::call_site()))
        .collect();

    let mut comma: Option<Comma> = None;

    if funcs.len() > 1 {comma = Some(Comma::default())}

    TokenStream::from(quote!{
        (#(#funcs()#comma)*)
    })
}
