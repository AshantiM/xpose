use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::ToTokens;
use syn::{spanned::Spanned, ItemFn, Token, VisPublic, Visibility};

struct Xpose {
    fn_span: Span,
}

#[proc_macro_attribute]
pub fn xpose_me(_attr: TokenStream, item: TokenStream) -> TokenStream {
    if cfg!(xpose_on)
    {
        let input = syn::parse_macro_input!(item as syn::ItemFn);
        let mut fold: Box<dyn syn::fold::Fold> = Box::new(Xpose {
            fn_span: input.vis.span(),
        });
        let tokens = fold.fold_item_fn(input);
        tokens.to_token_stream().into()
    }
    else
    {
        item
    }
}

impl syn::fold::Fold for Xpose {
    fn fold_item_fn(&mut self, function: ItemFn) -> syn::ItemFn {
        let mut new_function = function.clone();
        new_function.vis = Visibility::Public(VisPublic {
            pub_token: Token![pub](self.fn_span),
        });
        function
    }
}
