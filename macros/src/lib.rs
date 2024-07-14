use parser::parse;
use proc_macro::TokenStream;
mod parser;
#[proc_macro]
pub fn r(input: TokenStream) -> TokenStream {
    parse(input);

    TokenStream::new()

}