use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn tokio_runtime(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(item as ItemFn);

    if input_fn.sig.asyncness.is_none() {
        return syn::Error::new(
            input_fn.sig.span(),
            "El atributo #[tokio_runtime] solo puede aplicarse a funciones 'async'",
        )
        .to_compile_error()
        .into();
    }

    let original_block = &input_fn.block;

    input_fn.block = syn::parse_quote!({
        crate::runtime::enter(async move #original_block).await
    });

    TokenStream::from(quote!(#input_fn))
}
