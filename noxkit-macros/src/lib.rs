use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, Ident, Token, parse::{Parse, ParseStream}, braced};
use syn::punctuated::Punctuated;

enum ViewElement {
    Widget {
        name: Ident,
        children: Vec<ViewElement>,
    },
    Expr(Expr),
}

impl Parse for ViewElement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Ident) && input.peek2(syn::token::Brace) {
            let name: Ident = input.parse()?;
            let content;
            braced!(content in input);
            let children = content.parse_terminated(ViewElement::parse, Token![,])?;
            Ok(ViewElement::Widget {
                name,
                children: children.into_iter().collect(),
            })
        } else {
            let expr: Expr = input.parse()?;
            Ok(ViewElement::Expr(expr))
        }
    }
}

#[proc_macro]
pub fn view(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ViewElement);
    let expanded = expand_view_element(&input);
    TokenStream::from(expanded)
}

fn expand_view_element(element: &ViewElement) -> proc_macro2::TokenStream {
    match element {
        ViewElement::Widget { name, children } => {
            let expanded_children = children.iter().map(|child| {
                let expanded = expand_view_element(child);
                quote! { Box::new(#expanded) as Box<dyn noxkit::view::View> }
            });
            quote! {
                noxkit::widgets::#name::new(vec![#(#expanded_children),*])
            }
        }
        ViewElement::Expr(expr) => {
            quote! { #expr }
        }
    }
}
