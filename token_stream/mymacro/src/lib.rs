use proc_macro2::{TokenStream, TokenTree};
use quote::quote;
use std::iter::FromIterator;

#[proc_macro]
pub fn macro1(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();

    let mut ret: Vec<TokenTree> = Vec::new();
    let mut plus = Vec::new();
    let mut minus = Vec::new();
    let mut ident = None;
    let mut plus_idents = Vec::new();
    let mut minus_idents = Vec::new();

    for token in input {
        match &token {
            TokenTree::Punct(x) if x.as_char() == '+' => plus.push(x.clone()),
            TokenTree::Punct(x) if x.as_char() == '-' => minus.push(x.clone()),
            x => {
                for x in &plus {
                    ret.push(TokenTree::Punct(x.clone()));
                }
                for x in &minus {
                    ret.push(TokenTree::Punct(x.clone()));
                }
                plus.clear();
                minus.clear();
                ret.push(x.clone());
                if let TokenTree::Ident(x) = x {
                    ident = Some(x.clone());
                }
            }
        }
        if plus.len() == 2 {
            plus.clear();
            if let Some(ref x) = ident {
                plus_idents.push(TokenTree::Ident(x.clone()));
            }
        }
        if minus.len() == 2 {
            minus.clear();
            if let Some(ref x) = ident {
                minus_idents.push(TokenTree::Ident(x.clone()));
            }
        }
    }
    let ret: TokenStream = TokenStream::from_iter(ret.into_iter()).into();
    let plus_idents = plus_idents.into_iter();
    let minus_idents = minus_idents.into_iter();

    let gen = quote! {
        {
            let tmp = #ret;
            #(
                #plus_idents += 1;
            )*
            #(
                #minus_idents -= 1;
            )*
            tmp
        }
    };

    gen.into()
}