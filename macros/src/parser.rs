extern crate proc_macro;

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, TokenStream, TokenTree};

pub fn parse(token: TokenStream) -> Vec<TokenTree> {
    let mut peekable = token.clone().into_iter().peekable();
    let mut tokens: Vec<TokenTree> = vec![];
    let iter = token.into_iter();

    for token in iter {
        peekable.next();

        if tokens.len() > 1
            && tokens.last().unwrap().to_string().as_str() == "="
            && token.to_string().as_str() == "-"
        {
            continue;
        }

        match token {
            TokenTree::Group(token) => {
                if token.delimiter() == Delimiter::Bracket {
                    tokens.push(TokenTree::Group(Group::new(
                        Delimiter::Bracket,
                        token.stream(),
                    )));
                    parse(token.stream());
                } else {
                    tokens.push(TokenTree::Group(Group::new(
                        Delimiter::Parenthesis,
                        peekable.clone().collect(),
                    )));
                    parse(token.stream());
                }
            }

            TokenTree::Literal(token) => {
                if token.to_string().contains('"') {
                    tokens.push(TokenTree::Literal(Literal::string(&token.to_string())));
                } else {
                    let float: f64 = token.to_string().parse().unwrap();
                    tokens.push(TokenTree::Literal(Literal::f64_unsuffixed(float)))
                }
            }

            TokenTree::Punct(token) => match token.to_string().as_str() {
                "+" => tokens.push(TokenTree::Punct(Punct::new('+', Spacing::Alone))),
                "-" => tokens.push(TokenTree::Punct(Punct::new('-', Spacing::Alone))),
                "/" => tokens.push(TokenTree::Punct(Punct::new('/', Spacing::Alone))),
                "*" => tokens.push(TokenTree::Punct(Punct::new('*', Spacing::Alone))),
                "%" => tokens.push(TokenTree::Punct(Punct::new('%', Spacing::Alone))),
                "<" => {
                    if peekable.peek().unwrap().to_string().as_str() == "-" {
                        tokens.push(TokenTree::Punct(Punct::new('=', Spacing::Alone)));
                    } else {
                        tokens.push(TokenTree::Punct(Punct::new('<', Spacing::Alone)))
                    }
                }
                ";" => tokens.push(TokenTree::Punct(Punct::new(';', Spacing::Alone))),
                _ => tokens.push(TokenTree::Punct(token)),
            },

            TokenTree::Ident(token) => match token.to_string().as_str() {
                "print" => {
                    tokens.push(TokenTree::Ident(Ident::new("print", token.span())));
                    tokens.push(TokenTree::Punct(Punct::new('!', Spacing::Alone)))
                }
                _ => {
                    tokens.push(TokenTree::Ident(Ident::new("let", token.span())));
                    tokens.push(TokenTree::Ident(token));
                }
            },
        }
    }

    //for token in tokens.clone().into_iter() {
    //    eprintln!("Final: {}", token)
    //}
    return tokens;
}
