#[cfg(feature = "anchor")]
use {anyhow::Result, proc_macro2::TokenStream, quote::quote, syn::ItemFn};

#[cfg(feature = "anchor")]
use crate::program::remove_verify_ignore_statements;

#[cfg(feature = "anchor")]
pub fn helper_fn(input: TokenStream) -> Result<TokenStream> {
    let Ok(mut item) = syn::parse2::<ItemFn>(input) else {
        panic!("use #[helper_fn] on a function")
    };
    remove_verify_ignore_statements(&mut item);

    Ok(quote! {
        #item
    })
}
