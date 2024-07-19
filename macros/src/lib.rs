use parser::parse;
use proc_macro::TokenStream;
mod parser;
#[proc_macro]
pub fn r(input: TokenStream) -> TokenStream {
    let parsed = parse(input);

    let mut new = TokenStream::new();

    new.extend(parsed.into_iter());
    return new;
}
