use anyhow::Result;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse2, ItemEnum};

pub fn error_code(_args: TokenStream, item: TokenStream) -> Result<TokenStream> {
    let mut val: ItemEnum = parse2(item.clone())?;
    for variant in val.variants.iter_mut() {
        variant.attrs.retain(|attr| !attr.path.is_ident("msg"));
    }

    let filtered_item = val.clone().into_token_stream();

    let ident = val.ident;
    let generics = val.generics;

    let res = quote! {
        #[derive(Debug, thiserror::Error)]
        #filtered_item

        impl #generics std::fmt::Display for #ident #generics {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                <Self as std::fmt::Debug>::fmt(self, f)
            }
        }

        impl From<#ident> for anchor_lang::prelude::Error {
            fn from(value: #ident) -> Self {
                anchor_lang::prelude::Error::CustomError(value.to_string())
            }
        }
    };
    Ok(res)
}
