use proc_macro::{TokenStream, TokenTree};

pub fn parse(token: TokenStream) -> () {
    for token in token.into_iter() {
        match token {
            TokenTree::Group(token) => {
                eprintln!("Group: {}", token);
                parse(token.stream())
            }
            
            TokenTree::Literal(_) => {
                eprintln!("Literal: {}", token);
            }
            
            TokenTree::Punct(_) => {
                eprintln!("Punct: {}", token);
            }
            
            TokenTree::Ident(_) => {
                eprintln!("Ident: {}", token);
            }
            
        }
    }

}